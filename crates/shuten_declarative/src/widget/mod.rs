use crate::{
    geom::{Constraints, FlexFit, Flow, Vec2f},
    input::{Event, EventCtx, Handled, Interest},
    layout::LayoutCtx,
    paint::PaintCtx,
};

mod root_widget;
pub use root_widget::RootWidget;

mod dummy_widget;
pub use dummy_widget::DummyWidget;

mod erased_widget;
pub use erased_widget::ErasedWidget;

mod response;
pub use response::{NoResponse, Response};

mod widget_ext;
pub use widget_ext::WidgetExt;

pub trait Props: std::fmt::Debug {}

impl<T: std::fmt::Debug> Props for T {}

pub trait Widget: 'static + Default + std::fmt::Debug {
    type Props<'a>: Props;
    type Response;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response;

    fn flex(&self) -> (u16, FlexFit) {
        (0, FlexFit::Loose)
    }

    fn flow(&self) -> Flow {
        Flow::Inline
    }

    fn interest(&self) -> Interest {
        Interest::NONE
    }

    #[allow(unused)]
    fn event(&mut self, ctx: EventCtx<'_>, event: &Event) -> Handled {
        Handled::default()
    }

    fn layout(&self, ctx: LayoutCtx<'_>, input: Constraints) -> Vec2f {
        self.default_layout(ctx, input)
    }

    fn default_layout(&self, mut ctx: LayoutCtx<'_>, input: Constraints) -> Vec2f {
        let node = ctx.tree.get_current();
        let mut size = Vec2f::ZERO;
        for &child in &node.children {
            size = size.max(ctx.calculate(child, input));
        }
        input.constrain_min(size)
    }

    fn paint(&self, ctx: PaintCtx<'_, '_>) {
        self.default_paint(ctx);
    }

    fn default_paint(&self, mut ctx: PaintCtx<'_, '_>) {
        let node = ctx.tree.get_current();
        for &child in &node.children {
            ctx.paint(child);
        }
    }
}
