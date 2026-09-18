mod v1;

use axum::{Router, middleware, routing::get};

use crate::{controller, middleware::{cors, rate_limit}, routes, state::AppState};

/// To'liq HTTP router — barcha versiyalar va global middleware lar.
///
/// Qatlamlanish tartibi (tashqaridan ichkariga):
///   CORS  →  api rate limit  →  v1 routes (auth rate limit ichkarida)
pub fn build(state: AppState, cors_origin: &str) -> Router {
    let api_limiter = rate_limit::api();

    Router::new()
        .nest(routes::API_V1, v1::router())
        .route(routes::AUTH_CALLBACK, get(controller::auth::callback))
        .layer(middleware::from_fn(move |req, next| {
            rate_limit::enforce(api_limiter.clone(), req, next)
        }))
        .layer(cors::layer(cors_origin))
        .with_state(state)
}
