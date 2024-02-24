use shuten::{
    event::MouseButton,
    geom::{Pos2f, Vec2f},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MouseEvent {
    MouseMove {
        pos: Pos2f,
    },
    MouseClicked {
        pos: Pos2f,
        button: MouseButton,
    },
    MouseHeld {
        pos: Pos2f,
        button: MouseButton,
    },
    MouseDragStart {
        pos: Pos2f,
        button: MouseButton,
    },
    MouseDrag {
        origin: Pos2f,
        pos: Pos2f,
        delta: Vec2f,
        button: MouseButton,
    },
    MouseDragReleased {
        origin: Pos2f,
        pos: Pos2f,
        delta: Vec2f,
        button: MouseButton,
    },
    MouseScroll {
        delta: Vec2f,
        pos: Pos2f,
    },
}

pub trait TranslateMouseEvent {
    fn translate(&self) -> Option<MouseEvent>;
}

impl TranslateMouseEvent for shuten::event::MouseEvent {
    fn translate(&self) -> Option<MouseEvent> {
        match *self {
            Self::Move { pos } => Some(MouseEvent::MouseMove { pos: pos.into() }),

            Self::Clicked { pos, button } => Some(MouseEvent::MouseClicked {
                pos: pos.into(),
                button,
            }),

            Self::Held { pos, button } => Some(MouseEvent::MouseHeld {
                pos: pos.into(),
                button,
            }),

            Self::DragStart { pos, button } => Some(MouseEvent::MouseDragStart {
                pos: pos.into(),
                button,
            }),

            Self::Drag {
                origin,
                pos,
                delta,
                button,
            } => Some(MouseEvent::MouseDrag {
                origin: origin.into(),
                pos: pos.into(),
                delta: delta.into(),
                button,
            }),

            Self::DragReleased {
                origin,
                pos,
                delta,
                button,
            } => Some(MouseEvent::MouseDragReleased {
                origin: origin.into(),
                pos: pos.into(),
                delta: delta.into(),
                button,
            }),

            Self::Scroll { dir, pos } => Some(MouseEvent::MouseScroll {
                pos: pos.into(),
                delta: dir.into(),
            }),
            _ => None,
        }
    }
}
