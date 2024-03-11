use super::{vec2f, Pos2, Vec2f};

/// Create a new [`Pos2f`] with the provided coordinates
pub const fn pos2f(x: f32, y: f32) -> Pos2f {
    Pos2f { x, y }
}

/// A two dimensional position, using floats
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Pos2f {
    pub x: f32,
    pub y: f32,
}

impl Default for Pos2f {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Pos2f {
    pub const ZERO: Self = pos2f(0.0, 0.0);

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const fn to_vec2(&self) -> Vec2f {
        vec2f(self.x, self.y)
    }

    pub fn min(&self, other: Self) -> Self {
        pos2f(self.x.min(other.x), self.y.min(other.y))
    }

    pub fn max(&self, other: Self) -> Self {
        pos2f(self.x.max(other.x), self.y.max(other.y))
    }
}

impl std::ops::Add<Vec2f> for Pos2f {
    type Output = Self;
    fn add(self, other: Vec2f) -> Self {
        pos2f(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Sub<Vec2f> for Pos2f {
    type Output = Self;
    fn sub(self, other: Vec2f) -> Self {
        pos2f(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::Add for Pos2f {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        pos2f(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Sub for Pos2f {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        pos2f(self.x - other.x, self.y - other.y)
    }
}

impl From<Pos2> for Pos2f {
    fn from(value: Pos2) -> Self {
        pos2f(value.x as f32, value.y as f32)
    }
}

impl From<Pos2f> for Pos2 {
    fn from(value: Pos2f) -> Self {
        Self {
            x: u16::try_from(value.x.ceil() as i16).unwrap_or(0),
            y: u16::try_from(value.y.ceil() as i16).unwrap_or(0),
        }
    }
}

impl std::ops::Neg for Pos2f {
    type Output = Self;
    fn neg(self) -> Self::Output {
        pos2f(-self.x, -self.y)
    }
}
