pub mod auth;
pub mod users;

use crate::domain::{AppError, FieldError};

pub(crate) fn to_status(e: AppError) -> tonic::Status {
    use tonic::Status;
    match e {
        AppError::NotFound { entity } =>
            Status::not_found(format!("{entity} topilmadi")),
        AppError::AlreadyExists { entity, field } =>
            Status::already_exists(format!("{entity}: {field} allaqachon mavjud")),
        AppError::Unauthorized =>
            Status::unauthenticated("autentifikatsiya talab qilinadi"),
        AppError::Forbidden =>
            Status::permission_denied("ruxsat yo'q"),
        AppError::Validation(errors) => {
            let msg = errors.iter()
                .map(|FieldError { field, message }| format!("{field}: {message}"))
                .collect::<Vec<_>>()
                .join("; ");
            Status::invalid_argument(msg)
        }
        AppError::ServiceUnavailable(msg) =>
            Status::unavailable(msg),
        AppError::Internal(msg) =>
            Status::internal(msg),
    }
}
