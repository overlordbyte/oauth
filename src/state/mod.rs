use std::sync::Arc;

use crate::usecase::{jwt::JwtService, users::UsersUseCase};

#[derive(Clone)]
pub struct AppState {
    pub users: Arc<UsersUseCase>,
    pub jwt: Arc<JwtService>,
}
