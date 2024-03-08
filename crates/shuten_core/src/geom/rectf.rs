use super::{pos2f, Pos2f, Rect, Vec2f};

/// A rectangle that uses floats to store its positions
///
/// (e.g. a 2d matrix)
///
/// This is composed of a `min` point (the _left-top_) and a `max` point (the _right-bottom_)
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Rectf {
    pub min: Pos2f,
    pub max: Pos2f,
}

impl Default for Rectf {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Rectf {
    pub const ZERO: Self = Self {
        min: Pos2f::ZERO,
        max: Pos2f::ZERO,
    };

    pub fn from_min_max(min: Pos2f, max: Pos2f) -> Self {
        Self { min, max }
    }

    pub fn from_min_size(min: Pos2f, size: Vec2f) -> Self {
        Self {
            min,
            max: min + size,
        }
    }

    pub fn from_center_size(center: Pos2f, size: Vec2f) -> Self {
        Self {
            min: center - (size * 0.5),
            max: center + (size * 0.5),
        }
    }

    pub fn width(&self) -> f32 {
        self.size().x
    }

    pub fn height(&self) -> f32 {
        self.size().y
    }

    pub const fn x(&self) -> f32 {
        self.min.x
    }

    pub const fn y(&self) -> f32 {
        self.min.y
    }

    pub const fn left(&self) -> f32 {
        self.x()
    }

    pub const fn top(&self) -> f32 {
        self.y()
    }

    pub const fn right(&self) -> f32 {
        self.max.x
    }

    pub const fn bottom(&self) -> f32 {
        self.max.y
    }

    pub fn center(&self) -> Pos2f {
        pos2f(
            (self.min.x + self.max.x) * 0.5,
            (self.min.y + self.max.y) * 0.5,
        )
    }

    pub const fn left_top(&self) -> Pos2f {
        pos2f(self.left(), self.top())
    }

    pub const fn right_top(&self) -> Pos2f {
        pos2f(self.right(), self.top())
    }

    pub const fn left_bottom(&self) -> Pos2f {
        pos2f(self.left(), self.bottom())
    }

    pub const fn right_bottom(&self) -> Pos2f {
        pos2f(self.right(), self.bottom())
    }

    pub fn size(&self) -> Vec2f {
        (self.max - self.min).to_vec2()
    }

    pub fn union(&self, other: Self) -> Self {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    pub fn intersect(&self, other: Self) -> Self {
        Self {
            min: self.min.max(other.min),
            max: self.max.min(other.max),
        }
    }

    pub fn intersects(&self, other: Self) -> bool {
        self.min.x <= other.max.x
            && other.min.x <= self.max.x
            && self.min.y <= other.max.y
            && other.min.y <= self.max.y
    }

    pub fn contains_rect(&self, other: Self) -> bool {
        self.contains(other.min) && self.contains(other.max)
    }

    pub fn contains(&self, pos: Pos2f) -> bool {
        self.min.x <= pos.x && pos.x < self.max.x && self.min.y <= pos.y && pos.y < self.max.y
    }

    pub fn set_pos(&mut self, pos: Pos2f) {
        *self = Self::from_min_size(pos, self.size())
    }

    pub fn with_size(self, size: Vec2f) -> Self {
        Self::from_min_size(self.min, size)
    }

    pub fn with_pos(self, min: Pos2f) -> Self {
        Self::from_min_max(min, self.max)
    }

    pub fn translate(self, size: Vec2f) -> Self {
        Self::from_min_size(self.min + size, self.size())
    }
}

impl From<Rect> for Rectf {
    fn from(value: Rect) -> Self {
        Self {
            min: value.min.into(),
            max: value.max.into(),
        }
    }
}

impl From<Rectf> for Rect {
    fn from(value: Rectf) -> Self {
        Self {
            min: value.min.into(),
            max: value.max.into(),
        }
    }
}
