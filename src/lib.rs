pub mod settings;
pub mod timer;

use std::time::Duration;

use dioxus::prelude::*;

use timer::components::TimerControls;
use timer::logic::use_timer;

use crate::settings::components::SettingsControls;
use crate::settings::data::TimerState;

const DIOXUS_TIMER_CSS: Asset = asset!("/assets/dioxus_timer.css");

#[component]
pub fn DioxusTimer() -> Element {
    // let initial_duration = Duration::from_secs(1500);
    let duration = TimerState {
        duration: use_signal(|| 1500)
    };
    /*
        초기값은 고정이고
    */
    use_context_provider(|| duration);

    let (timer, tx) = use_timer(duration);

    rsx! {

        document::Stylesheet { href: DIOXUS_TIMER_CSS }

        div {
            class: "dioxus-timer",

            TimerControls {timer, tx}
            SettingsControls {}
        }

    }
}
