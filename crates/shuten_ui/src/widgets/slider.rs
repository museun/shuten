use std::ops::RangeInclusive;

use shuten::{
    geom::{remap, vec2f, Constraints, Rectf, Vec2f},
    style::{Color, Rgb},
    Cell,
};

use crate::{
    input::{Handled, MouseClick, MouseDrag, MouseMove},
    ui::{LayoutCtx, PaintCtx},
    Interest, Widget,
};

use super::{Dragging, Orientation};

#[derive(Copy, Clone, Debug)]
pub struct SliderStyle {
    pub track: char,
    pub knob: char,
    pub track_bg: Color,
    pub filled_bg: Color,
    pub knob_fg: Color,
}

impl Default for SliderStyle {
    fn default() -> Self {
        Self::FILLED
    }
}

impl SliderStyle {
    pub const FILLED: Self = Self {
        track: ' ',
        knob: ' ',
        track_bg: Color::Rgb(Rgb::from_u32(0x222222)),
        filled_bg: Color::Rgb(Rgb::from_u32(0x222222)),
        knob_fg: Color::Rgb(Rgb::from_u32(0x444444)),
    };

    pub const SLIM: Self = Self {
        track: '─',
        knob: '●', //'◆',
        ..Self::FILLED
    };
}

#[derive(Default, Debug)]
pub struct Slider {
    min: f32,
    max: f32,
    pos: f32,

    // TOOD step by
    style: SliderStyle,
    orientation: Orientation,
    min_size: Vec2f,
}

impl Slider {
    const MAX_AXIS_SIZE: f32 = 15.0;

    pub const fn new(pos: f32, range: RangeInclusive<f32>) -> Self {
        Self {
            min: *range.start(),
            max: *range.end(),
            pos,

            style: SliderStyle::FILLED,
            orientation: Orientation::Horizontal,
            min_size: vec2f(Self::MAX_AXIS_SIZE, 1.0),
        }
    }

    pub const fn style(mut self, style: SliderStyle) -> Self {
        self.style = style;
        self
    }

    pub const fn horizontal(self) -> Self {
        self.orientation(Orientation::Horizontal)
    }

    pub const fn vertical(self) -> Self {
        self.orientation(Orientation::Vertical)
    }

    pub const fn orientation(mut self, orientation: Orientation) -> Self {
        self.min_size = match orientation {
            Orientation::Horizontal => vec2f(Self::MAX_AXIS_SIZE, 1.0),
            Orientation::Vertical => vec2f(1.0, Self::MAX_AXIS_SIZE),
        };
        self.orientation = orientation;
        self
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SliderResponse {
    pub value: f32,
}

#[derive(Debug, Default)]
pub struct SliderWidget {
    props: Slider,
    state: Option<Dragging>,
    rect: std::cell::Cell<Option<Rectf>>,
}

impl Widget for SliderWidget {
    type Response = SliderResponse;
    type Props<'a> = Slider;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        SliderResponse {
            value: std::mem::replace(&mut self.props, props).pos,
        }
    }

    fn interest(&self) -> Interest {
        Interest::MOUSE_DRAG | Interest::MOUSE_ENTER | Interest::MOUSE_LEAVE | Interest::MOUSE_CLICK
    }

    fn on_mouse_enter(&mut self, event: MouseMove) -> Handled {
        let _ = event;
        Handled::Sink
    }

    fn on_mouse_leave(&mut self, event: MouseMove) {
        let _ = event;
    }

    fn on_mouse_click(&mut self, event: MouseClick) -> Handled {
        let _ = event;
        Handled::Sink
    }

    fn on_mouse_drag(&mut self, event: MouseDrag) -> Handled {
        if event.button.is_primary() {
            if !event.released {
                self.state = Some(Dragging {
                    origin: event.origin,
                    current: event.pos,
                    delta: event.delta,
                });
                return Handled::Sink;
            }
            if self.state.take().is_some() {
                return Handled::Sink;
            }
        }

        Handled::Bubble
    }

    fn layout(&self, ctx: LayoutCtx, input: Constraints) -> Vec2f {
        let _ = ctx;
        input.constrain_min(self.props.min_size)
    }

    fn paint(&self, ctx: PaintCtx) {
        self.rect.set(Some(ctx.rect));

        let mut x = 0.0;
        if let Some(val) = self.state.map(|c| c.current) {
            x = remap(
                val.x,
                self.props.min..=self.props.max,
                ctx.rect.left()..=ctx.rect.right(),
            )
        }

        eprintln!("{x:?}");

        ctx.canvas.fill(self.props.style.track_bg);
        let mut canvas = ctx
            .canvas
            .crop(ctx.rect.with_size(vec2f(x, ctx.rect.height())).into());
        canvas.rect(canvas.area(), Cell::new(self.props.style.track));
    }
}

fn round_to_step(value: f32, step: f32) -> f32 {
    if step == 0.0 {
        return value;
    }
    (value / step).round() * step
}
