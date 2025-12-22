use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, Row};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    pub id: String,
    pub name: String,
    pub rpc_url: String,
    pub rpc_user: String,
    pub rpc_password: String,
    pub network: String,
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
        DbError::DatabaseError(err.to_string())
    }
}

/// Repository trait for node configuration operations
/// This abstraction allows easy migration to different databases
#[async_trait]
pub trait NodeConfigRepository: Send + Sync {
    async fn create(&self, config: NewNodeConfig) -> Result<NodeConfig, DbError>;
    async fn get(&self, id: &str) -> Result<Option<NodeConfig>, DbError>;
    async fn get_active(&self) -> Result<Option<NodeConfig>, DbError>;
    async fn list(&self) -> Result<Vec<NodeConfig>, DbError>;
    async fn set_active(&self, id: &str) -> Result<(), DbError>;
    async fn delete(&self, id: &str) -> Result<(), DbError>;
}

/// SQLite implementation of NodeConfigRepository
pub struct SqliteNodeConfigRepository {
    pool: SqlitePool,
}

impl SqliteNodeConfigRepository {
    pub async fn new(database_url: &str) -> Result<Self, DbError> {
        let pool = SqlitePool::connect(database_url).await?;

        // Create table if it doesn't exist
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS node_configs (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                rpc_url TEXT NOT NULL,
                rpc_user TEXT NOT NULL,
                rpc_password TEXT NOT NULL,
                network TEXT NOT NULL,
                is_active INTEGER NOT NULL DEFAULT 0,
                created_at INTEGER NOT NULL
            )
            "#
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}

#[async_trait]
impl NodeConfigRepository for SqliteNodeConfigRepository {
    async fn create(&self, config: NewNodeConfig) -> Result<NodeConfig, DbError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();

        // If this is the first config, make it active
        let is_first = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM node_configs")
            .fetch_one(&self.pool)
            .await? == 0;

        sqlx::query(
            r#"
            INSERT INTO node_configs (id, name, rpc_url, rpc_user, rpc_password, network, is_active, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&id)
        .bind(&config.name)
        .bind(&config.rpc_url)
        .bind(&config.rpc_user)
        .bind(&config.rpc_password)
        .bind(&config.network)
        .bind(if is_first { 1 } else { 0 })
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(NodeConfig {
            id,
            name: config.name,
            rpc_url: config.rpc_url,
            rpc_user: config.rpc_user,
            rpc_password: config.rpc_password,
            network: config.network,
            is_active: is_first,
        })
    }

    async fn get(&self, id: &str) -> Result<Option<NodeConfig>, DbError> {
        let row = sqlx::query(
            r#"
            SELECT id, name, rpc_url, rpc_user, rpc_password, network, is_active
            FROM node_configs
            WHERE id = ?
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| NodeConfig {
            id: r.get("id"),
            name: r.get("name"),
            rpc_url: r.get("rpc_url"),
            rpc_user: r.get("rpc_user"),
            rpc_password: r.get("rpc_password"),
            network: r.get("network"),
            is_active: r.get::<i32, _>("is_active") == 1,
        }))
    }

    async fn get_active(&self) -> Result<Option<NodeConfig>, DbError> {
        let row = sqlx::query(
            r#"
            SELECT id, name, rpc_url, rpc_user, rpc_password, network, is_active
            FROM node_configs
            WHERE is_active = 1
            LIMIT 1
            "#
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| NodeConfig {
            id: r.get("id"),
            name: r.get("name"),
            rpc_url: r.get("rpc_url"),
            rpc_user: r.get("rpc_user"),
            rpc_password: r.get("rpc_password"),
            network: r.get("network"),
            is_active: true,
        }))
    }

    async fn list(&self) -> Result<Vec<NodeConfig>, DbError> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, rpc_url, rpc_user, rpc_password, network, is_active
            FROM node_configs
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| NodeConfig {
            id: r.get("id"),
            name: r.get("name"),
            rpc_url: r.get("rpc_url"),
            rpc_user: r.get("rpc_user"),
            rpc_password: r.get("rpc_password"),
            network: r.get("network"),
            is_active: r.get::<i32, _>("is_active") == 1,
        }).collect())
    }

    async fn set_active(&self, id: &str) -> Result<(), DbError> {
        // First, verify the config exists
        let exists = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM node_configs WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await? > 0;

        if !exists {
            return Err(DbError::NotFound);
        }

        // Start a transaction to ensure atomicity
        let mut tx = self.pool.begin().await?;

        // Deactivate all configs
        sqlx::query("UPDATE node_configs SET is_active = 0")
            .execute(&mut *tx)
            .await?;

        // Activate the selected one
        sqlx::query("UPDATE node_configs SET is_active = 1 WHERE id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<(), DbError> {
        let result = sqlx::query("DELETE FROM node_configs WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound);
        }

        Ok(())
    }
}
