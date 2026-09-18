use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};

/// Autentifikatsiyadan o'tgan foydalanuvchi konteksti
///
/// Axum handler'larida bevosita ishlatiladi:
///   `async fn handler(principal: UserPrincipal) -> ...`
#[derive(Debug, Clone)]
pub struct UserPrincipal {
    pub user_id: i32,
    pub username: String,
    pub roles: Vec<String>,
}

impl UserPrincipal {
    pub fn new(user_id: i32, username: String, roles: Vec<String>) -> Self {
        Self { user_id, username, roles }
    }

    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }
}

/// Axum FromRequestParts — `Authorization: Bearer <token>` headeridan principal yaratadi.
///
/// `AppState`dan `JwtService` olib tokenni decode qiladi.
/// Handler'da `State<AppState>` ham bo'lishi kerak emas — extractor o'zi oladi.
impl<S> FromRequestParts<S> for UserPrincipal
where
    S: Send + Sync,
    // AppState dan JwtService'ni extract qilish uchun
    crate::state::AppState: axum::extract::FromRef<S>,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        use axum::extract::FromRef;
        use crate::state::AppState;

        let token = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or((StatusCode::UNAUTHORIZED, "Authorization: Bearer header yo'q"))?;

        let app_state = AppState::from_ref(state);
        let claims = app_state.jwt
            .verify_access(token)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Token noto'g'ri yoki muddati o'tgan"))?;

        Ok(UserPrincipal {
            user_id: claims.sub,
            username: claims.username,
            roles: claims.roles,
        })
    }
}
