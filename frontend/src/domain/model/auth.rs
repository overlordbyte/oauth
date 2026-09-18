use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Ro'yxatdan o'tishning birinchi qadami: ma'lumotlar yuboriladi va emailga
/// tasdiqlash kodi ketadi. Hisob [`VerifyEmailRequest`] dan keyin faollashadi.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

/// Ikkinchi qadam: emailga kelgan kodni tasdiqlash.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerifyEmailRequest {
    pub email: String,
    pub code: String,
}

/// Kodni qayta yuborish so'rovi.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResendCodeRequest {
    pub email: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
}
