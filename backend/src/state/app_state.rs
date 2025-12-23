use crate::config::bitcoin::BitcoinConfig;
use crate::services::bitcoin_rpc::BitcoinRpc;
use sqlx::{sqlite::{SqliteConnectOptions, SqlitePool}};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub bitcoin: Arc<BitcoinRpc>,
    pub db_pool: SqlitePool,
}

impl AppState {
    pub async fn initialize() -> Self {
        let bitcoin_config = BitcoinConfig::from_env();

        let bitcoin = Arc::new(BitcoinRpc::new(
            bitcoin_config.rpc_url,
            bitcoin_config.rpc_user,
            bitcoin_config.rpc_pass,
        ));

        // Initialize database pool
        let db_file_name = std::env::var(crate::config::constants::env_keys::DATABASE_URL)
            .unwrap_or_else(|_| "ohla.db".to_string());
        let option = SqliteConnectOptions::new().filename(db_file_name).create_if_missing(true);
        let db_pool = SqlitePool::connect_with(option).await.unwrap();

        // Run migrations/setup
        Self::setup_database(&db_pool).await;

        AppState {
            bitcoin,
            db_pool,
        }
    }

    async fn setup_database(pool: &SqlitePool) {
        // Create node_configs table if it doesn't exist
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
        .execute(pool)
        .await
        .expect("Failed to create node_configs table");
    }
}
