use std::cell::RefCell;

use crate::widget::prelude::*;

pub enum PanelKind {
    Vertical,
    Horizontal,
}

pub struct Panel {
    kind: PanelKind,
}

impl Panel {
    pub const fn new(kind: PanelKind) -> Self {
        Self { kind }
    }

    pub const fn vertical() -> Self {
        Self::new(PanelKind::Vertical)
    }

    pub const fn horizontal() -> Self {
        Self::new(PanelKind::Horizontal)
    }

    pub fn show(self, children: impl FnOnce()) -> Response {
        PanelWidget::show_children(children, self)
    }
}

struct PanelWidget {
    props: Panel,
    size: RefCell<Vec2f>,
}

impl Widget for PanelWidget {
    type Props<'a> = Panel;
    type Response = NoResponse;

    fn new() -> Self {
        Self {
            props: Panel::vertical(),
            size: RefCell::new(Vec2f::ZERO),
        }
    }

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    // BUG this isn't working
    fn layout(&self, mut ctx: LayoutCtx<'_>, input: Constraints) -> Vec2f {
        let node = ctx.tree.get_current();
        let mut size = input.constrain(*self.size.borrow());
        match self.props.kind {
            PanelKind::Vertical if input.max.y.is_finite() => size.y = input.max.y,
            PanelKind::Horizontal if input.max.x.is_finite() => size.x = input.max.x,
            _ => {}
        }

        let constraints = Constraints::tight(size);
        for &child in &node.children {
            size = size.max(ctx.calculate(child, constraints))
        }

        *self.size.borrow_mut() = size;
        input.constrain_min(size)
    }

    // fn interest(&self) -> Interest {
    //     Interest::MOUSE_INSIDE | Interest::MOUSE_OUTSIDE
    // }

    // fn event(&mut self, _: EventCtx<'_>, event: &Event) -> Handled {
    //     // TODO this
    //     match event {
    //         Event::MouseMoved { pos: _ } => {}
    //         _ => {}
    //     }

    //     Handled::Bubble
    // }
}

pub fn panel(kind: PanelKind, children: impl FnOnce()) -> Response {
    Panel::new(kind).show(children)
}

pub fn horizontal_panel(children: impl FnOnce()) -> Response {
    Panel::horizontal().show(children)
}

pub fn vertical_panel(children: impl FnOnce()) -> Response {
    Panel::vertical().show(children)
}
