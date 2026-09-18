/// Tashqi autentifikatsiya provayderlari.
///
/// Yangi provayder qo'shish uchun: bu yerga variant qo'shing, `id` va `label`
/// ga bitta qatordan yozing, so'ng `ui/components/provider_button.rs` da
/// ikonkasini ko'rsating. Login sahifasining o'zi o'zgarishsiz qoladi.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OAuthProvider {
    Google,
    Github,
    Gitlab,
    Telegram,
}

impl OAuthProvider {
    /// Login sahifasida ko'rsatiladigan tartib.
    pub const ALL: [Self; 4] = [Self::Google, Self::Github, Self::Gitlab, Self::Telegram];

    /// Backend route'idagi nom: `/api/v1/auth/{id}`
    pub fn id(&self) -> &'static str {
        match self {
            Self::Google => "google",
            Self::Github => "github",
            Self::Gitlab => "gitlab",
            Self::Telegram => "telegram",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Google => "Google",
            Self::Github => "GitHub",
            Self::Gitlab => "GitLab",
            Self::Telegram => "Telegram",
        }
    }
}
