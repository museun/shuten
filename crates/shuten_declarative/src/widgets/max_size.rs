use crate::{
    context::LayoutCtx,
    geom::{vec2f, Constraints, Vec2f},
    widget::Response,
    Widget, WidgetExt as _,
};

use super::NoResponse;

#[derive(Debug)]
pub struct MaxSize {
    max_size: Vec2f,
}

impl MaxSize {
    pub const fn new(max_size: Vec2f) -> Self {
        Self { max_size }
    }

    pub fn show(self, children: impl FnOnce()) -> Response {
        MaxSizeWidget::show_children(children, self)
    }
}

#[derive(Debug)]
struct MaxSizeWidget {
    props: MaxSize,
}

impl Default for MaxSizeWidget {
    fn default() -> Self {
        Self {
            props: MaxSize::new(Vec2f::splat(f32::INFINITY)),
        }
    }
}

impl Widget for MaxSizeWidget {
    type Props<'a> = MaxSize;
    type Response = NoResponse;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, mut ctx: LayoutCtx<'_>, input: Constraints) -> Vec2f {
        let constraint = Constraints::loose(input.max.min(dbg!(self.props.max_size)));
        let node = ctx.tree.get_current();
        let mut size = Vec2f::ZERO;
        for &child in &node.children {
            size = size.max(ctx.calculate(child, constraint))
        }
        size
    }
}

pub fn max_width(max_width: f32, children: impl FnOnce()) -> Response {
    MaxSize::new(vec2f(max_width, f32::INFINITY)).show(children)
}

pub fn max_height(max_height: f32, children: impl FnOnce()) -> Response {
    MaxSize::new(vec2f(f32::INFINITY, max_height)).show(children)
}

pub fn max_size(max_size: Vec2f, children: impl FnOnce()) -> Response {
    MaxSize::new(max_size).show(children)
}
