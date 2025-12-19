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
- `GET /api/node/info` - Get Bitcoin node information (blockchain info)

## Configuration

Create a `.env` file in the project root with your Bitcoin node credentials:

```env
BTC_RPC_URL=http://127.0.0.1:18443
BTC_RPC_USER=your_rpc_username
BTC_RPC_PASS=your_rpc_password
```

For regtest:
- Default RPC port: `18443`
- For mainnet: `8332`
- For testnet: `18332`

## Project Structure

```
src/
├── app.rs              # Application setup and routing
├── main.rs             # Entry point
├── config/             # Configuration management
│   ├── bitcoin.rs      # Bitcoin RPC configuration
│   ├── constants.rs    # Environment variable constants
│   └── env.rs          # Environment loading
├── domain/             # Domain models
│   └── node.rs         # Node information models
├── routes/             # API route handlers
│   ├── health.rs       # Health check routes
│   └── node.rs         # Node info routes
├── services/           # Business logic
│   └── bitcoin_rpc.rs  # Bitcoin RPC client
└── state/              # Application state management
    └── app_state.rs
```
