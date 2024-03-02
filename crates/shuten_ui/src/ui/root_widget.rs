use shuten::geom::{Constraints, Vec2f};

use crate::{NoResponse, Ui, Widget};

use super::LayoutCtx;

#[derive(Default, Debug)]
pub(crate) struct Root;

impl Widget for Root {
    type Response = NoResponse;
    type Props<'a> = ();

    fn update(&mut self, _: &Ui, _: Self::Props<'_>) -> Self::Response {}

    fn layout(&self, ctx: LayoutCtx, input: Constraints) -> Vec2f {
        ctx.layout.new_layer(ctx.nodes.current_id());
        let children = ctx.nodes.children();
        for &child in children {
            ctx.layout.compute(child, input);
        }
        input.max
    }
}
