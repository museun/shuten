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
    #[cfg_attr(feature = "profiling", profiling::function)]
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
    #[cfg_attr(feature = "profiling", profiling::function)]
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

            // BUG: this needs to reset if the left attribute doesn't coalesce with this one
            if matches!(change.attr, CellAttr::New(..)) {
                wrote_reset = true;
                out.reset_attr()?;
            }

            match state.maybe_attr(change.attr) {
                Some(CellAttr::Attr(attr) | CellAttr::New(attr)) => {
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

#[cfg_attr(feature = "profiling", profiling::all_functions)]
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
        log::info!("{attr:?} | {:?}", self.attr);

        match (attr, self.attr) {
            (CellAttr::Reset, None) => {
                self.attr.replace(attr);
                return Some(attr);
            }
            (CellAttr::Reset, Some(CellAttr::Reset)) => return None,
            _ => {}
        }

        (self.attr.replace(attr) != Some(attr)).then_some(attr)
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

#[cfg(test)]
mod tests {
    use crate::{
        geom::{pos2, rect, vec2},
        style::{Attribute, Rgb},
        Cell,
    };

    use super::*;

    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum Command {
        Begin,
        End,
        ClearScreen,
        MoveTo(Pos2),
        SetFg(Rgb),
        SetBg(Rgb),
        SetAttr(Attribute),
        ResetFg,
        ResetBg,
        ResetAttr,
        Write(char),
    }

    #[derive(Debug, Default, PartialEq)]
    struct TestRenderer {
        commands: Vec<Command>,
    }

    impl Renderer for TestRenderer {
        fn begin(&mut self) -> std::io::Result<()> {
            self.commands.push(Command::Begin);
            Ok(())
        }

        fn end(&mut self) -> std::io::Result<()> {
            self.commands.push(Command::End);
            Ok(())
        }

        fn clear_screen(&mut self) -> std::io::Result<()> {
            self.commands.push(Command::ClearScreen);
            Ok(())
        }

        fn move_to(&mut self, pos: Pos2) -> std::io::Result<()> {
            self.commands.push(Command::MoveTo(pos));
            Ok(())
        }

        fn set_fg(&mut self, color: Rgb) -> std::io::Result<()> {
            self.commands.push(Command::SetFg(color));
            Ok(())
        }

        fn set_bg(&mut self, color: Rgb) -> std::io::Result<()> {
            self.commands.push(Command::SetBg(color));
            Ok(())
        }

        fn set_attr(&mut self, attr: Attribute) -> std::io::Result<()> {
            self.commands.push(Command::SetAttr(attr));
            Ok(())
        }

        fn reset_fg(&mut self) -> std::io::Result<()> {
            self.commands.push(Command::ResetFg);
            Ok(())
        }

        fn reset_bg(&mut self) -> std::io::Result<()> {
            self.commands.push(Command::ResetBg);
            Ok(())
        }

        fn reset_attr(&mut self) -> std::io::Result<()> {
            self.commands.push(Command::ResetAttr);
            Ok(())
        }

        fn write(&mut self, char: char) -> std::io::Result<()> {
            self.commands.push(Command::Write(char));
            Ok(())
        }
    }

    struct TestContext {
        context: Context,
    }

    impl TestContext {
        fn new(size: Vec2) -> Self {
            Self {
                context: Context::new(rect(size)),
            }
        }

        fn canvas(&mut self) -> Canvas {
            self.context.canvas()
        }

        fn render(&mut self) -> TestRenderer {
            let mut renderer = TestRenderer::default();
            self.context.end_frame(&mut renderer).unwrap();
            renderer
        }
    }

    #[test]
    fn attribute_cache() {
        let mut ctx = TestContext::new(vec2(3, 3));

        ctx.canvas().put(pos2(1, 1), Cell::new('a').fg(0xFF0000));
        let a = ctx.render();
        eprintln!("{a:#?}");

        ctx.canvas().put(pos2(1, 1), Cell::new('b').fg(0x00FF00));
        let b = ctx.render();
        eprintln!("{b:#?}");

        ctx.canvas().put(
            pos2(1, 1),
            Cell::new('b').fg(0x00FF00).attr(Attribute::ITALIC),
        );
        let c = ctx.render();
        eprintln!("{c:#?}");

        ctx.canvas().put(
            pos2(1, 1),
            Cell::new('b').fg(0x00FF00).attr(Attribute::BOLD),
        );
        let d = ctx.render();
        eprintln!("{d:#?}");
    }
}
