use super::{offset, pos2, Offset, Pos2, Vec2f};

/// A two dimensional vector
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Vec2 {
    pub x: u16,
    pub y: u16,
}

impl std::fmt::Debug for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { x, y } = self;
        write!(f, "{{{x:?}, {y:?}}}")
    }
}

/// Create a new two-dimension vector
///
/// These are generally used for "sizes"
pub const fn vec2(x: u16, y: u16) -> Vec2 {
    Vec2 { x, y }
}

impl Vec2 {
    /// A zero vector
    pub const ZERO: Self = vec2(0, 0);

    /// Create a new vector
    pub const fn new(x: u16, y: u16) -> Self {
        vec2(x, y)
    }

    /// Create a vector using `d` as both components
    pub const fn splat(d: u16) -> Self {
        vec2(d, d)
    }

    /// Convert this vector to a position
    pub const fn to_pos2(self) -> Pos2 {
        pos2(self.x, self.y)
    }

    /// Get the length of this vector
    pub fn length(self) -> f32 {
        (self.x as f32).hypot(self.y as f32)
    }

    /// Get the length of this vector, squared
    pub fn length_sq(self) -> f32 {
        let (x, y) = (self.x as f32, self.y as f32);
        x.mul_add(x, y * y)
    }

    /// Swap the components in this vector
    ///
    /// e.g. swap `x` and `y`
    pub const fn swap(self) -> Self {
        vec2(self.y, self.x)
    }

    /// Get the smallest vector from the provided vectors
    ///
    /// E.g. the smallest `x` _AND_ smallest `y`
    pub fn min(self, other: Self) -> Self {
        vec2(self.x.min(other.x), self.y.min(other.y))
    }

    /// Get the largest vector from the provided vectors
    ///
    /// E.g. the largest `x` _AND_ largest `y`
    pub fn max(self, other: Self) -> Self {
        vec2(self.x.max(other.x), self.y.max(other.y))
    }

    /// Clamp this vector to a min, max set
    pub fn clamp(self, min: Self, max: Self) -> Self {
        vec2(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y))
    }

    /// Convert this vector to an [`Offset`]
    pub const fn as_offset(self) -> Offset {
        offset(self.x as i32, self.y as i32)
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        vec2(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        vec2(self.x.saturating_sub(rhs.x), self.y.saturating_sub(rhs.y))
    }
}

impl std::ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl std::ops::Div for Vec2 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        vec2(self.x / rhs.x, self.y / rhs.y)
    }
}

impl std::ops::Div<u16> for Vec2 {
    type Output = Self;
    fn div(self, rhs: u16) -> Self::Output {
        vec2(self.x / rhs, self.y / rhs)
    }
}

impl std::ops::Mul for Vec2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        vec2(self.x * rhs.x, self.y * rhs.y)
    }
}

impl std::ops::MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl From<(u16, u16)> for Vec2 {
    fn from((x, y): (u16, u16)) -> Self {
        vec2(x, y)
    }
}

impl From<Vec2f> for Vec2 {
    fn from(value: Vec2f) -> Self {
        Self {
            x: u16::try_from(value.x.ceil() as i16).unwrap_or(0),
            y: u16::try_from(value.y.ceil() as i16).unwrap_or(0),
        }
    }
}
