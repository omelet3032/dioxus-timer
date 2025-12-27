use std::time::Duration;

use dioxus::prelude::*;
use futures_util::StreamExt;

use crate::timer::data::*;

pub fn use_timer(initial_duration: Duration) -> (Signal<Timer>, Coroutine<TimerCommand>) {
    // let initial_duration = Duration::from_secs(10);
    let timer = use_signal(|| Timer::new(initial_duration));

    let tx = use_coroutine(move |mut rx: UnboundedReceiver<TimerCommand>| {
        to_owned![timer];

        async move {
            while let Some(command) = rx.next().await {
                match command {
                    TimerCommand::Start => {
                        timer.with_mut(|timer| timer.start());

                        loop {
                            tokio::select! {
                                timer_command = rx.next() => {
                                    match timer_command {
                                        Some(TimerCommand::Pause) => {
                                            timer.with_mut(|timer| timer.pause());
                                            break;
                                        },
                                        Some(TimerCommand::Reset) => {
                                            timer.with_mut(|timer| timer.reset());
                                            break;
                                        },
                                        _ => {}
                                    }
                                }

                                _ = tokio::time::sleep(Duration::from_secs(1)) => {
                                        timer.with_mut(|timer| timer.update());

                                        if timer.read().state == TimerState::Inactive {
                                        break;
                                        }
                                },


                            }
                        }
                    }

                    TimerCommand::Reset => {
                        timer.with_mut(|timer| timer.reset());
                    }

                    _ => {}
                }
            }
        }
    });

    (timer, tx)
}