use crate::domain::error::{AppError, FieldError};

/// Ro'yxatdan o'tish buyrug'i — qarzer: domain/dtos/request/RegisterRequest
pub struct RegisterCommand {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl RegisterCommand {
    pub fn validate(&self) -> Result<(), AppError> {
        let mut errors = vec![];
        if self.name.trim().is_empty() {
            errors.push(FieldError { field: "name", message: "bo'sh bo'lmasligi kerak".into() });
        }
        if !self.email.contains('@') {
            errors.push(FieldError { field: "email", message: "noto'g'ri format".into() });
        }
        if self.password.len() < 8 {
            errors.push(FieldError { field: "password", message: "kamida 8 ta belgi".into() });
        }
        if errors.is_empty() { Ok(()) } else { Err(AppError::Validation(errors)) }
    }
}
