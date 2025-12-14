use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use crate::models::ApiResponse;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    InvalidPath(String),
    PermissionDenied(String),
    IoError(String),
    InvalidRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::InvalidPath(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::PermissionDenied(msg) => (StatusCode::FORBIDDEN, msg),
            AppError::IoError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::InvalidRequest(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        let response: ApiResponse<()> = ApiResponse::error(status.as_u16() as i32, message);
        (status, Json(response)).into_response()
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err.to_string())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::IoError(err.to_string())
    }
}
