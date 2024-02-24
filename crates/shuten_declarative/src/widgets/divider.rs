use shuten_core::{geom::Rect, Canvas, Rgb};

use crate::{
    context::LayoutCtx,
    geom::{vec2f, Constraints, Rectf, Vec2f},
    painter::{self, PaintCtx},
    widget::Response,
    Widget, WidgetExt,
};

use super::NoResponse;

pub struct Divider {
    color: Rgb,
    thickness: f32,
    height: f32,
    left_pad: f32,
    right_pad: f32,
}

impl Divider {
    pub fn new(color: impl Into<Rgb>, height: f32, thickness: f32) -> Self {
        Self {
            color: color.into(),
            thickness,
            height,
            left_pad: 0.0,
            right_pad: 0.0,
        }
    }

    pub fn show(self) -> Response {
        DividerWidget::show(self)
    }
}

struct DividerWidget {
    props: Divider,
}

impl Widget for DividerWidget {
    type Props<'a> = Divider;
    type Response = NoResponse;

    fn new() -> Self {
        Self {
            props: Divider::new(0xFF0000, 1.0, f32::INFINITY),
        }
    }

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, _: LayoutCtx<'_>, input: Constraints) -> Vec2f {
        vec2f(input.min.x, input.constrain_height(self.props.height))
    }

    fn paint(&self, ctx: PaintCtx<'_, '_>) {
        let Some(parent) = ctx.tree.get_current().parent else {
            return;
        };

        let width = ctx.layout.get(parent).unwrap().rect.width();
        let rect = ctx.current_rect();

        let pos = rect.min
            + vec2f(
                self.props.left_pad,
                (rect.height() - self.props.thickness) / 2.0,
            );
        let size = vec2f(
            width - self.props.left_pad - self.props.right_pad,
            self.props.thickness,
        );
        let rect = Rect::from(Rectf::from_min_size(pos, size));
        ctx.canvas.fill_rect(rect, self.props.color);
    }
}

pub fn divider(color: impl Into<Rgb>, height: f32, thickness: f32) -> Response {
    Divider::new(color, height, thickness).show()
}
