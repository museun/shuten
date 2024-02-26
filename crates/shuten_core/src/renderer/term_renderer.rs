use crate::{
    geom::Pos2,
    renderer::Renderer,
    style::{Attribute, Rgb},
};

/// A terminal [`Renderer`]
///
/// This converts the diff (and patch) from the [`Context`](crate::Context) to escape sequences used by a terminal emulator to draw things
pub struct TermRenderer<W> {
    pub(crate) out: W,
}

impl<W> TermRenderer<W>
where
    W: std::io::Write,
{
    /// Create a new [`Renderer`] from this [`Writer`](std::io::Write)
    pub const fn new(writer: W) -> Self {
        Self { out: writer }
    }
    /// Get the inner writer
    pub fn inner(&mut self) -> &mut W {
        &mut self.out
    }
}

#[cfg_attr(feature = "profiling", profiling::all_functions)]
impl<W> Renderer for TermRenderer<W>
where
    W: std::io::Write,
{
    fn begin(&mut self) -> std::io::Result<()> {
        self.out.write_all(b"\x1b[?2026h")
    }

    fn end(&mut self) -> std::io::Result<()> {
        self.out.write_all(b"\x1b[?2026l")?;
        self.out.flush()
    }

    fn clear_screen(&mut self) -> std::io::Result<()> {
        self.out.write_all(b"\x1b[?2J")
    }

    #[inline]
    fn move_to(&mut self, pos: Pos2) -> std::io::Result<()> {
        self.out
            .write_fmt(format_args!("\x1b[{y};{x};H", x = pos.x + 1, y = pos.y + 1))
    }

    #[inline]
    fn set_fg(&mut self, color: Rgb) -> std::io::Result<()> {
        let Rgb(r, g, b) = color;
        self.out.write_fmt(format_args!("\x1b[38;2;{r};{g};{b}m"))
    }

    #[inline]
    fn set_bg(&mut self, color: Rgb) -> std::io::Result<()> {
        let Rgb(r, g, b) = color;
        self.out.write_fmt(format_args!("\x1b[48;2;{r};{g};{b}m"))
    }

    #[inline]
    fn set_attr(&mut self, attr: Attribute) -> std::io::Result<()> {
        // TODO make this expansive by iterating from the lsb to the msb
        if attr.is_bold() {
            self.out.write_all(b"\x1b[1m")?
        }
        if attr.is_faint() {
            self.out.write_all(b"\x1b[2m")?
        }
        if attr.is_italic() {
            self.out.write_all(b"\x1b[3m")?
        }
        if attr.is_underline() {
            self.out.write_all(b"\x1b[4m")?
        }
        if attr.is_blink() {
            self.out.write_all(b"\x1b[5m")?
        }
        if attr.is_reverse() {
            self.out.write_all(b"\x1b[7m")?
        }
        if attr.is_strike_out() {
            self.out.write_all(b"\x1b[9m")?
        }
        Ok(())
    }

    #[inline]
    fn reset_fg(&mut self) -> std::io::Result<()> {
        self.out.write_all(b"\x1b[39m")
    }

    #[inline]
    fn reset_bg(&mut self) -> std::io::Result<()> {
        self.out.write_all(b"\x1b[49m")
    }

    #[inline]
    fn reset_attr(&mut self) -> std::io::Result<()> {
        self.out.write_all(b"\x1b[0m")
    }

    #[inline]
    fn write(&mut self, char: char) -> std::io::Result<()> {
        self.out.write_all(char.encode_utf8(&mut [0; 4]).as_bytes())
    }

    fn set_title(&mut self, title: &str) -> std::io::Result<()> {
        self.out.write_fmt(format_args!("\x1b]2;{title}\x07"))
    }

    fn hide_cursor(&mut self) -> std::io::Result<()> {
        self.out.write_all(b"\x1b[?25l")
    }

    fn show_cursor(&mut self) -> std::io::Result<()> {
        self.out.write_all(b"\x1b[?25h")
    }

    fn capture_mouse(&mut self) -> std::io::Result<()> {
        self.out
            .write_all(b"\x1b[?1000h\x1b[?1002h\x1b[?1003h\x1b[?1006h\x1b[?1015h")
    }

    fn release_mouse(&mut self) -> std::io::Result<()> {
        self.out
            .write_all(b"\x1b[?1000l\x1b[?1002l\x1b[?1003l\x1b[?1006l\x1b[?1015l")
    }

    fn enter_alt_screen(&mut self) -> std::io::Result<()> {
        self.out.write_all(b"\x1b[?1049h")
    }

    fn leave_alt_screen(&mut self) -> std::io::Result<()> {
        self.out.write_all(b"\x1b[?1049l")
    }

    fn enable_line_wrap(&mut self) -> std::io::Result<()> {
        self.out.write_all(b"\x1b[?7h")
    }

    fn disable_line_wrap(&mut self) -> std::io::Result<()> {
        self.out.write_all(b"\x1b[?7l")
    }
}
