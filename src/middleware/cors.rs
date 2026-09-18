//! CORS (Cross-Origin Resource Sharing) tekshiruvi va sozlamasi.
//!
//! `tower-http` ning `CorsLayer` ini clean arch usulida o'raydi:
//! barcha konfiguratsiya bir joyda, router va main.rs bexabar.
//!
//! Qoidalar:
//! - Faqat ruxsat etilgan origin qabul qilinadi (`CORS_ORIGIN` env)
//! - Preflight (`OPTIONS`) so'rovlari avtomatik javob oladi
//! - Credentials (cookie, Authorization) ruxsat etiladi
//! - Yo'q origin (server-to-server) ham o'tkaziladi

use axum::http::{
    HeaderValue, Method,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
};
use tower_http::cors::{AllowOrigin, CorsLayer};

/// Ruxsat etilgan HTTP metodlar ro'yxati.
const ALLOWED_METHODS: [Method; 5] = [
    Method::GET,
    Method::POST,
    Method::PUT,
    Method::DELETE,
    Method::OPTIONS,
];

/// Berilgan origin uchun CORS layer yaratadi.
///
/// # Panic
/// `origin` noto'g'ri `HeaderValue` formatida bo'lsa panic qiladi —
/// bu konfiguratsiya xatosi bo'lgani uchun dastur ishga tushmasligi kerak.
pub fn layer(origin: &str) -> CorsLayer {
    let allowed = origin
        .parse::<HeaderValue>()
        .unwrap_or_else(|_| panic!("CORS_ORIGIN noto'g'ri format: '{origin}'"));

    CorsLayer::new()
        .allow_origin(AllowOrigin::exact(allowed))
        .allow_methods(ALLOWED_METHODS)
        .allow_headers([CONTENT_TYPE, ACCEPT, AUTHORIZATION])
        .allow_credentials(true)
        .max_age(std::time::Duration::from_secs(3600)) // preflight cache: 1 soat
}
