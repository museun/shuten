#![cfg_attr(debug_assertions, allow(dead_code, unused_variables,))]

use super::{Color, Rgb};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

impl Rgba {
    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self(red, green, blue, alpha)
    }

    pub const fn transparent() -> Self {
        Self(0, 0, 0, 0)
    }

    pub const fn from_u32(rrggbbaa: u32) -> Self {
        Self::new(
            ((rrggbbaa >> 24) & 0xFF) as u8,
            ((rrggbbaa >> 16) & 0xFF) as u8,
            ((rrggbbaa >> 8) & 0xFF) as u8,
            (rrggbbaa & 0xFF) as u8,
        )
    }

    pub fn blend(&self, other: Self) -> Self {
        if self.alpha() == 0 {
            return other;
        }

        if self.alpha() == 255 {
            return *self;
        }

        let a = self.alpha() as i32;
        let r = ((a * self.red() as i32 + (255 - a) * other.red() as i32) / 255) as u8;
        let g = ((a * self.green() as i32 + (255 - a) * other.green() as i32) / 255) as u8;
        let b = ((a * self.blue() as i32 + (255 - a) * other.blue() as i32) / 255) as u8;

        Self(r, g, b, 255)
    }

    pub fn fade(&self, target: Self, dt: f32) -> Self {
        fn compute(dt: f32, delta: f32) -> f32 {
            if delta < 0.0 {
                return (-dt).max(delta);
            }
            if delta > 0.0 {
                return dt.min(delta);
            }
            0.0
        }

        let r = compute(dt, target.red() as f32) - self.red() as f32;
        let g = compute(dt, target.green() as f32) - self.green() as f32;
        let b = compute(dt, target.blue() as f32) - self.blue() as f32;

        Self::new(
            (self.red() as f32 + r) as u8,
            (self.green() as f32 + g) as u8,
            (self.blue() as f32 + b) as u8,
            self.alpha(),
        )
    }

    pub const fn red(&self) -> u8 {
        self.0
    }

    pub const fn green(&self) -> u8 {
        self.1
    }

    pub const fn blue(&self) -> u8 {
        self.2
    }

    pub const fn alpha(&self) -> u8 {
        self.3
    }
}

impl From<Rgba> for Color {
    fn from(value: Rgba) -> Self {
        Self::Rgb(Rgb::new(value.red(), value.green(), value.blue()))
    }
}
