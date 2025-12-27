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