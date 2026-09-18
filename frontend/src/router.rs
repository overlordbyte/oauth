use dioxus::prelude::*;
use crate::ui::pages::{home::HomePage, login::LoginPage, register::RegisterPage};

// Variant nomi komponent nomi bilan bir xil bo'lishi Dioxus `Routable` ning
// talabi — shuning uchun umumiy `Page` qo'shimchasidan qutulib bo'lmaydi.
#[allow(clippy::enum_variant_names)]
#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    HomePage {},
    #[route("/login")]
    LoginPage {},
    #[route("/register")]
    RegisterPage {},
}
