/// Modul za upravljanje s časom igre.
use std::time::{Duration, Instant};

/// Struktura, ki predstavlja časovnik igre.
/// Časovnik beleži čas od začetka igre ter omogoča ustavljanje in nadaljevanje.
pub struct GameClock {
    /// Čas začetka igre.
    start_time: Instant,
    /// Skupni čas igre v pavziranem stanju.
    total_paused: Duration,
    /// Čas začetka pavze, če je igra trenutno pavzirana.
    pause_start: Option<Instant>,
    /// Ali igra in časovnik tečeta.
    running: bool,
}

impl GameClock {
    /// Ustvari nov časovnik igre.
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            total_paused: Duration::ZERO,
            pause_start: None,
            running: true,
        }
    }

    /// Zaustavi časovnik.
    pub fn pause(&mut self) {
        if self.running {
            self.pause_start = Some(Instant::now());
            self.running = false;
        }
    }

    /// Znova zažene časovnik po pavzi.
    pub fn resume(&mut self) {
        if let Some(pause_time) = self.pause_start.take() {
            self.total_paused += pause_time.elapsed();
            self.running = true;
        }
    }

    /// Izračuna pretečen čas igre.
    fn elapsed(&self) -> Duration {
        if self.running {
            Instant::now() - self.start_time - self.total_paused
        } else {
            self.pause_start.unwrap() - self.start_time - self.total_paused
        }
    }

    /// Vrne formatiran čas v obliki "mm:ss".
    /// Uporablja se za prikaz časa na zaslonu.
    pub fn formatted_time(&self) -> String {
        let secs = self.elapsed().as_secs();
        format!("{:02}:{:02}", secs / 60, secs % 60)
    }
}
