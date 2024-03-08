#![cfg_attr(debug_assertions, allow(dead_code, unused_variables,))]

use std::{
    collections::HashMap,
    f64::consts::{PI, TAU},
};

use shuten::{
    event::{Event, Key, MouseEvent},
    Config, Terminal,
};
use shuten_core::{
    geom::{pos2, vec2, Vec2},
    style::{Color, Rgb},
    Canvas, Cell,
};

fn main() -> std::io::Result<()> {
    let mut terminal = Terminal::new(Config::default().fixed_timer(60.0))?;

    let mut lines = <Vec<Line>>::new();
    let mut render = Render::default();

    let mut n = 1.0_f64;
    while let Ok(ev) = terminal.wait_for_next_event() {
        if ev.is_quit() {
            break;
        }

        if let Event::Blend(blend) = ev {
            n += blend as f64 * 100.0;
        }

        if let Event::Keyboard(Key::Char('r'), ..) = ev {
            lines.clear();
            render.clear();
        }

        if let Event::Keyboard(Key::Char('d'), ..) = ev {
            lines.pop();
        }

        if let Event::Mouse(mouse, ..) = ev {
            match mouse {
                MouseEvent::DragStart { pos, .. } => {
                    lines.push(Line {
                        start: vec2(pos.x, pos.y),
                        end: vec2(pos.x, pos.y),
                        color: next_color(lines.len() as f32),
                    });
                }
                MouseEvent::Drag { pos, .. } | MouseEvent::DragReleased { pos, .. } => {
                    lines.last_mut().unwrap().end = vec2(pos.x, pos.y);
                }
                _ => {}
            }
        }

        terminal.paint(|mut canvas| {
            canvas.erase();
            // for line in &lines {
            //     line.render(&mut render);
            // }

            // let h = canvas.area().height() as f64;
            // for x in (0..3600).step_by(20) {
            //     let x = x as f64;
            //     render.put(
            //         x / 20.0,
            //         h * 0.5 + (n + x.to_radians().sin() * n),
            //         Rgb::from_u32(0xFF0000),
            //     );

            //     // render.put(
            //     //     x / 10.0 + 30.0,
            //     //     h * 0.5 + (n + x.to_radians().sin() * n as f64),
            //     //     Rgb::from_u32(0x00FF00),
            //     // );

            //     // render.put(
            //     //     x / 10.0 + 60.0,
            //     //     h * 0.5 + (n + x.to_radians().sin() * n as f64),
            //     //     Rgb::from_u32(0x0000FF),
            //     // );
            // }

            let h = canvas.area().height() as f64 * 0.3;
            for x in (0..3600).step_by(20) {
                let x = x as f64;
                render.put(
                    (PI - n) + x / PI,
                    h * 2.0 + x.to_radians().sin() * TAU * 2.0,
                    0xFF0000,
                );
            }
            render.render(&mut canvas);

            for x in (0..3600).step_by(20) {
                let x = x as f64;
                render.put(
                    (PI - n + 60.0) + x / PI,
                    h * 3.0 + 15.0 + x.to_radians().sin() * TAU * 2.0,
                    0x00FF00,
                );
            }
            render.render(&mut canvas);

            for x in (0..3600).step_by(20) {
                let x = x as f64;
                render.put(
                    (PI - n + 120.0) + x / PI,
                    h * 4.0 + 15.0 + x.to_radians().sin() * TAU * 2.0,
                    0x0000FF,
                );
            }

            render.render(&mut canvas);
        })?;
    }

    Ok(())
}

#[allow(dead_code)]
fn next_color(n: f32) -> Rgb {
    use std::f32::consts::PI;
    let h = n * ((1.0 + 5.0_f32.sqrt()) / 2.0);
    let h = (h + 0.5) * -1.0;
    let r = (PI * h).sin();
    let g = (PI * (h + 0.3)).sin();
    let b = (PI * (h + 0.6)).sin();
    Rgb::from_float([r * r, g * g, b * b])
}

struct Line {
    start: Vec2,
    end: Vec2,
    color: Rgb,
}

