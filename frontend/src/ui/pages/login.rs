use dioxus::prelude::*;
use crate::{
    application::state::auth_state::set_tokens,
    components::{
        button::{Button, ButtonSize},
        input::Input,
        label::Label,
        separator::Separator,
    },
    domain::model::{auth::LoginRequest, provider::OAuthProvider},
    infrastructure::api::auth_api,
    router::Route,
    ui::{components::provider_button::ProviderButton, layouts::auth_layout::AuthLayout},
};

#[component]
pub fn LoginPage() -> Element {
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut error = use_signal(|| Option::<String>::None);
    let mut loading = use_signal(|| false);
    let nav = use_navigator();

    let on_submit = move |evt: FormEvent| {
        evt.prevent_default();
        let req = LoginRequest {
            email: email.read().clone(),
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
        AuthLayout {
            title: "Hisobingizga kiring",
            description: "Davom etish uchun usulni tanlang",

            div { class: "auth-providers",
                for provider in OAuthProvider::ALL {
                    ProviderButton { key: "{provider.id()}", provider }
                }
            }

            div { class: "auth-divider",
                Separator { horizontal: true, decorative: true }
                span { class: "auth-divider-text", "yoki" }
                Separator { horizontal: true, decorative: true }
            }

            form { class: "auth-form", onsubmit: on_submit,
                div { class: "auth-field",
                    Label { html_for: "email", "Email" }
                    Input {
                        id: "email",
                        r#type: "email",
                        autocomplete: "email",
                        placeholder: "siz@example.com",
                        value: "{email}",
                        oninput: move |e: FormEvent| email.set(e.value()),
                    }
                }
                div { class: "auth-field",
                    Label { html_for: "password", "Parol" }
                    Input {
                        id: "password",
                        r#type: "password",
                        autocomplete: "current-password",
                        value: "{password}",
                        oninput: move |e: FormEvent| password.set(e.value()),
                    }
                }

                if let Some(err) = error.read().as_ref() {
                    p { class: "auth-error", role: "alert", "{err}" }
                }

                p { class: "auth-alt",
                    "Hisobingiz yo'qmi? "
                    Link { to: Route::RegisterPage {}, "Ro'yxatdan o'ting" }
                }

                Button {
                    r#type: "submit",
                    size: ButtonSize::Lg,
                    disabled: *loading.read(),
                    if *loading.read() { "Tekshirilmoqda..." } else { "Kirish" }
                }
            }
        }
    }
}
