use dioxus::prelude::*;

use std::time::Duration;

// use dioxus_timer::settings::components;
use dioxus_timer::timer::components::Timer;
use dioxus_timer::timer::logic::use_timer;
use dioxus_timer::settings::components::SettingsButton;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }

        div {
            class: "app-container",
            DioxusTimerDisplay {}
        }
    }
}

#[component]
fn DioxusTimerDisplay() -> Element {
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

// type TimerValue = (Signal<DioxusTimer>, Coroutine<DioxusTimerCommand>);

// timer는 signal과 코루틴을 튜플로 반환해야 한다

// 설정 버튼뿐만 아니라 설정 화면도 만들어야 한다.
// Settings UI
