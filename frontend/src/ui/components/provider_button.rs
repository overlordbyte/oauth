use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_brands_icons::{FaGithub, FaGitlab, FaGoogle, FaTelegram};

use crate::components::button::{Button, ButtonVariant};
use crate::domain::model::provider::OAuthProvider;
use crate::infrastructure::api::auth_api;

/// Bitta provayder tugmasi. Ikonkalar bir xil o'lchamda va `currentColor` da —
/// shuning uchun dark/light rejimda avtomatik moslashadi.
#[component]
pub fn ProviderButton(provider: OAuthProvider) -> Element {
    let icon = match provider {
        OAuthProvider::Google => rsx! {
            Icon {
                width: 18,
                height: 18,
                class: "provider-icon",
                icon: FaGoogle,
            }
        },
        OAuthProvider::Github => rsx! {
            Icon {
                width: 18,
                height: 18,
                class: "provider-icon",
                icon: FaGithub,
            }
        },
        OAuthProvider::Gitlab => rsx! {
            Icon {
                width: 18,
                height: 18,
                class: "provider-icon",
                icon: FaGitlab,
            }
        },
        OAuthProvider::Telegram => rsx! {
            Icon {
                width: 18,
                height: 18,
                class: "provider-icon",
                icon: FaTelegram,
            }
        },
    };

    rsx! {
        Button {
            class: "provider-button",
            variant: ButtonVariant::Outline,
            r#type: "button",
            onclick: move |_| auth_api::start_oauth(provider),
            {icon}
            "{provider.label()} bilan davom etish"
        }
    }
}
