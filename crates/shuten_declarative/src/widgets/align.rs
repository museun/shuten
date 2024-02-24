use shuten::geom::Align2;

use crate::{
    geom::{vec2f, Constraints, Vec2f},
    layout::LayoutCtx,
    widget::{Response, Widget},
    WidgetExt as _,
};

use super::NoResponse;

#[derive(Debug)]
pub struct Align {
    align: Align2,
}

impl Align {
    pub const fn new(align: Align2) -> Self {
        Self { align }
    }

    pub const fn center() -> Self {
        Self::new(Align2::CENTER_CENTER)
    }

    pub fn show(self, children: impl FnOnce()) -> Response {
        AlignWidget::show_children(children, self)
    }
}

#[derive(Debug)]
struct AlignWidget {
    props: Align,
}

impl Default for AlignWidget {
    fn default() -> Self {
        Self {
            props: Align::center(),
        }
    }
}

impl Widget for AlignWidget {
    type Props<'a> = Align;
    type Response = NoResponse;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, mut ctx: LayoutCtx<'_>, input: Constraints) -> Vec2f {
        let node = ctx.tree.get_current();
        let constraints = Constraints::loose(input.max);

        let mut size = input.size();
        let align = vec2f(self.props.align.x.factor(), self.props.align.y.factor());

        for &child in &node.children {
            let new_size = ctx.calculate(child, constraints);
            size = size.max(new_size);
            let pos = (align * size - align * new_size).to_pos2();
            ctx.layout.set_pos(child, pos);
        }

        size
    }
}

pub fn align(align: Align2, children: impl FnOnce()) -> Response {
    Align::new(align).show(children)
}

pub fn center(children: impl FnOnce()) -> Response {
    Align::center().show(children)
}
