use std::any::Any;

use crate::{
    geom::{Constraints, FlexFit, Flow, Vec2f},
    input::{Event, EventCtx, Handled, Interest},
    layout::LayoutCtx,
    paint::PaintCtx,
    Widget,
};

pub trait ErasedWidget: std::any::Any + std::fmt::Debug {
    fn layout(&self, ctx: LayoutCtx<'_>, input: Constraints) -> Vec2f;

    fn flex(&self) -> (u16, FlexFit);
    fn flow(&self) -> Flow;

    fn paint(&self, ctx: PaintCtx<'_, '_>);

    fn interest(&self) -> Interest;
    fn event(&mut self, ctx: EventCtx<'_>, event: &Event) -> Handled;

    fn type_name(&self) -> &'static str;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Widget> ErasedWidget for T {
    fn layout(&self, ctx: LayoutCtx<'_>, input: Constraints) -> Vec2f {
        <T as Widget>::layout(self, ctx, input)
    }

    fn flex(&self) -> (u16, FlexFit) {
        <T as Widget>::flex(self)
    }

    fn flow(&self) -> Flow {
        <T as Widget>::flow(self)
    }

    fn paint(&self, ctx: PaintCtx<'_, '_>) {
        <T as Widget>::paint(self, ctx)
    }

    fn interest(&self) -> Interest {
        <T as Widget>::interest(self)
    }

    fn event(&mut self, ctx: EventCtx<'_>, event: &Event) -> Handled {
        <T as Widget>::event(self, ctx, event)
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }

    fn as_any(&self) -> &dyn Any {
        self as _
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as _
    }
}
