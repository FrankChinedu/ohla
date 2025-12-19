use std::env;

use super::constants::env_keys;

/// Load environment variables from `.env` (dev only)
/// and ensure the process has access to required vars.
pub fn load_env() {
    // Load .env file if present (local development)
    dotenvy::dotenv().ok();

    // Validate required environment variables
    validate_required(env_keys::BTC_RPC_URL);
    validate_required(env_keys::BTC_RPC_USER);
    validate_required(env_keys::BTC_RPC_PASS);
}

/// Panic early if a required variable is missing
fn validate_required(key: &str) {
    if env::var(key).is_err() {
        panic!("Missing required environment variable: {}", key);
    }
}
