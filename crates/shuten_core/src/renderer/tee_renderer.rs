use crate::{
    geom::Pos2,
    style::{Attribute, Rgb},
};

use super::Renderer;
use std::io::Result;

pub struct TeeRenderer<L: Renderer, R: Renderer> {
    left: L,
    right: R,
}

impl<L: Renderer, R: Renderer> TeeRenderer<L, R> {
    pub const fn new(left: L, right: R) -> Self {
        Self { left, right }
    }

    pub fn into_inner(self) -> (L, R) {
        (self.left, self.right)
    }
}

impl<L: Renderer, R: Renderer> Renderer for TeeRenderer<L, R> {
    fn begin(&mut self) -> Result<()> {
        self.left.begin()?;
        self.right.begin()
    }

    fn end(&mut self) -> Result<()> {
        self.left.end()?;
        self.right.end()
    }

    fn clear_screen(&mut self) -> Result<()> {
        self.left.clear_screen()?;
        self.right.clear_screen()
    }

    fn move_to(&mut self, pos: Pos2) -> Result<()> {
        self.left.move_to(pos)?;
        self.right.move_to(pos)
    }

    fn set_fg(&mut self, color: Rgb) -> Result<()> {
        self.left.set_fg(color)?;
        self.right.set_fg(color)
    }

    fn set_bg(&mut self, color: Rgb) -> Result<()> {
        self.left.set_bg(color)?;
        self.right.set_bg(color)
    }

    fn set_attr(&mut self, attr: Attribute) -> Result<()> {
        self.left.set_attr(attr)?;
        self.right.set_attr(attr)
    }

    fn reset_fg(&mut self) -> Result<()> {
        self.left.reset_fg()?;
        self.right.reset_fg()
    }

    fn reset_bg(&mut self) -> Result<()> {
        self.left.reset_bg()?;
        self.right.reset_bg()
    }

    fn reset_attr(&mut self) -> Result<()> {
        self.left.reset_attr()?;
        self.right.reset_attr()
    }

    fn write(&mut self, char: char) -> Result<()> {
        self.left.write(char)?;
        self.right.write(char)
    }

    fn set_title(&mut self, title: &str) -> Result<()> {
        self.left.set_title(title)?;
        self.right.set_title(title)
    }

    fn hide_cursor(&mut self) -> Result<()> {
        self.left.hide_cursor()?;
        self.right.hide_cursor()
    }

    fn show_cursor(&mut self) -> Result<()> {
        self.left.show_cursor()?;
        self.right.show_cursor()
    }

    fn capture_mouse(&mut self) -> Result<()> {
        self.left.capture_mouse()?;
        self.right.capture_mouse()
    }

    fn release_mouse(&mut self) -> Result<()> {
        self.left.release_mouse()?;
        self.right.release_mouse()
    }

    fn enter_alt_screen(&mut self) -> Result<()> {
        self.left.enter_alt_screen()?;
        self.right.enter_alt_screen()
    }

    fn leave_alt_screen(&mut self) -> Result<()> {
        self.left.leave_alt_screen()?;
        self.right.leave_alt_screen()
    }

    fn enable_line_wrap(&mut self) -> Result<()> {
        self.left.enable_line_wrap()?;
        self.right.enable_line_wrap()
    }

    fn disable_line_wrap(&mut self) -> Result<()> {
        self.left.disable_line_wrap()?;
        self.right.disable_line_wrap()
    }
}
