use std::f32::consts::TAU;

use shuten::{event::Event, Config, Terminal};
use shuten_core::{
    geom::{lerp, pos2},
    renderer::metrics::FrameStats,
    style::Rgb,
    Canvas, Cell,
};

fn main() -> std::io::Result<()> {
    let mut terminal = Terminal::new(Config::default().fixed_timer(120.0))?;

    let mut demo = Wave {
        fg: Rgb::from_u32(0x6494ED),
        bg: Rgb::from_u32(0x9400D3),
        ..Default::default()
    };
    let mut stats = <FrameStats<100>>::new();

    while let Ok(ev) = terminal.wait_for_next_event() {
        if ev.is_quit() {
            break;
        }

        if ev.is_invalidate() {
            terminal.set_title(&format!(
                "{}x{} (area: {})",
                terminal.rect().width(),
                terminal.rect().height(),
                terminal.rect().area()
            ))?;
        }

        if let Event::Blend(blend) = ev {
            demo.integrate(blend);

            terminal.paint_with_metrics(&mut stats, |mut canvas, stats| {
                demo.draw(&mut canvas);
                stats.draw(&mut canvas);
            })?;
        }
    }

    Ok(())
}

#[derive(Copy, Clone, Default)]
struct Wave {
    value: f32,
    fg: Rgb,
    bg: Rgb,
}

impl Wave {
    fn draw(&self, canvas: &mut Canvas) {
        let rect = canvas.area();
        let w = rect.width() as usize;
        let t = lerp(0.0..=w as f32, self.value);

        for (i, n) in self.wave(w).skip(t as usize).take(w).enumerate() {
            let cell = Cell::new(' ').bg(n);
            let x = rect.left() + i as u16;
            for y in rect.top()..=rect.bottom() {
                canvas.put(pos2(x, y), cell)
            }
        }
    }

    fn integrate(&mut self, blend: f32) {
        self.value += blend;
        self.value %= 1.0
    }

    fn wave(&self, width: usize) -> impl Iterator<Item = Rgb> + Clone + '_ {
        let mut time = 0.0f32;
        std::iter::repeat(()).map(move |()| {
            let value = 0.5 + 0.5 * (TAU * time + 5.0).sin();
            time += 1.0 / width as f32;
            self.fg.blend_flat(self.bg, value)
        })
    }
}
