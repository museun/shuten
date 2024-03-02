use shuten::{
    geom::{pos2, Constraints, Margin, Vec2f},
    style::Color,
    Cell,
};

use crate::{
    ui::{LayoutCtx, PaintCtx},
    widgets::MarginWidget,
    NoResponse, Ui, Widget,
};

#[derive(Debug)]
pub struct BorderStyle {
    pub left_top: char,
    pub right_top: char,
    pub right_bottom: char,
    pub left_bottom: char,
    pub top: char,
    pub right: char,
    pub bottom: char,
    pub left: char,

    pub fg: Color,
    pub bg: Color,
}

impl Default for BorderStyle {
    fn default() -> Self {
        Self::THIN
    }
}

impl BorderStyle {
    pub fn fg(mut self, fg: impl Into<Color>) -> Self {
        self.fg = fg.into();
        self
    }

    pub fn bg(mut self, bg: impl Into<Color>) -> Self {
        self.bg = bg.into();
        self
    }
}

impl BorderStyle {
    pub const EMPTY: Self = Self {
        left_top: ' ',
        top: ' ',
        right_top: ' ',
        right: ' ',
        right_bottom: ' ',
        bottom: ' ',
        left_bottom: ' ',
        left: ' ',

        fg: Color::Reset,
        bg: Color::Reuse,
    };

    pub const THIN: Self = Self {
        left_top: '┌',
        top: '─',
        right_top: '┐',
        right: '│',
        right_bottom: '┘',
        bottom: '─',
        left_bottom: '└',
        left: '│',

        fg: Color::Reset,
        bg: Color::Reuse,
    };

    pub const THIN_TALL: Self = Self {
        left_top: '▔',
        top: '▔',
        right_top: '▔',
        right: '▕',
        right_bottom: '▁',
        bottom: '▁',
        left_bottom: '▁',
        left: '▏',

        fg: Color::Reset,
        bg: Color::Reuse,
    };

    pub const THIN_WIDE: Self = Self {
        left_top: '▁',
        top: '▁',
        right_top: '▁',
        right: '▕',
        right_bottom: '▔',
        bottom: '▔',
        left_bottom: '▔',
        left: '▏',

        fg: Color::Reset,
        bg: Color::Reuse,
    };

    pub const ROUNDED: Self = Self {
        left_top: '╭',
        top: '─',
        right_top: '╮',
        right: '│',
        right_bottom: '╯',
        bottom: '─',
        left_bottom: '╰',
        left: '│',

        fg: Color::Reset,
        bg: Color::Reuse,
    };

    pub const DOUBLE: Self = Self {
        left_top: '╔',
        top: '═',
        right_top: '╗',
        right: '║',
        right_bottom: '╝',
        bottom: '═',
        left_bottom: '╚',
        left: '║',

        fg: Color::Reset,
        bg: Color::Reuse,
    };

    pub const THICK: Self = Self {
        left_top: '┏',
        top: '━',
        right_top: '┓',
        right: '┃',
        right_bottom: '┛',
        bottom: '━',
        left_bottom: '┗',
        left: '┃',

        fg: Color::Reset,
        bg: Color::Reuse,
    };

    pub const THICK_TALL: Self = Self {
        left_top: '▛',
        top: '▀',
        right_top: '▜',
        right: '▐',
        right_bottom: '▟',
        bottom: '▄',
        left_bottom: '▙',
        left: '▌',

        fg: Color::Reset,
        bg: Color::Reuse,
    };

    pub const THICK_WIDE: Self = Self {
        left_top: '▗',
        top: '▄',
        right_top: '▖',
        right: '▌',
        right_bottom: '▘',
        bottom: '▀',
        left_bottom: '▝',
        left: '▐',

        fg: Color::Reset,
        bg: Color::Reuse,
    };
}

#[derive(Debug, Default)]
pub struct BorderWidget {
    props: BorderStyle,
}

impl Widget for BorderWidget {
    type Response = NoResponse;
    type Props<'a> = BorderStyle;

    fn update(&mut self, _: &Ui, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, ctx: LayoutCtx, input: Constraints) -> Vec2f {
        // let mut margin = MarginWidget::default();
        // margin.update(Margin::same(1));
        // margin.layout(ctx, input)

        todo!()
    }

    fn paint(&self, ctx: PaintCtx) {
        let rect = ctx.canvas.area();

        let (left_top, right_top, right_bottom, left_bottom) = (
            rect.left_top(),
            rect.right_top(),
            rect.right_bottom(),
            rect.left_bottom(),
        );

        // TODO this is ultra ugly
        for (range, y, ch) in [
            (left_top.x..=right_top.x, rect.top(), self.props.top),
            (
                left_bottom.x..=right_bottom.x,
                rect.bottom(),
                self.props.bottom,
            ),
        ] {
            for x in range {
                ctx.canvas.put(
                    pos2(x, y),
                    Cell::new(ch).fg(self.props.fg).bg(self.props.bg),
                );
            }
        }

        for (range, x, ch) in [
            (left_top.y..=left_bottom.y, rect.left(), self.props.left),
            (right_top.y..=right_bottom.y, rect.right(), self.props.right),
        ] {
            for y in range {
                ctx.canvas.put(
                    pos2(x, y),
                    Cell::new(ch).fg(self.props.fg).bg(self.props.bg),
                );
            }
        }

        for (pos, cell) in [
            (left_top, self.props.left_top),
            (right_top, self.props.right_top),
            (right_bottom, self.props.right_bottom),
            (left_bottom, self.props.left_bottom),
        ] {
            ctx.canvas.put(
                pos,
                Cell::new(cell) //
                    .fg(self.props.fg)
                    .bg(self.props.bg),
            );
        }

        self.default_paint(ctx)
    }
}
