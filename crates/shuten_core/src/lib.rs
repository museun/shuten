//! Shuten is an opionate crate for drawing terminal interfaces
//!
//! This is the low level `core` crate that provides simple drawing and event handling
//!
//! ## Basic usage:
//! ```rust,no_run
//! use shuten_core::{Terminal, event::Event, Cell};
//! # fn main() -> std::io::Result<()> {
//! // create a new terminal
//! let mut terminal = Terminal::new(Config::default())?;
//! // wait for an event
//! while let Ok(event) = terminal.wait_for_next_event() {
//!     // if its a quit event, just break
//!     if event.is_quit() { break }
//!
//!     // get a canvas to paint on
//!     terminal.paint(|mut canvas| {
//!         // fill it with `red`
//!         canvas.fill(0xFF0000);
//!     })?;
//! }
//!
//! # Ok(())
//! # }
//! ```

pub mod event;
pub mod geom;
// pub mod layout;
pub mod renderer;
pub mod style;

pub mod terminal;
#[doc(inline)]
pub use terminal::{Config, Terminal};

mod timer;

mod context;
pub use context::Context;

mod surface;
pub use surface::{Canvas, Cell, Surface};
