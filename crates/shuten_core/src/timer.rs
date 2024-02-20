use std::time::{Duration, Instant};

/// An event loop timer
#[derive(Copy, Clone, Default, Debug)]
pub enum Timer {
    Fixed(FixedTimer),
    #[default]
    Reactive,
}

impl Timer {
    /// Create a new fixed timestamp timer targeting `fps` frames
    pub fn fixed(fps: f64) -> Self {
        Self::Fixed(FixedTimer::fixed(fps))
    }
    /// Create a reactive timer, that'll only produce events as the terminal produces them
    pub const fn reactive() -> Self {
        Self::Reactive
    }
}

#[derive(Copy, Clone, Debug)]
pub struct FixedTimer {
    last: Instant,
    accum: Duration,
    target: Duration,
}

impl FixedTimer {
    pub fn fixed(tick: f64) -> Self {
        let target = Duration::from_secs_f64(1.0 / tick);
        Self {
            last: Instant::now(),
            accum: Duration::ZERO,
            target,
        }
    }

    pub fn tick(&mut self) {
        self.advance_time();
    }

    pub fn tick_until_ready(&mut self) {
        self.advance_time();
        while self.accum < self.target {
            std::thread::sleep(Duration::from_millis(1));
            self.advance_time();
        }
    }

    pub fn reset(&mut self) {
        self.last = Instant::now();
        self.accum = Duration::ZERO
    }

    pub fn consume(&mut self) -> bool {
        let ready = self.accum >= self.target;
        if ready {
            self.accum -= self.target
        }
        ready
    }

    pub const fn delta(&self) -> Duration {
        self.target
    }

    pub fn factor(&self) -> f32 {
        self.accum.as_secs_f32() / self.target.as_secs_f32()
    }

    pub fn advance_time(&mut self) {
        let current = Instant::now();
        self.accum += current - self.last;
        self.last = current;
    }
}
