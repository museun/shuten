use shuten::event::MouseButton;

use crate::{
    geom::{Pos2f, Vec2f},
    input::{Event, EventCtx, Handled, Interest},
    widget::Response,
    Widget, WidgetExt,
};

#[derive(Copy, Clone, Debug)]
pub struct DraggableResponse {
    pub dragging: Option<Dragging>,
}

#[derive(Copy, Clone, Debug)]
pub struct Dragging {
    pub origin: Pos2f,
    pub current: Pos2f,
    pub delta: Vec2f,
}

#[derive(Debug)]
pub struct Draggable;

impl Draggable {
    fn show(self, children: impl FnOnce()) -> Response<DraggableResponse> {
        DraggableWidget::show_children(children, self)
    }
}

#[derive(Default, Debug)]
struct DraggableWidget {
    state: Option<Dragging>,
}

impl Widget for DraggableWidget {
    type Props<'a> = Draggable;
    type Response = DraggableResponse;

    fn update(&mut self, _: Self::Props<'_>) -> Self::Response {
        DraggableResponse {
            dragging: self.state,
        }
    }

    fn interest(&self) -> Interest {
        Interest::MOUSE_ALL
    }

    fn event(&mut self, _: EventCtx<'_>, event: &Event) -> Handled {
        match *event {
            Event::MouseDrag {
                origin,
                pos,
                delta,
                button: MouseButton::Primary,
                ..
            } => {
                self.state = Some(Dragging {
                    origin,
                    delta,
                    current: pos,
                });
                Handled::Sink
            }

            Event::MouseDragRelease {
                button: MouseButton::Primary,
                ..
            } if self.state.is_some() => {
                self.state.take();
                Handled::Sink
            }

            _ => Handled::Bubble,
        }
    }
}

pub fn draggable(children: impl FnOnce()) -> Response<DraggableResponse> {
    Draggable.show(children)
}
