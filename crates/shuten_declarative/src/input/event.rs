use shuten::{
    event::{Modifiers, MouseButton},
    geom::{Pos2f, Vec2f},
};

use super::KeyEvent;

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Handled {
    #[default]
    Bubble,
    Sink,
}

impl Handled {
    pub const fn is_bubble(&self) -> bool {
        matches!(self, Self::Bubble)
    }

    pub const fn is_sink(&self) -> bool {
        matches!(self, Self::Sink)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Event {
    MouseEnter,
    MouseLeave,
    MouseMoved {
        pos: Pos2f,
    },
    MouseScroll {
        pos: Pos2f,
        delta: Vec2f,
        modifiers: Modifiers,
    },
    MouseDrag {
        origin: Pos2f,
        pos: Pos2f,
        delta: Vec2f,
        button: MouseButton,
        modifiers: Modifiers,
    },
    MouseDragRelease {
        origin: Pos2f,
        pos: Pos2f,
        delta: Vec2f,
        button: MouseButton,
        modifiers: Modifiers,
    },
    MouseRelease {
        inside: bool,
        pos: Pos2f,
        button: MouseButton,
        modifiers: Modifiers,
    },
    MouseHeld {
        inside: bool,
        pos: Pos2f,
        button: MouseButton,
        modifiers: Modifiers,
    },
    KeyChanged {
        key: KeyEvent,
        modifiers: Modifiers,
    },
    FocusGained,
    FocusLost,
}

impl Event {
    pub fn is_mouse_primary(&self) -> bool {
        match self {
            Self::MouseDrag { button, .. }
            | Self::MouseDragRelease { button, .. }
            | Self::MouseRelease { button, .. }
            | Self::MouseHeld { button, .. } => *button == MouseButton::Primary,
            _ => false,
        }
    }

    pub const fn mouse_position(&self) -> Option<Pos2f> {
        match self {
            Self::MouseMoved { pos }
            | Self::MouseScroll { pos, .. }
            | Self::MouseDrag { pos, .. }
            | Self::MouseDragRelease { pos, .. }
            | Self::MouseRelease { pos, .. }
            | Self::MouseHeld { pos, .. } => Some(*pos),
            _ => None,
        }
    }

    pub const fn mouse_enter(&self) -> bool {
        matches!(self, Self::MouseEnter)
    }

    pub const fn mouse_leave(&self) -> bool {
        matches!(self, Self::MouseLeave)
    }

    pub const fn held_inside(&self) -> bool {
        matches!(self, Self::MouseHeld { inside: true, .. })
    }
    pub const fn released_inside(&self) -> bool {
        matches!(self, Self::MouseRelease { inside: true, .. })
    }
}
