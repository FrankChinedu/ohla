use std::env;

use super::constants::env_keys;

pub struct BitcoinConfig {
    pub rpc_url: String,
    pub rpc_user: String,
    pub rpc_pass: String,
}

impl BitcoinConfig {
    pub fn from_env() -> Self {
        Self {
            rpc_url: env::var(env_keys::BTC_RPC_URL)
                .unwrap_or_else(|_| panic!("{} must be set", env_keys::BTC_RPC_URL)),
            rpc_user: env::var(env_keys::BTC_RPC_USER)
                .unwrap_or_else(|_| panic!("{} must be set", env_keys::BTC_RPC_USER)),
            rpc_pass: env::var(env_keys::BTC_RPC_PASS)
                .unwrap_or_else(|_| panic!("{} must be set", env_keys::BTC_RPC_PASS)),
        }
    }
}
