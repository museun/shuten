use super::{Offset, Vec2};

use super::{pos2f, Pos2f};

/// Create a new [`Vec2f`] with the provided sizes
pub const fn vec2f(x: f32, y: f32) -> Vec2f {
    Vec2f { x, y }
}

/// A two dimensional vector, using floats
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Default for Vec2f {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Vec2f {
    pub const ZERO: Self = vec2f(0.0, 0.0);
    pub const INFINITY: Self = vec2f(f32::INFINITY, f32::INFINITY);

    pub const fn new(x: f32, y: f32) -> Self {
        vec2f(x, y)
    }

    pub const fn splat(d: f32) -> Self {
        vec2f(d, d)
    }

    pub fn max(self, other: Self) -> Self {
        vec2f(self.x.max(other.x), self.y.max(other.y))
    }

    pub fn min(self, other: Self) -> Self {
        vec2f(self.x.min(other.x), self.y.min(other.y))
    }

    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    pub const fn to_pos2(&self) -> Pos2f {
        pos2f(self.x, self.y)
    }
}

impl std::ops::Add for Vec2f {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        vec2f(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::AddAssign for Vec2f {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for Vec2f {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        vec2f(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Mul for Vec2f {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        vec2f(self.x * rhs.x, self.y * rhs.y)
    }
}

impl From<Vec2> for Vec2f {
    fn from(value: Vec2) -> Self {
        vec2f(value.x as f32, value.y as f32)
    }
}

impl From<Offset> for Vec2f {
    fn from(value: Offset) -> Self {
        vec2f(value.x as f32, value.y as f32)
    }
}

impl std::ops::Mul<f32> for Vec2f {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        vec2f(self.x * rhs, self.y * rhs)
    }
}

impl std::ops::Div<f32> for Vec2f {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        vec2f(self.x / rhs, self.y / rhs)
    }
}

impl std::ops::Neg for Vec2f {
    type Output = Self;

    fn neg(self) -> Self::Output {
        vec2f(-self.x, -self.y)
    }
}
