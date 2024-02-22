use std::{io::Result, str::Utf8Error};

use crate::{
    geom::Pos2,
    renderer::{Renderer, TermRenderer},
    style::{Attribute, Rgb},
    Canvas, Context,
};

struct DebugRenderer(TermRenderer<Vec<u8>>);

/// Show a somewhat readable form of the ansi escape sequences required to update the screen
///
/// See [explain_frame](crate::renderer::help::explain_frame) for an alternative renderer
///
/// Usage:
/// ```rust,no_run
/// use shuten_core::geom::{pos2, vec2, rect};
/// use shuten_core::{Cell, Canvas, Context};
/// use shuten_core::renderer::help::debug_frame;
///
/// fn my_draw_code(mut canvas: Canvas) {
///     canvas.put(pos2(2, 3), Cell::new('?').fg(0xFF0000));
///     canvas.put(pos2(3, 2), Cell::new('@').bg(0x0000FF));
/// }
///
/// let mut context = Context::new(rect(vec2(10, 5)));
///
/// // returns an error if invalid utf-8 was found
/// // `true` for a less linenoise output, `false` for the actual output
/// let explain = debug_frame(true, &mut context, my_draw_code).unwrap();
/// println!("{explain}"); // show the explaination
///
/// // with 'humanize = true'
/// /*
/// ?2026h
/// 3;4;H
/// 0m
/// 48;2;0;0;255m@
/// 4;3;H
/// 38;2;255;0;0m?
/// 1;1;H
/// 49m
/// 39m
/// 0m
/// ?2026l
/// */
///
/// // with 'humanize = false'
/// // \u{1b}[?2026h\u{1b}[3;4;H\u{1b}[0m\u{1b}[48;2;0;0;255m@\u{1b}[4;3;H\u{1b}[38;2;255;0;0m?\u{1b}[1;1;H\u{1b}[49m\u{1b}[39m\u{1b}[0m\u{1b}[?2026l
/// ```
pub fn debug_frame(
    humanize: bool,
    ctx: &mut Context,
    apply: impl FnOnce(Canvas<'_>),
) -> std::result::Result<String, Utf8Error> {
    apply(ctx.canvas());

    let buf = Vec::new();
    let mut this = DebugRenderer(TermRenderer::new(buf));
    ctx.end_frame(&mut this)
        .expect("no IO should be performed here");

    let s = std::str::from_utf8(&this.0.out)?
        .escape_default()
        .to_string();

    if humanize {
        Ok(s.replace("\\u{1b}[", "\n"))
    } else {
        Ok(s)
    }
}

impl Renderer for DebugRenderer {
    fn begin(&mut self) -> Result<()> {
        self.0.begin()
    }

    fn end(&mut self) -> Result<()> {
        self.0.end()
    }

    fn move_to(&mut self, pos: Pos2) -> Result<()> {
        self.0.move_to(pos)
    }

    fn set_fg(&mut self, color: Rgb) -> Result<()> {
        self.0.set_fg(color)
    }

    fn set_bg(&mut self, color: Rgb) -> Result<()> {
        self.0.set_bg(color)
    }

    fn set_attr(&mut self, attr: Attribute) -> Result<()> {
        self.0.set_attr(attr)
    }

    fn reset_fg(&mut self) -> Result<()> {
        self.0.reset_fg()
    }

    fn reset_bg(&mut self) -> Result<()> {
        self.0.reset_bg()
    }

    fn reset_attr(&mut self) -> Result<()> {
        self.0.reset_attr()
    }

    fn write(&mut self, char: char) -> Result<()> {
        self.0.write(char)
    }

    fn set_title(&mut self, title: &str) -> Result<()> {
        self.0.set_title(title)
    }

    fn hide_cursor(&mut self) -> Result<()> {
        self.0.hide_cursor()
    }

    fn show_cursor(&mut self) -> Result<()> {
        self.0.show_cursor()
    }

    fn capture_mouse(&mut self) -> Result<()> {
        self.0.capture_mouse()
    }

    fn release_mouse(&mut self) -> Result<()> {
        self.0.release_mouse()
    }
}
