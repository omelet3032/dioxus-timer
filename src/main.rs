use dioxus::prelude::*;
use dioxus_timer::{PomoTimer, PomoTimerState, PomoTimerCommand};

use futures_util::StreamExt;
use std::time::{Duration};
use tokio::time::sleep;

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
            Timer{}
        }
    }
}

#[component]
fn Timer() -> Element {
    let initial_duration = Duration::from_secs(10);
    let timer = use_signal(|| PomoTimer::new(initial_duration));

    let tx = use_coroutine(move |mut rx: UnboundedReceiver<PomoTimerCommand>| {
        to_owned![timer];

        async move {
            while let Some(command) = rx.next().await {
                match command {
                    PomoTimerCommand::Start => {
                        timer.with_mut(|timer| timer.start());

                        loop {
                            if let Ok(Some(command)) = rx.try_next() {
                                match command {
                                    PomoTimerCommand::Pause => {
                                        timer.with_mut(|timer| timer.pause());
                                        break;
                                    } // 일시정지
                                    PomoTimerCommand::Reset => {
                                        timer.with_mut(|timer| timer.reset());
                                        break;
                                    } // 시간 초기화
                                    _ => {}
                                }
                            }

                            timer.with_mut(|timer| timer.update());

                            if timer.read().state == PomoTimerState::Inactive {
                                break;
                            }

                            sleep(Duration::from_secs(1)).await;
                        }
                    }
                    PomoTimerCommand::Reset => {
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
                        if let PomoTimerState::Working = timer.read().state {
                            tx.send(PomoTimerCommand::Pause);
                        } else {
                            tx.send(PomoTimerCommand::Start);
                        }
                    },

                    if let PomoTimerState::Working = timer.read().state {
                        "pause👀"
                    } else {
                        "start❤️"
                    }
                }

                button {
                    class : "timer__button timer__button--reset",
                    onclick: move |_| {
                        if PomoTimerState::Inactive != timer.read().state {
                            tx.send(PomoTimerCommand::Reset);
                        }
                    },
                    "reset😎"
                }
            }
        }
    }
}

