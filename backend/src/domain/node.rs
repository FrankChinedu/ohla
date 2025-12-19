use serde::Serialize;

#[derive(Serialize)]
pub struct NodeInfo {
    pub network: String,
    pub block_height: u64,
    pub best_block_hash: String,
    pub sync: SyncInfo,
    pub pruned: bool,
    pub verification_progress: f64,
    pub backend: BackendInfo,
}

#[derive(Serialize)]
pub struct SyncInfo {
    pub is_synced: bool,
    pub progress: f64,
}

#[derive(Serialize)]
pub struct BackendInfo {
    pub version: String,
    pub node_type: &'static str,
}
