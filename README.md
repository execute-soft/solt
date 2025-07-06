# Solt - Redis CLI Management Tool

A comprehensive Redis CLI management tool built in Rust that provides powerful Redis administration capabilities with a modern, user-friendly interface.

## Features

### 🔌 Connection & Configuration

- **Multi-environment support** (dev, staging, prod)
- **Connection management** with host, port, password, DB index
- **TLS support** for secure connections
- **Connection testing** and health checks
- **Configuration persistence** with TOML files

### 🔍 Key Inspection & Management

- **List keys** by pattern with detailed information
- **Key inspection** showing type, TTL, memory usage, encoding
- **Key counting** and statistics
- **Pattern-based operations**

### 📊 Value Viewing & Editing

- **Get values** for all Redis data types (strings, hashes, lists, sets, sorted sets)
- **Pretty-print JSON** values automatically
- **Set values** with TTL support
- **Hash field operations**
- **List operations** (push, range)
- **Set operations** (add members)
- **Sorted set operations** (add with scores)

### 🔎 Search & Filter

- **Pattern-based search** with wildcards
- **Filter by TTL** or key size
- **Type-based filtering**
- **Advanced search capabilities**

### ✏️ Editing & Writing

- **Set/update strings** with expiration
- **Hash field management**
- **List operations** (push left/right)
- **Set member management**
- **Sorted set score updates**

### 🗑️ Deletion & Cleanup

- **Delete individual keys**
- **Pattern-based deletion** with confirmation
- **Database flushing** (current/all)
- **Safe deletion** with preview

### 📦 Bulk Operations

- **Bulk delete** by pattern
- **Key renaming** operations
- **Copy keys** between databases
- **Dump/restore** functionality

### 📈 Monitoring & Debug

- **Real-time monitoring** (MONITOR)
- **Slow log analysis**
- **Client list** and connection info
- **Performance statistics**
- **Memory usage tracking**

### 💾 Backup & Export

- **SAVE/BGSAVE** operations
- **AOF rewrite** triggering
- **JSON/CSV export** of keys
- **Data backup** utilities

### 📡 Pub/Sub Operations

- **Channel subscription**
- **Message publishing**
- **Real-time message streaming**

### 🏗️ Cluster & Sentinel

- **Cluster node** information
- **Slot distribution** viewing
- **Sentinel master/slave** info
- **High availability** monitoring

### ⭐ UX Features

- **Command history** management
- **Favorites** for frequently used keys
- **Auto-completion** support
- **Color-coded output**
- **Progress indicators**

### 🔐 Security & Audit

- **Authentication** support
- **ACL management**
- **Security auditing**
- **Access control**

## Installation

### Prerequisites

- Rust 1.70+ and Cargo
- Redis server (for testing)

### Build from Source

```bash
git clone <repository-url>
cd solt
cargo build --release
```

### Install Locally

```bash
cargo install --path .
```

## Quick Start

### 1. Connect to Redis

```bash
# Connect to default Redis instance
solt connect

# Connect to specific Redis instance
solt connect --host redis.example.com --port 6379 --password mypassword

# Test connection only
solt connect --test
```

### 2. List Keys

```bash
# List all keys
solt keys

# List keys with pattern
solt keys "user:*"

# Detailed key information
solt keys --detailed

# Count keys only
solt keys --count
```

### 3. Get Values

```bash
# Get string value
solt get mykey

# Get with pretty JSON formatting
solt get mykey --pretty

# Get hash field
solt get myhash --hash-field "key:field"

# Get list range
solt get mylist --list-range "0-10"
```

### 4. Set Values

```bash
# Set string value
solt set mykey "my value"

# Set with TTL
solt set mykey "my value" --ttl 3600

# Set hash field
solt set myhash --hash-field "key:field:value"

# Push to list
solt set mylist "new item" --push-list "right"

# Add to set
solt set myset --add-set "new member"

# Add to sorted set
solt set myzset --add-zset "member:10.5"
```

