use crate::{
    geom::{pos2, vec2f, Align, Vec2, Vec2f},
    style::{Attribute, Color},
    Canvas, Cell,
};

use super::Label;

// TODO support multi-line strings (Align2 and count '\n')
// TODO maybe use bwrap or text-wrap here (justification)
#[derive(Copy, Clone, Debug, Default)]
pub struct Styled<T> {
    pub item: T,
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

    pub fn size(&self) -> Vec2f {
        // TODO multi-line labels
        vec2f(self.item.width() as f32, 1.0)
    }

    pub fn is_empty(&self) -> bool {
        self.item.width() == 0
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

    // TODO vertical-align
    pub fn render(&self, canvas: &mut Canvas) -> Vec2 {
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
        start.to_vec2()
    }
}
