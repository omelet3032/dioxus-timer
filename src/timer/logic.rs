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