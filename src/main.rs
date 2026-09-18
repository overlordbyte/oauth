use std::{net::SocketAddr, sync::Arc};

use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::bb8::Pool;
use oauth::{
    config::AppConfig,
    router,
    state::AppState,
    store::users::PgUsersStore,
    usecase::{jwt::JwtService, users::{UsersPort, UsersUseCase}},
};
use tokio::{net::TcpListener, signal};

#[tokio::main]
async fn main() {
    let app_env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
    dotenvy::from_filename(format!(".env.{app_env}")).ok();
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cfg = AppConfig::from_env();
    tracing::info!("{} ishga tushmoqda", cfg.project_name);

    let pool = Pool::builder()
        .build(AsyncDieselConnectionManager::<AsyncPgConnection>::new(&cfg.database_url))
        .await
        .expect("DB pool yaratib bo'lmadi");

    let users_store: Arc<dyn UsersPort> = Arc::new(PgUsersStore::new(pool));
    let state = AppState {
        users: Arc::new(UsersUseCase::new(users_store)),
        jwt: Arc::new(JwtService::new(&cfg.jwt_secret)),
    };

    let addr = format!("0.0.0.0:{}", cfg.http_port);
    let listener = TcpListener::bind(&addr).await.expect("portni band qilib bo'lmadi");
    tracing::info!("HTTP listening on {}", listener.local_addr().unwrap());

    axum::serve(
        listener,
        // ConnectInfo: rate limiting uchun IP manzilni olish imkonini beradi
        router::build(state, &cfg.cors_origin).into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .expect("HTTP server xatosi");
}
async fn shutdown_signal() {
    signal::ctrl_c().await.unwrap();
}
