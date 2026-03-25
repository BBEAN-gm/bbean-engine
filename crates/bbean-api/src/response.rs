use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Json<Self> {
        Json(Self {
            success: true,
            data: Some(data),
            error: None,
        })
    }

    pub fn err(msg: impl Into<String>) -> Json<Self> {
        Json(Self {
            success: false,
            data: None,
            error: Some(msg.into()),
        })
    }
}

pub struct ApiError {
    pub status: StatusCode,
    pub message: String,
}

impl ApiError {
    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: msg.into(),
        }
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            message: msg.into(),
        }
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: msg.into(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = serde_json::json!({
            "success": false,
            "error": self.message,
        });
        (self.status, Json(body)).into_response()
    }
}

impl From<bbean_core::EngineError> for ApiError {
    fn from(err: bbean_core::EngineError) -> Self {
        match &err {
            bbean_core::EngineError::TaskNotFound(_)
            | bbean_core::EngineError::NodeNotFound(_) => Self::not_found(err.to_string()),
            bbean_core::EngineError::InvalidTask(_)
            | bbean_core::EngineError::TaskTooLarge { .. }
            | bbean_core::EngineError::DuplicateTaskId(_) => Self::bad_request(err.to_string()),
            _ => Self::internal(err.to_string()),
        }
    }
}
