pub mod auth;
pub mod users;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::domain::{AppError, FieldError};

/// Xato javobining yagona shakli. Frontend `message` ni ko'rsatadi,
/// `errors` esa maydon bo'yicha validatsiya xatolari uchun.
#[derive(Serialize)]
struct ErrorBody {
    message: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    errors: Vec<FieldErrorBody>,
}

/// Domen `FieldError` ining transport ko'rinishi — domen qatlami
/// serializatsiyadan bexabar qolishi uchun alohida tur.
#[derive(Serialize)]
struct FieldErrorBody {
    field: &'static str,
    message: String,
}

/// Domen xatosini HTTP javobiga o'giradi. Status kodlari `AppError`
/// izohlarida ko'rsatilganidek.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, errors) = match &self {
            AppError::NotFound { .. } => (StatusCode::NOT_FOUND, vec![]),
            AppError::AlreadyExists { .. } => (StatusCode::CONFLICT, vec![]),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, vec![]),
            AppError::Forbidden => (StatusCode::FORBIDDEN, vec![]),
            AppError::Validation(errors) => (
                StatusCode::BAD_REQUEST,
                errors
                    .iter()
                    .map(|FieldError { field, message }| FieldErrorBody {
                        field,
                        message: message.clone(),
                    })
                    .collect(),
            ),
            AppError::ServiceUnavailable(_) => (StatusCode::SERVICE_UNAVAILABLE, vec![]),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, vec![]),
        };

        // Ichki xato matni mijozga chiqmaydi — faqat logga
        if let AppError::Internal(detail) = &self {
            tracing::error!(%detail, "ichki xato");
        }
        let message = match &self {
            AppError::Internal(_) => "ichki xato".to_string(),
            other => other.to_string(),
        };

        (status, Json(ErrorBody { message, errors })).into_response()
    }
}
