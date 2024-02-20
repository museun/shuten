use crate::{
    geom::{pos2, Pos2, Vec2},
    style::Color,
};

mod canvas;
pub use canvas::Canvas;

mod cell;
pub use cell::{Cell, CellAttr};

/// Surface is a grid that you can write cells to
///
/// This is generally a lower-level type, normally you'd interact with the
/// `Surface` via the [`Context`](crate::Context) and [`Canvas`] types
#[derive(Debug, Default)]
pub struct Surface {
    pub(crate) cells: Vec<Cell>,
    size: Vec2,
}

impl Surface {
    /// Create a new surface with the fixed size
    pub fn new(size: Vec2) -> Self {
        let cells = vec![Cell::EMPTY; size.x as usize * size.y as usize];
        Self { cells, size }
    }

    /// Resize the surface to a new size
    pub fn resize(&mut self, size: Vec2) {
        *self = Self::new(size)
    }

    /// Does this surface contain this point?
    pub const fn contains(&self, pos: Pos2) -> bool {
        pos.x < self.size.x && pos.y < self.size.y
    }

    /// Generate a diff of two surfaces, yielding the [locations](pos2) and [`Cell`]s that are different
    ///
    /// This mutates the original cell to cache future changes
    pub fn diff<'a>(&'a mut self, other: &'a Self) -> impl Iterator<Item = (Pos2, Cell)> + '_ {
        let filter = |(i, (left, right)): (usize, (&mut Cell, Cell))| {
            if Self::filter_cell(left, &right) {
                return None;
            }
            *left = right;
            Some((index_to_pos(i, self.size.x), right))
        };

        self.cells
            .iter_mut()
            .zip(other.cells.iter().copied())
            .enumerate()
            .filter_map(filter)
    }

    fn filter_cell(left: &Cell, right: &Cell) -> bool {
        if left.char != right.char {
            return false;
        }
        if right.fg != Color::Reuse || right.bg != Color::Reuse {
            return false;
        }

        right.fg == left.fg && right.bg == left.bg
    }
}

impl std::ops::Index<Pos2> for Surface {
    type Output = Cell;
    #[inline(always)]
    /// **NOTE**: this will panic if the [`Pos2`] is out of bounds
    fn index(&self, index: Pos2) -> &Self::Output {
        assert!(
            index.x < self.size.x && index.y < self.size.y,
            "{x},{y} out of bounds of {w},{h}",
            x = index.x,
            y = index.y,
            w = self.size.x,
            h = self.size.y
        );
        &self.cells[pos_to_index(index, self.size.x)]
    }
}

impl std::ops::IndexMut<Pos2> for Surface {
    #[inline(always)]
    /// **NOTE**: this will panic if the [`Pos2`] is out of bounds
    fn index_mut(&mut self, index: Pos2) -> &mut Self::Output {
        assert!(
            index.x < self.size.x && index.y < self.size.y,
            "{x},{y} out of bounds of {w},{h}",
            x = index.x,
            y = index.y,
            w = self.size.x,
            h = self.size.y
        );
        &mut self.cells[pos_to_index(index, self.size.x)]
    }
}

const fn pos_to_index(pos: Pos2, w: u16) -> usize {
    (pos.y * w + pos.x) as usize
}

const fn index_to_pos(index: usize, w: u16) -> Pos2 {
    let index = index as u16;
    pos2(index % w, index / w)
}
