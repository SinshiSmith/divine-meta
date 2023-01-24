use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseError {
    message: String,
}

// Make our own error that wraps `anyhow::Error`.
pub enum AppError {
    Anyhow(anyhow::Error),
    UserError {
        code: StatusCode,
        message: Json<ResponseError>,
    },
}

impl AppError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self::UserError {
            code,
            message: Json(ResponseError {
                message: message.into(),
            }),
        }
    }
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Anyhow(anyhow_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {}", anyhow_error),
            )
                .into_response(),
            AppError::UserError { code, message } => (code, message).into_response(),
        }
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::Anyhow(err.into())
    }
}
