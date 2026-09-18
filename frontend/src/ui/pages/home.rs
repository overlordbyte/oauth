use dioxus::prelude::*;
use crate::{
    application::state::auth_state::{clear_tokens, is_logged_in},
    router::Route,
};

#[component]
pub fn HomePage() -> Element {
    let nav = use_navigator();

    // Token bo'lmasa (yoki chiqilgach) — login sahifasiga qaytaramiz
    use_effect(move || {
        if !is_logged_in() {
            nav.replace(Route::LoginPage {});
        }
    });

    let on_logout = move |_| {
        clear_tokens();
        nav.push(Route::LoginPage {});
    };

    rsx! {
        div { style: "padding:2rem;",
            h1 { style: "font-size:1.5rem;margin-bottom:1rem;", "Bosh sahifa" }
            button {
                style: "padding:.5rem 1rem;background:#dc2626;color:#fff;border:none;border-radius:4px;cursor:pointer;",
                onclick: on_logout,
                "Chiqish"
            }
        }
    }
}
