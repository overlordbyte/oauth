use dioxus::prelude::*;

use crate::components::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::ui::components::logo::Logo;

/// Kirish va ro'yxatdan o'tish sahifalarining umumiy qobig'i:
/// markazlashtirilgan karta, tepasida logotip.
#[component]
pub fn AuthLayout(title: String, description: String, children: Element) -> Element {
    rsx! {
        main { class: "auth-page",
            Logo {}

            div { class: "auth-card",
                Card {
                    CardHeader {
                        CardTitle { "{title}" }
                        CardDescription { "{description}" }
                    }
                    CardContent {
                        div { class: "auth-body", {children} }
                    }
                }
            }
        }
    }
}
