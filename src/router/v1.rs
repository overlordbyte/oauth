use axum::{Router, middleware, routing::get};

use crate::{controller, middleware::rate_limit, routes, state::AppState};

/// `/api/v1` ostidagi barcha routelar.
///
/// Auth endpointlari alohida, qattiq rate limit bilan o'ralgan —
/// boshqa endpointlar umumiy `api` limitini mod.rs dan meros oladi.
pub fn router() -> Router<AppState> {
    let auth_limiter = rate_limit::auth();

    let auth_routes = Router::new()
        .route(routes::AUTH_LOGIN, get(controller::auth::login))
        .layer(middleware::from_fn(move |req, next| {
            rate_limit::enforce(auth_limiter.clone(), req, next)
        }));

    let user_routes = Router::new()
        .route(routes::USERS, get(controller::users::list))
        .route(routes::USER_BY_ID, get(controller::users::get));

    Router::new().merge(auth_routes).merge(user_routes)
}
