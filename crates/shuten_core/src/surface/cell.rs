use crate::style::{Attribute, Color};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CellAttr {
    Attr(Attribute),
    Reset,
}

/// Cells are written to the [`Surface`](crate::Surface) and interpreted by a [`Context`](crate::Context) and used by a [`Canvas`](crate::Canvas)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    pub(crate) char: char,
    pub(crate) fg: Color,
    pub(crate) bg: Color,
    pub(crate) attr: CellAttr,
}

impl Cell {
    /// Create an `Empty` cell
    pub const EMPTY: Self = Self::new(' ');
    /// Create a `Reset` cell
    pub const RESET: Self = Self {
        fg: Color::Reset,
        bg: Color::Reset,
        ..Self::new(' ')
    };

    /// Create a new cell that reuses prior colors
    pub const fn new(char: char) -> Self {
        Self {
            char,
            fg: Color::Reuse,
            bg: Color::Reuse,
            attr: CellAttr::Reset,
        }
    }

    /// Set the foreground of this cell
    pub fn fg(mut self, fg: impl Into<Color>) -> Self {
        self.fg = fg.into();
        self
    }

    /// Set the background of this cell
    pub fn bg(mut self, bg: impl Into<Color>) -> Self {
        self.bg = bg.into();
        self
    }

    /// Set the attribute of this cell
    ///
    /// If `none` is provided, then the attribute is reset
    pub fn attr(mut self, attr: impl Into<Option<Attribute>>) -> Self {
        self.attr = attr.into().map(CellAttr::Attr).unwrap_or(CellAttr::Reset);
        self
    }
}

impl Cell {
    pub const fn reset_attr(mut self) -> Self {
        self.attr = CellAttr::Reset;
        self
    }

    pub const fn reset_fg(mut self) -> Self {
        self.fg = Color::Reset;
        self
    }

    pub const fn reset_bg(mut self) -> Self {
        self.bg = Color::Reset;
        self
    }

    pub const fn reuse_fg(mut self) -> Self {
        self.fg = Color::Reuse;
        self
    }

    pub const fn reuse_bg(mut self) -> Self {
        self.bg = Color::Reuse;
        self
    }
}
