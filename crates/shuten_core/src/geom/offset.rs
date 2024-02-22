/// An offset is a signed position/vector
///
/// This allows to, say, translate by a negative value
///
/// ```rust
/// use shuten_core::geom::{Rect, pos2, vec2, offset};
/// let rect = Rect::from_min_size(pos2(10, 10), vec2(3, 3));
/// let new = rect.translate(offset(-1, -3));
/// assert_eq!(new, Rect::from_min_size(pos2(9, 7), vec2(3, 3)))
/// ```
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Offset {
    pub x: i32,
    pub y: i32,
}

impl std::fmt::Debug for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({x}, {y})", x = self.x, y = self.y)
    }
}

impl Offset {
    /// A zero offset
    pub const ZERO: Self = Self { x: 0, y: 0 };
    /// A up offset
    pub const UP: Self = Self { x: 0, y: -1 };
    /// An down offset
    pub const DOWN: Self = Self { x: 0, y: 1 };

    /// A left offset
    pub const LEFT: Self = Self { x: -1, y: 0 };
    /// A right offset
    pub const RIGHT: Self = Self { x: 1, y: 0 };
}

/// Create a new offset from the provided components
pub const fn offset(x: i32, y: i32) -> Offset {
    Offset { x, y }
}

impl std::ops::Add for Offset {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        offset(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for Offset {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        offset(self.x - rhs.x, self.y - rhs.y)
    }
}
