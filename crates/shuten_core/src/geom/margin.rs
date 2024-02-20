use super::{vec2, Rect, Vec2};

/// A margin is an inset of a rectangle
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Margin {
    pub left: u16,
    pub top: u16,
    pub right: u16,
    pub bottom: u16,
}

impl From<u16> for Margin {
    fn from(value: u16) -> Self {
        Self::same(value)
    }
}

impl From<Vec2> for Margin {
    fn from(value: Vec2) -> Self {
        Self::symmetric(value.x, value.y)
    }
}

impl Default for Margin {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Margin {
    /// A zero-sized margin
    pub const ZERO: Self = Self {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };

    /// A margin with the same vertical and horizontal inset
    pub const fn same(margin: u16) -> Self {
        Self::symmetric(margin, margin)
    }

    /// A margin with a horizontal and vertical inset
    pub const fn symmetric(x: u16, y: u16) -> Self {
        Self {
            left: x,
            top: y,
            right: x,
            bottom: y,
        }
    }

    /// Sum the horizontal margins together, and the vertical margins together
    pub const fn sum(&self) -> Vec2 {
        vec2(self.left + self.right, self.top + self.bottom)
    }

    /// Get the left-top inset
    pub const fn left_top(&self) -> Vec2 {
        vec2(self.left, self.top)
    }

    /// get the right-bottom inset
    pub const fn right_bottom(&self) -> Vec2 {
        vec2(self.right, self.bottom)
    }

    /// Is this a uniform inset?
    pub const fn is_same(&self) -> bool {
        self.left == self.right && self.left == self.top && self.left == self.bottom
    }

    /// Expand a rect to fit the inset
    pub fn expand_rect(&self, rect: Rect) -> Rect {
        Rect::from_min_max(
            rect.min - self.left_top(), //
            rect.max + self.right_bottom(),
        )
    }

    /// Shrink a rect to fit the inset
    pub fn shrink_rect(&self, rect: Rect) -> Rect {
        Rect::from_min_max(
            rect.min + self.left_top(), //
            rect.max - self.right_bottom(),
        )
    }
}
