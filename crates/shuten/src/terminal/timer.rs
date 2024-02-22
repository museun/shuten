use std::time::{Duration, Instant};

#[derive(Copy, Clone, Default, Debug)]
pub struct Timer {
    pub kind: TimerKind,
}

#[derive(Copy, Clone, Default, Debug)]
pub enum TimerKind {
    Fixed(FixedTimer),
    #[default]
    Reactive,
}

impl Timer {
    /// Create a new fixed timestamp timer targeting `fps` frames
    pub fn fixed(fps: f64) -> Self {
        Self {
            kind: TimerKind::Fixed(FixedTimer::fixed(fps)),
        }
    }
    /// Create a reactive timer, that'll only produce events as the terminal produces them
    pub const fn reactive() -> Self {
        Self {
            kind: TimerKind::Reactive,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct FixedTimer {
    last: Instant,
    accum: Duration,
    target: Duration,
}

impl FixedTimer {
    pub fn tick_until_ready(&mut self) {
        self.advance_time();
        while self.accum < self.target {
            std::thread::sleep(Duration::from_millis(1));
            self.advance_time();
        }
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

    fn fixed(tick: f64) -> Self {
        let target = Duration::from_secs_f64(1.0 / tick);
        Self {
            last: Instant::now(),
            accum: Duration::ZERO,
            target,
        }
    }

    fn advance_time(&mut self) {
        let current = Instant::now();
        self.accum += current - self.last;
        self.last = current;
    }
}

#[allow(dead_code)]
impl FixedTimer {
    fn tick(&mut self) {
        self.advance_time();
    }

    fn reset(&mut self) {
        self.last = Instant::now();
        self.accum = Duration::ZERO
    }

    fn factor(&self) -> f32 {
        self.accum.as_secs_f32() / self.target.as_secs_f32()
    }
}
