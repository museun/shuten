//! A simple linear layout calculator
use crate::geom::{Pos2, Rect, Vec2};

/// An axis to allocate sizes in
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

/// A layout calculator
///
/// This will `allocate` sizes in a total rect in a specific [`Axis`] and optional [`wrap`](Self::wrap)
#[derive(Copy, Clone, Debug)]
pub struct Linear {
    wrap: bool,
    clip: bool,
    axis: Axis,
    spacing: Vec2,
}

impl Linear {
    const DEFAULT: Self = Self {
        wrap: false,
        clip: false,
        spacing: Vec2::ZERO,
        axis: Axis::Horizontal,
    };

    /// Create a new Layout with the provided Axis
    pub const fn axis(mut self, axis: Axis) -> Self {
        self.axis = axis;
        self
    }

    /// Create a horizontal layout
    pub const fn horizontal() -> Self {
        Self::DEFAULT
    }

    /// Create a vertical layout
    pub const fn vertical() -> Self {
        Self {
            axis: Axis::Vertical,
            ..Self::DEFAULT
        }
    }

    /// Should it wrap?
    ///
    /// e.g.
    ///
    /// - in horizontal mode, if we reach the right side, we should translate the y axis and start at the left-side
    /// - in vertical mode, if we reach the bottom side, we should translate the x axis and start at the top-side
    pub const fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    /// If we're not wrapping, should we clip at the edge?
    pub const fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    /// The x,y spacing between items
    pub const fn spacing(mut self, spacing: Vec2) -> Self {
        self.spacing = spacing;
        self
    }

    /// Create the layout with the max rect
    ///
    /// With the [`LinearLayout`] type you can allocate [`Vec2`] sizes to get [`Rect`]s
    pub const fn layout(self, rect: Rect) -> LinearLayout {
        LinearLayout {
            linear: self,
            cursor: rect.left_top(),
            rect,
            max: 0,
        }
    }

    /// Giving an iterator of sizes and a max rect, calculate the output rectangles
    pub fn layout_iter(
        self,
        rect: Rect,
        input: impl IntoIterator<Item = Vec2>,
    ) -> impl Iterator<Item = Rect> {
        let mut layout = self.layout(rect);
        input
            .into_iter()
            .filter_map(move |size| layout.layout(size))
    }
}

/// An incremental layout calculator
///
/// This is create with [`Linear::layout`]
pub struct LinearLayout {
    linear: Linear,
    cursor: Pos2,
    rect: Rect,
    max: u16,
}

impl LinearLayout {
    /// Layout this size, returning a `Rect` if possible
    pub fn layout(&mut self, size: Vec2) -> Option<Rect> {
        match self.linear.axis {
            Axis::Horizontal => self.horizontal(size),
            Axis::Vertical => self.vertical(size),
        }
    }

    fn horizontal(&mut self, size: Vec2) -> Option<Rect> {
        self.max = self.max.max(size.y);
        if self.cursor.x + size.x >= self.rect.right() {
            if self.linear.clip && !self.linear.wrap {
                return None;
            }
            if self.linear.wrap {
                self.cursor.y += self.linear.spacing.y + self.max;
                self.max += size.y;
                self.cursor.x = self.rect.left();
            }
        }
        if self.cursor.y + size.y * self.linear.clip as u16 >= self.rect.bottom() {
            return None;
        }
        let rect = Rect::from_min_size(self.cursor, size);
        self.cursor.x += size.x + self.linear.spacing.x;
        Some(rect)
    }

    fn vertical(&mut self, size: Vec2) -> Option<Rect> {
        if self.cursor.y + size.y >= self.rect.bottom() {
            if self.linear.clip && !self.linear.wrap {
                return None;
            }
            if self.linear.wrap {
                self.cursor.x += size.x + self.linear.spacing.x;
                self.cursor.y = self.rect.top();
            }
        }
        if self.cursor.x + size.x * self.linear.clip as u16 >= self.rect.right() {
            return None;
        }
        let rect = Rect::from_min_size(self.cursor, size);
        self.cursor.y += size.y + self.linear.spacing.y;
        Some(rect)
    }
}
