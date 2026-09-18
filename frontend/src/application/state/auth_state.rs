use dioxus::prelude::*;
use crate::domain::model::auth::AuthTokens;

pub static AUTH: GlobalSignal<Option<AuthTokens>> = Signal::global(|| None);

pub fn is_logged_in() -> bool {
    AUTH.read().is_some()
}

pub fn set_tokens(tokens: AuthTokens) {
    *AUTH.write() = Some(tokens);
}

pub fn clear_tokens() {
    *AUTH.write() = None;
}
