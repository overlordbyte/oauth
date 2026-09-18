mod application;
mod domain;
mod infrastructure;
mod router;
mod ui;

use dioxus::prelude::*;
use router::Route;

fn main() {
    dioxus_logger::init(dioxus_logger::tracing::Level::INFO).expect("logger ishga tushmadi");
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
