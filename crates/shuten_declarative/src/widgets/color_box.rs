use shuten::style::Color;

use crate::widget::prelude::*;

#[derive(Debug)]
pub struct ColorBox {
    color: Color,
    min_size: Vec2f,
}

impl ColorBox {
    pub fn new(color: impl Into<Color>, size: Vec2f) -> Self {
        Self {
            color: color.into(),
            min_size: size,
        }
    }

    pub fn show(self) -> Response {
        ColorBoxWidget::show(self)
    }

    pub fn show_children(self, children: impl FnOnce()) -> Response {
        ColorBoxWidget::show_children(children, self)
    }
}

#[derive(Debug)]
struct ColorBoxWidget {
    props: ColorBox,
}

impl Default for ColorBoxWidget {
    fn default() -> Self {
        Self {
            props: ColorBox::new(Color::Reset, Vec2f::ZERO),
        }
    }
}

impl Widget for ColorBoxWidget {
    type Props<'a> = ColorBox;
    type Response = NoResponse;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, mut ctx: LayoutCtx<'_>, input: Constraints) -> Vec2f {
        let node = ctx.tree.get_current();
        let mut size = self.props.min_size;
        for &child in &node.children {
            size = size.max(ctx.calculate(child, input))
        }
        input.constrain_min(size)
    }

    fn paint(&self, mut ctx: PaintCtx<'_, '_>) {
        ctx.cropped_canvas().fill(self.props.color);
        let node = ctx.tree.get_current();
        for &child in &node.children {
            ctx.paint(child)
        }
    }
}

pub fn color_box(color: impl Into<Color>, size: Vec2f) -> Response {
    ColorBox::new(color, size).show()
}

pub fn container(bg: impl Into<Color>, children: impl FnOnce()) -> Response {
    ColorBox::new(bg, Vec2f::ZERO).show_children(children)
}
