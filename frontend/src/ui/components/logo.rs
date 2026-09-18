use dioxus::prelude::*;

use crate::PROJECT_NAME;

/// Loyiha belgisi: ochiq halqa — OAuth'ning "open authorization" ma'nosi,
/// va halqadagi tirqishda turgan nuqta — uzatilayotgan token. Nuqta ayni paytda
/// nomdagi nuqta vazifasini ham bajaradi.
///
/// Rang `currentColor` dan olinadi, shuning uchun dark/light rejimga o'zi moslashadi.
#[component]
pub fn Logo() -> Element {
    rsx! {
        div { class: "logo",
            svg {
                class: "logo-mark",
                width: "34",
                height: "34",
                view_box: "0 0 24 24",
                fill: "none",
                role: "img",
                "aria-label": PROJECT_NAME,
                path {
                    d: "M18.55 16.59 A 8 8 0 1 1 18.55 7.41",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                }
                circle { cx: "20.5", cy: "12", r: "2.25", fill: "currentColor" }
            }
            span { class: "logo-word", "{PROJECT_NAME}" }
        }
    }
}
