//! IP asosida so'rovlar chastotasini cheklash (rate limiting).
//!
//! Ikki daraja:
//! - [`auth`]  — autentifikatsiya endpointlari uchun qattiq limit
//!   (brute-force va credential stuffing hujumlaridan himoya).
//! - [`api`]   — qolgan API endpointlari uchun yumshoq limit.
//!
//! Arxitektura: `middleware` qatlami — domain/usecase ga mutlaqo bog'liq emas,
//! faqat transport (HTTP) qatlamiga tegishli cross-cutting concern.

use std::{
    net::{IpAddr, SocketAddr},
    num::NonZeroU32,
    sync::Arc,
};

use axum::{
    extract::{ConnectInfo, Request},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use governor::{DefaultKeyedRateLimiter, Quota, RateLimiter};

/// IP manzil bo'yicha cheklovchi — dastur davomida bir marta yaratiladi,
/// `Arc` orqali routerlar orasida bo'lishiladi.
pub type IpRateLimiter = Arc<DefaultKeyedRateLimiter<IpAddr>>;

/// Auth endpointlari uchun: **5 so'rov/daqiqa** per IP.
///
/// Login, register, refresh — brute-force hujumlariga nishon.
/// Burst 5 ta ruxsat etiladi (bir vaqtda bir foydalanuvchi bir nechta
/// qurilmadan kirishi mumkin), keyin taxminan 12 soniyada bir token.
pub fn auth() -> IpRateLimiter {
    let quota = Quota::per_minute(NonZeroU32::new(5).unwrap());
    Arc::new(RateLimiter::keyed(quota))
}

/// Umumiy API uchun: **120 so'rov/daqiqa** per IP.
///
/// Normal foydalanuvchi uchun etarli, DDoS dan asosiy himoya.
pub fn api() -> IpRateLimiter {
    let quota =
        Quota::per_minute(NonZeroU32::new(120).unwrap()).allow_burst(NonZeroU32::new(20).unwrap());
    Arc::new(RateLimiter::keyed(quota))
}

/// Tower/Axum middleware: limitni tekshirib, oshib ketsa `429` qaytaradi.
///
/// IP manzilni `ConnectInfo<SocketAddr>` extensiondan oladi —
/// buning uchun server `into_make_service_with_connect_info` bilan
/// ishga tushirilishi shart (main.rs da sozlangan).
pub async fn enforce(limiter: IpRateLimiter, request: Request, next: Next) -> Response {
    let ip = request
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|info| info.0.ip())
        .unwrap_or(IpAddr::from([0, 0, 0, 0]));

    match limiter.check_key(&ip) {
        Ok(_) => next.run(request).await,
        Err(_) => {
            tracing::warn!(%ip, "rate limit oshdi — 429 qaytarilmoqda");
            (
                StatusCode::TOO_MANY_REQUESTS,
                [("Retry-After", "60")],
                "Juda ko'p so'rov. Biroz kuting.",
            )
                .into_response()
        }
    }
}
