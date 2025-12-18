# Bitcoin Backend

A Rust backend service built with Axum framework.

## Prerequisites

- Rust (latest stable version)
- Cargo

## Setup

1. Clone the repository
2. Install dependencies:
```bash
cargo build
```

## Development

### Run the application

```bash
cargo run
```

### Watch mode (hot reload)

Install cargo-watch (one-time setup):
```bash
cargo install cargo-watch
```

Run in watch mode with automatic restart on file changes:
```bash
cargo dev
```

This is a convenient alias for `cargo watch -c -w src -x run` that:
- Clears screen between runs
- Watches the src directory for changes
- Executes `cargo run` on changes

Alternative watch commands:
```bash
# Watch with checks before running
cargo watch-check

# Full cargo-watch command
cargo watch -c -w src -x run
```

## API Endpoints

All routes are prefixed with `/api`:

- `GET /api/health` - Health check endpoint

## Project Structure

```
src/
├── app.rs          # Application setup and routing
├── main.rs         # Entry point
├── routes/         # API route handlers
│   └── health.rs   # Health check routes
└── state/          # Application state management
    └── app_state.rs
```
