use shuten::geom::almost_eq;

use crate::{
    geom::Pos2f,
    input::{Event, EventCtx, Handled, Interest},
    widget::Response,
    Widget, WidgetExt,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MouseAreaResponse {
    pub contains: bool,
    pub clicked: bool,
    pub held: bool,
    pub pos: Pos2f,
    pub scrolled: Option<f32>,
}

#[derive(Default, Debug)]
struct MouseAreaWidget {
    state: MouseState,
    pos: Pos2f,
    clicked: bool,
    scrolled: Option<f32>,
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
enum MouseState {
    #[default]
    None,
    Hovering,
    MouseDown,
}

impl Widget for MouseAreaWidget {
    type Props<'a> = ();
    type Response = MouseAreaResponse;

    fn update(&mut self, _: Self::Props<'_>) -> Self::Response {
        let clicked = self.clicked;
        self.clicked = false;
        MouseAreaResponse {
            contains: !matches!(self.state, MouseState::None),
            clicked,
            pos: self.pos,
            held: matches!(self.state, MouseState::MouseDown),
            scrolled: self.scrolled.take().filter(|&c| !almost_eq(c, 0.0)),
        }
    }

    fn interest(&self) -> Interest {
        Interest::MOUSE_INSIDE
    }

    fn event(&mut self, _: EventCtx<'_>, event: &Event) -> Handled {
        std::mem::take(&mut self.state);

        if let Some(pos) = event.mouse_position() {
            self.pos = pos;
        }

        self.state = match event {
            Event::MouseEnter => MouseState::Hovering,
            Event::MouseLeave => MouseState::None,
            Event::MouseHeld { .. } if event.is_mouse_primary() => MouseState::MouseDown,
            Event::MouseRelease { .. } if event.is_mouse_primary() => {
                self.clicked = true;
                MouseState::Hovering
            }
            Event::MouseScroll { delta, .. } => {
                self.scrolled.replace(delta.y);
                MouseState::Hovering
            }
            _ => return Handled::Bubble,
        };

        Handled::Bubble
    }
}

pub fn mouse_area(children: impl FnOnce()) -> Response<MouseAreaResponse> {
    MouseAreaWidget::show_children(children, ())
}
