use shuten::geom::{Align2, Dimension, Dimension2};

use crate::widget::prelude::*;

#[derive(Debug)]
pub struct Reflow {
    anchor: Align2,
    offset: Dimension2,
}

impl Reflow {
    pub const fn new(anchor: Align2, offset: Dimension2) -> Self {
        Self { anchor, offset }
    }

    pub fn show(self, children: impl FnOnce()) -> Response {
        ReflowWidget::show_children(children, self)
    }
}

#[derive(Debug)]
struct ReflowWidget {
    props: Reflow,
}

impl Default for ReflowWidget {
    fn default() -> Self {
        Self {
            props: Reflow::new(
                Align2::LEFT_TOP,
                Dimension2::new(Dimension::absolute(1.0), Dimension::absolute(1.0)),
            ),
        }
    }
}

impl Widget for ReflowWidget {
    type Props<'a> = Reflow;
    type Response = NoResponse;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn flow(&self) -> Flow {
        Flow::Relative {
            anchor: self.props.anchor,
            offset: self.props.offset,
        }
    }

    fn layout(&self, mut ctx: LayoutCtx<'_>, _: Constraints) -> Vec2f {
        let node = ctx.tree.get_current();
        for &child in &node.children {
            ctx.calculate(child, Constraints::none());
        }
        Vec2f::ZERO
    }
}

pub fn reflow(anchor: Align2, offset: Dimension2, children: impl FnOnce()) -> Response {
    Reflow::new(anchor, offset).show(children)
}

pub fn flow(anchor: Align2, children: impl FnOnce()) -> Response {
    Reflow::new(anchor, Dimension2::ZERO).show(children)
}
