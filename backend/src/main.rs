mod app;
mod config;
mod domain;
mod errors;
mod routes;
mod services;
mod state;
mod utils;

use crate::app::create_app;
use crate::config::env::load_env;
use crate::state::app_state::AppState;
use std::net::SocketAddr;
use tokio::signal;

#[tokio::main]
async fn main() {
    // Load and validate environment variables
    load_env();

    let app_state = AppState::initialize();
    let app = create_app(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");
    println!("🚀 Server running at http://{}", addr);

    // Start server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Server failed");
}

async fn shutdown_signal() {
    let _ = signal::ctrl_c().await;
    println!("🛑 Shutdown signal received");
}
