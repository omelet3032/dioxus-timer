use dioxus::prelude::*;

use dioxus_timer::DioxusTimer;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {

        div {
            class: "app-container",
            DioxusTimer {}
        }
    }
}
