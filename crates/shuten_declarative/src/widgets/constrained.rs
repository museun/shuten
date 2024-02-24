use crate::widget::prelude::*;

#[derive(Debug)]
struct Constrained {
    constraints: Constraints,
}

impl Constrained {
    const fn new(constraints: Constraints) -> Self {
        Self { constraints }
    }

    fn show(self, children: impl FnOnce()) -> Response {
        ConstrainedWidget::show_children(children, self)
    }
}

#[derive(Debug)]
struct ConstrainedWidget {
    props: Constrained,
}

impl Default for ConstrainedWidget {
    fn default() -> Self {
        Self {
            props: Constrained::new(Constraints {
                min: Vec2f::ZERO,
                max: Vec2f::ZERO,
            }),
        }
    }
}

impl Widget for ConstrainedWidget {
    type Props<'a> = Constrained;
    type Response = NoResponse;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, mut ctx: LayoutCtx<'_>, input: Constraints) -> Vec2f {
        let node = ctx.tree.get_current();
        let mut size = Vec2f::ZERO;
        let constraints = Constraints {
            min: input.min.max(self.props.constraints.min),
            max: input.max.max(self.props.constraints.max),
        };
        for &child in &node.children {
            size = size.max(ctx.calculate(child, constraints));
        }
        input.constrain(constraints.constrain(size))
    }
}

pub fn constrained(constraints: Constraints, children: impl FnOnce()) -> Response {
    Constrained::new(constraints).show(children)
}
