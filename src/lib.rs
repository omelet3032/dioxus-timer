pub mod timer;
pub mod settings;

use std::time::Duration;

use dioxus::prelude::*;

use timer::components::Timer;
use timer::logic::use_timer;
use settings::components::SettingsButton;

#[component]
pub fn DioxusTimerDisplay() -> Element {
    /*
       initial_duration과 SettingsUI를 연결
    */
    let initial_duration = Duration::from_secs(10);
    let (timer, tx) = use_timer(initial_duration);
    rsx! {
        div {
            class: "dioxus-timer-display",
            Timer {timer, tx}
            SettingsButton {}
        }

    }
}

