use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use chrono::Utc;

/// Generic API response wrapper for all endpoints
/// Provides consistent structure with status, data, and message fields
#[derive(Serialize)]
pub struct ApiResponse<T> {
    /// HTTP status code
    pub status: u16,
    /// Response data (null for errors)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    /// Human-readable message
    pub message: String,
    /// Optional error details (only present on error responses)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Optional additional details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// ISO 8601 timestamp
    pub timestamp: String,
}

impl<T: Serialize> ApiResponse<T> {
    /// Create a successful response
    pub fn success(data: T, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::OK.as_u16(),
            data: Some(data),
            message: message.into(),
            error: None,
            details: None,
            timestamp: Utc::now().to_rfc3339(),
        }
    }

    /// Create a successful response with custom status code
    #[allow(dead_code)]
    pub fn success_with_status(status: StatusCode, data: T, message: impl Into<String>) -> Self {
        Self {
            status: status.as_u16(),
            data: Some(data),
            message: message.into(),
            error: None,
            details: None,
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}

impl ApiResponse<()> {
    /// Create an error response
    pub fn error(
        status: StatusCode,
        error: impl Into<String>,
        message: impl Into<String>,
        details: Option<String>,
    ) -> Self {
        Self {
            status: status.as_u16(),
            data: None,
            message: message.into(),
            error: Some(error.into()),
            details,
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self)).into_response()
    }
}
