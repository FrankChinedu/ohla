use async_trait::async_trait;
use serde::{Deserialize, Serialize};

// ============================================================================
// Domain Models
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NodeConfig {
    pub id: String,
    pub name: String,
    pub rpc_url: String,
    pub rpc_user: String,
    pub rpc_password: String,
    pub network: String,
    #[sqlx(default)]
    pub is_active: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewNodeConfig {
    pub name: String,
    pub rpc_url: String,
    pub rpc_user: String,
    pub rpc_password: String,
    pub network: String,
}

// ============================================================================
// Error Types
// ============================================================================

#[derive(Debug)]
pub enum DbError {
    NotFound,
    DatabaseError(String),
    InvalidInput(String),
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::NotFound => write!(f, "Record not found"),
            DbError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            DbError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl std::error::Error for DbError {}

impl From<sqlx::Error> for DbError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => DbError::NotFound,
            _ => DbError::DatabaseError(err.to_string()),
        }
    }
}

// ============================================================================
// Repository Traits
// ============================================================================

/// Repository trait for node configuration operations
/// This abstraction allows easy migration to different databases
#[async_trait]
pub trait NodeConfigRepository: Send + Sync {
    /// Create a new node configuration
    async fn create(&self, config: NewNodeConfig) -> Result<NodeConfig, DbError>;

    /// Get a node configuration by ID
    async fn get(&self, id: &str) -> Result<Option<NodeConfig>, DbError>;

    /// Get the currently active node configuration
    async fn get_active(&self) -> Result<Option<NodeConfig>, DbError>;

    /// List all node configurations
    async fn list(&self) -> Result<Vec<NodeConfig>, DbError>;

    /// Set a node configuration as active (deactivates all others)
    async fn set_active(&self, id: &str) -> Result<(), DbError>;

    /// Delete a node configuration
    async fn delete(&self, id: &str) -> Result<(), DbError>;

    /// Test if a connection can be established with the given configuration
    async fn test_connection(&self, config: &NodeConfig) -> Result<bool, DbError>;
}
