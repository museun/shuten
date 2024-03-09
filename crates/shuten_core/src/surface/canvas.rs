use super::{pos_to_index, Surface};
use crate::{
    geom::{pos2, Pos2, Rect},
    label::{Label, Styled},
    style::Color,
    Cell,
};

/// Canvas allows you to `paint` to a [`Surface`]
pub struct Canvas<'a> {
    surface: &'a mut Surface,
    rect: Rect,
}

impl<'a> Canvas<'a> {
    /// Create a new Canvas with the provided [`Rect`] and target [`Surface`]
    pub fn new(rect: Rect, surface: &'a mut Surface) -> Self {
        Self { surface, rect }
    }

    /// Get the [`Rect`] of this canvas
    pub const fn area(&self) -> Rect {
        self.rect
    }

    /// Create this canvas to a new (smaller) [`Rect`]
    ///
    /// This allows you to 'localize' positions into a new rect
    ///
    /// The [`Canvas::area`] of this cropped canvas will be this [`Rect`]
    ///
    pub fn crop<'b>(&'b mut self, rect: Rect) -> Canvas<'b>
    where
        Self: 'b,
    {
        Canvas {
            surface: self.surface,
            rect: Rect::from_min_size(rect.left_top(), rect.size()),
        }
    }

    /// Erase this canvas
    pub fn erase(&mut self) {
        self.erase_rect(self.area())
    }

    /// Erase this region at [`Rect`]
    pub fn erase_rect(&mut self, rect: Rect) {
        self.rect(rect, Cell::RESET)
    }

    // TODO should this be relative to our rect?
    /// Put this [`Cell`] at [`point`](Pos2)
    pub fn put(&mut self, pos: Pos2, mut cell: Cell) {
        if !self.area().contains(pos) || !self.surface.contains(pos) {
            return;
        }
        if cell.bg == Color::Reuse {
            cell.bg = self.surface[pos].bg;
        }
        if cell.fg == Color::Reuse {
            cell.fg = self.surface[pos].fg;
        }
        self.surface[pos] = cell;
    }

    /// Get the [`Cell`] at this [`point`](Pos2)
    pub fn get_mut(&mut self, pos: Pos2) -> Option<&mut Cell> {
        if !self.area().contains(pos) || !self.surface.contains(pos) {
            return None;
        }

        self.surface
            .cells
            .get_mut(pos_to_index(pos, self.surface.size.x))
    }

    /// Fill the entire canvas with the provided [`Color`]
    pub fn fill(&mut self, bg: impl Into<Color>) {
        self.fill_rect(self.area(), bg)
    }

    /// Fill the specified [`Rect`] with the provided [`Color`]
    pub fn fill_rect(&mut self, rect: Rect, bg: impl Into<Color>) {
        self.rect(rect, Cell::EMPTY.bg(bg))
    }

    /// Fill the specified [`Rect`] with the provided [`Cell`]
    pub fn rect(&mut self, rect: Rect, cell: Cell) {
        for pos in rect.indices() {
            self.put(pos, cell)
        }
    }

    pub fn label<L: Label>(&mut self, pos: Pos2, label: impl Into<Styled<L>>) {
        let mut canvas = self.crop(Rect::from_min_max(
            self.area().left_top() + pos,
            self.area().right_bottom(),
        ));

        let label = label.into();
        label.render(&mut canvas);
    }

    // TODO this
    #[allow(dead_code)]
    fn line(&mut self, start: Pos2, end: Pos2, cell: Cell) {
        let (start, end) = (start.min(end), start.max(end));

        let vertical = start.y..end.y;
        let horizontal = start.x..end.x;

        match () {
            _ if vertical.is_empty() => {
                for x in start.x..=end.x {
                    self.put(pos2(x, start.y), cell)
                }
            }
            _ if horizontal.is_empty() => {
                for y in start.y..=end.y {
                    self.put(pos2(start.x, y), cell)
                }
            }
            _ => {}
        }
    }

    /// Tries to set the [`foreground`](Color) and [`background`](Color) at the specified [`point`](Pos2)
    pub fn set_color(&mut self, pos: Pos2, fg: impl Into<Color>, bg: impl Into<Color>) {
        if let Some(cell) = self.get_mut(pos) {
            cell.fg = fg.into();
            cell.bg = bg.into();
        }
    }
}
