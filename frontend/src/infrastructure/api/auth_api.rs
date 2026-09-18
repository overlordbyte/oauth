use gloo_net::http::Request;
use serde::Serialize;

use crate::domain::model::auth::{
    AuthTokens, LoginRequest, RegisterRequest, ResendCodeRequest, VerifyEmailRequest,
};
use crate::domain::model::provider::OAuthProvider;

pub async fn login(req: LoginRequest) -> Result<AuthTokens, String> {
    post_json("/api/v1/auth/login", &req).await?.tokens().await
}

/// Birinchi qadam — emailga tasdiqlash kodi yuboriladi, token hali berilmaydi.
pub async fn register(req: RegisterRequest) -> Result<(), String> {
    post_json("/api/v1/auth/register", &req).await.map(|_| ())
}

/// Ikkinchi qadam — kod to'g'ri bo'lsa hisob faollashadi va token qaytadi.
pub async fn verify_email(req: VerifyEmailRequest) -> Result<AuthTokens, String> {
    post_json("/api/v1/auth/verify-email", &req)
        .await?
        .tokens()
        .await
}

pub async fn resend_code(email: String) -> Result<(), String> {
    post_json("/api/v1/auth/resend-code", &ResendCodeRequest { email })
        .await
        .map(|_| ())
}

/// Provayder oqimini boshlaydi — brauzerni backend redirect endpointiga yuboradi.
///
/// OAuth uchun aynan to'liq sahifa navigatsiyasi kerak (fetch emas), chunki
/// foydalanuvchi provayder saytida parolini kiritishi lozim.
pub fn start_oauth(provider: OAuthProvider) {
    let url = format!("/api/v1/auth/{}", provider.id());
    if let Some(window) = web_sys::window() {
        let _ = window.location().set_href(&url);
    }
}

// ── ichki yordamchilar ────────────────────────────────────────────────────────

struct Ok2xx(gloo_net::http::Response);

impl Ok2xx {
    async fn tokens(self) -> Result<AuthTokens, String> {
        self.0.json::<AuthTokens>().await.map_err(|e| e.to_string())
    }
}

/// JSON POST yuboradi va HTTP statusini tekshiradi.
///
/// Xato matnini backend `{"message": "..."}` ko'rinishida qaytarsa o'shani,
/// aks holda status kodini ko'rsatamiz.
async fn post_json<T: Serialize>(url: &str, body: &T) -> Result<Ok2xx, String> {
    let resp = Request::post(url)
        .json(body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        return Ok(Ok2xx(resp));
    }

    let status = resp.status();
    match resp.json::<ApiError>().await {
        Ok(e) => Err(e.message),
        Err(_) => Err(format!("So'rov bajarilmadi ({status})")),
    }
}

#[derive(serde::Deserialize)]
struct ApiError {
    message: String,
}
