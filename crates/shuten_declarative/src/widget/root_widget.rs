use crate::widget::prelude::*;

#[derive(Debug, Default)]
pub struct RootWidget;

impl Widget for RootWidget {
    type Props<'a> = ();
    type Response = NoResponse;

    fn layout(&self, mut ctx: LayoutCtx<'_>, input: Constraints) -> Vec2f {
        ctx.layout.new_layer(ctx.tree);
        let node = ctx.tree.get_current();
        for &child in &node.children {
            ctx.calculate(child, input);
        }
        input.max
    }

    fn update(&mut self, _: Self::Props<'_>) -> Self::Response {}
}
