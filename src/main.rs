use dioxus::prelude::*;
use dioxus_timer::{DioxusTimer, DioxusTimerCommand, DioxusTimerState};

use futures_util::StreamExt;
use std::time::Duration;
// use tokio::time::sleep;
// use tokio::select;

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
            // Timer{}
            // Settings {}
        }
    }
}

/* 
    DioxusTimerDisplay -> 앱 디스플레이 총칭
        div start/pause/reset button
            Timer
        div settings button
            Settings
        

*/

#[component]
fn DioxusTimerDisplay() -> Element {
    let initial_duration = Duration::from_secs(10);
    rsx! {
        div {
            class: "dioxus-tinmer-display",
            Timer {initial_duration}
            Settings {  }
        }

    }
}

#[component]
fn Timer(initial_duration:Duration) -> Element {
    
    // let initial_duration = Duration::from_secs(10);
    let timer = use_signal(|| DioxusTimer::new(initial_duration));

    let tx = use_coroutine(move |mut rx: UnboundedReceiver<DioxusTimerCommand>| {
        to_owned![timer];

        async move {
            while let Some(command) = rx.next().await {
                match command {
                    DioxusTimerCommand::Start => {
                        timer.with_mut(|timer| timer.start());

                        loop {
                           /*  if let Ok(Some(command)) = rx.try_next() {
                                match command {
                                    DioxusTimerCommand::Pause => {
                                        timer.with_mut(|timer| timer.pause());
                                        break;
                                    } // 일시정지
                                    DioxusTimerCommand::Reset => {
                                        timer.with_mut(|timer| timer.reset());
                                        break;
                                    } // 시간 초기화
                                    _ => {}
                                }
                            } */

                            timer.with_mut(|timer| timer.update());

                            if timer.read().state == DioxusTimerState::Inactive {
                                break;
                            }

                            /* 
                                이 부분에  tokio::select!를 사용한다.
                             */
                            tokio::select! {
                                _ = tokio::time::sleep(Duration::from_secs(1)) => {},

                            }

                            // sleep(Duration::from_secs(1)).await;
                        }
                    }
                    DioxusTimerCommand::Reset => {
                        timer.with_mut(|timer| timer.reset());
                    }
                    _ => {}
                }
            }
        }
    });

    rsx! {

        div {
            class : "timer",

            div {
                class : "timer__display",
                "{timer}"
            }

            div {
                class : "timer__controls",

                button {
                    class : "timer__button timer__button--start",
                    onclick: move |_| {
                        if let DioxusTimerState::Working = timer.read().state {
                            tx.send(DioxusTimerCommand::Pause);
                        } else {
                            tx.send(DioxusTimerCommand::Start);
                        }
                    },

                    if let DioxusTimerState::Working = timer.read().state {
                        "pause👀"
                    } else {
                        "start❤️"
                    }
                }

                button {
                    class : "timer__button timer__button--reset",
                    onclick: move |_| {
                        if DioxusTimerState::Inactive != timer.read().state {
                            tx.send(DioxusTimerCommand::Reset);
                        }
                    },
                    "reset😎"
                }
            }
        }

    }
}

// 설정 버튼뿐만 아니라 설정 화면도 만들어야 한다.
#[component]
fn Settings() -> Element {
    rsx! {
        div {
            class : "settings",

            button {
                class : "settings__button settings__button--open",
                onclick: move|_| {

                },
                "settings⚙️"
            }
        }
    }
}
