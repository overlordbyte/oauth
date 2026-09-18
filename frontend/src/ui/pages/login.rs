use dioxus::prelude::*;
use crate::{
    application::state::auth_state::set_tokens,
    domain::model::auth::LoginRequest,
    infrastructure::api::auth_api,
    router::Route,
};

#[component]
pub fn LoginPage() -> Element {
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut error = use_signal(|| Option::<String>::None);
    let mut loading = use_signal(|| false);
    let nav = use_navigator();

    let on_submit = move |evt: FormEvent| {
        evt.prevent_default();
        let req = LoginRequest {
            username: username.read().clone(),
            password: password.read().clone(),
        };
        spawn(async move {
            loading.set(true);
            error.set(None);
            match auth_api::login(req).await {
                Ok(tokens) => {
                    set_tokens(tokens);
                    nav.push(Route::HomePage {});
                }
                Err(e) => error.set(Some(e)),
            }
            loading.set(false);
        });
    };

    rsx! {
        div {
            style: "display:flex;align-items:center;justify-content:center;min-height:100vh;",
            div {
                style: "background:#fff;padding:2rem;border-radius:8px;box-shadow:0 2px 8px rgba(0,0,0,.1);width:360px;",
                h1 { style: "margin-bottom:1.5rem;font-size:1.5rem;", "Kirish" }
                if let Some(err) = error.read().as_ref() {
                    p { style: "color:red;margin-bottom:1rem;font-size:.875rem;", "{err}" }
                }
                form { onsubmit: on_submit,
                    div { style: "margin-bottom:1rem;",
                        label { style: "display:block;margin-bottom:.25rem;font-size:.875rem;", "Foydalanuvchi nomi" }
                        input {
                            style: "width:100%;padding:.5rem;border:1px solid #d1d5db;border-radius:4px;",
                            r#type: "text",
                            value: "{username}",
                            oninput: move |e| username.set(e.value()),
                        }
                    }
                    div { style: "margin-bottom:1.5rem;",
                        label { style: "display:block;margin-bottom:.25rem;font-size:.875rem;", "Parol" }
                        input {
                            style: "width:100%;padding:.5rem;border:1px solid #d1d5db;border-radius:4px;",
                            r#type: "password",
                            value: "{password}",
                            oninput: move |e| password.set(e.value()),
                        }
                    }
                    button {
                        style: "width:100%;padding:.625rem;background:#2563eb;color:#fff;border:none;border-radius:4px;cursor:pointer;font-size:1rem;",
                        r#type: "submit",
                        disabled: *loading.read(),
                        if *loading.read() { "Yuklanmoqda..." } else { "Kirish" }
                    }
                }
            }
        }
    }
}
