use base64::Engine;
use reqwest::{header, Client};
use serde::Deserialize;
use serde_json::json;

use crate::domain::node::NodeInfo;
use crate::errors::AppError;

pub struct BitcoinRpc {
    url: String,
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
        // Create authorization header value
        let auth = format!("{}:{}", username, password);
        let auth_header = format!(
            "Basic {}",
            base64::engine::general_purpose::STANDARD.encode(auth.as_bytes())
        );

        // Build client with default headers
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&auth_header)
                .expect("Invalid authorization header"),
        );
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build HTTP client");

        Self {
            url,
            client,
        }
    }

    pub async fn get_blockchain_info(&self) -> Result<BlockchainInfo, AppError> {
        let payload = json!({
            "jsonrpc": "1.0",
            "id": "getblockchaininfo",
            "method": "getblockchaininfo",
            "params": []
        });

        let response = self
            .client
            .post(&self.url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| AppError::BitcoinRpcConnection(e.to_string()))?;

        let rpc_response: RpcResponse<BlockchainInfo> = response
            .json()
            .await
            .map_err(|e| AppError::BitcoinRpcParse(e.to_string()))?;

        if let Some(error) = rpc_response.error {
            return Err(AppError::BitcoinRpcError {
                code: error.code,
                message: error.message,
            });
        }

        rpc_response
            .result
            .ok_or(AppError::BitcoinRpcNoResult)
    }

    pub async fn get_node_info(&self) -> Result<NodeInfo, AppError> {
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
