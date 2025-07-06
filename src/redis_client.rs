use crate::config::RedisConfig;
use anyhow::{anyhow, Result};
use colored::*;
use redis::{aio::Connection, AsyncCommands, Value};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::time::Duration;

pub struct RedisClient {
    pub connection: Connection,
}

impl RedisClient {
    pub async fn connect(config: RedisConfig) -> Result<Self> {
        let client = redis::Client::open(config.to_redis_url())?;
        let connection = client.get_async_connection().await?;

        Ok(Self { connection })
    }

    pub async fn ping(&mut self) -> Result<String> {
        let result: String = redis::cmd("PING").query_async(&mut self.connection).await?;
        Ok(result)
    }

    pub async fn info(&mut self) -> Result<HashMap<String, String>> {
        let result: String = redis::cmd("INFO").query_async(&mut self.connection).await?;

        let mut info_map = HashMap::new();
        for line in result.lines() {
            if let Some((key, value)) = line.split_once(':') {
                info_map.insert(key.to_string(), value.to_string());
            }
        }

        Ok(info_map)
    }

    #[allow(dead_code)]
    pub async fn select_db(&mut self, db: u8) -> Result<()> {
        redis::cmd("SELECT")
            .arg(db)
            .query_async::<_, ()>(&mut self.connection)
            .await?;
        Ok(())
    }

    pub async fn keys(&mut self, pattern: &str) -> Result<Vec<String>> {
        let keys: Vec<String> = self.connection.keys(pattern).await?;
        Ok(keys)
    }

    pub async fn key_info(&mut self, key: &str) -> Result<KeyInfo> {
        let mut pipe = redis::pipe();
        pipe.atomic()
            .cmd("TYPE")
            .arg(key)
            .cmd("TTL")
            .arg(key)
            .cmd("MEMORY")
            .arg("USAGE")
            .arg(key)
            .cmd("OBJECT")
            .arg("ENCODING")
            .arg(key);

        let results: Vec<Value> = pipe.query_async(&mut self.connection).await?;

        let key_type = match &results[0] {
            Value::Data(ref data) => String::from_utf8_lossy(data).to_string(),
            Value::Status(ref status) => status.to_string(),
            _ => "unknown".to_string(),
        };

        let ttl = match &results[1] {
            Value::Int(ttl) => Some(*ttl),
            _ => None,
        };

        let memory_usage = match &results[2] {
            Value::Int(usage) => Some(*usage as usize),
            _ => None,
        };

        let encoding = match &results[3] {
            Value::Data(ref data) => String::from_utf8_lossy(data).to_string(),
            Value::Status(ref status) => status.to_string(),
            _ => "unknown".to_string(),
        };

        Ok(KeyInfo {
            key: key.to_string(),
            key_type,
            ttl,
            memory_usage,
            encoding,
        })
    }

    pub async fn get_string(&mut self, key: &str) -> Result<Option<String>> {
        let value: Option<String> = self.connection.get(key).await?;
        Ok(value)
    }

    pub async fn set_string(
        &mut self,
        key: &str,
        value: &str,
        ttl: Option<Duration>,
    ) -> Result<()> {
        if let Some(ttl) = ttl {
            redis::pipe()
                .cmd("SET")
                .arg(key)
                .arg(value)
                .ignore()
                .cmd("EXPIRE")
                .arg(key)
                .arg(ttl.as_secs() as usize)
                .query_async::<_, ()>(&mut self.connection)
                .await?;
        } else {
            redis::cmd("SET")
                .arg(key)
                .arg(value)
                .query_async::<_, ()>(&mut self.connection)
                .await?;
        }
        Ok(())
    }

    pub async fn get_hash(&mut self, key: &str) -> Result<HashMap<String, String>> {
        let hash: HashMap<String, String> = self.connection.hgetall(key).await?;
        Ok(hash)
    }

