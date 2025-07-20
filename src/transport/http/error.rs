use axum::{Json, http, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::app::error::AppError;

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::UrlNotFound => (http::StatusCode::NOT_FOUND, "Not found".to_owned()),
            AppError::StorageInternalError(_) => (
                http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error. Try again later.".to_owned(),
            ),
            AppError::UrlParseError => (http::StatusCode::BAD_REQUEST, "Invalid Url".to_owned()),
        };
        (status, Json(ErrorResponse { message })).into_response()
    }
}
