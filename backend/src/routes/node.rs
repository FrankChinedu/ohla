use axum::{Json, Router, extract::State, routing::get};
use std::sync::Arc;

use crate::domain::node::NodeInfo;
use crate::errors::AppError;
use crate::state::app_state::AppState;

/// GET /node/info - Get Bitcoin node information
async fn get_node_info(State(state): State<Arc<AppState>>) -> Result<Json<NodeInfo>, AppError> {
    state.bitcoin.get_node_info().await.map(Json)
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/node/info", get(get_node_info))
}
