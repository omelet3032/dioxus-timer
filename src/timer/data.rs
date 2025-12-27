use std::fmt::{Display, Formatter};
use std::time::{Duration, Instant};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DioxusTimer {
    pub work_duration: Duration,
    pub deadline: Instant,
    pub state: DioxusTimerState,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DioxusTimerState {
    Working,
    Inactive,
    Paused(Duration),
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DioxusTimerCommand {
    Start, // 맨 처음 시작시 loop진입전 시작 명령
    Pause,
    Reset,
}

impl DioxusTimer {
    pub fn new(work_duration: Duration) -> Self {
        Self {
            work_duration: work_duration,
            deadline: Instant::now(),
            state: DioxusTimerState::Inactive,
        }
    }

    pub fn start(&mut self) {
        match self.state {
            DioxusTimerState::Inactive => {
                self.deadline = Instant::now() + self.work_duration;
                self.state = DioxusTimerState::Working;
            }
            DioxusTimerState::Paused(remaining) => {
                self.deadline = Instant::now() + remaining;
                self.state = DioxusTimerState::Working;
            }
            _ => {}
        }
    }

    pub fn pause(&mut self) {
        if let DioxusTimerState::Working = self.state {
            let remaining = self.time_left();
            self.state = DioxusTimerState::Paused(remaining)
        }
    }

    pub fn reset(&mut self) {
        self.state = DioxusTimerState::Inactive;
        self.deadline = Instant::now() + self.work_duration;
    }

    pub fn update(&mut self) {
        if let DioxusTimerState::Working = self.state {
            if self.time_left().is_zero() {
                self.state = DioxusTimerState::Inactive;
            }
        }
    }

    fn time_left(&self) -> Duration {
        self.deadline
            .checked_duration_since(Instant::now())
            .unwrap_or(Duration::ZERO)
    }
}

impl Display for DioxusTimer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let time_left = match self.state {
            DioxusTimerState::Paused(remaining) => remaining,
            DioxusTimerState::Inactive => self.work_duration,
            _ => self.time_left(),
        };

        let secs = time_left.as_secs_f64().ceil() as u64;

        let minutes_left = secs / 60;
        let seconds_left = secs % 60;

        write!(f, "{:02}:{:02}", minutes_left, seconds_left)
    }
}
