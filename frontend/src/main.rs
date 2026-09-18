mod application;
mod components;
mod domain;
mod infrastructure;
mod router;
mod ui;

use dioxus::prelude::*;
use router::Route;

/// Barcha CSS `with_static_head(true)` bilan ro'yxatga olinadi — shunda dx ularni
/// index.html ning `<head>` iga statik `<link>` qilib yozadi va brauzer sahifani
/// chizishdan oldin yuklab oladi.
///
/// Diqqat: bu fayllar ataylab `#[css_module]` emas. css_module `<link>` ni
/// klass nomi birinchi marta render bo'lganda qo'shadi — natijada element
/// stilsiz bir zum ko'rinib ketadi (FOUC).
macro_rules! static_css {
    ($name:ident, $path:literal) => {
        #[allow(dead_code)]
        const $name: Asset = asset!($path, AssetOptions::css().with_static_head(true));
    };
}

// Global: rang tokenlari va bazaviy stillar
static_css!(THEME_CSS, "/assets/dx-components-theme.css");
static_css!(BASE_CSS, "/assets/main.css");

// dx-components
static_css!(BUTTON_CSS, "/src/components/button/style.css");
static_css!(CARD_CSS, "/src/components/card/style.css");
static_css!(INPUT_CSS, "/src/components/input/style.css");
static_css!(LABEL_CSS, "/src/components/label/style.css");
static_css!(SEPARATOR_CSS, "/src/components/separator/style.css");

// Sahifa va o'z komponentlarimiz
static_css!(AUTH_CSS, "/src/ui/pages/auth.css");
static_css!(LOGO_CSS, "/src/ui/components/logo.css");
static_css!(PROVIDER_BUTTON_CSS, "/src/ui/components/provider_button.css");

const FAVICON: Asset = asset!("/assets/favicon.svg");

/// Loyihaning ko'rinadigan nomi — `.env.{APP_ENV}` dagi `PROJECT_NAME`.
/// Qiymatni build.rs kompilyatsiya vaqtida kiritadi.
pub const PROJECT_NAME: &str = env!("PROJECT_NAME");

/// Kanonik domen — `.env.{APP_ENV}` dagi `PROJECT_DOMAIN`.
/// Hozircha faqat backend ishlatadi; frontendda kerak bo'lsa shu yerda tayyor.
#[allow(dead_code)]
pub const PROJECT_DOMAIN: &str = env!("PROJECT_DOMAIN");

fn main() {
    dioxus_logger::init(dioxus_logger::tracing::Level::INFO).expect("logger ishga tushmadi");
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Title { "{PROJECT_NAME}" }
        document::Link { rel: "icon", href: FAVICON }
        // Stillar bu yerda emas — hammasi <head> da statik <link> sifatida.
        Router::<Route> {}
    }
}
