use crate::{
    geom::{Constraints, Pos2f, Vec2f},
    layout::LayoutCtx,
    widget::{Response, Widget},
    WidgetExt,
};

use super::NoResponse;

#[derive(Default, Debug)]
struct Offset {
    pos: Pos2f,
}

impl Offset {
    pub const fn new(pos: Pos2f) -> Self {
        Self { pos }
    }

    pub fn show(self, children: impl FnOnce()) -> Response {
        OffsetWidget::show_children(children, self)
    }
}

#[derive(Default, Debug)]
struct OffsetWidget {
    props: Offset,
}

impl Widget for OffsetWidget {
    type Props<'a> = Offset;
    type Response = NoResponse;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, mut ctx: LayoutCtx<'_>, input: Constraints) -> Vec2f {
        let node = ctx.tree.get_current();
        let constraints = Constraints::loose(input.max);

        let mut size = input.size();
        for &child in &node.children {
            size = size.max(ctx.calculate(child, constraints));
            ctx.layout.set_pos(child, self.props.pos);
        }
        size
    }
}

pub fn offset(pos: Pos2f, children: impl FnOnce()) -> Response {
    Offset::new(pos).show(children)
}
