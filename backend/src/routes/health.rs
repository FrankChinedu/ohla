use axum::{Router, routing::get};
use serde::Serialize;

use crate::responses::ApiResponse;

#[derive(Serialize)]
struct HealthData {
    status: &'static str,
    service: &'static str,
}

/// GET /health
async fn health_check() -> ApiResponse<HealthData> {
    ApiResponse::success(
        HealthData {
            status: "ok",
            service: "bitcoin-backend",
        },
        "Service is healthy",
    )
}

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new().route("/health", get(health_check))
}
