use dioxus::prelude::*;
use dioxus_timer::{PomoTimer, PomoTimerState};

use futures_util::StreamExt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::time::{Duration, Instant};
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
    let mut timer = use_signal(|| PomoTimer::new(initial_duration));

    let tx = use_coroutine(move |mut rx: UnboundedReceiver<TimerCommand>| {
        to_owned![timer];

        async move {
            while let Some(TimerCommand) = rx.next().await {

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
                            tx.send(TimerCommand::Pause);
                        }
                    },

                    if let PomoTimerState::Working = timer.read().state {
                        "pause👀"
                    } else {
                        "start❤️"
                    }
                }

                button {
                    class : "timer__button timer__button--stop",
                    onclick: move |_| {
                        if PomoTimerState::Inactive != timer.read().state {
                            tx.send(TimerCommand::Stop);
                        }
                    },
                    "stopstop😎"
                }
            }
        }
    }
}

enum TimerCommand {
    Start,
    Stop,
    Pause,
}
