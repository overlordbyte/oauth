/// Ilova konfiguratsiyasi — qarzer: application.yml + @ConfigurationProperties
///
/// Barcha qiymatlar muhit o'zgaruvchilaridan o'qiladi.
pub struct AppConfig {
    pub database_url: String,
    pub http_port: u16,
    pub grpc_port: u16,
    pub cors_origin: String,
    /// JWT imzolash uchun maxfiy kalit (kamida 32 belgi bo'lishi tavsiya etiladi)
    pub jwt_secret: String,
}

impl AppConfig {
    /// Muhit o'zgaruvchilaridan konfiguratsiya yuklaydi.
    /// Majburiy o'zgaruvchilar yo'q bo'lsa, aniq xato xabari chiqaradi.
    pub fn from_env() -> Self {
        Self {
            database_url: required("DATABASE_URL"),
            http_port: optional("HTTP_PORT", 3000),
            grpc_port: optional("GRPC_PORT", 50051),
            cors_origin: std::env::var("CORS_ORIGIN")
                .unwrap_or_else(|_| "https://oauth.uz".to_string()),
            jwt_secret: required("JWT_SECRET"),
        }
    }
}

fn required(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| panic!("{key} muhit o'zgaruvchisi sozlanmagan"))
}

fn optional<T: std::str::FromStr>(key: &str, default: T) -> T
where
    T::Err: std::fmt::Debug,
{
    std::env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}
