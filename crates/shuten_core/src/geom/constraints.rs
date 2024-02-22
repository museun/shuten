use super::Vec2f;

/// Constraints for a box used in a layout
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Constraints {
    /// The minimum size that is allowed by these constraints
    pub min: Vec2f,
    /// The maximum size that is allowed by these constraints
    pub max: Vec2f,
}

impl Constraints {
    /// A constraint where the minimum size is zero and the max size is provided
    pub const fn loose(max: Vec2f) -> Self {
        Self {
            min: Vec2f::ZERO,
            max,
        }
    }

    /// A constraint fixed to the provided size
    pub const fn tight(value: Vec2f) -> Self {
        Self {
            min: value,
            max: value,
        }
    }

    /// A constraint with the minimum size of zero and maximum size of infinity
    pub const fn none() -> Self {
        Self {
            min: Vec2f::ZERO,
            max: Vec2f::splat(f32::INFINITY),
        }
    }

    /// Get the size closest to the given size that satisfied the minimum constraints
    pub fn constrain_min(&self, base: Vec2f) -> Vec2f {
        base.max(self.min)
    }

    /// Get the size closest to the given size that fits the constraints
    pub fn constrain(&self, base: Vec2f) -> Vec2f {
        base.max(self.min).min(self.max)
    }

    /// Get the width closest to the given width that fits the constraints
    pub fn constrain_width(&self, width: f32) -> f32 {
        width.max(self.min.x).min(self.max.x)
    }

    /// Get the width closest to the given width that fits the constraints
    pub fn constrain_height(&self, height: f32) -> f32 {
        height.max(self.min.y).min(self.max.y)
    }

    /// Constraints are loose if there is no minimum size
    pub fn is_loose(&self) -> bool {
        self.min == Vec2f::ZERO
    }

    /// Constraints are tight if the minimum and maximum size are the same
    pub fn is_tight(&self) -> bool {
        self.min == self.max
    }

    /// Constraints are bounded if the maximum size on both axii are finite
    pub fn is_bounded(&self) -> bool {
        self.max.x.is_finite() && self.max.y.is_finite()
    }

    /// Constraints are unbounded if either of the maximum sizes on the axii are infinite
    pub fn is_unbounded(&self) -> bool {
        !self.is_bounded()
    }

    /// Get the size of the constraint
    pub fn size(&self) -> Vec2f {
        if self.max.is_finite() {
            self.max
        } else if self.min.is_finite() {
            self.min
        } else {
            Vec2f::ZERO
        }
    }
}
