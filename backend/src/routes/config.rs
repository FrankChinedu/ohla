use axum::{
    Router,
    extract::{State, Path},
    routing::{get, post, put, delete},
    Json,
};
use std::sync::Arc;

use crate::db::{NodeConfigRepository, NodeConfig, NewNodeConfig, SqliteNodeConfigRepository};
use crate::errors::AppError;
use crate::responses::ApiResponse;
use crate::state::app_state::AppState;

/// POST /config/nodes - Create a new node configuration
async fn create_node_config(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NewNodeConfig>,
) -> Result<ApiResponse<NodeConfig>, AppError> {
    let repo = SqliteNodeConfigRepository::new(state.db_pool.clone());
    let config = repo.create(payload).await?;
    Ok(ApiResponse::success(config, "Node configuration created successfully"))
}

/// GET /config/nodes - List all node configurations
async fn list_node_configs(
    State(state): State<Arc<AppState>>,
) -> Result<ApiResponse<Vec<NodeConfig>>, AppError> {
    let repo = SqliteNodeConfigRepository::new(state.db_pool.clone());
    let configs = repo.list().await?;
    Ok(ApiResponse::success(configs, "Node configurations retrieved successfully"))
}

/// GET /config/nodes/:id - Get a specific node configuration
async fn get_node_config(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<ApiResponse<NodeConfig>, AppError> {
    let repo = SqliteNodeConfigRepository::new(state.db_pool.clone());
    let config = repo.get(&id).await?
        .ok_or(AppError::NotFound(format!("Node configuration with id {} not found", id)))?;
    Ok(ApiResponse::success(config, "Node configuration retrieved successfully"))
}

/// GET /config/nodes/active - Get the currently active node configuration
async fn get_active_node_config(
    State(state): State<Arc<AppState>>,
) -> Result<ApiResponse<NodeConfig>, AppError> {
    let repo = SqliteNodeConfigRepository::new(state.db_pool.clone());
    let config = repo.get_active().await?
        .ok_or(AppError::NotFound("No active node configuration found".to_string()))?;
    Ok(ApiResponse::success(config, "Active node configuration retrieved successfully"))
}

/// PUT /config/nodes/:id/activate - Set a node configuration as active
async fn set_active_node_config(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<ApiResponse<()>, AppError> {
    let repo = SqliteNodeConfigRepository::new(state.db_pool.clone());
    repo.set_active(&id).await?;
    Ok(ApiResponse::success((), "Node configuration activated successfully"))
}

/// DELETE /config/nodes/:id - Delete a node configuration
async fn delete_node_config(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<ApiResponse<()>, AppError> {
    let repo = SqliteNodeConfigRepository::new(state.db_pool.clone());
    repo.delete(&id).await?;
    Ok(ApiResponse::success((), "Node configuration deleted successfully"))
}

#[derive(serde::Deserialize)]
struct TestConnectionPayload {
    rpc_url: String,
    rpc_user: String,
    rpc_password: String,
}

#[derive(serde::Serialize)]
struct TestConnectionResponse {
    success: bool,
    message: String,
}

/// POST /config/nodes/test - Test a node connection without saving
async fn test_node_connection(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<TestConnectionPayload>,
) -> Result<ApiResponse<TestConnectionResponse>, AppError> {
    let test_config = NodeConfig {
        id: String::new(),
        name: String::new(),
        rpc_url: payload.rpc_url,
        rpc_user: payload.rpc_user,
        rpc_password: payload.rpc_password,
        network: String::new(),
        is_active: false,
    };

    let repo = SqliteNodeConfigRepository::new(state.db_pool.clone());
    let success = repo.test_connection(&test_config).await?;

    let response = TestConnectionResponse {
        success,
        message: if success {
            "Connection successful".to_string()
        } else {
            "Connection failed".to_string()
        },
    };

    Ok(ApiResponse::success(response, "Connection test completed"))
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/config/nodes", post(create_node_config))
        .route("/config/nodes", get(list_node_configs))
        .route("/config/nodes/active", get(get_active_node_config))
        .route("/config/nodes/test", post(test_node_connection))
        .route("/config/nodes/:id", get(get_node_config))
        .route("/config/nodes/:id/activate", put(set_active_node_config))
        .route("/config/nodes/:id", delete(delete_node_config))
}
