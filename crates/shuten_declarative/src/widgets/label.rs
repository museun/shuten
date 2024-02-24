use shuten::{
    geom::vec2f,
    style::{Attribute, Color},
    Cell,
};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr as _};

use crate::widget::prelude::*;

#[derive(Debug)]
pub struct Label {
    // TODO cache this? (intern it?)
    label: String,
    fg: Color,
    attr: Option<Attribute>,
}

impl Label {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            fg: Color::Reset,
            attr: None,
        }
    }

    pub fn fg(mut self, fg: impl Into<Color>) -> Self {
        self.fg = fg.into();
        self
    }

    pub const fn attribute(mut self, attr: Attribute) -> Self {
        self.attr = Some(attr);
        self
    }

    pub fn show(self) -> Response {
        LabelWidget::show(self)
    }
}

#[derive(Debug)]
struct LabelWidget {
    props: Label,
}

impl Default for LabelWidget {
    fn default() -> Self {
        Self {
            props: Label::new(""),
        }
    }
}

impl Widget for LabelWidget {
    type Props<'a> = Label;
    type Response = NoResponse;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, _ctx: LayoutCtx<'_>, input: Constraints) -> Vec2f {
        // TODO wrap
        input.constrain_min(vec2f(self.props.label.width() as f32, 1.0))
    }

    fn paint(&self, mut ctx: PaintCtx<'_, '_>) {
        let mut canvas = ctx.cropped_canvas();
        let mut start = canvas.area().left_top();

        for ch in self.props.label.chars() {
            if start.x > canvas.area().right() {
                break;
            }
            canvas.put(start, Cell::new(ch).fg(self.props.fg).attr(self.props.attr));
            start.x += ch.width().unwrap_or(1) as u16
        }
    }
}

pub fn label(text: impl Into<String>) -> Response {
    Label::new(text).show()
}
