pub mod settings;
pub mod timer;

use std::time::Duration;

use dioxus::prelude::*;

use settings::components::SettingsButton;
use timer::components::TimerControls;
use timer::logic::use_timer;

use crate::settings::components::SettingsControls;

const DIOXUS_TIMER_CSS: Asset = asset!("/assets/dioxus_timer.css");

#[component]
pub fn DioxusTimer() -> Element {

    let initial_duration = Duration::from_secs(1500);
    /* 
        지금은 고정된 값이지만 settings에서 불러와야 한다.
     */

    let (timer, tx) = use_timer(initial_duration);
    
    rsx! {
        
        document::Stylesheet { href: DIOXUS_TIMER_CSS }
        
        div {
            class: "dioxus-timer",
            
            TimerControls {timer, tx}
            // SettingsButton {}
            SettingsControls {}
        }

    }
}
