use std::sync::{Arc, Mutex};

use crate::terminal::Timer;

/// Configuration for a [`Terminal`](crate::Terminal)
///
/// ### Defaults for this type:
/// | option | value |
/// | --- | --- |
/// | [hide cursor](Self::hide_cursor)    | `true` |
/// | [mouse capture](Self::mouse_capture)  | `true` |
/// | [ctrl-c quits](Self::ctrl_c_quits)   | `true` |
/// | [use alt screen](Self::use_alt_screen) | `true` |
/// | [ctrl z switches](Self::ctrl_z_switches) | `false` |
/// | [timer](Self::reactive_timer)          | `reactive` |
#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub struct Config {
    pub(crate) hide_cursor: bool,
    pub(crate) mouse_capture: bool,
    pub(crate) ctrl_c_quits: bool,
    pub(crate) ctrl_z_switches: bool,
    pub(crate) use_alt_screen: bool,
    pub(crate) timer: Timer,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hide_cursor: true,
            mouse_capture: true,
            ctrl_c_quits: true,
            ctrl_z_switches: false,
            use_alt_screen: true,
            timer: Timer::default(),
        }
    }
}

impl Config {
    /// Should it hide the cursor?
    pub const fn hide_cursor(mut self, hide_cursor: bool) -> Self {
        self.hide_cursor = hide_cursor;
        self
    }

    /// Should it capture mouse inputs?
    pub const fn mouse_capture(mut self, mouse_capture: bool) -> Self {
        self.mouse_capture = mouse_capture;
        self
    }

    /// Should `Ctrl-C` trigger an [`Event::Quit`](crate::event::Event::Quit)
    pub const fn ctrl_c_quits(mut self, ctrl_c_quits: bool) -> Self {
        self.ctrl_c_quits = ctrl_c_quits;
        self
    }

    /// Should `Ctrl-z` switch between the alternative buffer screen and the normal screen?
    pub const fn ctrl_z_switches(mut self, ctrl_z_switches: bool) -> Self {
        self.ctrl_z_switches = ctrl_z_switches;
        self
    }

    /// Should it use an alternative screen?
    ///
    /// Alternative screens allow you to restore the previous screen before
    /// running the program after the program exists
    pub const fn use_alt_screen(mut self, use_alt_screen: bool) -> Self {
        self.use_alt_screen = use_alt_screen;
        self
    }

    /// Should it use a fixed timer?
    ///
    /// This takes in a desired framerate (`fps`) and produces a [`Event::Blend`](crate::event::Event::Blend) inbetween frames so you can interpolate your application state
    ///
    /// If the above is undesireable, consider using a **reactive timer** which'll only produce events as events are read from the terminal
    pub fn fixed_timer(mut self, fps: f64) -> Self {
        self.timer = Timer::fixed(fps);
        self
    }

    /// Should it use a reactive timer?
    ///
    /// This only reacts to events produced by the terminal.
    ///
    /// A [`Event::Blend`](crate::event::Event::Blend) will be produced, but its blend factor will be `1.0`
    pub const fn reactive_timer(mut self) -> Self {
        self.timer = Timer::reactive();
        self
    }
}

#[derive(Clone)]
pub struct ShareableConfig {
    inner: Arc<Mutex<Config>>,
}

impl From<Config> for ShareableConfig {
    fn from(value: Config) -> Self {
        Self {
            inner: Arc::new(Mutex::new(value)),
        }
    }
}

impl ShareableConfig {
    pub fn mutate(&self, mut f: impl FnMut(&mut Config)) {
        f(&mut *self.inner.lock().unwrap())
    }

    pub fn get<T>(&self, mut f: impl FnMut(&Config) -> T) -> T {
        f(&*self.inner.lock().unwrap())
    }
}
