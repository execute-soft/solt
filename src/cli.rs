use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "solt",
    about = "A comprehensive Redis CLI management tool",
    version,
    long_about = "Solt is a powerful Redis CLI tool that provides comprehensive Redis management capabilities including connection management, key inspection, value viewing, monitoring, and more."
)]
pub struct Cli {
    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,

    /// Environment to use (dev, staging, prod, etc.)
    #[arg(short, long, value_name = "ENVIRONMENT")]
    pub environment: Option<String>,

    /// The command to run
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Version,

    // Connection & Config commands
    /// Connect to Redis and test connection
    Connect(ConnectArgs),

    /// Manage configurations and environments
    Config(ConfigArgs),

    // Key Inspection commands
    /// List and inspect Redis keys
    Keys(KeysArgs),

    /// Inspect key details
    Inspect(InspectArgs),

    // Value Viewing commands
    /// Get values from Redis keys
    Get(GetArgs),

    /// Set values in Redis
    Set(SetArgs),

    // Search & Filter commands
    /// Search keys by pattern
    Search(SearchArgs),

    /// Filter keys by criteria
    Filter(FilterArgs),

    // Editing & Writing commands
    /// Edit Redis values
    Edit(EditArgs),

    /// Delete Redis keys
    Delete(DeleteArgs),

    // Bulk Operations commands
    /// Perform bulk operations
    Bulk(BulkArgs),

    /// Copy keys between databases
    Copy(CopyArgs),

    // Monitoring & Debug commands
    /// Monitor Redis in real-time
    Monitor(MonitorArgs),

    /// Debug Redis operations
    Debug(DebugArgs),

    /// Get Redis statistics
    Stats(StatsArgs),

    // Backup & Export commands
    /// Backup Redis data
    Backup(BackupArgs),

    /// Export Redis data
    Export(ExportArgs),

    // Pub/Sub commands
    /// Pub/Sub operations
    Pubsub(PubsubArgs),

    // Cluster & Sentinel commands
    /// Cluster operations
    Cluster(ClusterArgs),

    /// Sentinel operations
    Sentinel(SentinelArgs),

    // UX Features commands
    /// Manage favorites
    Favorites(FavoritesArgs),

    /// View command history
    History(HistoryArgs),
}

#[derive(Args)]
pub struct ConnectArgs {
    /// Redis host
    #[arg(long)]
    pub host: Option<String>,

    /// Redis port
    #[arg(long)]
    pub port: Option<u16>,

    /// Redis password
    #[arg(long)]
    pub password: Option<String>,

    /// Redis database index
    #[arg(long)]
    pub db: Option<u8>,

    /// Connection timeout in seconds
    #[arg(long)]
    pub timeout: Option<u64>,

    /// Use TLS connection
    #[arg(long)]
    pub tls: bool,

    /// Test connection only
    #[arg(long)]
    pub test: bool,
}

#[derive(Args)]
pub struct ConfigArgs {
    /// Show current configuration
    #[arg(long)]
    pub show: bool,

    /// Add new environment
    #[arg(long)]
    pub add_env: Option<String>,

    /// Remove environment
    #[arg(long)]
    pub remove_env: Option<String>,

    /// Set default environment
    #[arg(long)]
    pub set_default: Option<String>,

    /// Set output format (json, table, csv, plain)
    #[arg(long)]
    pub output_format: Option<String>,

    /// Set history size
    #[arg(long)]
    pub history_size: Option<usize>,
}

#[derive(Args)]
pub struct KeysArgs {
    /// Key pattern to match
    #[arg(default_value = "*")]
    pub pattern: String,

    /// Show detailed information
    #[arg(long)]
    pub detailed: bool,

    /// Count keys only
    #[arg(long)]
    pub count: bool,
}

#[derive(Args)]
pub struct InspectArgs {
    /// Key to inspect
    pub key: String,
}

#[derive(Args)]
pub struct GetArgs {
    /// Key to get
    pub key: String,

    /// Pretty print JSON values
    #[arg(long)]
    pub pretty: bool,

    /// Get hash field (format: key:field)
    #[arg(long)]
    pub hash_field: Option<String>,

    /// Get list range (format: start-stop)
    #[arg(long)]
    pub list_range: Option<String>,
}

#[derive(Args)]
pub struct SetArgs {
    /// Key to set
    pub key: String,

    /// Value to set
    pub value: String,

    /// TTL in seconds
    #[arg(long)]
    pub ttl: Option<u64>,

    /// Set hash field (format: key:field:value)
    #[arg(long)]
    pub hash_field: Option<String>,

    /// Push to list (left or right)
    #[arg(long)]
    pub push_list: Option<String>,

    /// Add to set
    #[arg(long)]
    pub add_set: Option<String>,

