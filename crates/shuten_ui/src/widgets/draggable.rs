use shuten::geom::{Pos2f, Vec2f};

use crate::{
    input::{Handled, MouseDrag},
    Interest, Widget,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Dragging {
    pub origin: Pos2f,
    pub current: Pos2f,
    pub delta: Vec2f,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DraggableResponse {
    pub dragging: Option<Dragging>,
}

impl DraggableResponse {
    pub fn current(&self) -> Option<Pos2f> {
        self.dragging.map(|c| c.current)
    }
}

#[derive(Debug, Default)]
pub struct Draggable;

#[derive(Debug, Default)]
pub struct DraggableWidget {
    state: Option<Dragging>,
}

impl Widget for DraggableWidget {
    type Response = DraggableResponse;
    type Props<'a> = Draggable;

    fn update(&mut self, _: Self::Props<'_>) -> Self::Response {
        DraggableResponse {
            dragging: self.state,
        }
    }

    fn interest(&self) -> Interest {
        Interest::MOUSE_DRAG
    }

    fn on_mouse_drag(&mut self, event: MouseDrag) -> Handled {
        if event.button.is_primary() {
            if !event.released {
                self.state = Some(Dragging {
                    origin: event.origin,
                    current: event.pos,
                    delta: event.delta,
                });
                return Handled::Sink;
            }
            if self.state.take().is_some() {
                return Handled::Sink;
            }
        }

        Handled::Bubble
    }
}
