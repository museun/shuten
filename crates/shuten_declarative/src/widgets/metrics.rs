use shuten::{
    geom::{vec2, Margin, Pos2, Pos2f, Rect},
    renderer::metrics::{FrameStats, FrameStatsConfig},
};

use crate::input::Keybind;
use crate::widget::*;
use crate::widgets::*;

use super::{draggable, float, margin, toggle_bind};

use shuten::renderer::metrics::Stats;

#[derive(Debug, Default)]
pub struct Metrics {
    stats: Vec<(&'static str, Stats)>,
    pos: Pos2f,
}

impl Metrics {
    fn show(self) -> Response {
        MetricsWidget::show(self)
    }
}

#[derive(Debug, Default)]
struct MetricsWidget {
    props: Metrics,
}

impl Widget for MetricsWidget {
    type Props<'a> = Metrics;
    type Response = NoResponse;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props
    }

    fn paint(&self, ctx: prelude::PaintCtx<'_, '_>) {
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

        let (min, max, avg, left) =
            self.props
                .stats
                .iter()
                .fold((0, 0, 0, 0), |(min, max, avg, out), (label, stats)| {
                    (
                        min.max(count_digits(stats.minimum)),
                        max.max(count_digits(stats.maximum)),
                        avg.max(count_digits(stats.average)),
                        out.max(label.len()),
                    )
                });

        let mut data = String::new();
        for (label, stats) in &self.props.stats {
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
            Rect::from_min_size(
                ctx.canvas.area().left_top() + Pos2::from(self.props.pos),
                vec2(x as u16, y as u16),
            )
        };

        ctx.canvas.fill_rect(rect, u32::MIN);

        let mut canvas = ctx.canvas.crop(rect);
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
            canvas.put(start, shuten::Cell::new(ch).fg(u32::MAX).bg(u32::MIN));
            start.x += 1;
        }
    }
}

pub fn metrics<const N: usize>(metrics: &FrameStats<N>, keybind: impl Into<Keybind>) -> Response {
    let pos = state(|| Pos2f::ZERO);
    toggle_bind(keybind, || {
        offset(pos.get(), || {
            float(|| {
                let resp = draggable(|| {
                    margin(Margin::symmetric(3, 5), || {
                        Metrics {
                            stats: metrics.filter_metrics(FrameStatsConfig::all()),
                            pos: pos.get(),
                        }
                        .show();
                    });
                });

                pos.set_if(resp.current())
            });
        });
    })
    .map(|_| ())
}
