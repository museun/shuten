use std::string::FromUtf8Error;

use crate::{
    geom::Pos2,
    renderer::Renderer,
    style::{Attribute, Rgb},
    Canvas, Context,
};

/// Explain, in a human form, what operations are required to update the screen
///
/// See [debug_frame](crate::renderer::help::debug_frame) for an alternative renderer
///
/// Usage:
/// ```rust,no_run
/// use shuten_core::geom::{pos2, vec2, rect};
/// use shuten_core::{Cell, Canvas, Context};
/// use shuten_core::renderer::help::explain_frame;
///
/// fn my_draw_code(mut canvas: Canvas) {
///     canvas.put(pos2(2, 3), Cell::new('?').fg(0xFF0000));
///     canvas.put(pos2(3, 2), Cell::new('@').bg(0x0000FF));
/// }
///
/// let mut context = Context::new(rect(vec2(10, 5)));
///
/// // returns an error if invalid utf-8 was found
/// let explain = explain_frame(&mut context, my_draw_code).unwrap();
/// println!("{explain}"); // show the explaination
///
/// /*
/// begin frame
///   move to [3,2]
///   reset attr
///   set bg rgb(0,0,255)
///     @
///   move to [2,3]
///   set fg rgb(255,0,0)
///     ?
///   move to [0,0]
///   reset bg
///   reset fg
///   reset attr
/// end frame
/// */
///
/// ```
pub fn explain_frame(
    ctx: &mut Context,
    apply: impl FnOnce(Canvas<'_>),
) -> Result<String, FromUtf8Error> {
    apply(ctx.canvas());
    let mut buf = Vec::new();
    ctx.end_frame(&mut ExplainRenderer::new(&mut buf))
        .expect("no IO should be performed here");
    String::from_utf8(buf)
}

pub struct ExplainRenderer<W> {
    out: W,
    incomplete: bool,
}

impl<W: std::io::Write> ExplainRenderer<W> {
    const fn new(out: W) -> Self {
        Self {
            out,
            incomplete: false,
        }
    }

    fn next_entry(&mut self) -> std::io::Result<()> {
        if self.incomplete {
            writeln!(&mut self.out)?;
            self.incomplete = !self.incomplete
        }
        Ok(())
    }
}

impl<W: std::io::Write> Renderer for ExplainRenderer<W> {
    fn begin(&mut self) -> std::io::Result<()> {
        self.next_entry()?;
        writeln!(&mut self.out, "begin frame")
    }

    fn end(&mut self) -> std::io::Result<()> {
        self.next_entry()?;
        writeln!(&mut self.out, "end frame")
    }

    fn move_to(&mut self, pos: Pos2) -> std::io::Result<()> {
        self.next_entry()?;
        writeln!(&mut self.out, "  move to {pos:?}")
    }

    fn set_fg(&mut self, color: Rgb) -> std::io::Result<()> {
        self.next_entry()?;
        writeln!(&mut self.out, "  set fg {color:?}")
    }

    fn set_bg(&mut self, color: Rgb) -> std::io::Result<()> {
        self.next_entry()?;
        writeln!(&mut self.out, "  set bg {color:?}")
    }

    fn set_attr(&mut self, attr: Attribute) -> std::io::Result<()> {
        self.next_entry()?;
        writeln!(&mut self.out, "  set attr {attr:?}")
    }

    fn reset_fg(&mut self) -> std::io::Result<()> {
        self.next_entry()?;
        writeln!(&mut self.out, "  reset fg")
    }

    fn reset_bg(&mut self) -> std::io::Result<()> {
        self.next_entry()?;
        writeln!(&mut self.out, "  reset bg")
    }

    fn reset_attr(&mut self) -> std::io::Result<()> {
        self.next_entry()?;
        writeln!(&mut self.out, "  reset attr")
    }

    fn write(&mut self, char: char) -> std::io::Result<()> {
        if !self.incomplete {
            write!(&mut self.out, "    ")?;
        }
        self.incomplete = true;
        let d = match char {
            ' ' => '░',
            d => d,
        };

        write!(&mut self.out, "{}", d.escape_debug())
    }

    fn set_title(&mut self, _: &str) -> std::io::Result<()> {
        Ok(())
    }

    fn hide_cursor(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn show_cursor(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn capture_mouse(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn release_mouse(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
