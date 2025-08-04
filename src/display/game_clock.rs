use std::time::{Duration, Instant};
pub struct GameClock {
    start_time: Instant,
    total_paused: Duration,
    pause_start: Option<Instant>,
    running: bool,
}

impl GameClock {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            total_paused: Duration::ZERO,
            pause_start: None,
            running: true,
        }
    }

    pub fn pause(&mut self) {
        if self.running {
            self.pause_start = Some(Instant::now());
            self.running = false;
        }
    }

    pub fn resume(&mut self) {
        if let Some(pause_time) = self.pause_start.take() {
            self.total_paused += pause_time.elapsed();
            self.running = true;
        }
    }

    fn elapsed(&self) -> Duration {
        if self.running {
            Instant::now() - self.start_time - self.total_paused
        } else {
            self.pause_start.unwrap() - self.start_time - self.total_paused
        }
    }

    pub fn formatted_time(&self) -> String {
        let secs = self.elapsed().as_secs();
        format!("{:02}:{:02}", secs / 60, secs % 60)
    }
}
