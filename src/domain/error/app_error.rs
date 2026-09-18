use std::fmt;

use super::FieldError;

/// Unified domain error — qarzer: BaseException ierarxiyasi
#[derive(Debug)]
pub enum AppError {
    /// 404 — NOT_FOUND  (qarzer: ResourceNotFoundException)
    NotFound { entity: &'static str },
    /// 409 — ALREADY_EXISTS  (qarzer: ConflictException)
    AlreadyExists { entity: &'static str, field: &'static str },
    /// 401 — UNAUTHORIZED  (qarzer: UnauthorizedException)
    Unauthorized,
    /// 403 — FORBIDDEN  (qarzer: ForbiddenException)
    Forbidden,
    /// 400 — INVALID_ARGUMENT  (qarzer: BadRequestException)
    Validation(Vec<FieldError>),
    /// 503 — UNAVAILABLE  (qarzer: ServiceUnavailableException)
    ServiceUnavailable(String),
    /// 500 — INTERNAL  (qarzer: InternalServerException)
    Internal(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound { entity } => write!(f, "{entity} topilmadi"),
            Self::AlreadyExists { entity, field } => write!(f, "{entity}: {field} allaqachon mavjud"),
            Self::Unauthorized => write!(f, "autentifikatsiya talab qilinadi"),
            Self::Forbidden => write!(f, "ruxsat yo'q"),
            Self::Validation(errors) => {
                let msg: Vec<_> = errors.iter().map(|e| format!("{}: {}", e.field, e.message)).collect();
                write!(f, "validatsiya xatolari: {}", msg.join("; "))
            }
            Self::ServiceUnavailable(msg) => write!(f, "xizmat mavjud emas: {msg}"),
            Self::Internal(msg) => write!(f, "ichki xato: {msg}"),
        }
    }
}

impl std::error::Error for AppError {}
