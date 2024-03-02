use shuten::geom::{Constraints, FlexFit, Flow, Vec2f};

use crate::{
    input::{Handled, KeyPressed, MouseClick, MouseDrag, MouseHeld, MouseMove, MouseScroll},
    ui::{LayoutCtx, PaintCtx},
    Interest, Ui,
};

mod erased;
pub(crate) use erased::ErasedWidget;

mod widget_ext;
pub use widget_ext::WidgetExt;

pub type NoResponse = ();

pub trait Props: std::fmt::Debug {}
impl<T: std::fmt::Debug> Props for T {}

pub trait Widget: Default + std::fmt::Debug + 'static {
    type Response;
    type Props<'a>: Props;

    fn update(&mut self, ui: &Ui, props: Self::Props<'_>) -> Self::Response;

    fn flex(&self) -> (u16, FlexFit) {
        (0, FlexFit::Loose)
    }

    fn flow(&self) -> Flow {
        Flow::Inline
    }

    fn layout(&self, ctx: LayoutCtx, input: Constraints) -> Vec2f {
        self.default_layout(ctx, input)
    }

    fn paint(&self, ctx: PaintCtx) {
        self.default_paint(ctx);
    }

    fn interest(&self) -> Interest {
        Interest::NONE
    }

    fn on_key_pressed(&mut self, event: KeyPressed) -> Handled {
        Handled::Bubble
    }

    fn on_mouse_enter(&mut self, event: MouseMove) -> Handled {
        Handled::Bubble
    }

    fn on_mouse_leave(&mut self, event: MouseMove) {}

    fn on_mouse_move(&mut self, event: MouseMove) -> Handled {
        Handled::Bubble
    }

    fn on_mouse_held(&mut self, event: MouseHeld) -> Handled {
        Handled::Bubble
    }

    fn on_mouse_click(&mut self, event: MouseClick) -> Handled {
        Handled::Bubble
    }

    fn on_mouse_drag(&mut self, event: MouseDrag) -> Handled {
        Handled::Bubble
    }

    fn on_mouse_scroll(&mut self, event: MouseScroll) -> Handled {
        Handled::Bubble
    }

    fn default_layout(&self, ctx: LayoutCtx, input: Constraints) -> Vec2f {
        let mut size = Vec2f::ZERO;
        for &child in ctx.nodes.children() {
            size = size.max(ctx.layout.compute(child, input))
        }
        input.constrain_min(size)
    }

    fn default_paint(&self, mut ctx: PaintCtx) {
        for &child in ctx.ui.get_current().children() {
            ctx.paint(child)
        }
    }
}
