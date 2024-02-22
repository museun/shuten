use super::{vec2f, Vec2f};

/// Describes the length of one or more measurements added together
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Dimension {
    /// The absolute measure of this dimension (in cells)
    pub absolute: f32,
    /// A value scaled based on the parent's measure on the axis
    ///
    /// 1.0 corresponds to 100% of the parent's length while 0.0 would be 0%
    pub ratio: f32,
}

impl Dimension {
    /// A length of zero
    pub const ZERO: Self = Self {
        absolute: 0.0,
        ratio: 0.0,
    };

    /// A length using absolute measurements (in cells)
    pub const fn absolute(absolute: f32) -> Self {
        Self {
            absolute,
            ratio: 0.0,
        }
    }

    /// A length using a proportional measurement (percentage of parent container)
    pub const fn ratio(ratio: f32) -> Self {
        Self {
            absolute: 0.0,
            ratio,
        }
    }

    /// Resolve the size to a singel value in cells using information from the surrounding context
    pub fn resolve(&self, parent: f32) -> f32 {
        self.absolute + parent * self.ratio
    }
}

/// A two dimensional size based on one or more measurements added together
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Dimension2 {
    /// The dimension of the x axis
    pub x: Dimension,
    /// The dimension of the y axis
    pub y: Dimension,
}

impl Dimension2 {
    /// A two dimensional size where both axii are zero
    pub const ZERO: Self = Self {
        x: Dimension::ZERO,
        y: Dimension::ZERO,
    };

    /// Create a new two dimensional size
    pub const fn new(x: Dimension, y: Dimension) -> Self {
        Self { x, y }
    }

    /// Create a new dimensional size, using absolute measurements (cells)
    pub const fn absolute(x: f32, y: f32) -> Self {
        Self {
            x: Dimension::absolute(x),
            y: Dimension::absolute(y),
        }
    }

    /// Resolve the size to a measure in cells using information regarding the surrounding context
    pub fn resolve(&self, parent: Vec2f) -> Vec2f {
        vec2f(self.x.resolve(parent.x), self.y.resolve(parent.y))
    }
}
