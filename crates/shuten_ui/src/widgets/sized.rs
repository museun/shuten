use shuten::geom::{vec2f, Constraints, Vec2f};

use crate::{ui::LayoutCtx, NoResponse, Response, Ui, Widget, WidgetExt};

#[derive(Debug)]
pub struct Sized {
    min: Vec2f,
    max: Vec2f,
}

impl Sized {
    pub const fn new(min: Vec2f, max: Vec2f) -> Self {
        Self { min, max }
    }

    pub const fn max(max: Vec2f) -> Self {
        Self::new(Vec2f::ZERO, max)
    }

    pub const fn min(min: Vec2f) -> Self {
        Self::new(min, Vec2f::INFINITY)
    }

    pub const fn min_height(min_height: f32) -> Self {
        Self::min(vec2f(f32::INFINITY, min_height))
    }

    pub const fn min_width(min_width: f32) -> Self {
        Self::min(vec2f(min_width, f32::INFINITY))
    }

    pub const fn max_height(max_height: f32) -> Self {
        Self::max(vec2f(f32::INFINITY, max_height))
    }

    pub const fn max_width(max_width: f32) -> Self {
        Self::max(vec2f(max_width, f32::INFINITY))
    }
}

#[derive(Debug)]
pub struct SizedWidget {
    props: Sized,
}

impl Default for SizedWidget {
    fn default() -> Self {
        Self {
            props: Sized::new(Vec2f::ZERO, Vec2f::INFINITY),
        }
    }
}

impl Widget for SizedWidget {
    type Response = NoResponse;
    type Props<'a> = Sized;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, ctx: LayoutCtx, mut input: Constraints) -> Vec2f {
        input.min = input.min.max(self.props.min);
        input.max = input.max.min(self.props.max);

        let mut size = Vec2f::ZERO;
        for &child in ctx.nodes.children() {
            size = size.max(ctx.layout.compute(child, input))
        }
        size
    }
}

pub fn sized<R>(ui: &Ui, size: Sized, show: impl FnOnce(&Ui) -> R) -> Response {
    SizedWidget::show_children(ui, size, show)
}

pub fn min_size<R>(ui: &Ui, min_size: Vec2f, show: impl FnOnce(&Ui) -> R) -> Response {
    sized(ui, Sized::min(min_size), show)
}

pub fn max_size<R>(ui: &Ui, max_size: Vec2f, show: impl FnOnce(&Ui) -> R) -> Response {
    sized(ui, Sized::max(max_size), show)
}

pub fn min_height<R>(ui: &Ui, min_height: f32, show: impl FnOnce(&Ui) -> R) -> Response {
    sized(ui, Sized::min_height(min_height), show)
}

pub fn min_width<R>(ui: &Ui, min_width: f32, show: impl FnOnce(&Ui) -> R) -> Response {
    sized(ui, Sized::min_width(min_width), show)
}

pub fn max_height<R>(ui: &Ui, max_height: f32, show: impl FnOnce(&Ui) -> R) -> Response {
    sized(ui, Sized::max_height(max_height), show)
}

pub fn max_width<R>(ui: &Ui, max_width: f32, show: impl FnOnce(&Ui) -> R) -> Response {
    sized(ui, Sized::max_width(max_width), show)
}
