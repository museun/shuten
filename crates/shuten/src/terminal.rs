//! Terminal types
//!
//! A [`Terminal`] is a useful abstraction for reading and writing to a terminal

use std::{
    io::{BufWriter, Write},
    time::Instant,
};

use crate::{
    event::{Event, EventKind, Key, Modifiers, MouseState},
    Config, ShareableConfig,
};

use shuten_core::{
    geom::{self, Rect, Vec2},
    renderer::{Renderer, TermRenderer},
    style::Color,
    Canvas, Context, Surface,
};

mod timer;
pub use timer::{Timer, TimerKind};

pub mod helpers;

/// A terminal abstraction
///
/// This is generally the type you ***want***
///
/// It sets up the terminal, provides a canvas to paint on and multiple ways to wait for events
pub struct Terminal {
    context: Context,
    timer: timer::Timer,
    config: ShareableConfig,
    mouse_state: MouseState,
    timer_state: TimerState,
    start: Instant,
    out: std::io::BufWriter<std::io::Stdout>,
    _guard: helpers::Guard,
}

impl Terminal {
    /// Create a new [`Terminal`] with the provided [`Config`]
    pub fn new(config: Config) -> std::io::Result<Self> {
        let (rect, out, _guard, config) = helpers::setup(config)?;
        helpers::install_panic_hook(config.clone());

        // this is an average of every cell set to an rgb color, rounded up
        let capacity = (rect.area() as usize * 21).next_power_of_two();
        Ok(Self {
            context: Context::new(rect),
            timer: config.get(|s| s.timer),
            out: BufWriter::with_capacity(capacity, out),
            mouse_state: MouseState::default(),
            timer_state: TimerState::default(),
            start: Instant::now(),
            config,
            _guard,
        })
    }

    /// Is this [`Terminal`] using a fixed timer?
    pub const fn using_fixed_timer(&self) -> bool {
        matches!(self.timer.kind, TimerKind::Fixed(..))
    }

    /// Get the current [`Rect`] for the [`Terminal`]
    pub const fn rect(&self) -> Rect {
        self.context.rect()
    }

    /// Toggle the alternative screen buffer
    pub fn toggle_alt_screen(&mut self) -> std::io::Result<()> {
        if self.is_in_alt_screen() {
            self.leave_alt_screen()
        } else {
            self.enter_alt_screen()
        }
    }
}

/// Get drawing operations
impl Terminal {
    /// Get the [`Context`] for the [`Terminal`]
    pub fn context(&mut self) -> &mut Context {
        &mut self.context
    }

    /// Get the current active [`Surface`]
    pub fn surface(&mut self) -> &mut Surface {
        self.context.surface()
    }

    /// Create a [`Canvas`] to draw to
    pub fn canvas(&mut self) -> Canvas {
        self.context.canvas()
    }

    /// Flush any pending changes to a [`TermRenderer`]
    pub fn flush(&mut self) -> std::io::Result<()> {
        self.context
            .end_frame(&mut TermRenderer::new(&mut self.out))
    }

    /// Gives you a closure with a [`Canvas`] and calls [`Flush`](Self::flush) after it returns
    pub fn paint(&mut self, mut frame: impl FnMut(Canvas<'_>)) -> std::io::Result<()> {
        frame(self.context.canvas());
        self.flush()
    }
}

/// Wait for events
impl Terminal {
    /// Wait for a specific [`Key`] press.
    ///
    /// This blocks until the [`Key`] press is produced
    pub fn wait_for_key(&mut self, key: Key, modifiers: Modifiers) {
        while let Ok(ev) = self.wait_for_next_event() {
            if let Event::Keyboard(ev, m) = ev {
                if ev == key && m == modifiers {
                    break;
                }
            }
        }
    }

    /// Wait for the [`Event::Quit`] event to be produced
    pub fn wait_for_quit(&mut self) {
        let _ = self.wait_for_event(Event::quit());
    }

    /// Wait for a specific [`Event`]
    ///
    /// You can generate an [`EventKind`]
    ///
    /// via [`Event`]
    /// - [`Event::invalidate`]
    /// - [`Event::mouse`]
    /// - [`Event::keyboard`]
    /// - [`Event::blend`]
    /// - [`Event::quit`]
    ///
    /// via: [`EventKind`]
    /// - [`EventKind::invalidate`]
    /// - [`EventKind::mouse`]
    /// - [`EventKind::keyboard`]
    /// - [`EventKind::blend`]
    /// - [`EventKind::quit`]
    ///
    pub fn wait_for_event(&mut self, event: EventKind) -> std::io::Result<Event> {
        loop {
            let ev = self.wait_for_next_event()?;
            if std::mem::discriminant(&ev) == event.0 {
                return Ok(ev);
            }
        }
    }

