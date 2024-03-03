use shuten::geom::{vec2f, Align2, Constraints, Vec2f};

use crate::ui::LayoutCtx;
use crate::{NoResponse, Response, Ui, Widget, WidgetExt};

#[derive(Debug, Default)]
pub struct AlignWidget {
    align: Align2,
}

impl Widget for AlignWidget {
    type Response = NoResponse;
    type Props<'a> = Align2;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.align = props;
    }

    fn layout(&self, ctx: LayoutCtx, input: Constraints) -> Vec2f {
        let node = ctx.nodes.current();
        let constraints = Constraints::loose(input.max);

        let mut size = input.size();
        let align = vec2f(self.align.x.factor(), self.align.y.factor());

        for &child in node.children() {
            let next = ctx.layout.compute(child, constraints);
            size = size.max(next);
            let pos = (align * size - align * next).to_pos2();
            ctx.layout.set_pos(child, pos);
        }
        size
    }
}

pub fn align<R>(ui: &Ui, align: Align2, show: impl FnOnce(&Ui) -> R) -> Response {
    AlignWidget::show_children(ui, align, show)
}
