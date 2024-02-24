use std::f32::consts::{PI, TAU};

use shuten::{event::Event, Config, Terminal};
use shuten_core::{
    geom::{lerp, pos2, Pos2, Rect},
    style::Rgb,
    Canvas, Cell,
};

fn main() -> std::io::Result<()> {
    let mut terminal = Terminal::new(Config::default().fixed_timer(30.0))?;

    let mut vista = Vista::default();
    let mut sinebows = [Sinebow { pos: 0.0 }, Sinebow { pos: 120.0 }];
    let mut waves = [
        Wave {
            fg: Rgb::from_u32(0xFFD900),
            bg: Rgb::from_u32(0x0000FF),
            ..Default::default()
        },
        Wave {
            fg: Rgb::from_u32(0x7C68EE),
            bg: Rgb::from_u32(0xA52A2A),
            ..Default::default()
        },
    ];
    while let Ok(ev) = terminal.wait_for_next_event() {
        if ev.is_quit() {
            break;
        }

        if let Event::Blend(blend) = ev {
            vista.integrate(blend);
            for sinebow in &mut sinebows {
                sinebow.integrate(blend)
            }

            for wave in &mut waves {
                wave.integerate(blend);
            }

            terminal.paint(|mut canvas| {
                canvas.erase();
                let (top, bottom) = canvas.area().split_vertical_ratio(0.5);
                vista.draw(&mut canvas.crop(top));

                let (left, right) = bottom.split_horizontal_ratio(0.5);

                let (top, bottom) = left.split_vertical_ratio(0.3);
                waves[0].draw(&mut canvas.crop(top));
                sinebows[0].draw(&mut canvas.crop(bottom));

                let (top, bottom) = right.split_vertical_ratio(0.3);
                waves[1].draw(&mut canvas.crop(bottom));
                sinebows[1].draw(&mut canvas.crop(top));
            })?;
        }
    }

    Ok(())
}

struct Vista {
    pos: Pos2,
    rect: Rect,
    flip: i32,
}

impl Default for Vista {
    fn default() -> Self {
        Self {
            pos: Default::default(),
            rect: Default::default(),
            flip: 1,
        }
    }
}

impl Vista {
    fn draw(&mut self, canvas: &mut Canvas) {
        if self.rect == Rect::ZERO {
            self.pos = canvas.area().center();
        }
        self.rect = canvas.area();

        let h = self.rect.height();
        for pos in self.rect.indices() {
            let dx = pos.x as f32 - self.pos.x as f32;
            let dy = 2.0 * pos.y as f32 - 2.0 * self.pos.y as f32;
            let distance = (dx * dx + dy * dy).sqrt();

            let fg = Rgb::new(
                (255.0 * 10.0 / distance).clamp(1.0, 255.0) as u8,
                255 - (255.0 * self.pos.y as f32 / h as f32).clamp(1.0, 255.0) as u8,
                (distance * 5.0).clamp(1.0, 255.0) as u8,
            );
            canvas.put(pos, Cell::new(' ').bg(fg));
        }
    }

    fn integrate(&mut self, _blend: f32) {
        if self.pos.x >= self.rect.right() || self.pos.x <= self.rect.left() {
            self.flip = -self.flip
        }

        let height = self.rect.height() as f32;
        let top = self.rect.top() as f32;

        let x = (self.pos.x as i32).saturating_add(self.flip);
        let sin = (self.pos.x as f32 / 10.0).sin();
        let y = height * sin / 4.0 + top + height / 2.0;
        self.pos = pos2(x as u16, y as u16);
    }
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

    fn integerate(&mut self, blend: f32) {
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

#[derive(Copy, Clone, Default)]
struct Sinebow {
    pos: f32,
}

impl Sinebow {
    fn draw(&self, canvas: &mut Canvas) {
        let color = Self::next_color(self.pos);
        let offset = canvas.area().left_top();
        let w = canvas.area().width();

        for (pos, c) in ('a'..='z')
            .chain('A'..='Z')
            .chain('0'..='9')
            .cycle()
            .take(canvas.area().area() as usize)
            .enumerate()
            .map(|(i, c)| (pos2(i as u16 % w, i as u16 / w) + offset, c))
        {
            canvas.put(pos, Cell::new(c).fg(color))
        }
    }

    fn integrate(&mut self, blend: f32) {
        self.pos += 0.2 * blend;
    }

    fn next_color(n: f32) -> Rgb {
        let h = n * ((1.0 + 5.0_f32.sqrt()) / 2.0);
        let h = (h + 0.5) * -1.0;
        let r = (PI * h).sin();
        let g = (PI * (h + 0.3)).sin();
        let b = (PI * (h + 0.6)).sin();
        Rgb::from_float([r * r, g * g, b * b])
    }
}
