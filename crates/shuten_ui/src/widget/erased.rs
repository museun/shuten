use shuten::geom::{Constraints, FlexFit, Flow, Vec2f};

use super::{
    super::input::{Handled, KeyPressed, MouseClick, MouseDrag, MouseHeld, MouseMove, MouseScroll},
    Interest, LayoutCtx, PaintCtx, Widget,
};

pub(crate) trait ErasedWidget: std::any::Any + std::fmt::Debug {
    fn flex(&self) -> (u16, FlexFit);
    fn flow(&self) -> Flow;

    fn layout(&self, ctx: LayoutCtx, input: Constraints) -> Vec2f;
    fn paint(&self, ctx: PaintCtx);

    fn interest(&self) -> Interest;
    fn on_key_pressed(&mut self, event: KeyPressed) -> Handled;
    fn on_mouse_enter(&mut self, event: MouseMove) -> Handled;
    fn on_mouse_leave(&mut self, event: MouseMove);
    fn on_mouse_move(&mut self, event: MouseMove) -> Handled;
    fn on_mouse_click(&mut self, event: MouseClick) -> Handled;
    fn on_mouse_held(&mut self, event: MouseHeld) -> Handled;
    fn on_mouse_drag(&mut self, event: MouseDrag) -> Handled;
    fn on_mouse_scroll(&mut self, event: MouseScroll) -> Handled;

    fn default_layout(&self, ctx: LayoutCtx, input: Constraints) -> Vec2f;
    fn default_paint(&self, ctx: PaintCtx);

    fn type_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T: Widget> ErasedWidget for T {
    fn flex(&self) -> (u16, FlexFit) {
        <Self as Widget>::flex(self)
    }

    fn flow(&self) -> Flow {
        <Self as Widget>::flow(self)
    }

    fn layout(&self, ctx: LayoutCtx, input: Constraints) -> Vec2f {
        <Self as Widget>::layout(self, ctx, input)
    }

    fn paint(&self, ctx: PaintCtx) {
        <Self as Widget>::paint(self, ctx)
    }

    fn interest(&self) -> Interest {
        <Self as Widget>::interest(self)
    }

    fn on_key_pressed(&mut self, event: KeyPressed) -> Handled {
        <Self as Widget>::on_key_pressed(self, event)
    }

    fn on_mouse_enter(&mut self, event: MouseMove) -> Handled {
        <Self as Widget>::on_mouse_enter(self, event)
    }

    fn on_mouse_leave(&mut self, event: MouseMove) {
        <Self as Widget>::on_mouse_leave(self, event)
    }

    fn on_mouse_move(&mut self, event: MouseMove) -> Handled {
        <Self as Widget>::on_mouse_move(self, event)
    }

    fn on_mouse_click(&mut self, event: MouseClick) -> Handled {
        <Self as Widget>::on_mouse_click(self, event)
    }

    fn on_mouse_held(&mut self, event: MouseHeld) -> Handled {
        <Self as Widget>::on_mouse_held(self, event)
    }

    fn on_mouse_drag(&mut self, event: MouseDrag) -> Handled {
        <Self as Widget>::on_mouse_drag(self, event)
    }

    fn on_mouse_scroll(&mut self, event: MouseScroll) -> Handled {
        <Self as Widget>::on_mouse_scroll(self, event)
    }

    fn default_layout(&self, ctx: LayoutCtx, input: Constraints) -> Vec2f {
        <Self as Widget>::default_layout(self, ctx, input)
    }

    fn default_paint(&self, ctx: PaintCtx) {
        <Self as Widget>::default_paint(self, ctx)
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self as _
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as _
    }
}
