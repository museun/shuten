use crate::{
    geom::{Pos2, Rect, Vec2},
    renderer::Renderer,
    style::Color,
    surface::{CellAttr, Surface},
    Canvas,
};

/// Context allows efficient double buffering
///
/// You'd generally use this type over a raw [`Surface`]
pub struct Context {
    pub(crate) rect: Rect,
    pub(crate) front: Surface,
    pub(crate) back: Surface,
}

impl Context {
    /// Create a new [`Context`] bounded by a specific [`Rect`]
    pub fn new(rect: Rect) -> Self {
        Self {
            rect,
            front: Surface::new(rect.size()),
            back: Surface::new(rect.size()),
        }
    }

    /// Get the [`Rect`] for this [`Context`]
    pub const fn rect(&self) -> Rect {
        self.rect
    }

    /// Create a [`Canvas`] for this [`Context`]
    ///
    /// A [`Canvas`] allows you to easily draw things onto a [`Surface`]
    ///
    /// They are cheap to make, but you cannot copy nor clone them.
    ///
    /// If you need to share them, do it via `&mut` borrow
    pub fn canvas(&mut self) -> Canvas {
        Canvas::new(self.rect, &mut self.back)
    }

    /// Get the current [`Surface`] that'll be drawn upon
    pub fn surface(&mut self) -> &mut Surface {
        &mut self.back
    }

    /// Resize this [`Context`] using a provided [size](Vec2)
    pub fn resize(&mut self, size: Vec2) {
        self.front.resize(size);
        self.back.resize(size);
        self.rect = Rect::from_min_size(Pos2::ZERO, size);
    }

    /// End frame diffs the internal [`Surface`]s and writes a minimal amount of commands to the provided [`Renderer`].
    ///
    /// This returns any I/O error produced by the provided [`Renderer`]
    ///
    /// ---
    ///
    /// Some available [`Renderer`]s:
    /// - [`TermRenderer`](crate::renderer::TermRenderer)
    ///     - This is generally what you'd use to write things to a terminal
    /// - [`NullRenderer`](crate::renderer::NullRenderer)
    ///     - This does nothing
    pub fn end_frame(&mut self, out: &mut impl Renderer) -> std::io::Result<()> {
        let mut state = CursorState::default();
        let mut seen = false;
        let mut wrote_reset = false;

        for (pos, change) in self.front.diff(&self.back) {
            if !seen {
                out.begin()?;
                seen = true;
            }

            let pos = self.rect.left_top() + pos;

            if state.maybe_move(pos) {
                out.move_to(pos)?;
            }

            match state.maybe_attr(change.attr) {
                Some(CellAttr::Attr(attr)) => {
                    wrote_reset = false;
                    out.set_attr(attr)?
                }
                Some(CellAttr::Reset) => {
                    wrote_reset = true;
                    out.reset_attr()?
                }
                _ => {}
            }

            match state.maybe_fg(change.fg, wrote_reset) {
                Some(Color::Rgb(fg)) => out.set_fg(fg)?,
                Some(Color::Reset) => out.reset_fg()?,
                _ => {}
            }

            match state.maybe_bg(change.bg, wrote_reset) {
                Some(Color::Rgb(bg)) => out.set_bg(bg)?,
                Some(Color::Reset) => out.reset_bg()?,
                _ => {}
            }

            wrote_reset = false;
            out.write(change.char)?;
        }

        if seen {
            // move the cursor back to the beginning.
            if state.maybe_move(Pos2::ZERO) {
                // this should reduce possible flicker
                out.move_to(Pos2::ZERO)?;
            }

            // reset the colors so interleaved output is reset
            out.reset_bg()?;
            out.reset_fg()?;
            out.reset_attr()?;

            out.end()?;
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
struct CursorState {
    last: Option<Pos2>,
    fg: Option<Color>,
    bg: Option<Color>,
    attr: Option<CellAttr>,
}

impl CursorState {
    fn maybe_move(&mut self, pos: Pos2) -> bool {
        let should_move = match self.last {
            Some(last) if last.y != pos.y || last.x != pos.x.saturating_sub(1) => true,
            None => true,
            _ => false,
        };
        self.last = Some(pos);
        should_move
    }

    fn maybe_attr(&mut self, attr: CellAttr) -> Option<CellAttr> {
        match (attr, self.attr) {
            (CellAttr::Reset, None) => {
                self.attr.replace(attr);
                return Some(attr);
            }
            (CellAttr::Reset, Some(CellAttr::Reset)) => return None,
            _ => {}
        }

        let replace = self.attr.replace(attr) != Some(attr);
        replace.then_some(attr)
    }

    fn maybe_fg(&mut self, color: Color, resetting: bool) -> Option<Color> {
        Self::maybe_color(color, resetting, &mut self.fg)
    }

    fn maybe_bg(&mut self, color: Color, resetting: bool) -> Option<Color> {
        Self::maybe_color(color, resetting, &mut self.bg)
    }

    fn maybe_color(color: Color, resetting: bool, cache: &mut Option<Color>) -> Option<Color> {
        // if we're reusing a color, don't write a new one
        if matches!(color, Color::Reuse) {
            return None;
        }

        if resetting {
            cache.replace(color);
            return Some(color);
        }

        // if its a reset, we're going to be resetting until the next color
        match (color, *cache) {
            (Color::Reset, None) => {
                cache.replace(color);
                Some(Color::Reset)
            }
            // but if we're already resetting don't write anything
            (Color::Reset, Some(Color::Reset)) => None,

            _ => {
                let prev = cache.replace(color);
                (prev != Some(color)).then_some(color)
            }
        }
    }
}
