use std::{net::SocketAddr, sync::Arc};

use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::bb8::Pool;
use oauth::{
    config::AppConfig,
    controller, middleware,
    router,
    state::AppState,
    store::users::PgUsersStore,
    usecase::{jwt::JwtService, users::{UsersPort, UsersUseCase}},
};
use tokio::{net::TcpListener, signal};
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;

#[tokio::main]
async fn main() {
    let app_env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
    dotenvy::from_filename(format!(".env.{app_env}")).ok();
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cfg = AppConfig::from_env();

    let pool = Pool::builder()
        .build(AsyncDieselConnectionManager::<AsyncPgConnection>::new(&cfg.database_url))
        .await
        .expect("DB pool yaratib bo'lmadi");

    let users_store: Arc<dyn UsersPort> = Arc::new(PgUsersStore::new(pool));
    let state = AppState {
        users: Arc::new(UsersUseCase::new(users_store)),
        jwt: Arc::new(JwtService::new(&cfg.jwt_secret)),
    };

    let http = {
        let state = state.clone();
        let cors_origin = cfg.cors_origin.clone();
        let http_port = cfg.http_port;
        async move {
            let addr = format!("0.0.0.0:{http_port}");
            let listener = TcpListener::bind(&addr).await.unwrap();
            tracing::info!("HTTP  listening on {}", listener.local_addr().unwrap());
            axum::serve(
                listener,
                // ConnectInfo: rate limiting uchun IP manzilni olish imkonini beradi
                router::build(state, &cors_origin)
                    .into_make_service_with_connect_info::<SocketAddr>(),
            )
            .await
            .unwrap();
        }
    };

    // gRPC uchun CORS — middleware modulidan
    let grpc_cors = middleware::cors::layer(&cfg.cors_origin);
    let grpc_addr = format!("0.0.0.0:{}", cfg.grpc_port).parse().unwrap();
    let grpc = async move {
        tracing::info!("gRPC  listening on {grpc_addr}");
        Server::builder()
            .accept_http1(true)
            .layer(grpc_cors)
            .layer(GrpcWebLayer::new())
            .add_service(controller::users::service(state))
            .serve(grpc_addr)
            .await
            .unwrap();
    };

    tokio::select! {
        _ = http => {},
        _ = grpc => {},
        _ = shutdown_signal() => {},
    }
}

async fn shutdown_signal() {
    signal::ctrl_c().await.unwrap();
}
