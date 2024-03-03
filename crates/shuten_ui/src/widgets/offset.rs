use shuten::geom::{Constraints, Pos2f, Vec2f};

use crate::{ui::LayoutCtx, NoResponse, Response, Ui, Widget, WidgetExt};

#[derive(Debug, Default)]
pub struct OffsetWidget {
    props: Pos2f,
}

impl Widget for OffsetWidget {
    type Response = NoResponse;
    type Props<'a> = Pos2f;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, ctx: LayoutCtx, input: Constraints) -> Vec2f {
        let constraints = Constraints::loose(input.max);
        let mut size = input.size();
        for &child in ctx.nodes.children() {
            size = size.max(ctx.layout.compute(child, constraints));
            ctx.layout.set_pos(child, self.props)
        }
        size
    }
}

pub fn offset<R>(ui: &Ui, pos: Pos2f, show: impl FnOnce(&Ui) -> R) -> Response {
    OffsetWidget::show_children(ui, pos, show)
}
