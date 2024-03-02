use shuten::{
    geom::{Constraints, Vec2f},
    style::Rgb,
    Cell,
};

use crate::{
    ui::{LayoutCtx, PaintCtx},
    NoResponse, Widget,
};

#[derive(Default, Debug)]
pub struct Filled {
    cell: Cell,
    min_size: Vec2f,
}

impl Filled {
    pub fn bg(color: impl Into<Rgb>) -> Self {
        Self {
            cell: Cell::new(' ').bg(color.into()),
            min_size: Vec2f::ZERO,
        }
    }

    pub const fn min_size(mut self, min_size: Vec2f) -> Self {
        self.min_size = min_size;
        self
    }
}

#[derive(Default, Debug)]
pub struct FilledWidget {
    props: Filled,
}

impl Widget for FilledWidget {
    type Response = NoResponse;
    type Props<'a> = Filled;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, ctx: LayoutCtx, input: Constraints) -> Vec2f {
        let mut size = self.props.min_size;
        for &child in ctx.nodes.children() {
            size = size.max(ctx.layout.compute(child, input))
        }
        input.constrain_min(size)
    }

    fn paint(&self, ctx: PaintCtx) {
        ctx.canvas.rect(ctx.canvas.area(), self.props.cell);
        self.default_paint(ctx)
    }
}
