use std::io::Result;

use crate::{
    geom::Pos2,
    renderer::Renderer,
    style::{Attribute, Rgb},
};

/// A no-op [`Renderer`]
pub struct NullRenderer;

impl Renderer for NullRenderer {
    fn begin(&mut self) -> Result<()> {
        Ok(())
    }

    fn end(&mut self) -> Result<()> {
        Ok(())
    }

    fn clear_screen(&mut self) -> Result<()> {
        Ok(())
    }

    fn move_to(&mut self, _pos: Pos2) -> Result<()> {
        Ok(())
    }

    fn set_fg(&mut self, _color: Rgb) -> Result<()> {
        Ok(())
    }

    fn set_bg(&mut self, _color: Rgb) -> Result<()> {
        Ok(())
    }

    fn set_attr(&mut self, _attr: Attribute) -> Result<()> {
        Ok(())
    }

    fn reset_fg(&mut self) -> Result<()> {
        Ok(())
    }

    fn reset_bg(&mut self) -> Result<()> {
        Ok(())
    }

    fn reset_attr(&mut self) -> Result<()> {
        Ok(())
    }

    fn write(&mut self, _char: char) -> Result<()> {
        Ok(())
    }

    fn set_title(&mut self, _title: &str) -> Result<()> {
        Ok(())
    }

    fn hide_cursor(&mut self) -> Result<()> {
        Ok(())
    }

    fn show_cursor(&mut self) -> Result<()> {
        Ok(())
    }

    fn capture_mouse(&mut self) -> Result<()> {
        Ok(())
    }

    fn release_mouse(&mut self) -> Result<()> {
        Ok(())
    }

    fn enter_alt_screen(&mut self) -> Result<()> {
        Ok(())
    }

    fn leave_alt_screen(&mut self) -> Result<()> {
        Ok(())
    }

    fn enable_line_wrap(&mut self) -> Result<()> {
        Ok(())
    }

    fn disable_line_wrap(&mut self) -> Result<()> {
        Ok(())
    }
}