### 5. Monitor Redis

```bash
# Real-time monitoring
solt monitor

# Show slow log
solt monitor --slowlog

# Show connected clients
solt monitor --clients
```

### 6. Delete Operations

```bash
# Delete single key
solt delete mykey

# Delete by pattern (with confirmation)
solt delete --pattern "temp:*" --confirm

# Flush current database
solt delete --flush-db --confirm
```

## Configuration

### Environment Management

```bash
# Show current configuration
solt config --show

# Add new environment
solt config --add-env production

# Set default environment
solt config --set-default production

# Set output format
solt config --output-format json
```

### Configuration File

The configuration is stored in `~/.solt/config.toml`:

```toml
default_environment = "dev"
history_size = 1000
output_format = "table"

[environments.dev]
name = "dev"
host = "localhost"
port = 6379
password = ""
db = 0
timeout = 30
tls = false

[environments.production]
name = "production"
host = "redis.prod.com"
port = 6379
password = "secret"
db = 0
timeout = 30
tls = true

favorites = [
    "user:profile:*",
    "session:*",
    "cache:*"
]
```

## Advanced Usage

### Bulk Operations

```bash
# Bulk delete keys
solt bulk delete "temp:*" --confirm

# Copy keys between databases
solt copy source_key dest_key --source-env dev --dest-env staging
```

### Export Data

```bash
# Export to JSON
solt export json --output data.json --pattern "user:*"

# Export to CSV
solt export csv --output data.csv --pattern "session:*"
```

### Cluster Operations

```bash
# Show cluster nodes
solt cluster --nodes

# Show cluster slots
solt cluster --slots
```

### Pub/Sub

```bash
# Subscribe to channel
solt pubsub --subscribe "notifications"

# Publish message
solt pubsub --publish "notifications" "Hello World!"
```

## Command Reference

### Connection Commands

- `connect` - Connect to Redis instance
- `config` - Manage configurations

### Key Commands

- `keys` - List and inspect keys
- `inspect` - Detailed key inspection
- `get` - Get values from keys
- `set` - Set values in keys
- `delete` - Delete keys

### Search & Filter

- `search` - Search keys by pattern
- `filter` - Filter keys by criteria

### Monitoring

- `monitor` - Real-time monitoring
- `stats` - Get Redis statistics
- `debug` - Debug operations

### Bulk Operations

- `bulk` - Bulk operations
- `copy` - Copy between databases

### Backup & Export

- `backup` - Backup operations
- `export` - Export data

### Advanced

- `pubsub` - Pub/Sub operations
- `cluster` - Cluster operations
- `sentinel` - Sentinel operations
- `favorites` - Manage favorites
- `history` - Command history

## Examples

### Development Workflow

```bash
# 1. Connect to development Redis
solt connect --environment dev

# 2. List all keys
solt keys

# 3. Get specific user data
solt get "user:123" --pretty

# 4. Set cache data
solt set "cache:user:123" '{"name":"John","email":"john@example.com"}' --ttl 3600

# 5. Monitor operations
solt monitor
```

### Production Monitoring

```bash
# 1. Connect to production
solt connect --environment prod

# 2. Check memory usage
solt stats --memory

# 3. Monitor slow queries
solt monitor --slowlog --slowlog-count 20

# 4. Check connected clients
solt monitor --clients

# 5. Export data for analysis
solt export json --output backup.json --pattern "user:*"
```

## Error Handling

The application provides comprehensive error handling:

- **Connection errors** with detailed diagnostics
- **Configuration validation** with helpful messages
- **Redis operation errors** with context
- **User-friendly error messages** with suggestions

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Support

For issues and questions:

- Create an issue on GitHub
- Check the documentation
- Review the examples

---

**Solt** - Redis CLI Management Tool - Making Redis administration simple and powerful! 🚀
