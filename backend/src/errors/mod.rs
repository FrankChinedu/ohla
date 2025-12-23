use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::responses::ApiResponse;

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
    /// Resource not found
    NotFound(String),
    /// Database errors
    DatabaseError(String),
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
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
            AppError::NotFound(_) => "not_found",
            AppError::DatabaseError(_) => "database_error",
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
            AppError::NotFound(msg) => msg.clone(),
            AppError::DatabaseError(msg) => {
                format!("Database error: {}", msg)
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
        let error_response = ApiResponse::<()>::error(
            status,
            self.error_type(),
            self.message(),
            self.details(),
        );

        error_response.into_response()
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for AppError {}

// Conversion from database errors
impl From<crate::db::DbError> for AppError {
    fn from(err: crate::db::DbError) -> Self {
        match err {
            crate::db::DbError::NotFound => AppError::NotFound(err.to_string()),
            crate::db::DbError::DatabaseError(msg) => AppError::DatabaseError(msg),
            crate::db::DbError::InvalidInput(msg) => AppError::DatabaseError(msg),
        }
    }
}
