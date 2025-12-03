use std::fmt::{Display, Formatter};
use std::time::{Duration, Instant};

#[derive(Copy, Clone, Debug)]
pub struct PomoTimer {
    pub work_duration: Duration,
    pub deadline: Instant,
    pub state: PomoTimerState,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PomoTimerState {
    Working,
    Inactive,
    Paused(Duration),
}

impl PomoTimer {
    pub fn new(work_duration: Duration) -> Self {
        Self {
            work_duration: work_duration,
            deadline: Instant::now(),
            state: PomoTimerState::Inactive,
        }
    }

    pub fn start(&mut self) {
        match self.state {
            PomoTimerState::Inactive => {
                self.deadline = Instant::now() + self.work_duration;
                self.state = PomoTimerState::Working;
            }
            PomoTimerState::Paused(remaining) => {
                self.deadline = Instant::now() + remaining;
                self.state = PomoTimerState::Working;
            }
            _ => {}
        }
    }

    pub fn stop(&mut self) {
        match self.state {
            PomoTimerState::Working => {
                let remaining = self.time_left();
                self.state = PomoTimerState::Paused(remaining)
            }
            _ => (),
        }
    }

    pub fn reset(&mut self) {
        self.state = PomoTimerState::Inactive;
        self.deadline = Instant::now() + self.work_duration;
    }

    pub fn update(&mut self) {
        match self.state {
            PomoTimerState::Working => {
                if self.time_left().is_zero() {
                    self.state = PomoTimerState::Inactive;
                }
            }
            _ => {}
        }
    }

    pub fn time_left(&self) -> Duration {
        self.deadline
            .checked_duration_since(Instant::now())
            .unwrap_or(Duration::ZERO)
    }
}

impl Display for PomoTimer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let time_left = match self.state {
            PomoTimerState::Paused(remaining) => remaining,
            PomoTimerState::Inactive => self.work_duration,
            _ => self.time_left(),
        };
        let minutes_left = time_left.as_secs_f64().ceil() as u64 / 60;
        let secs_left = time_left.as_secs_f64().ceil() as u64 % 60;

        write!(f, "{:02}:{:02}", minutes_left, secs_left)
    }
}