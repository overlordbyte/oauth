use dioxus::prelude::*;
use crate::{
    application::state::auth_state::set_tokens,
    components::{
        button::{Button, ButtonSize, ButtonVariant},
        input::Input,
        label::Label,
    },
    domain::model::auth::{RegisterRequest, VerifyEmailRequest},
    infrastructure::api::auth_api,
    router::Route,
    ui::layouts::auth_layout::AuthLayout,
};

/// Emailga yuboriladigan tasdiqlash kodi uzunligi.
const CODE_LEN: usize = 6;

/// Ro'yxatdan o'tish ikki qadamda: forma to'ldiriladi → emailga kod ketadi →
/// kod tasdiqlanadi. Ikkala qadam ham bitta komponentda, chunki ikkinchi qadamga
/// birinchisidan kelgan email kerak.
#[derive(Clone, Copy, PartialEq)]
enum Step {
    Form,
    Verify,
}

#[component]
pub fn RegisterPage() -> Element {
    let step = use_signal(|| Step::Form);
    let email = use_signal(String::new);

    match *step.read() {
        Step::Form => rsx! { RegisterForm { step, email } },
        Step::Verify => rsx! { VerifyForm { step, email } },
    }
}

#[component]
fn RegisterForm(step: Signal<Step>, email: Signal<String>) -> Element {
    let mut step = step;
    let mut email = email;
    let mut first_name = use_signal(String::new);
    let mut last_name = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut password_repeat = use_signal(String::new);
    let mut error = use_signal(|| Option::<String>::None);
    let mut loading = use_signal(|| false);

    let on_submit = move |evt: FormEvent| {
        evt.prevent_default();

        if password.read().as_str() != password_repeat.read().as_str() {
            error.set(Some("Parollar mos kelmadi".to_string()));
            return;
        }

        let req = RegisterRequest {
            first_name: first_name.read().clone(),
            last_name: last_name.read().clone(),
            email: email.read().clone(),
            password: password.read().clone(),
        };
        spawn(async move {
            loading.set(true);
            error.set(None);
            match auth_api::register(req).await {
                Ok(()) => step.set(Step::Verify),
                Err(e) => error.set(Some(e)),
            }
            loading.set(false);
        });
    };

    rsx! {
        AuthLayout {
            title: "Hisob yarating",
            description: "Email va parol bilan ro'yxatdan o'ting",

            form { class: "auth-form", onsubmit: on_submit,
                div { class: "auth-row",
                    div { class: "auth-field",
                        Label { html_for: "first_name", "Ism" }
                        Input {
                            id: "first_name",
                            r#type: "text",
                            autocomplete: "given-name",
                            value: "{first_name}",
                            oninput: move |e: FormEvent| first_name.set(e.value()),
                        }
                    }
                    div { class: "auth-field",
                        Label { html_for: "last_name", "Familiya" }
                        Input {
                            id: "last_name",
                            r#type: "text",
                            autocomplete: "family-name",
                            value: "{last_name}",
                            oninput: move |e: FormEvent| last_name.set(e.value()),
                        }
                    }
                }

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
                        autocomplete: "new-password",
                        value: "{password}",
                        oninput: move |e: FormEvent| password.set(e.value()),
                    }
                }

                div { class: "auth-field",
                    Label { html_for: "password_repeat", "Parolni takrorlang" }
                    Input {
                        id: "password_repeat",
                        r#type: "password",
                        autocomplete: "new-password",
                        value: "{password_repeat}",
                        oninput: move |e: FormEvent| password_repeat.set(e.value()),
                    }
                }

                if let Some(err) = error.read().as_ref() {
                    p { class: "auth-error", role: "alert", "{err}" }
                }

                Button {
                    r#type: "submit",
                    size: ButtonSize::Lg,
                    disabled: *loading.read(),
                    if *loading.read() { "Yuborilmoqda..." } else { "Davom etish" }
                }
            }

            p { class: "auth-alt",
                "Hisobingiz bormi? "
                Link { to: Route::LoginPage {}, "Kirish" }
            }
        }
    }
}

/// Kod katagiga fokusni ko'chiradi. Dioxus'da element referensini uzatishdan
/// ko'ra `id` bo'yicha topish soddaroq, chunki kataklar bir xil shablonda.
fn focus_code_box(index: usize) {
    use wasm_bindgen::JsCast;

    let Some(document) = web_sys::window().and_then(|w| w.document()) else {
        return;
    };
    if let Some(element) = document.get_element_by_id(&format!("code-{index}"))
        && let Ok(element) = element.dyn_into::<web_sys::HtmlElement>()
    {
        let _ = element.focus();
    }
}

