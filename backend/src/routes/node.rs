use axum::{Json, Router, extract::State, routing::get};
use std::sync::Arc;

use crate::domain::node::{NodeInfo, BlockCount};
use crate::errors::AppError;
use crate::state::app_state::AppState;

/// GET /node/info - Get Bitcoin node information
async fn get_node_info(State(state): State<Arc<AppState>>) -> Result<Json<NodeInfo>, AppError> {
    state.bitcoin.get_node_info().await.map(Json)
}

async fn get_node_block_count(State(state): State<Arc<AppState>>) -> Result<Json<BlockCount>, AppError> {
    state.bitcoin.get_block_count().await.map(|block_count| Json(BlockCount { block_count }))
}

pub fn routes() -> Router<Arc<AppState>> {
    // let route = Router::
    Router::new().route("/node/info", get(get_node_info)).route("/node/block-count", get(get_node_block_count))
}