    /// Add to sorted set (format: member:score)
    #[arg(long)]
    pub add_zset: Option<String>,
}

#[derive(Args)]
pub struct SearchArgs {
    /// Search pattern
    pub pattern: String,

    /// Show count only
    #[arg(long)]
    pub count: bool,
}

#[derive(Args)]
pub struct FilterArgs {
    /// Filter by TTL (format: min-max)
    #[arg(long)]
    pub ttl: Option<String>,

    /// Filter by size (format: min-max)
    #[arg(long)]
    pub size: Option<String>,

    /// Filter by type
    #[arg(long)]
    pub type_filter: Option<String>,
}

#[derive(Args)]
pub struct EditArgs {
    /// Key to edit
    pub key: String,

    /// New value
    pub value: String,
}

#[derive(Args)]
pub struct DeleteArgs {
    /// Key to delete
    pub key: String,

    /// Delete by pattern
    #[arg(long)]
    pub pattern: Option<String>,

    /// Confirm deletion
    #[arg(long)]
    pub confirm: bool,

    /// Flush current database
    #[arg(long)]
    pub flush_db: bool,

    /// Flush all databases
    #[arg(long)]
    pub flush_all: bool,
}

#[derive(Args)]
pub struct BulkArgs {
    /// Bulk operation type
    #[arg(value_enum)]
    pub operation: BulkOperation,

    /// Pattern for keys
    pub pattern: String,

    /// Confirm operation
    #[arg(long)]
    pub confirm: bool,
}

#[derive(Args)]
pub struct CopyArgs {
    /// Source key
    pub source: String,

    /// Destination key
    pub destination: String,

    /// Source environment
    #[arg(long)]
    pub source_env: Option<String>,

    /// Destination environment
    #[arg(long)]
    pub dest_env: Option<String>,
}

#[derive(Args)]
pub struct MonitorArgs {
    /// Show slow log entries
    #[arg(long)]
    pub slowlog: bool,

    /// Number of slow log entries to show
    #[arg(long, default_value = "10")]
    pub slowlog_count: usize,

    /// Show client list
    #[arg(long)]
    pub clients: bool,
}

#[derive(Args)]
pub struct DebugArgs {
    /// Debug command
    pub command: String,
}

#[derive(Args)]
pub struct StatsArgs {
    /// Show memory stats
    #[arg(long)]
    pub memory: bool,

    /// Show command stats
    #[arg(long)]
    pub commands: bool,

    /// Show replication stats
    #[arg(long)]
    pub replication: bool,
}

#[derive(Args)]
pub struct BackupArgs {
    /// Trigger SAVE
    #[arg(long)]
    pub save: bool,

    /// Trigger BGSAVE
    #[arg(long)]
    pub bgsave: bool,

    /// Trigger AOF rewrite
    #[arg(long)]
    pub bgrewriteaof: bool,
}

#[derive(Args)]
pub struct ExportArgs {
    /// Export format (json, csv)
    #[arg(value_enum)]
    pub format: ExportFormat,

    /// Output file
    #[arg(short, long)]
    pub output: String,

    /// Key pattern to export
    #[arg(default_value = "*")]
    pub pattern: String,
}

#[derive(Args)]
pub struct PubsubArgs {
    /// Subscribe to channel
    #[arg(long)]
    pub subscribe: Option<String>,

    /// Publish to channel
    #[arg(long)]
    pub publish: Option<String>,

    /// Message to publish
    pub message: Option<String>,
}

#[derive(Args)]
pub struct ClusterArgs {
    /// Show cluster nodes
    #[arg(long)]
    pub nodes: bool,

    /// Show cluster slots
    #[arg(long)]
    pub slots: bool,
}

#[derive(Args)]
pub struct SentinelArgs {
    /// Show sentinel masters
    #[arg(long)]
    pub masters: bool,

    /// Show sentinel slaves
    #[arg(long)]
    pub slaves: bool,
}

#[derive(Args)]
pub struct FavoritesArgs {
    /// Add key to favorites
    #[arg(long)]
    pub add: Option<String>,

    /// Remove key from favorites
    #[arg(long)]
    pub remove: Option<String>,

    /// List favorites
    #[arg(long)]
    pub list: bool,
}

#[derive(Args)]
pub struct HistoryArgs {
    /// Show command history
    #[arg(long)]
    pub show: bool,

    /// Clear command history
    #[arg(long)]
    pub clear: bool,
}

#[derive(clap::ValueEnum, Clone)]
pub enum BulkOperation {
    Delete,
    Rename,
    Copy,
    Dump,
}

#[derive(clap::ValueEnum, Clone)]
pub enum ExportFormat {
    Json,
    Csv,
}