#[component]
fn VerifyForm(step: Signal<Step>, email: Signal<String>) -> Element {
    let mut step = step;
    let mut digits = use_signal(|| vec![String::new(); CODE_LEN]);
    let mut error = use_signal(|| Option::<String>::None);
    let mut notice = use_signal(|| Option::<String>::None);
    let mut loading = use_signal(|| false);
    let nav = use_navigator();

    let on_submit = move |evt: FormEvent| {
        evt.prevent_default();

        let code = digits.read().concat();
        if code.chars().count() != CODE_LEN {
            error.set(Some(format!("Kodning {CODE_LEN} ta raqamini ham kiriting")));
            return;
        }

        let req = VerifyEmailRequest {
            email: email.read().clone(),
            code,
        };
        spawn(async move {
            loading.set(true);
            error.set(None);
            notice.set(None);
            match auth_api::verify_email(req).await {
                Ok(tokens) => {
                    set_tokens(tokens);
                    nav.push(Route::HomePage {});
                }
                Err(e) => error.set(Some(e)),
            }
            loading.set(false);
        });
    };

    let on_resend = move |_| {
        let address = email.read().clone();
        spawn(async move {
            error.set(None);
            match auth_api::resend_code(address).await {
                Ok(()) => notice.set(Some("Yangi kod yuborildi".to_string())),
                Err(e) => error.set(Some(e)),
            }
        });
    };

    rsx! {
        AuthLayout {
            title: "Emailingizni tasdiqlang",
            description: format!("{} manziliga {CODE_LEN} xonali kod yubordik", email.read()),

            form { class: "auth-form", onsubmit: on_submit,
                div { class: "auth-field",
                    Label { html_for: "code-0", "Tasdiqlash kodi" }
                    div { class: "auth-code-boxes",
                        for index in 0..CODE_LEN {
                            Input {
                                key: "{index}",
                                id: "code-{index}",
                                class: "auth-code-box",
                                r#type: "text",
                                inputmode: "numeric",
                                // Faqat birinchi katak — brauzer kodni bir marta taklif qiladi
                                autocomplete: if index == 0 { "one-time-code" } else { "off" },
                                value: "{digits.read()[index]}",
                                oninput: move |e: FormEvent| {
                                    let typed: String = e.value().chars().filter(char::is_ascii_digit).collect();

                                    if typed.is_empty() {
                                        digits.write()[index] = String::new();
                                        return;
                                    }

                                    // Bir nechta raqam kelsa (paste yoki tez yozish) —
                                    // shu katakdan boshlab taqsimlaymiz
                                    let mut cursor = index;
                                    {
                                        let mut slots = digits.write();
                                        for ch in typed.chars() {
                                            if cursor >= CODE_LEN {
                                                break;
                                            }
                                            slots[cursor] = ch.to_string();
                                            cursor += 1;
                                        }
                                    }
                                    focus_code_box(cursor.min(CODE_LEN - 1));
                                },
                                onkeydown: move |e: KeyboardEvent| {
                                    // Bo'sh katakda Backspace — oldingisiga qaytamiz
                                    if e.key() == Key::Backspace
                                        && index > 0
                                        && digits.read()[index].is_empty()
                                    {
                                        focus_code_box(index - 1);
                                    }
                                },
                            }
                        }
                    }
                }

                if let Some(err) = error.read().as_ref() {
                    p { class: "auth-error", role: "alert", "{err}" }
                }
                if let Some(msg) = notice.read().as_ref() {
                    p { class: "auth-notice", role: "status", "{msg}" }
                }

                Button {
                    r#type: "submit",
                    size: ButtonSize::Lg,
                    disabled: *loading.read(),
                    if *loading.read() { "Tekshirilmoqda..." } else { "Tasdiqlash" }
                }
            }

            p { class: "auth-alt",
                "Kod kelmadimi? "
                button { r#type: "button", onclick: on_resend, "Qayta yuborish" }
            }

            Button {
                variant: ButtonVariant::Ghost,
                r#type: "button",
                onclick: move |_| step.set(Step::Form),
                "Boshqa email bilan"
            }
        }
    }
}
