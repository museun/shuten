use std::ops::{Range, RangeInclusive};

use super::{pos2, Pos2, Rect, Vec2};

/// Alignment
#[derive(Copy, Clone, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Align {
    #[default]
    /// Min (Left, or Top)
    Min,
    /// Center
    Center,
    /// Max (Right, or Bottom)
    Max,
}

impl Align {
    pub const LEFT: Self = Self::Min;
    pub const RIGHT: Self = Self::Max;
    pub const TOP: Self = Self::Min;
    pub const BOTTOM: Self = Self::Max;
}

impl Align {
    pub fn align_size_in_range(
        self,
        size: u16,
        range: impl Into<RangeInclusive<u16>>,
    ) -> Range<u16> {
        let range = range.into();

        match self {
            Self::Min => *range.start()..*range.start() + size,
            Self::Center => {
                let left = (range.start() + range.end()) / 2;
                left..left + size
            }
            Self::Max => *range.end() - size..*range.end(),
        }
    }

    pub const fn factor(self) -> f32 {
        match self {
            Self::Min => 0.0,
            Self::Center => 0.5,
            Self::Max => 1.0,
        }
    }
}

/// Two dimensional alignment
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Align2 {
    /// Horizontal
    pub x: Align,
    /// Vertical
    pub y: Align,
}

// TODO get rid of this
impl Align2 {
    pub fn align_size_in_rect(self, size: Vec2, rect: Rect) -> Rect {
        let range = rect.left()..=rect.right();
        let x = self.x.align_size_in_range(size.x, range);
        let range = rect.top()..=rect.bottom();
        let y = self.y.align_size_in_range(size.y, range);
        Rect::from_min_max(pos2(x.start, y.start), pos2(x.end, y.end))
    }

    pub const fn anchor_rect(&self, rect: Rect) -> Rect {
        let x = match self.x {
            Align::Min => rect.left(),
            Align::Center => rect.left() + rect.width() / 2 - (rect.width() % 2),
            Align::Max => rect.width() - rect.left(),
        };
        let y = match self.y {
            Align::Min => rect.top(),
            Align::Center => rect.top() + rect.height() / 2,
            Align::Max => rect.height() - rect.top(),
        };
        Rect::from_min_max(pos2(x, y), rect.max)
    }

    pub const fn pos_rect(&self, rect: Rect) -> Pos2 {
        let x = match self.x {
            Align::Min => rect.left(),
            Align::Center => rect.center().x,
            Align::Max => rect.right(),
        };

        let y = match self.y {
            Align::Min => rect.top(),
            Align::Center => rect.center().y,
            Align::Max => rect.bottom(),
        };

        pos2(x, y)
    }
}

impl Default for Align2 {
    fn default() -> Self {
        Self::LEFT_TOP
    }
}

impl Align2 {
    pub const ALL_ALIGNMENTS: [Self; 9] = [
        Self::LEFT_TOP,
        Self::CENTER_TOP,
        Self::RIGHT_TOP,
        Self::LEFT_CENTER,
        Self::CENTER_CENTER,
        Self::RIGHT_CENTER,
        Self::LEFT_BOTTOM,
        Self::CENTER_BOTTOM,
        Self::RIGHT_BOTTOM,
    ];

    pub const LEFT_TOP: Self = Self {
        x: Align::Min,
        y: Align::Min,
    };

    pub const CENTER_TOP: Self = Self {
        x: Align::Center,
        y: Align::Min,
    };

    pub const RIGHT_TOP: Self = Self {
        x: Align::Max,
        y: Align::Min,
    };

    pub const LEFT_CENTER: Self = Self {
        x: Align::Min,
        y: Align::Center,
    };

    pub const CENTER_CENTER: Self = Self {
        x: Align::Center,
        y: Align::Center,
    };

    pub const RIGHT_CENTER: Self = Self {
        x: Align::Max,
        y: Align::Center,
    };

    pub const LEFT_BOTTOM: Self = Self {
        x: Align::Min,
        y: Align::Max,
    };

    pub const CENTER_BOTTOM: Self = Self {
        x: Align::Center,
        y: Align::Max,
    };

    pub const RIGHT_BOTTOM: Self = Self {
        x: Align::Max,
        y: Align::Max,
    };
}
