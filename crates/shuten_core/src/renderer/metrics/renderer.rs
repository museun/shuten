use std::io::Result;

use super::{FrameStats, StatsWindow as _};
use crate::renderer::Renderer;
use crate::{
    geom::Pos2,
    style::{Attribute, Rgb},
};

pub struct MetricsRenderer<'a, R: Renderer, const N: usize> {
    stats: &'a mut FrameStats<N>,
    renderer: R,
}

impl<'a, R: Renderer, const N: usize> MetricsRenderer<'a, R, N> {
    pub fn new(stats: &'a mut FrameStats<N>, renderer: R) -> Self {
        Self { stats, renderer }
    }
}

impl<'a, R: Renderer, const N: usize> Renderer for MetricsRenderer<'a, R, N> {
    fn begin(&mut self) -> Result<()> {
        self.stats.new_frame();
        self.renderer.begin()
    }

    fn end(&mut self) -> Result<()> {
        self.renderer.end()
    }

    fn clear_screen(&mut self) -> Result<()> {
        self.stats.clears.modify(|d| *d += 1);
        self.renderer.clear_screen()
    }

    fn move_to(&mut self, pos: Pos2) -> Result<()> {
        self.stats.moves.modify(|d| *d += 1);
        self.renderer.move_to(pos)
    }

    fn set_fg(&mut self, color: Rgb) -> Result<()> {
        self.stats.set_fg.modify(|d| *d += 1);
        self.renderer.set_fg(color)
    }

    fn set_bg(&mut self, color: Rgb) -> Result<()> {
        self.stats.set_bg.modify(|d| *d += 1);
        self.renderer.set_bg(color)
    }

    fn set_attr(&mut self, attr: Attribute) -> Result<()> {
        self.stats.set_attr.modify(|d| *d += 1);
        self.renderer.set_attr(attr)
    }

    fn reset_fg(&mut self) -> Result<()> {
        self.stats.reset_fg.modify(|d| *d += 1);
        self.renderer.reset_fg()
    }

    fn reset_bg(&mut self) -> Result<()> {
        self.stats.reset_bg.modify(|d| *d += 1);
        self.renderer.reset_bg()
    }

    fn reset_attr(&mut self) -> Result<()> {
        self.stats.reset_attr.modify(|d| *d += 1);
        self.renderer.reset_attr()
    }

    fn write(&mut self, char: char) -> Result<()> {
        self.stats.write.modify(|d| *d += 1);
        self.renderer.write(char)
    }

    fn set_title(&mut self, title: &str) -> Result<()> {
        self.renderer.set_title(title)
    }

    fn hide_cursor(&mut self) -> Result<()> {
        self.renderer.hide_cursor()
    }

    fn show_cursor(&mut self) -> Result<()> {
        self.renderer.show_cursor()
    }

    fn capture_mouse(&mut self) -> Result<()> {
        self.renderer.capture_mouse()
    }

    fn release_mouse(&mut self) -> Result<()> {
        self.renderer.release_mouse()
    }

    fn enter_alt_screen(&mut self) -> Result<()> {
        self.renderer.enter_alt_screen()
    }

    fn leave_alt_screen(&mut self) -> Result<()> {
        self.renderer.leave_alt_screen()
    }

    fn enable_line_wrap(&mut self) -> Result<()> {
        self.renderer.enable_line_wrap()
    }

    fn disable_line_wrap(&mut self) -> Result<()> {
        self.renderer.disable_line_wrap()
    }
}
