//! Render abstractions used by a [`Context`](crate::Context)
//!
//! Generally you'll only use the [`TermRenderer`]
//!
//! ---
//!
//! A no-op renderer is also provided:
//! - [`NullRenderer`]
use std::io::Result;

use crate::{
    geom::Pos2,
    style::{Attribute, Rgb},
};

/// A renderer is a simple abstraction over *what* operations are done to a terminal
pub trait Renderer {
    /// Begin a new frame
    fn begin(&mut self) -> Result<()>;
    /// End the current frame
    fn end(&mut self) -> Result<()>;

    /// Move the cursor to [`pos`](Pos2)
    fn move_to(&mut self, pos: Pos2) -> Result<()>;

    /// Set the foreground to [`Rgb`]
    fn set_fg(&mut self, color: Rgb) -> Result<()>;
    /// Set the background to [`Rgb`]
    fn set_bg(&mut self, color: Rgb) -> Result<()>;
    /// Set the attribute to [`Attribute`]
    fn set_attr(&mut self, attr: Attribute) -> Result<()>;

    /// Reset the foreground to the terminal default
    fn reset_fg(&mut self) -> Result<()>;
    /// Reset the background to the terminal default
    fn reset_bg(&mut self) -> Result<()>;
    /// Reset the current attribute
    fn reset_attr(&mut self) -> Result<()>;

    /// Write a character to the terminal
    fn write(&mut self, char: char) -> Result<()>;

    /// Set the title of the terminal
    fn set_title(&mut self, title: &str) -> Result<()>;

    /// Hide the cursor for the terminal
    fn hide_cursor(&mut self) -> Result<()>;
    /// Show the cursor for the terminal
    fn show_cursor(&mut self) -> Result<()>;

    /// Capture the mouse, producing mouse events
    fn capture_mouse(&mut self) -> Result<()>;
    /// Release the mouse, suppressing mouse events
    fn release_mouse(&mut self) -> Result<()>;
}

mod null_renderer;
pub use null_renderer::NullRenderer;

mod term_renderer;
pub use term_renderer::TermRenderer;

pub mod help;