    /// Wait for the _next_ event
    pub fn wait_for_next_event(&mut self) -> std::io::Result<Event> {
        loop {
            match &mut self.timer.kind {
                TimerKind::Fixed(t) if self.timer_state == TimerState::Between => {
                    if t.consume() {
                        let dt = t.delta().as_secs_f32();
                        return Ok(Event::Blend(dt));
                    }
                    self.timer_state = TimerState::Next;
                }
                TimerKind::Fixed(t) => {
                    t.tick_until_ready();
                    self.timer_state = TimerState::Between;
                }
                TimerKind::Reactive if self.timer_state != TimerState::Next => {
                    let elapsed = self.start.elapsed().as_secs_f32();
                    self.start = Instant::now();
                    self.timer_state = TimerState::Next;
                    return Ok(Event::Blend(elapsed));
                }
                _ => {}
            }

            if let Some(ev) = self.read_event()? {
                self.timer_state = TimerState::Between;
                return Ok(ev);
            }
        }
    }
}

/// Terminal mode helpers
impl Terminal {
    /// Set the title of the terminal
    pub fn set_title(&self, title: &str) -> std::io::Result<()> {
        self.immediate(|mut p| p.set_title(title))
    }

    /// Show the terminal cursor
    pub fn show_cursor(&self) -> std::io::Result<()> {
        self.immediate(|mut p| p.show_cursor())
    }

    /// Hide the terminal cursor
    pub fn hide_cursor(&self) -> std::io::Result<()> {
        self.immediate(|mut p| p.hide_cursor())
    }

    /// Determines if we're in the alternative screen buffer
    pub fn is_in_alt_screen(&self) -> bool {
        self.config.get(|c| c.use_alt_screen)
    }

    /// Leave the alternative screen, if in it
    pub fn leave_alt_screen(&self) -> std::io::Result<()> {
        if self.is_in_alt_screen() {
            self.config.mutate(|c| c.use_alt_screen = false);
            self.immediate(|mut p| p.leave_alt_screen())?
        }
        Ok(())
    }

    /// Enter the alternative screen, if not in it
    pub fn enter_alt_screen(&self) -> std::io::Result<()> {
        if !self.is_in_alt_screen() {
            self.config.mutate(|c| c.use_alt_screen = true);
            self.immediate(|mut p| {
                p.enter_alt_screen()?;
                p.clear_screen()
            })?;
        }
        Ok(())
    }

    /// Release the mouse, suppressing [`mouse events`](crate::event::MouseEvent)
    pub fn release_mouse(&self) -> std::io::Result<()> {
        self.immediate(|mut p| p.release_mouse())
    }

    /// Capture the mouse, producing [`mouse events`](crate::event::MouseEvent)
    pub fn capture_mouse(&self) -> std::io::Result<()> {
        self.immediate(|mut p| p.capture_mouse())
    }
}

impl Terminal {
    fn resize(&mut self, size: Vec2) {
        // replace our bufwriter with a new, empty one, then take the configured stdout
        let (out, _) = std::mem::replace(
            &mut self.out,
            BufWriter::with_capacity(0, std::io::stdout()),
        )
        .into_parts();

        // this is an average of every cell set to an rgb color, rounded up
        let capacity = (size.x as usize * size.y as usize).next_power_of_two();
        self.out = BufWriter::with_capacity(capacity, out);
        self.context.resize(size)
    }

    fn read_event(&mut self) -> std::io::Result<Option<Event>> {
        let mut running = true;
        let mut switch = false;
        let ev = self.config.get(|config| {
            Self::translate(
                &mut running,
                &mut switch,
                &mut self.mouse_state,
                &mut self.context,
                config,
            )
        })?;
        if let Some(Event::Invalidate(rect)) = ev {
            self.resize(rect.size());
        }

        if switch != false {
            self.toggle_alt_screen()?;
        }

        if !running {
            return Ok(Some(Event::Quit));
        }

        Ok(ev)
    }

    fn immediate<F>(&self, apply: F) -> std::io::Result<()>
    where
        F: Fn(TermRenderer<&mut dyn std::io::Write>) -> std::io::Result<()>,
    {
        let mut out = self.out.get_ref();
        apply(TermRenderer::new(&mut out))?;
        out.flush()
    }

    fn translate(
        running: &mut bool,
        switch: &mut bool,
        mouse_state: &mut MouseState,
        ctx: &mut Context,
        config: &Config,
    ) -> std::io::Result<Option<Event>> {
        use crossterm::event::Event as E;

        if !crossterm::event::poll(std::time::Duration::ZERO)? {
            return Ok(None);
        }

        let ev = match crossterm::event::read()? {
            E::Key(ev) if ev.kind == crossterm::event::KeyEventKind::Press => {
                let Ok(key) = ev.code.try_into() else {
                    return Ok(None);
                };
                let modifiers = Modifiers::from(ev.modifiers);
                if matches!(key, Key::Char('c')) && modifiers.is_ctrl() && config.ctrl_c_quits {
                    *running = false;
                }
                if matches!(key, Key::Char('z')) && modifiers.is_ctrl() && config.ctrl_z_switches {
                    *switch = true;
                }
                Event::Keyboard(key, modifiers)
            }
            E::Mouse(ev) => {
                let modifiers = Modifiers::from(ev.modifiers);
                let Some(ev) = mouse_state.update(ev) else {
                    return Ok(None);
                };
                Event::Mouse(ev, modifiers)
            }

            E::Resize(cols, rows) => {
                ctx.resize(geom::vec2(cols, rows));
                let rect = ctx.rect();
                ctx.canvas().fill(Color::Reset);
                return Ok(Some(Event::Invalidate(rect)));
            }
            _ => return Ok(None),
        };

        Ok(Some(ev))
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
enum TimerState {
    Between,
    #[default]
    Next,
}
