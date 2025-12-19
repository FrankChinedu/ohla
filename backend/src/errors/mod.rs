use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

/// Error response structure returned to clients
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

/// Application error types
#[derive(Debug)]
#[allow(unused)]
pub enum AppError {
    /// Bitcoin RPC connection or communication errors
    BitcoinRpcConnection(String),
    /// Bitcoin RPC returned an error response
    BitcoinRpcError { code: i32, message: String },
    /// Failed to parse Bitcoin RPC response
    BitcoinRpcParse(String),
    /// Bitcoin RPC returned no result
    BitcoinRpcNoResult,
    /// Environment configuration errors
    ConfigError(String),
    /// Internal server errors
    Internal(String),
}

impl AppError {
    /// Get the HTTP status code for this error
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::BitcoinRpcConnection(_) => StatusCode::BAD_GATEWAY,
            AppError::BitcoinRpcError { .. } => StatusCode::BAD_GATEWAY,
            AppError::BitcoinRpcParse(_) => StatusCode::BAD_GATEWAY,
            AppError::BitcoinRpcNoResult => StatusCode::BAD_GATEWAY,
            AppError::ConfigError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Get the error type as a string
    fn error_type(&self) -> &str {
        match self {
            AppError::BitcoinRpcConnection(_) => "bitcoin_rpc_connection_error",
            AppError::BitcoinRpcError { .. } => "bitcoin_rpc_error",
            AppError::BitcoinRpcParse(_) => "bitcoin_rpc_parse_error",
            AppError::BitcoinRpcNoResult => "bitcoin_rpc_no_result",
            AppError::ConfigError(_) => "config_error",
            AppError::Internal(_) => "internal_server_error",
        }
    }

    /// Get the error message
    fn message(&self) -> String {
        match self {
            AppError::BitcoinRpcConnection(msg) => {
                format!("Failed to connect to Bitcoin node: {}", msg)
            }
            AppError::BitcoinRpcError { code, message } => {
                format!("Bitcoin RPC error (code {}): {}", code, message)
            }
            AppError::BitcoinRpcParse(msg) => {
                format!("Failed to parse Bitcoin RPC response: {}", msg)
            }
            AppError::BitcoinRpcNoResult => "Bitcoin RPC returned no result".to_string(),
            AppError::ConfigError(msg) => {
                format!("Configuration error: {}", msg)
            }
            AppError::Internal(msg) => {
                format!("Internal server error: {}", msg)
            }
        }
    }

    /// Get additional error details if available
    fn details(&self) -> Option<String> {
        match self {
            AppError::BitcoinRpcError { code, message } => {
                Some(format!("RPC error code: {}, message: {}", code, message))
            }
            _ => None,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let error_response = ErrorResponse {
            error: self.error_type().to_string(),
            message: self.message(),
            details: self.details(),
        };

        (status, Json(error_response)).into_response()
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for AppError {}
