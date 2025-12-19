use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

use crate::domain::node::NodeInfo;

pub struct BitcoinRpc {
    url: String,
    username: String,
    password: String,
    client: Client,
}

#[derive(Deserialize, Debug)]
struct RpcResponse<T> {
    result: Option<T>,
    error: Option<RpcError>,
}

#[derive(Deserialize, Debug)]
struct RpcError {
    code: i32,
    message: String,
}

#[derive(Deserialize, Debug)]
pub struct BlockchainInfo {
    pub chain: String,
    pub blocks: u64,
    pub headers: u64,
    pub bestblockhash: String,
    pub difficulty: f64,
    pub verificationprogress: f64,
    pub pruned: bool,
    pub initialblockdownload: bool,
}

impl BitcoinRpc {
    pub fn new(url: String, username: String, password: String) -> Self {
        Self {
            url,
            username,
            password,
            client: Client::new(),
        }
    }

    pub async fn get_blockchain_info(&self) -> Result<BlockchainInfo, String> {
        let payload = json!({
            "jsonrpc": "1.0",
            "id": "getblockchaininfo",
            "method": "getblockchaininfo",
            "params": []
        });

        let response = self
            .client
            .post(&self.url)
            .basic_auth(&self.username, Some(&self.password))
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        let rpc_response: RpcResponse<BlockchainInfo> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        if let Some(error) = rpc_response.error {
            return Err(format!("RPC error {}: {}", error.code, error.message));
        }

        rpc_response
            .result
            .ok_or_else(|| "No result in response".to_string())
    }

    pub async fn get_node_info(&self) -> Result<NodeInfo, String> {
        let blockchain_info = self.get_blockchain_info().await?;

        Ok(NodeInfo {
            network: blockchain_info.chain,
            block_height: blockchain_info.blocks,
            best_block_hash: blockchain_info.bestblockhash,
            difficulty: blockchain_info.difficulty,
            headers: blockchain_info.headers,
            sync: crate::domain::node::SyncInfo {
                is_synced: !blockchain_info.initialblockdownload,
                progress: blockchain_info.verificationprogress,
            },
            pruned: blockchain_info.pruned,
            verification_progress: blockchain_info.verificationprogress,
            backend: crate::domain::node::BackendInfo {
                version: "Bitcoin Core".to_string(),
                node_type: "bitcoind",
            },
        })
    }
}
