use axum::{extract::State, http::StatusCode, Json, Router, routing::get};
use std::sync::Arc;

use crate::domain::node::NodeInfo;
use crate::state::app_state::AppState;

/// GET /node/info - Get Bitcoin node information
async fn get_node_info(
    State(state): State<Arc<AppState>>,
) -> Result<Json<NodeInfo>, (StatusCode, String)> {
    state
        .bitcoin
        .get_node_info()
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/node/info", get(get_node_info))
}
