//! Terminal helpers

use std::io::Write;

use super::Config;
use crate::geom::{vec2, Pos2, Rect};

/// Set up the terminal with the provided [`Config`]
///
/// ## Returns:
///
/// This returns a tuple of
/// - the current screen [`Rect`]
/// - the output sink
/// - and a [`Guard`] that restores the terminal when dropped
pub fn setup(config: Config) -> std::io::Result<(Rect, std::io::Stdout, Guard)> {
    crossterm::terminal::enable_raw_mode()?;

    let mut out = std::io::stdout();
    if config.use_alt_screen {
        out.write_all(b"\x1b[?1049h")?;
    }
    out.write_all(b"\x1b[?7l")?;

    if config.hide_cursor {
        out.write_all(b"\x1b[?25l")?;
    }
    if config.mouse_capture {
        crossterm::queue!(&mut out, crossterm::event::EnableMouseCapture)?;
    }
    out.flush()?;

    let (w, h) = crossterm::terminal::size()?;
    let rect = Rect::from_min_size(Pos2::ZERO, vec2(w, h));
    Ok((rect, out, Guard(config)))
}

/// Reset the terminal with the provided [`Config`]
///
/// Generally, you don't need to ever call this, the [`Guard`] type produced by [`setup`] will call this for you
pub fn reset(config: Config) -> std::io::Result<()> {
    let mut out = std::io::stdout();
    out.write_all(b"\x1b[?7h")?;

    if config.use_alt_screen {
        out.write_all(b"\x1b[?1049l")?;
    }
    if config.hide_cursor {
        out.write_all(b"\x1b[?25h")?;
    }
    if config.mouse_capture {
        out.write_all(b"\x1b[?1006l\x1b[?1015l\x1b[?1003l\x1b[?1002l\x1b[?1000l")?;
    }
    out.flush()?;

    crossterm::terminal::disable_raw_mode()
}

/// Install a panic hook that'll restore the terminal on panics
pub fn install_panic_hook(config: Config) {
    let old = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = reset(config);
        old(info);
    }));
}

/// A scope guard for restoring the terminal
pub struct Guard(Config);
impl Drop for Guard {
    fn drop(&mut self) {
        let _ = reset(self.0);
    }
}

#[cfg(test)]
mod tests {
    use crate::{event::Event, Terminal};

    use super::*;

    #[test]
    fn asdf() -> std::io::Result<()> {
        let mut terminal = Terminal::new(Config::default())?;
        while let Ok(ev) = terminal.wait_for_next_event() {
            if ev.is_quit() {
                break;
            }
            if let Event::Mouse(ev, ..) = ev {
                println!("{ev:?}");
            }
        }

        Ok(())
    }
}
