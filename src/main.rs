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

#[component]
fn Timer(timer: Signal<DioxusTimer>, tx: Coroutine<DioxusTimerCommand>) -> Element {
    // 난 TimerUI에 timer use signal과 initial_duration만 전달하면 되는거 아닌가?
    // fn timer()가 timer와 initial_duration을 반환하면 되는건가?
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

fn use_timer(initial_duration: Duration) -> (Signal<DioxusTimer>, Coroutine<DioxusTimerCommand>) {
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
                            tokio::select! {
                                timer_command = rx.next() => {
                                    match timer_command {
                                        Some(DioxusTimerCommand::Pause) => {
                                            timer.with_mut(|timer| timer.pause());
                                            break;
                                        },
                                        Some(DioxusTimerCommand::Reset) => {
                                            timer.with_mut(|timer| timer.reset());
                                            break;
                                        },
                                        _ => {}
                                    }
                                }

                                _ = tokio::time::sleep(Duration::from_secs(1)) => {
                                        timer.with_mut(|timer| timer.update());

                                        if timer.read().state == DioxusTimerState::Inactive {
                                        break;
                                        }
                                },


                            }
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

    (timer, tx)
}
/*
settings 버튼을 누른 후의 화면을 만들자
*/

#[component]
fn Settings() -> Element {
    /*
        설정 화면을 어떻게 구성할 것인가
        시간 설정외에도 추가적인 부분을 고려해야 한다
        그래도 지금은 시간만 생각하자
        디자인은?
        시간 입력
            5분 단위 업다운 and 60분 이상부터는 10분 단위 업다운 120분부터는 30분단위...
        그럼 화면 디자인은 단순해진다
        Settings()는 화면만 실제 로직은 use_setting?
        상태signal를 전달해야 하기 때문에 use_setting을 붙이면 되는건가?
    */

    rsx! {

    }
}

fn use_settings

#[component]
fn SettingsButton() -> Element {
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

// type TimerValue = (Signal<DioxusTimer>, Coroutine<DioxusTimerCommand>);

// timer는 signal과 코루틴을 튜플로 반환해야 한다

// 설정 버튼뿐만 아니라 설정 화면도 만들어야 한다.
// Settings UI
