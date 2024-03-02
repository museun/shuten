use shuten::geom::{Constraints, Margin, Vec2f};

use crate::{ui::LayoutCtx, NoResponse, Response, Ui, Widget, WidgetExt};

#[derive(Debug, Default)]
pub struct MarginWidget {
    props: Margin,
}

impl Widget for MarginWidget {
    type Response = NoResponse;
    type Props<'a> = Margin;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, ctx: LayoutCtx, input: Constraints) -> Vec2f {
        let margin = self.props.sum().into();
        let offset = self.props.left_top().to_pos2().into();
        let constraints = Constraints {
            min: (input.min - margin).max(Vec2f::ZERO),
            max: (input.max - margin).max(Vec2f::ZERO),
        };
        let mut size = Vec2f::ZERO;
        for &child in ctx.nodes.children() {
            size = ctx.layout.compute(child, constraints) + margin;
            ctx.layout.set_pos(child, offset)
        }
        constraints.constrain_min(size.max(margin))
    }
}

pub fn margin<R>(margin: Margin, show: impl FnOnce(&Ui) -> R) -> impl FnOnce(&Ui) -> Response {
    move |ui| MarginWidget::show_children(ui, margin, show)
}