impl Line {
    fn render(&self, canvas: &mut Render) {
        let Vec2 { x: sx, y: sy } = self.start;
        let Vec2 { x: ex, y: ey } = self.end;
        let (sx, sy, ex, ey) = (sx as i32, sy as i32, ex as i32, ey as i32);

        let x_diff = sx.max(ex) - sx.min(ex);
        let y_diff = sy.max(ey) - sy.min(ey);

        let x_delta = if sx <= ex { 1 } else { -1 };
        let y_delta = if sy <= ey { 1 } else { -1 };

        let slope = x_diff.max(y_diff);

        for i in 0..=slope {
            let mut x = sx;
            let mut y = sy;
            if x_diff != 0 {
                x += ((i * x_diff) / slope) * x_delta;
            }
            if y_diff != 0 {
                y += ((i * y_diff) / slope) * y_delta;
            }
            canvas.put(x as _, y as _, self.color)
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Pos2i {
    x: i32,
    y: i32,
}

impl Pos2i {
    fn new(x: f64, y: f64) -> Self {
        let (x, y) = (x.round() as i32, y.round() as i32);
        let x = if x < 0 { (x - 1) / 2 } else { x / 2 };
        let y = if y < 0 { (y + 1) / 4 - 1 } else { y / 4 };
        pos2i(x, y)
    }
}

const fn pos2i(x: i32, y: i32) -> Pos2i {
    Pos2i { x, y }
}

#[derive(Default, Debug)]
struct Size {
    x: f64,
    y: f64,
}

#[derive(Default, Debug)]
struct Render {
    min: Size,
    size: Pos2i,
    seen: HashMap<Pos2i, Pixel>,
}

#[derive(Copy, Clone, Debug)]
struct Pixel {
    ch: u32,
    color: Color,
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            ch: 0,
            color: Color::Reset,
        }
    }
}

impl Pixel {
    const fn new() -> Self {
        Self {
            ch: 0,
            color: Color::Reset,
        }
    }

    fn set(&mut self, x: impl Into<f64>, y: impl Into<f64>) {
        self.ch |= Self::get(x, y);
    }

    fn unset(&mut self, x: impl Into<f64>, y: impl Into<f64>) {
        self.ch &= !Self::get(x, y);
    }

    fn toggle(&mut self, x: impl Into<f64>, y: impl Into<f64>) {
        let (x, y) = (x.into(), y.into());
        if self.ch & Self::get(x, y) != 0 {
            self.unset(x, y)
        } else {
            self.set(x, y)
        }
    }

    fn get(x: impl Into<f64>, y: impl Into<f64>) -> u32 {
        static MATRIX: [[u32; 2]; 4] = [
            [0x01, 0x08], //
            [0x02, 0x10],
            [0x04, 0x20],
            [0x40, 0x80],
        ];

        let (x, y) = (x.into().round() as i32, y.into().round() as i32);
        let y = if y >= 0 {
            [3, 2, 1, 0][(y % 4) as usize]
        } else {
            [3, 0, 1, 2][(y % 4).unsigned_abs() as usize]
        };

        MATRIX[y as usize][(x % 2).unsigned_abs() as usize]
    }

    fn as_cell(&self) -> Cell {
        Cell::new(char::from_u32(0x2800 + self.ch).unwrap()).fg(self.color)
    }
}

impl Render {
    fn clear(&mut self) {
        self.seen.clear();
    }

    fn put(&mut self, x: f64, y: f64, color: impl Into<Color>) {
        let pos = self.get_pos(x, y);
        let p = self.seen.entry(pos).or_default();
        p.set(x, y);
        p.color = color.into();
    }

    fn toggle(&mut self, x: impl Into<f64> + Copy, y: impl Into<f64> + Copy) {
        let pos = self.get_pos(x, y);
        self.seen.entry(pos).or_default().toggle(x, y);
    }

    fn get_pos(&mut self, x: impl Into<f64>, y: impl Into<f64>) -> Pos2i {
        let (x, y) = (x.into(), y.into());
        if x < self.min.x {
            self.min.x = x;
        }

        if y < self.min.y {
            self.min.y = y;
        }

        let pos = Pos2i::new(x, y);
        if pos.y >= self.size.y {
            self.size.y = pos.y.abs() + 1;
        }

        if pos.x >= self.size.x {
            self.size.x = pos.x.abs() + 1;
        }
        pos
    }

    fn render(&mut self, canvas: &mut Canvas) {
        for (pos, pixel) in self.seen.drain() {
            let pos = pos2(pos.x as _, pos.y as _);
            let cell = pixel.as_cell();
            canvas.put(pos, cell)
        }
    }
}
