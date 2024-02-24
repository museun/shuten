use crate::{
    context::LayoutCtx,
    geom::{vec2f, Constraints, Vec2f},
    widget::Response,
    Widget, WidgetExt as _,
};

use super::NoResponse;

#[derive(Debug)]
pub struct MinSize {
    min_size: Vec2f,
}

impl MinSize {
    pub const fn new(min_size: Vec2f) -> Self {
        Self { min_size }
    }

    pub fn show(self, children: impl FnOnce()) -> Response {
        MinSizeWidget::show_children(children, self)
    }
}

#[derive(Debug)]
struct MinSizeWidget {
    props: MinSize,
}

impl Default for MinSizeWidget {
    fn default() -> Self {
        Self {
            props: MinSize::new(Vec2f::splat(0.0)),
        }
    }
}

impl Widget for MinSizeWidget {
    type Props<'a> = MinSize;
    type Response = NoResponse;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, mut ctx: LayoutCtx<'_>, mut input: Constraints) -> Vec2f {
        let node = ctx.tree.get_current();
        input.min.x = input.min.x.min(self.props.min_size.x);
        input.min.y = input.min.y.min(self.props.min_size.y);
        let mut size = Vec2f::ZERO;
        for &child in &node.children {
            size = size.max(ctx.calculate(child, input))
        }
        size
    }
}

pub fn min_width(min_width: f32, children: impl FnOnce()) -> Response {
    MinSize::new(vec2f(min_width, f32::INFINITY)).show(children)
}

pub fn min_height(min_height: f32, children: impl FnOnce()) -> Response {
    MinSize::new(vec2f(f32::INFINITY, min_height)).show(children)
}

pub fn min_size(min_size: Vec2f, children: impl FnOnce()) -> Response {
    MinSize::new(min_size).show(children)
}
