use axum::Router;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::routes;
use crate::state::app_state::AppState;

pub fn create_app(app_state: AppState) -> Router {
  let api_routes = Router::new().merge(routes::health::routes());
    Router::new()
        // Routes nested under /api prefix
        .nest("/api", api_routes)
        // Global middleware
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive()),
        )
        // Shared state (must be after routes and middleware)
        .with_state(app_state)
}
