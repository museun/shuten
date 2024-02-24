use shuten::geom::vec2f;

use crate::widget::prelude::*;

#[derive(Debug)]
struct Unconstrained {
    constrain_x: bool,
    constrain_y: bool,
}

impl Unconstrained {
    const fn new() -> Self {
        Self {
            constrain_x: false,
            constrain_y: false,
        }
    }

    fn show(self, children: impl FnOnce()) -> Response {
        UnconstrainedWidget::show_children(children, self)
    }
}

#[derive(Debug)]
struct UnconstrainedWidget {
    props: Unconstrained,
}

impl Default for UnconstrainedWidget {
    fn default() -> Self {
        Self {
            props: Unconstrained::new(),
        }
    }
}

impl Widget for UnconstrainedWidget {
    type Props<'a> = Unconstrained;
    type Response = NoResponse;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, mut ctx: LayoutCtx<'_>, input: Constraints) -> Vec2f {
        let node = ctx.tree.get_current();
        let (min_x, max_x) = if self.props.constrain_x {
            (0.0, input.max.x)
        } else {
            (0.0, f32::INFINITY)
        };
        let (min_y, max_y) = if self.props.constrain_y {
            (0.0, input.max.x)
        } else {
            (0.0, f32::INFINITY)
        };

        let constraints = Constraints {
            min: vec2f(min_x, max_x),
            max: vec2f(min_y, max_y),
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
