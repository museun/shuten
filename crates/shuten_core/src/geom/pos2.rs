use super::{lerp, offset, vec2, Offset, Vec2};

/// Create a new [`Pos2`] with `x` and `y`
pub const fn pos2(x: u16, y: u16) -> Pos2 {
    Pos2 { x, y }
}

/// A two-dimensional position
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pos2 {
    pub x: u16,
    pub y: u16,
}

impl std::fmt::Debug for Pos2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { x, y } = self;
        write!(f, "[{x:?},{y:?}]")
    }
}

impl Pos2 {
    ///  zero point
    pub const ZERO: Self = pos2(0, 0);

    /// Create a new point
    pub const fn new(x: u16, y: u16) -> Self {
        pos2(x, y)
    }

    /// Convert this point to a vector
    pub const fn to_vec2(self) -> Vec2 {
        vec2(self.x, self.y)
    }

    /// Swap the x,y components
    pub const fn swap(self) -> Self {
        pos2(self.y, self.x)
    }

    /// Distance from this point to the other point
    pub fn distance(self, other: Self) -> f32 {
        let (x1, y1) = (self.x as f32, self.y as f32);
        let (x2, y2) = (other.x as f32, other.y as f32);
        (x1 - x2).hypot(y1 - y2)
    }

    /// Distance from this point to the other point, squared
    pub fn distance_sq(self, other: Self) -> f32 {
        let x = self.x as f32 - other.x as f32;
        let y = self.y as f32 - other.y as f32;
        x.mul_add(x, y * y)
    }

    /// Clamp this point to a min, max set
    pub fn clamp(self, min: Self, max: Self) -> Self {
        pos2(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y))
    }

    /// Get the smallest point from the provided points
    ///
    /// E.g. the smallest `x` _AND_ smallest `y`
    pub fn min(self, other: Self) -> Self {
        pos2(self.x.min(other.x), self.y.min(other.y))
    }

    /// Get the largest point from the provided points
    ///
    /// E.g. the largest `x` _AND_ largest `y`
    pub fn max(self, other: Self) -> Self {
        pos2(self.x.max(other.x), self.y.max(other.y))
    }

    /// Get the vector length, squared of the components
    pub const fn length_sq(&self) -> u16 {
        self.x * self.x + self.y * self.y
    }

    /// Interpolate a value from point to another point
    pub fn lerp(self, other: Self, t: u16) -> Self {
        pos2(lerp(self.x..=other.x, t), lerp(self.y..=other.y, t))
    }

    /// Convert this position to an [`Offset`]
    pub const fn as_offset(self) -> Offset {
        offset(self.x as i32, self.y as i32)
    }
}

impl std::ops::Add for Pos2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        pos2(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::AddAssign for Pos2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl std::ops::Sub for Pos2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        pos2(self.x.saturating_sub(rhs.x), self.y.saturating_sub(rhs.y))
    }
}

impl std::ops::SubAssign for Pos2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl std::ops::Add<Vec2> for Pos2 {
    type Output = Self;
    fn add(self, rhs: Vec2) -> Self::Output {
        pos2(self.x.saturating_add(rhs.x), self.y.saturating_add(rhs.y))
    }
}

impl std::ops::Sub<Vec2> for Pos2 {
    type Output = Self;
    fn sub(self, rhs: Vec2) -> Self::Output {
        pos2(self.x.saturating_sub(rhs.x), self.y.saturating_sub(rhs.y))
    }
}
