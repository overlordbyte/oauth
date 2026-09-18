use serde::{Deserialize, Serialize};

/// JWT token ichidagi ma'lumotlar — RFC 7519 standart claim'lari
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject — foydalanuvchi ID
    pub sub: i32,
    /// Username
    pub username: String,
    /// Roles
    pub roles: Vec<String>,
    /// Expiration time (Unix timestamp)
    pub exp: u64,
    /// Issued at (Unix timestamp)
    pub iat: u64,
    /// Token type: "access" yoki "refresh"
    pub token_type: String,
}
