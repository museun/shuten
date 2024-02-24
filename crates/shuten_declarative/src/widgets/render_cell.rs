use shuten::{
    geom::{Constraints, Vec2f},
    Cell,
};

use crate::widget::prelude::*;

#[derive(Debug, Default)]
struct RenderCell(Cell);

#[derive(Debug, Default)]
struct RenderCellWidget {
    props: RenderCell,
}

impl Widget for RenderCellWidget {
    type Props<'a> = RenderCell;
    type Response = NoResponse;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, _: LayoutCtx<'_>, input: Constraints) -> Vec2f {
        input.constrain_min(Vec2f::splat(1.0))
    }

    fn paint(&self, mut ctx: PaintCtx<'_, '_>) {
        let mut canvas = ctx.cropped_canvas();
        for pos in canvas.area().indices() {
            canvas.put(pos, self.props.0)
        }
    }
}

pub fn render_cell(cell: Cell) -> Response {
    RenderCellWidget::show(RenderCell(cell))
}
