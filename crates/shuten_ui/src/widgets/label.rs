use std::borrow::Cow;

use shuten::{
    geom::{pos2, vec2f, Align, Constraints, Vec2f},
    style::{Attribute, Color},
    Canvas, Cell,
};

use crate::{
    ext::DigitExt,
    ui::{LayoutCtx, PaintCtx},
    NoResponse, Ui, Widget,
};

// TODO support multi-line strings (Align2 and count '\n')
// TODO maybe use bwrap or text-wrap here (justification)
#[derive(Debug, Default)]
pub struct Styled<T> {
    item: T,
    fg: Color,
    bg: Color,
    attr: Option<Attribute>,
    align: Align,
}

impl<T: Label> From<T> for Styled<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: Label> Styled<T> {
    pub const fn new(item: T) -> Self {
        Self {
            item,
            fg: Color::Reset,
            bg: Color::Reuse,
            attr: None,
            align: Align::Min,
        }
    }

    pub fn into_static(self) -> Styled<T::Static> {
        Styled {
            item: self.item.into_static(),
            fg: self.fg,
            bg: self.bg,
            attr: self.attr,
            align: self.align,
        }
    }

    pub fn fg(mut self, fg: impl Into<Color>) -> Self {
        self.fg = fg.into();
        self
    }

    pub fn bg(mut self, bg: impl Into<Color>) -> Self {
        self.bg = bg.into();
        self
    }

    pub fn attr(mut self, attr: impl Into<Option<Attribute>>) -> Self {
        self.attr = attr.into();
        self
    }

    pub const fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    fn render(&self, canvas: &mut Canvas) {
        let area = canvas.area();

        let offset = match self.align {
            Align::Min => 0,
            Align::Center => (area.width() / 2).saturating_sub(self.item.width() / 2),
            Align::Max => area.width().saturating_sub(self.item.width()),
        };

        let mut start = area.left_top() + pos2(offset, 0);
        for ch in self.item.chars() {
            if start.x > area.right() {
                break;
            }
            canvas.put(start, Cell::new(ch).fg(self.fg).bg(self.bg).attr(self.attr));
            start.x += 1;
        }
    }
}

// this just has to be 'static, the type erasure already works
pub trait Label: std::fmt::Debug + Default {
    type Static: Label + 'static + Sized;
    fn into_static(self) -> Self::Static;

    fn width(&self) -> u16;
    // this isn't object safe..
    fn chars(&self) -> impl Iterator<Item = char>;
}

impl<'a> Label for Cow<'a, str> {
    type Static = Cow<'static, str>;

    fn into_static(self) -> Self::Static {
        match self {
            Cow::Borrowed(s) => Cow::Owned(s.to_owned()),
            Cow::Owned(s) => Cow::Owned(s),
        }
    }

    fn width(&self) -> u16 {
        self.as_ref().width()
    }

    fn chars(&self) -> impl Iterator<Item = char> {
        self.as_ref().chars()
    }
}

impl Label for String {
    type Static = String;

    fn into_static(self) -> Self::Static {
        self
    }

    fn width(&self) -> u16 {
        self.as_str().width()
    }

    fn chars(&self) -> impl Iterator<Item = char> {
        self.as_str().chars()
    }
}

impl Label for &str {
    type Static = String;
    fn into_static(self) -> Self::Static {
        self.to_string()
    }

    fn width(&self) -> u16 {
        self.len() as _
    }

    fn chars(&self) -> impl Iterator<Item = char> {
        (*self).chars()
    }
}

impl Label for usize {
    type Static = Self;
    fn into_static(self) -> Self::Static {
        self
    }

    fn width(&self) -> u16 {
        <Self as DigitExt>::width(self) as u16
    }

    fn chars(&self) -> impl Iterator<Item = char> {
        <Self as DigitExt>::chars(self)
    }
}

impl Label for () {
    type Static = Self;

    fn into_static(self) -> Self::Static {
        self
    }

    fn width(&self) -> u16 {
        0
    }

    fn chars(&self) -> impl Iterator<Item = char> {
        std::iter::empty()
    }
}

// if we make it generic then the replace-and-swap stuff might break
#[derive(Debug, Default)]
pub struct LabelWidget<T: Label + 'static> {
    data: Styled<T>,
}

impl<T: Label + 'static> Widget for LabelWidget<T> {
    type Response = NoResponse;
    type Props<'a> = Styled<T>;

    fn update(&mut self, _: &Ui, props: Self::Props<'_>) -> Self::Response {
        self.data = props;
    }

    fn layout(&self, _: LayoutCtx, input: Constraints) -> Vec2f {
        input.constrain_min(vec2f(self.data.item.width() as f32, 1.0))
    }

    fn paint(&self, ctx: PaintCtx) {
        self.data.render(ctx.canvas)
    }
}
