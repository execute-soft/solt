pub mod version;

// Connection & Config commands
pub mod config;
pub mod connect;

// Key Inspection commands
pub mod inspect;
pub mod keys;

// Value Viewing commands
pub mod get;
pub mod set;

// Search & Filter commands
pub mod filter;
pub mod search;

// Editing & Writing commands
pub mod delete;
pub mod edit;

// Bulk Operations commands
pub mod bulk;
pub mod copy;

// Monitoring & Debug commands
pub mod debug;
pub mod monitor;
pub mod stats;

// Backup & Export commands
pub mod backup;
pub mod export;

// Pub/Sub commands
pub mod pubsub;

// Cluster & Sentinel commands
pub mod cluster;
pub mod sentinel;

// UX Features commands
pub mod favorites;
pub mod history;