    pub async fn set_hash_field(&mut self, key: &str, field: &str, value: &str) -> Result<()> {
        redis::cmd("HSET")
            .arg(key)
            .arg(field)
            .arg(value)
            .query_async::<_, ()>(&mut self.connection)
            .await?;
        Ok(())
    }

    pub async fn get_list(&mut self, key: &str, start: isize, stop: isize) -> Result<Vec<String>> {
        let list: Vec<String> = self.connection.lrange(key, start, stop).await?;
        Ok(list)
    }

    pub async fn push_list(&mut self, key: &str, value: &str, left: bool) -> Result<usize> {
        let len = if left {
            self.connection.lpush(key, value).await?
        } else {
            self.connection.rpush(key, value).await?
        };
        Ok(len)
    }

    pub async fn get_set(&mut self, key: &str) -> Result<Vec<String>> {
        let set: Vec<String> = self.connection.smembers(key).await?;
        Ok(set)
    }

    pub async fn add_to_set(&mut self, key: &str, member: &str) -> Result<bool> {
        let added: i32 = self.connection.sadd(key, member).await?;
        Ok(added > 0)
    }

    pub async fn get_sorted_set(
        &mut self,
        key: &str,
        start: isize,
        stop: isize,
        with_scores: bool,
    ) -> Result<Vec<(String, f64)>> {
        if with_scores {
            let zset: Vec<(String, f64)> = self
                .connection
                .zrangebyscore_withscores(key, start, stop)
                .await?;
            Ok(zset)
        } else {
            let zset: Vec<String> = self.connection.zrange(key, start, stop).await?;
            Ok(zset.into_iter().map(|v| (v, 0.0)).collect())
        }
    }

    pub async fn add_to_sorted_set(&mut self, key: &str, member: &str, score: f64) -> Result<bool> {
        let added: i32 = self.connection.zadd(key, member, score).await?;
        Ok(added > 0)
    }

    pub async fn delete_key(&mut self, key: &str) -> Result<bool> {
        let deleted: i32 = self.connection.del(key).await?;
        Ok(deleted > 0)
    }

    pub async fn delete_keys_by_pattern(&mut self, pattern: &str) -> Result<usize> {
        let keys: Vec<String> = self.connection.keys(pattern).await?;
        let deleted: i32 = self.connection.del(keys).await?;
        Ok(deleted as usize)
    }

