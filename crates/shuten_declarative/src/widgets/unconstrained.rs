use shuten::geom::vec2f;

use crate::widget::prelude::*;

#[derive(Debug, Default)]
pub struct Unconstrained {
    constrain_x: bool,
    constrain_y: bool,
}

impl Unconstrained {
    pub const fn new() -> Self {
        Self {
            constrain_x: false,
            constrain_y: false,
        }
    }

    pub const fn constrain_x(mut self, constrain_x: bool) -> Self {
        self.constrain_x = constrain_x;
        self
    }

    pub const fn constrain_y(mut self, constrain_y: bool) -> Self {
        self.constrain_y = constrain_y;
        self
    }

    pub fn show(self, children: impl FnOnce()) -> Response {
        UnconstrainedWidget::show_children(children, self)
    }
}

#[derive(Debug, Default)]
struct UnconstrainedWidget {
    props: Unconstrained,
}

impl Widget for UnconstrainedWidget {
    type Props<'a> = Unconstrained;
    type Response = NoResponse;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, mut ctx: LayoutCtx<'_>, input: Constraints) -> Vec2f {
        let node = ctx.tree.get_current();
        let max_x = if self.props.constrain_x {
            input.max.x
        } else {
            f32::INFINITY
        };
        let max_y = if self.props.constrain_y {
            input.max.y
        } else {
            f32::INFINITY
        };

        let constraints = Constraints {
            min: vec2f(0.0, max_x),
            max: vec2f(0.0, max_y),
        };

        let mut size = Vec2f::ZERO;
        for &child in &node.children {
            size = size.max(ctx.calculate(child, constraints))
        }
        input.constrain_min(size)
    }
}

pub fn unconstrained(children: impl FnOnce()) -> Response {
    Unconstrained::new().show(children)
}
