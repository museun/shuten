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
    // TODO wrap
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

    pub const fn bold(mut self) -> Self {
        self.attr = match self.attr {
            Some(attr) => Some(Attribute(attr.0 | Attribute::BOLD.0)),
            None => Some(Attribute::BOLD),
        };
        self
    }

    pub const fn faint(mut self) -> Self {
        self.attr = match self.attr {
            Some(attr) => Some(Attribute(attr.0 | Attribute::FAINT.0)),
            None => Some(Attribute::FAINT),
        };
        self
    }

    pub const fn italic(mut self) -> Self {
        self.attr = match self.attr {
            Some(attr) => Some(Attribute(attr.0 | Attribute::ITALIC.0)),
            None => Some(Attribute::ITALIC),
        };
        self
    }

    pub const fn underline(mut self) -> Self {
        self.attr = match self.attr {
            Some(attr) => Some(Attribute(attr.0 | Attribute::UNDERLINE.0)),
            None => Some(Attribute::UNDERLINE),
        };
        self
    }

    pub const fn blink(mut self) -> Self {
        self.attr = match self.attr {
            Some(attr) => Some(Attribute(attr.0 | Attribute::BLINK.0)),
            None => Some(Attribute::BLINK),
        };
        self
    }

    pub const fn reverse(mut self) -> Self {
        self.attr = match self.attr {
            Some(attr) => Some(Attribute(attr.0 | Attribute::REVERSE.0)),
            None => Some(Attribute::REVERSE),
        };
        self
    }

    pub const fn strike_out(mut self) -> Self {
        self.attr = match self.attr {
            Some(attr) => Some(Attribute(attr.0 | Attribute::STRIKE_OUT.0)),
            None => Some(Attribute::STRIKE_OUT),
        };
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
        let lines = self.props.label.lines().count();
        input.constrain_min(vec2f(self.props.label.width() as f32, lines as f32))
    }

    fn paint(&self, mut ctx: PaintCtx<'_, '_>) {
        let mut canvas = ctx.cropped_canvas();

        let rect = canvas.area();
        let mut start = rect.left_top();

        for ch in self.props.label.chars() {
            if start.y > rect.bottom() {
                break;
            }

            if ch == '\n' {
                start.y += 1;
                start.x = rect.left();
                continue;
            }

            if start.x > rect.right() {
                start.y += 1;
                start.x = rect.left();
            }

            canvas.put(start, Cell::new(ch).fg(self.props.fg).attr(self.props.attr));
            start.x += ch.width().unwrap_or(1) as u16;
        }
    }
}

// TODO use a Span we can set properties (italics, bold, underline, fg, bg)
pub fn label(text: impl Into<String>) -> Response {
    Label::new(text).show()
}
