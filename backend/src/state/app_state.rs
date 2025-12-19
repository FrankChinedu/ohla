use crate::config::bitcoin::BitcoinConfig;
use crate::services::bitcoin_rpc::BitcoinRpc;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub bitcoin: Arc<BitcoinRpc>,
}

impl AppState {
    pub fn initialize() -> Self {
        let bitcoin_config = BitcoinConfig::from_env();

        let bitcoin = Arc::new(BitcoinRpc::new(
            bitcoin_config.rpc_url,
            bitcoin_config.rpc_user,
            bitcoin_config.rpc_pass,
        ));
        AppState { bitcoin }
    }
}
