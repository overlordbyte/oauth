/// Ilova konfiguratsiyasi
///
/// Barcha qiymatlar muhit o'zgaruvchilaridan o'qiladi.
pub struct AppConfig {
    /// Loyihaning ko'rinadigan nomi — `PROJECT_NAME`
    pub project_name: String,
    /// Kanonik domen (sxemasiz) — `PROJECT_DOMAIN`
    pub project_domain: String,
    pub database_url: String,
    pub http_port: u16,
    pub cors_origin: String,
    /// JWT imzolash uchun maxfiy kalit (kamida 32 belgi bo'lishi tavsiya etiladi)
    pub jwt_secret: String,
}

impl AppConfig {
    /// Muhit o'zgaruvchilaridan konfiguratsiya yuklaydi.
    /// Majburiy o'zgaruvchilar yo'q bo'lsa, aniq xato xabari chiqaradi.
    pub fn from_env() -> Self {
        let project_domain = required("PROJECT_DOMAIN");
        Self {
            // CORS_ORIGIN berilmasa — loyiha domenining https varianti
            cors_origin: std::env::var("CORS_ORIGIN")
                .unwrap_or_else(|_| format!("https://{project_domain}")),
            project_domain,
            project_name: required("PROJECT_NAME"),
            database_url: required("DATABASE_URL"),
            http_port: optional("HTTP_PORT", 3000),
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