    pub async fn monitor(&mut self) -> Result<()> {
        println!("{}", "Monitor mode - press Ctrl+C to stop".yellow());
        println!(
            "{}",
            "Note: Full monitor implementation requires additional Redis client features".cyan()
        );
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            println!("{}", "Monitor: Waiting for commands...".green());
        }
    }

    pub async fn slowlog_get(&mut self, count: usize) -> Result<Vec<SlowLogEntry>> {
        let result: Vec<Value> = redis::cmd("SLOWLOG")
            .arg("GET")
            .arg(count)
            .query_async(&mut self.connection)
            .await?;

        let mut entries = Vec::new();
        for entry in result {
            if let Value::Bulk(items) = entry {
                if items.len() >= 4 {
                    let id = match &items[0] {
                        Value::Int(id) => *id,
                        _ => 0,
                    };

                    let timestamp = match &items[1] {
                        Value::Int(ts) => *ts,
                        _ => 0,
                    };

                    let duration = match &items[2] {
                        Value::Int(dur) => *dur,
                        _ => 0,
                    };

                    let command = match &items[3] {
                        Value::Bulk(cmd_items) => cmd_items
                            .iter()
                            .filter_map(|item| {
                                if let Value::Data(data) = item {
                                    Some(String::from_utf8_lossy(data).to_string())
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<_>>()
                            .join(" "),
                        _ => "unknown".to_string(),
                    };

                    entries.push(SlowLogEntry {
                        id,
                        timestamp,
                        duration,
                        command,
                    });
                }
            }
        }

        Ok(entries)
    }

    pub async fn client_list(&mut self) -> Result<Vec<ClientInfo>> {
        let result: String = redis::cmd("CLIENT")
            .arg("LIST")
            .query_async(&mut self.connection)
            .await?;

        let mut clients = Vec::new();
        for line in result.lines() {
            if let Ok(client) = ClientInfo::from_line(line) {
                clients.push(client);
            }
        }

        Ok(clients)
    }

    #[allow(dead_code)]
    pub async fn save(&mut self, background: bool) -> Result<()> {
        let cmd = if background { "BGSAVE" } else { "SAVE" };
        redis::cmd(cmd)
            .query_async::<_, ()>(&mut self.connection)
            .await?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn bgrewriteaof(&mut self) -> Result<()> {
        redis::cmd("BGREWRITEAOF")
            .query_async::<_, ()>(&mut self.connection)
            .await?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn subscribe(&mut self, channels: &[String]) -> Result<()> {
        // Simplified subscribe implementation
        println!(
            "{}",
            format!("Subscribing to channels: {:?}", channels).yellow()
        );
        println!(
            "{}",
            "Note: Full pub/sub implementation requires additional Redis client features".cyan()
        );

        // For now, just show a placeholder
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            println!("{}", "PubSub: Waiting for messages...".green());
        }
    }

    #[allow(dead_code)]
    pub async fn publish(&mut self, channel: &str, message: &str) -> Result<usize> {
        let result: usize = redis::cmd("PUBLISH")
            .arg(channel)
            .arg(message)
            .query_async(&mut self.connection)
            .await?;
        Ok(result)
    }

    #[allow(dead_code)]
    pub async fn cluster_nodes(&mut self) -> Result<Vec<ClusterNode>> {
        let result: String = redis::cmd("CLUSTER")
            .arg("NODES")
            .query_async(&mut self.connection)
            .await?;

        let mut nodes = Vec::new();
        for line in result.lines() {
            if let Ok(node) = ClusterNode::from_line(line) {
                nodes.push(node);
            }
        }

        Ok(nodes)
    }

    #[allow(dead_code)]
    pub async fn sentinel_masters(&mut self) -> Result<Vec<SentinelMaster>> {
        let result: Vec<Value> = redis::cmd("SENTINEL")
            .arg("MASTERS")
            .query_async(&mut self.connection)
            .await?;

        let mut masters = Vec::new();
        for master in result {
            if let Value::Bulk(items) = master {
                let mut master_info = SentinelMaster::default();
                for chunk in items.chunks(2) {
                    if chunk.len() == 2 {
                        if let (Value::Data(ref key), Value::Data(ref value)) =
                            (&chunk[0], &chunk[1])
                        {
                            let key = String::from_utf8_lossy(key);
                            let value = String::from_utf8_lossy(value);

                            match key.as_ref() {
                                "name" => master_info.name = value.to_string(),
                                "ip" => master_info.ip = value.to_string(),
                                "port" => master_info.port = value.parse().unwrap_or(0),
                                "flags" => master_info.flags = value.to_string(),
                                "num-slaves" => master_info.num_slaves = value.parse().unwrap_or(0),
                                "num-pending-sentinel" => {
                                    master_info.num_pending_sentinel = value.parse().unwrap_or(0)
                                }
                                "num-other-sentinels" => {
                                    master_info.num_other_sentinels = value.parse().unwrap_or(0)
                                }
                                "quorum" => master_info.quorum = value.parse().unwrap_or(0),
                                _ => {}
                            }
                        }
                    }
                }
                masters.push(master_info);
            }
        }

        Ok(masters)
    }

    pub fn pretty_print_json(&self, value: &str) -> Result<String> {
        if let Ok(json) = serde_json::from_str::<JsonValue>(value) {
            Ok(serde_json::to_string_pretty(&json)?)
        } else {
            Ok(value.to_string())
        }
    }
}

#[derive(Debug, Clone)]
pub struct KeyInfo {
    pub key: String,
    pub key_type: String,
    pub ttl: Option<i64>,
    pub memory_usage: Option<usize>,
    pub encoding: String,
}

#[derive(Debug, Clone)]
pub struct SlowLogEntry {
    pub id: i64,
    pub timestamp: i64,
    pub duration: i64,
    pub command: String,
}

#[derive(Debug, Clone)]
pub struct ClientInfo {
    pub id: String,
    pub addr: String,
    pub fd: String,
    pub name: String,
    pub age: String,
    pub idle: String,
    pub flags: String,
    pub db: String,
    pub sub: String,
    pub psub: String,
    pub multi: String,
    pub qbuf: String,
    pub qbuf_free: String,
    pub obl: String,
    pub oll: String,
    pub omem: String,
    pub events: String,
    pub cmd: String,
}

impl ClientInfo {
    fn from_line(line: &str) -> Result<Self> {
        let mut client = ClientInfo {
            id: String::new(),
            addr: String::new(),
            fd: String::new(),
            name: String::new(),
            age: String::new(),
            idle: String::new(),
            flags: String::new(),
            db: String::new(),
            sub: String::new(),
            psub: String::new(),
            multi: String::new(),
            qbuf: String::new(),
            qbuf_free: String::new(),
            obl: String::new(),
            oll: String::new(),
            omem: String::new(),
            events: String::new(),
            cmd: String::new(),
        };

        for pair in line.split(' ') {
            if let Some((key, value)) = pair.split_once('=') {
                match key {
                    "id" => client.id = value.to_string(),
                    "addr" => client.addr = value.to_string(),
                    "fd" => client.fd = value.to_string(),
                    "name" => client.name = value.to_string(),
                    "age" => client.age = value.to_string(),
                    "idle" => client.idle = value.to_string(),
                    "flags" => client.flags = value.to_string(),
                    "db" => client.db = value.to_string(),
                    "sub" => client.sub = value.to_string(),
                    "psub" => client.psub = value.to_string(),
                    "multi" => client.multi = value.to_string(),
                    "qbuf" => client.qbuf = value.to_string(),
                    "qbuf-free" => client.qbuf_free = value.to_string(),
                    "obl" => client.obl = value.to_string(),
                    "oll" => client.oll = value.to_string(),
                    "omem" => client.omem = value.to_string(),
                    "events" => client.events = value.to_string(),
                    "cmd" => client.cmd = value.to_string(),
                    _ => {}
                }
            }
        }

        Ok(client)
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ClusterNode {
    pub id: String,
    pub addr: String,
    pub flags: String,
    pub master: String,
    pub ping_sent: String,
    pub pong_recv: String,
    pub config_epoch: String,
    pub link_state: String,
    pub slots: Vec<String>,
}

#[allow(dead_code)]
impl ClusterNode {
    fn from_line(line: &str) -> Result<Self> {
        let parts: Vec<&str> = line.split(' ').collect();
        if parts.len() < 8 {
            return Err(anyhow!("Invalid cluster node line"));
        }

        let id = parts[0].to_string();
        let addr = parts[1].to_string();
        let flags = parts[2].to_string();
        let master = parts[3].to_string();
        let ping_sent = parts[4].to_string();
        let pong_recv = parts[5].to_string();
        let config_epoch = parts[6].to_string();
        let link_state = parts[7].to_string();
        let slots = parts[8..].iter().map(|s| s.to_string()).collect();

        Ok(ClusterNode {
            id,
            addr,
            flags,
            master,
            ping_sent,
            pong_recv,
            config_epoch,
            link_state,
            slots,
        })
    }
}

#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct SentinelMaster {
    pub name: String,
    pub ip: String,
    pub port: u16,
    pub flags: String,
    pub num_slaves: usize,
    pub num_pending_sentinel: usize,
    pub num_other_sentinels: usize,
    pub quorum: usize,
}
