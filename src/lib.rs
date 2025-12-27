pub mod settings;
pub mod timer;

use std::time::Duration;

use dioxus::prelude::*;

use settings::components::SettingsButton;
use timer::components::TimerControls;
use timer::logic::use_timer;

const DIOXUS_TIMER_CSS: Asset = asset!("/assets/dioxus_timer.css");

#[component]
pub fn DioxusTimer() -> Element {
    /*
       initial_duration과 SettingsUI를 연결
    */

    let initial_duration = Duration::from_secs(10);
    let (timer, tx) = use_timer(initial_duration);
    
    rsx! {
        
        document::Stylesheet { href: DIOXUS_TIMER_CSS }
        
        div {
            class: "dioxus-timer",
            
            TimerControls {timer, tx}
            SettingsButton {}
        }

    }
}
