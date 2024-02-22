//! Terminal helpers

use std::io::Write as _;

use shuten_core::geom::{rect, vec2, Rect};

use crate::Config;

/// Install a panic hook that'll restore the terminal on panics
pub fn install_panic_hook(config: Config) {
    let old = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = reset(config);
        old(info);
    }));
}

pub fn setup(config: Config) -> std::io::Result<(Rect, std::io::Stdout, Guard)> {
    use crossterm::{
        cursor, event,
        terminal::{self, *},
    };

    terminal::enable_raw_mode()?;

    let mut out = std::io::stdout();
    if config.use_alt_screen {
        crossterm::queue!(&mut out, EnterAlternateScreen)?;
    }
    crossterm::queue!(&mut out, DisableLineWrap)?;

    if config.hide_cursor {
        crossterm::queue!(&mut out, cursor::Hide)?;
    }
    if config.mouse_capture {
        crossterm::queue!(&mut out, event::EnableMouseCapture)?;
    }
    out.flush()?;

    let size = crossterm::terminal::size().map(|(w, h)| vec2(w, h))?;
    Ok((rect(size), out, Guard(config)))
}

pub fn reset(config: Config) -> std::io::Result<()> {
    use crossterm::{
        cursor, event,
        terminal::{self, *},
    };

    let mut out = std::io::stdout();
    crossterm::queue!(&mut out, EnableLineWrap)?;

    if config.use_alt_screen {
        crossterm::queue!(&mut out, LeaveAlternateScreen)?;
    }
    if config.hide_cursor {
        crossterm::queue!(&mut out, cursor::Show)?;
    }
    if config.mouse_capture {
        crossterm::queue!(&mut out, event::DisableMouseCapture)?
    }
    out.flush()?;

    terminal::disable_raw_mode()
}

/// A scope guard for restoring the terminal
pub struct Guard(pub(crate) Config);
impl Drop for Guard {
    fn drop(&mut self) {
        let _ = reset(self.0);
    }
}
