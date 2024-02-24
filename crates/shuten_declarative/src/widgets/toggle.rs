use crate::{
    input::{KeyEventKind, Keybind},
    widget::prelude::*,
};

#[derive(Debug)]
pub struct Toggle {
    keybind: Keybind,
}

impl Toggle {
    pub fn new(keybind: impl Into<Keybind>) -> Self {
        Self {
            keybind: keybind.into(),
        }
    }

    pub fn show(self, children: impl FnOnce()) -> Response<ToggleResponse> {
        ToggleWidget::show_children(children, self)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ToggleResponse {
    pub shown: bool,
}

#[derive(Debug)]
struct ToggleWidget {
    props: Toggle,
    shown: bool,
}

impl Default for ToggleWidget {
    fn default() -> Self {
        Self {
            props: Toggle::new(KeyEventKind::Escape),
            shown: false,
        }
    }
}

impl Widget for ToggleWidget {
    type Props<'a> = Toggle;
    type Response = ToggleResponse;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
        ToggleResponse { shown: self.shown }
    }

    fn interest(&self) -> Interest {
        Interest::KEY_INPUT
    }

    fn event(&mut self, _ctx: EventCtx<'_>, event: &Event) -> Handled {
        if let &Event::KeyChanged { key, modifiers } = event {
            if self.props.keybind == Keybind::new(key.kind, modifiers) {
                self.shown = !self.shown;
                return Handled::Sink;
            }
        }
        Handled::Bubble
    }

    fn layout(&self, ctx: LayoutCtx<'_>, input: Constraints) -> Vec2f {
        if !self.shown {
            // TODO hide children
            return Vec2f::ZERO;
        }
        self.default_layout(ctx, input)
    }

    fn paint(&self, ctx: PaintCtx<'_, '_>) {
        if !self.shown {
            return;
        }
        self.default_paint(ctx)
    }
}

pub fn toggle_bind(keybind: Keybind, children: impl FnOnce()) -> Response<ToggleResponse> {
    Toggle::new(keybind).show(children)
}
