use dioxus::prelude::*;
use crate::ui::pages::{home::HomePage, login::LoginPage};

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    HomePage {},
    #[route("/login")]
    LoginPage {},
}
