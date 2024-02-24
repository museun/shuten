use crate::{
    context::LayoutCtx,
    geom::{Constraints, Vec2f},
    NoResponse, Response, Widget, WidgetExt as _,
};

#[derive(Default, Debug)]
pub struct Float;

impl Float {
    pub fn show(self, children: impl FnOnce()) -> Response<NoResponse> {
        FloatWidget::show_children(children, self)
    }
}

#[derive(Default, Debug)]
pub struct FloatWidget {
    props: Float,
}

impl Widget for FloatWidget {
    type Props<'a> = Float;
    type Response = NoResponse;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, ctx: LayoutCtx<'_>, input: Constraints) -> Vec2f {
        ctx.layout.new_layer(ctx.tree);
        ctx.layout.clip(ctx.tree);
        self.default_layout(ctx, Constraints::tight(input.size()))
    }
}

pub fn float(children: impl FnOnce()) -> Response<NoResponse> {
    Float.show(children)
}
