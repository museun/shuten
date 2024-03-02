//! Events produced by a [`Terminal`](crate::Terminal)
use shuten_core::geom::{Pos2, Rect};

mod key;
pub use key::Key;

mod mouse;
pub(crate) use mouse::MouseState;
pub use mouse::{MouseButton, MouseEvent};

mod modifiers;
pub use modifiers::Modifiers;

/// Events produced by a [`Terminal`](crate::Terminal)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[non_exhaustive]
pub enum Event {
    /// A resize event happened, giving you the new screen [`Rect`]
    Invalidate(Rect),
    /// A mouse event happened, giving you the event and any [`Modifiers`]
    Mouse(MouseEvent, Modifiers),
    /// A keyboard event happened, giving you the event and any [`Modifiers`]
    Keyboard(Key, Modifiers),
    /// A blend happened, this allows you to interpolate by a `blend factor`
    Blend(f32),
    /// A quit event happened
    Quit,
}

impl Event {
    /// Get the [`Modifiers`] for this [`Event`]
    pub const fn modifiers(&self) -> Option<Modifiers> {
        match self {
            Self::Mouse(_, modifiers) | Self::Keyboard(_, modifiers) => Some(*modifiers),
            _ => None,
        }
    }

    /// Was this [`Event`] a [`Event::Quit`] event?
    pub const fn is_quit(&self) -> bool {
        matches!(self, Self::Quit)
    }

    /// Was this [`Event`] a [`Event::Invalidate`] event?
    pub const fn is_invalidate(&self) -> bool {
        matches!(self, Self::Invalidate(..))
    }

    /// Was this [`Event`] a [`Event::Blend`] event?
    pub const fn is_blend(&self) -> bool {
        matches!(self, Self::Blend(..))
    }

    /// Was this [`Event`] a [`MouseEvent::Move`]?
    pub const fn is_mouse_move(&self) -> bool {
        matches!(self, Self::Mouse(MouseEvent::Move { .. }, ..))
    }
}

/// Create [`EventKind`] filters for use with [`Terminal::wait_for_event`](crate::Terminal::wait_for_event)
impl Event {
    pub const fn invalidate() -> EventKind {
        EventKind::invalidate()
    }

    pub const fn mouse() -> EventKind {
        EventKind::mouse()
    }

    pub const fn keyboard() -> EventKind {
        EventKind::keyboard()
    }

    pub const fn blend() -> EventKind {
        EventKind::blend()
    }

    pub const fn quit() -> EventKind {
        EventKind::quit()
    }
}

/// Create [`EventKind`] filters for use with [`Terminal::wait_for_event`](crate::Terminal::wait_for_event)
#[derive(Copy, Clone)]
pub struct EventKind(pub(crate) std::mem::Discriminant<Event>);

impl EventKind {
    pub const fn invalidate() -> Self {
        Self(std::mem::discriminant(&Event::Invalidate(Rect::ZERO)))
    }

    pub const fn mouse() -> Self {
        Self(std::mem::discriminant(&Event::Mouse(
            MouseEvent::Move { pos: Pos2::ZERO },
            Modifiers::NONE,
        )))
    }

    pub const fn keyboard() -> Self {
        Self(std::mem::discriminant(&Event::Keyboard(
            Key::Escape,
            Modifiers::NONE,
        )))
    }

    pub const fn blend() -> Self {
        Self(std::mem::discriminant(&Event::Blend(0.0)))
    }

    pub const fn quit() -> Self {
        Self(std::mem::discriminant(&Event::Quit))
    }
}
