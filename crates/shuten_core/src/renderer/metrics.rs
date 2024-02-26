use crate::{
    geom::{vec2, Rect},
    Canvas, Cell, Queue,
};

mod renderer;
pub use renderer::MetricsRenderer;

pub const STATS_WINDOW: usize = 30;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Stats {
    pub minimum: usize,
    pub maximum: usize,
    pub average: usize,
}

pub trait StatsWindow<const N: usize> {
    fn push(&mut self, item: usize);
    fn modify(&mut self, f: impl Fn(&mut usize));
    fn stats(&self) -> Stats;
}

impl<const N: usize> StatsWindow<N> for Queue<usize, N> {
    fn push(&mut self, item: usize) {
        Queue::push(self, item)
    }

    fn modify(&mut self, f: impl Fn(&mut usize)) {
        if let Some(last) = self.last_mut() {
            f(last)
        }
    }

    fn stats(&self) -> Stats {
        let (mut minimum, mut maximum) = (usize::MAX, usize::MIN);
        let mut total = 0;
        let len = self.len();
        for &sample in self {
            minimum = minimum.min(sample);
            maximum = maximum.max(sample);
            total += sample;
        }
        let avg = total / len.max(1);
        Stats {
            minimum,
            maximum,
            average: avg,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FrameStats<const N: usize = STATS_WINDOW> {
    pub clears: Queue<usize, N>,
    pub moves: Queue<usize, N>,
    pub set_fg: Queue<usize, N>,
    pub set_bg: Queue<usize, N>,
    pub set_attr: Queue<usize, N>,
    pub reset_fg: Queue<usize, N>,
    pub reset_bg: Queue<usize, N>,
    pub reset_attr: Queue<usize, N>,
    pub write: Queue<usize, N>,
}

impl<const N: usize> FrameStats<N> {
    pub fn new() -> Self {
        Self {
            clears: Queue::new(),
            moves: Queue::new(),
            set_fg: Queue::new(),
            set_bg: Queue::new(),
            set_attr: Queue::new(),
            reset_fg: Queue::new(),
            reset_bg: Queue::new(),
            reset_attr: Queue::new(),
            write: Queue::new(),
        }
    }

    pub fn new_frame(&mut self) {
        self.clears.push(0);
        self.moves.push(0);
        self.set_fg.push(0);
        self.set_bg.push(0);
        self.set_attr.push(0);
        self.reset_fg.push(0);
        self.reset_bg.push(0);
        self.reset_attr.push(0);
        self.write.push(0);
    }
}

impl Default for FrameStats<STATS_WINDOW> {
    fn default() -> Self {
        Self {
            clears: Queue::new(),
            moves: Queue::new(),
            set_fg: Queue::new(),
            set_bg: Queue::new(),
            set_attr: Queue::new(),
            reset_fg: Queue::new(),
            reset_bg: Queue::new(),
            reset_attr: Queue::new(),
            write: Queue::new(),
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct FrameStatsConfig {
    pub clears: bool,
    pub moves: bool,
    pub set_fg: bool,
    pub set_bg: bool,
    pub set_attr: bool,
    pub reset_fg: bool,
    pub reset_bg: bool,
    pub reset_attr: bool,
    pub write: bool,
}

impl FrameStatsConfig {
    pub const fn all() -> Self {
        Self {
            clears: true,
            moves: true,
            set_fg: true,
            set_bg: true,
            set_attr: true,
            reset_fg: true,
            reset_bg: true,
            reset_attr: true,
            write: true,
        }
    }

    fn build<const N: usize>(&self, stats: &FrameStats<N>) -> Vec<(&'static str, Stats)> {
        [
            (self.write, "write", &stats.write),
            (self.moves, "moves", &stats.moves),
            (self.set_fg, "set_fg", &stats.set_fg),
            (self.set_bg, "set_bg", &stats.set_bg),
            (self.set_attr, "set_attr", &stats.set_attr),
            (self.reset_fg, "reset_fg", &stats.reset_fg),
            (self.reset_bg, "reset_bg", &stats.reset_bg),
            (self.reset_attr, "reset_attr", &stats.reset_attr),
            (self.clears, "clears", &stats.clears),
        ]
        .into_iter()
        .flat_map(|(ok, label, stats)| ok.then(|| (label, stats.stats())))
        .collect()
    }
}

impl<const N: usize> FrameStats<N> {
    pub fn draw(&self, canvas: &mut Canvas) {
        self.draw_specific_stats(FrameStatsConfig::all(), canvas)
    }

    pub fn filter_metrics(&self, stats: FrameStatsConfig) -> Vec<(&'static str, Stats)> {
        stats.build(self)
    }

    pub fn draw_specific_stats(&self, stats: FrameStatsConfig, canvas: &mut Canvas) {
        let out = stats.build(self);

        let (min, max, avg, left) =
            out.iter()
                .fold((0, 0, 0, 0), |(min, max, avg, out), (label, stats)| {
                    (
                        min.max(count_digits(stats.minimum)),
                        max.max(count_digits(stats.maximum)),
                        avg.max(count_digits(stats.average)),
                        out.max(label.len()),
                    )
                });

        let mut data = String::new();
        for (label, stats) in out {
            use std::fmt::Write as _;
            let _ = write!(
                &mut data,
                "{: <left$} | {: <min$} | {: <max$} | {: <avg$}\n",
                label,
                stats.minimum,
                stats.maximum,
                stats.average,
                left = left,
                min = min,
                max = max,
                avg = avg,
            );
        }

        if data.trim().is_empty() {
            return;
        }

        let rect = {
            let x = data.lines().map(|c| c.len()).max().unwrap_or(1) + 1;
            let y = data.lines().count();
            Rect::from_min_size(canvas.area().left_top(), vec2(x as u16, y as u16))
        };

        canvas.fill_rect(rect, u32::MIN);

        let mut canvas = canvas.crop(rect);
        let area = canvas.area();
        let mut start = area.left_top();
        for ch in data.chars() {
            if ch == '\n' {
                start.y += 1;
                start.x = area.left();
                continue;
            }
            if start.x >= area.right() {
                start.y += 1;
                start.x = area.left()
            }
            if start.y > area.bottom() {
                break;
            }
            canvas.put(start, Cell::new(ch).fg(u32::MAX).bg(u32::MIN));
            start.x += 1;
        }
    }
}

fn count_digits(d: usize) -> usize {
    let (mut len, mut n) = (1, 1);
    while len < 20 {
        n *= 10;
        if n > d {
            return len;
        }
        len += 1;
    }
    len
}
