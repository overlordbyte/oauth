use std::sync::Arc;

use async_trait::async_trait;
use tracing::{info, warn};

use crate::domain::{AppError, LoginCommand, Page, Pagination, Port, RegisterCommand, User, UserRequest};

// ── Output port — usecase defines, store implements ───────────────────────────

/// Output port — qarzer: domain/repository/general/UsersRepository
#[async_trait]
pub trait UsersPort: Port {
    async fn find_by_id(&self, id: i32) -> Result<Option<User>, AppError>;
    async fn find_all(&self, pagination: &Pagination, params: &UserRequest) -> Result<Page<User>, AppError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    async fn create(&self, name: String, email: String, password_hash: String) -> Result<User, AppError>;
}

// ── Use case ──────────────────────────────────────────────────────────────────

pub struct UsersUseCase {
    port: Arc<dyn UsersPort>,
}

impl UsersUseCase {
    pub fn new(port: Arc<dyn UsersPort>) -> Self {
        Self { port }
    }

    pub async fn get(&self, id: i32) -> Result<User, AppError> {
        self.port.find_by_id(id).await?.ok_or(AppError::NotFound { entity: "user" })
    }

    pub async fn list(&self, pagination: &Pagination, params: &UserRequest) -> Result<Page<User>, AppError> {
        self.port.find_all(pagination, params).await
    }

    pub async fn register(&self, cmd: RegisterCommand) -> Result<User, AppError> {
        cmd.validate()?;
        info!(email = %cmd.email, "foydalanuvchi ro'yxatdan o'tmoqda");
        let password_hash = hash_password(&cmd.password);
        // ACID: DB UNIQUE constraint on email prevents race conditions
        let user = self.port.create(cmd.name, cmd.email, password_hash).await?;
        info!(id = user.id, "yangi foydalanuvchi yaratildi");
        Ok(user)
    }

    pub async fn authenticate(&self, cmd: LoginCommand) -> Result<User, AppError> {
        info!(email = %cmd.email, "autentifikatsiya urinishi");
        let user = self.port.find_by_email(&cmd.email).await?
            .ok_or(AppError::Unauthorized)?;
        if !verify_password(&cmd.password, &user.password_hash) {
            warn!(email = %cmd.email, "noto'g'ri parol");
            return Err(AppError::Unauthorized);
        }
        Ok(user)
    }
}

fn hash_password(password: &str) -> String {
    use argon2::{Argon2, PasswordHasher};
    use argon2::password_hash::SaltString;
    use rand_core::OsRng;

    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("argon2 xato")
        .to_string()
}

fn verify_password(password: &str, hash: &str) -> bool {
    use argon2::{Argon2, PasswordVerifier};
    use argon2::password_hash::PasswordHash;

    let parsed = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => return false,
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}
