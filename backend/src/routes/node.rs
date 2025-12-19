use axum::{Router, extract::State, routing::get};
use std::sync::Arc;

use crate::domain::node::{NodeInfo, BlockCount};
use crate::errors::AppError;
use crate::responses::ApiResponse;
use crate::state::app_state::AppState;

/// GET /node/info - Get Bitcoin node information
async fn get_node_info(State(state): State<Arc<AppState>>) -> Result<ApiResponse<NodeInfo>, AppError> {
    let node_info = state.bitcoin.get_node_info().await?;
    Ok(ApiResponse::success(node_info, "Node information retrieved successfully"))
}

async fn get_node_block_count(State(state): State<Arc<AppState>>) -> Result<ApiResponse<BlockCount>, AppError> {
    let block_count = state.bitcoin.get_block_count().await?;
    Ok(ApiResponse::success(
        BlockCount { block_count },
        "Block count retrieved successfully",
    ))
}

pub fn routes() -> Router<Arc<AppState>> {
    // let route = Router::
    Router::new().route("/node/info", get(get_node_info)).route("/node/block-count", get(get_node_block_count))
}
