use crate::widget::prelude::*;

use shuten::geom::Margin;

#[derive(Default, Debug)]
struct MarginWidget {
    props: Margin,
}

impl Widget for MarginWidget {
    type Props<'a> = Margin;
    type Response = ();

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, mut ctx: LayoutCtx<'_>, input: Constraints) -> Vec2f {
        let node = ctx.tree.get_current();
        let margin = self.props.sum().into();
        let offset = self.props.left_top().to_pos2().into();
        let constraints = Constraints {
            min: (input.min - margin).max(Vec2f::ZERO),
            max: (input.max - margin).max(Vec2f::ZERO),
        };

        let mut this = Vec2f::ZERO;
        for &child in &node.children {
            this = ctx.calculate(child, constraints) + margin;
            ctx.layout.set_pos(child, offset);
        }
        this = this.max(margin);
        constraints.constrain_min(this)
    }
}

pub fn margin(margin: Margin, children: impl FnOnce()) -> Response {
    MarginWidget::show_children(children, margin)
}
