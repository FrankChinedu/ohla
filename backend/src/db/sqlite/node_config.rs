use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

use crate::db::traits::{DbError, NewNodeConfig, NodeConfig, NodeConfigRepository};
use crate::services::bitcoin_rpc::BitcoinRpc;

/// SQLite implementation of NodeConfigRepository
pub struct SqliteNodeConfigRepository {
    pool: SqlitePool,
}

impl SqliteNodeConfigRepository {
    /// Create a new SQLite repository from an existing pool
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Get the connection pool (useful for other operations)
    #[allow(unused)]
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}

#[async_trait]
impl NodeConfigRepository for SqliteNodeConfigRepository {
    async fn create(&self, config: NewNodeConfig) -> Result<NodeConfig, DbError> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().timestamp();

        // Check if this is the first config
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM node_configs")
            .fetch_one(&self.pool)
            .await?;

        let is_first = count == 0;

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
            "#,
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
            "#,
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
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| NodeConfig {
                id: r.get("id"),
                name: r.get("name"),
                rpc_url: r.get("rpc_url"),
                rpc_user: r.get("rpc_user"),
                rpc_password: r.get("rpc_password"),
                network: r.get("network"),
                is_active: r.get::<i32, _>("is_active") == 1,
            })
            .collect())
    }

    async fn set_active(&self, id: &str) -> Result<(), DbError> {
        // First, verify the config exists
        let exists: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM node_configs WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        if exists == 0 {
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

    async fn test_connection(&self, config: &NodeConfig) -> Result<bool, DbError> {
        // Create a temporary RPC client with the provided config
        let client = BitcoinRpc::new(
            config.rpc_url.clone(),
            config.rpc_user.clone(),
            config.rpc_password.clone(),
        );

        // Try to get blockchain info to test the connection
        match client.get_blockchain_info().await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}
