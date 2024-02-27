use std::cell::Cell;

use shuten::{
    geom::{almost_eq, pos2f, remap, vec2f, Rectf},
    style::Rgb,
};

use crate::widget::prelude::*;

use super::{color_box, draggable};

#[derive(Copy, Clone, Debug)]
pub struct Slider {
    value: f32,
    min: f32,
    max: f32,
    step: Option<f32>,

    filled: Rgb,
    knob: Option<Rgb>,
    track: Option<Rgb>,
}

impl Slider {
    pub const fn new(value: f32, min: f32, max: f32) -> Self {
        Self {
            value,
            min,
            max,
            step: None,

            filled: Rgb::from_u32(0x222222),
            knob: None,
            track: None,
        }
    }

    pub fn filled(mut self, filled: impl Into<Rgb>) -> Self {
        self.filled = filled.into();
        self
    }

    pub fn knob(mut self, knob: impl Into<Rgb>) -> Self {
        self.knob = Some(knob.into());
        self
    }

    pub fn track(mut self, track: impl Into<Rgb>) -> Self {
        self.track = Some(track.into());
        self
    }

    pub const fn empty() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    pub fn show(self) -> Response<SliderResponse> {
        SliderWidget::show(self)
    }
}

fn round_to_step(value: f32, step: f32) -> f32 {
    if step == 0.0 {
        return value;
    }
    (value / step).round() * step
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SliderResponse {
    pub value: Option<f32>,
}

#[derive(Debug)]
struct SliderWidget {
    props: Slider,
    rect: Cell<Option<Rectf>>,
}

impl SliderWidget {
    const TRACK_SIZE: Vec2f = vec2f(0.0, Self::TRACK_HEIGHT);
    const KNOB_SIZE: Vec2f = vec2f(Self::TRACK_HEIGHT, Self::TRACK_HEIGHT);
    const TRACK_WIDTH: f32 = 30.0;
    const TRACK_HEIGHT: f32 = 1.0;
}

impl Default for SliderWidget {
    fn default() -> Self {
        Self {
            props: Slider::empty(),
            rect: Cell::new(None),
        }
    }
}

impl Widget for SliderWidget {
    type Props<'a> = Slider;
    type Response = SliderResponse;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;

        let knob = props.knob.unwrap_or_else(|| props.filled.lighten(0.3));
        let track = props.track.unwrap_or_else(|| props.filled.darken(0.2));

        color_box(props.filled, Self::TRACK_SIZE);
        color_box(track, Self::TRACK_SIZE);
        let resp = draggable(|| {
            color_box(knob, Self::KNOB_SIZE);
        });

        let mut value = self.props.value;

        // TODO vertical sliders
        if let (Some(drag), Some(rect)) = (resp.dragging, self.rect.get()) {
            let min = rect.left();
            let max = rect.right();
            let pos = drag.current.x.clamp(min, max);
            let p = (pos - min) / (max - min);
            value = p.mul_add(self.props.max - self.props.min, self.props.min);
        }

        if let Some(step) = self.props.step {
            value = round_to_step(value, step)
        }

        let value = (!almost_eq(value, self.props.value)).then_some(value);
        SliderResponse { value }
    }

    fn layout(&self, mut ctx: LayoutCtx<'_>, input: Constraints) -> Vec2f {
        let node = ctx.tree.get_current();
        let size = vec2f(
            input
                .constrain_width(Self::TRACK_WIDTH)
                .max(Self::KNOB_SIZE.x),
            input.min.y.max(Self::TRACK_HEIGHT),
        );

        let &[track, active, knob] = &*node.children else {
            return size;
        };

        let constraints = Constraints::tight(vec2f(size.x, Self::TRACK_HEIGHT));
        ctx.calculate(track, constraints);
        ctx.layout.set_pos(track, pos2f(0.0, 0.0));

        let pos =
            (self.props.value - self.props.min) / (self.props.max - self.props.min).clamp(0.0, 1.0);
        ctx.calculate(knob, Constraints::none());

        let p = remap(pos, self.props.min..=self.props.max, 0.0..=size.x).clamp(0.0, size.x);
        let pos = pos2f(p, 0.0);
        ctx.layout.set_pos(knob, pos);

        ctx.calculate(active, Constraints::tight(vec2f(size.x - pos.x, 1.0)));
        ctx.layout.set_pos(active, pos);
        size
    }

    fn paint(&self, ctx: PaintCtx<'_, '_>) {
        self.rect.set(Some(ctx.rect));
        self.default_paint(ctx)
    }
}

// BUG: the smoothing in this is garbage for small ranges
pub fn slider(value: f32, min: f32, max: f32) -> Response<SliderResponse> {
    Slider::new(value, min, max).show()
}
