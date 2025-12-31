use dioxus::signals::Signal;

#[derive(Clone)]
pub struct TimerState {
   pub duration: Signal<u32> 
}