use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

use crate::domain::{AppError, Claims, TokenPair, User};

/// JWT token yaratish va tekshirish xizmati — qarzer: JwtService / TokenService
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    /// Access token muddati (soniyalarda), standart: 15 daqiqa
    access_ttl_secs: u64,
    /// Refresh token muddati (soniyalarda), standart: 7 kun
    refresh_ttl_secs: u64,
}

impl JwtService {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            access_ttl_secs: 15 * 60,
            refresh_ttl_secs: 7 * 24 * 60 * 60,
        }
    }

    /// Foydalanuvchi uchun access + refresh token juftligi yaratadi
    pub fn issue(&self, user: &User, roles: Vec<String>) -> Result<TokenPair, AppError> {
        let now = now_secs();
        let access_exp = now + self.access_ttl_secs;
        let refresh_exp = now + self.refresh_ttl_secs;

        let access_claims = Claims {
            sub: user.id,
            username: user.name.clone(),
            roles: roles.clone(),
            exp: access_exp,
            iat: now,
            token_type: "access".into(),
        };
        let refresh_claims = Claims {
            sub: user.id,
            username: user.name.clone(),
            roles,
            exp: refresh_exp,
            iat: now,
            token_type: "refresh".into(),
        };

        let access_token = encode(&Header::default(), &access_claims, &self.encoding_key)
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let refresh_token = encode(&Header::default(), &refresh_claims, &self.encoding_key)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(TokenPair { access_token, refresh_token, expires_in: self.access_ttl_secs })
    }

    /// Access token'ni tekshirib Claims qaytaradi
    pub fn verify_access(&self, token: &str) -> Result<Claims, AppError> {
        let data = decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map_err(|_| AppError::Unauthorized)?;

        if data.claims.token_type != "access" {
            return Err(AppError::Unauthorized);
        }
        Ok(data.claims)
    }

    /// Refresh token'ni tekshirib Claims qaytaradi
    pub fn verify_refresh(&self, token: &str) -> Result<Claims, AppError> {
        let data = decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map_err(|_| AppError::Unauthorized)?;

        if data.claims.token_type != "refresh" {
            return Err(AppError::Unauthorized);
        }
        Ok(data.claims)
    }
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("vaqt xatosi")
        .as_secs()
}
