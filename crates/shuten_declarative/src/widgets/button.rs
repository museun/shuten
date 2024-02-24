use std::borrow::Cow;

use shuten::{
    geom::Margin,
    style::{Color, Rgb},
};

use crate::widget::prelude::*;

use super::{margin::margin, ColorBox, Label};

#[derive(Clone, Debug)]
pub struct Button {
    label: Cow<'static, str>,
    margin: Margin,
    fg: Color,
    bg: Color,
}

impl Button {
    pub fn new(label: impl Into<Cow<'static, str>>) -> Self {
        Self {
            label: label.into(),
            margin: Margin::symmetric(1, 0),
            bg: Color::Reset,
            fg: Color::Reset,
        }
    }

    pub const fn with_style(mut self, style: ButtonStyle) -> Self {
        self.margin = style.margin;
        self.fg = style.fg;
        self.bg = style.bg;
        self
    }

    pub fn empty() -> Self {
        Self::new("")
    }

    pub const fn margin(mut self, margin: Margin) -> Self {
        self.margin = margin;
        self
    }

    pub fn label(mut self, label: impl Into<Cow<'static, str>>) -> Self {
        self.label = label.into();
        self
    }

    pub fn bg(mut self, bg: impl Into<Color>) -> Self {
        self.bg = bg.into();
        self
    }

    pub fn fg(mut self, fg: impl Into<Color>) -> Self {
        self.fg = fg.into();
        self
    }

    pub fn show(self) -> Response<ButtonResponse> {
        ButtonWidget::show(self)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ButtonResponse {
    pub clicked: bool,
}

#[derive(Debug)]
struct ButtonWidget {
    props: Button,
    state: MouseState,
    clicked: bool,
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
enum MouseState {
    #[default]
    None,
    Hovering,
    MouseDown,
}

impl Default for ButtonWidget {
    fn default() -> Self {
        Self {
            props: Button::new(""),
            state: MouseState::default(),
            clicked: false,
        }
    }
}

impl Widget for ButtonWidget {
    type Props<'a> = Button;
    type Response = ButtonResponse;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;

        let mut color = self.props.bg;

        match self.state {
            MouseState::Hovering => color = color.lighten(0.2),
            MouseState::MouseDown => color = color.darken(0.1),
            _ => {}
        }

        ColorBox::new(color, Vec2f::ZERO).show_children(|| {
            margin(self.props.margin, || {
                Label::new(&*self.props.label).fg(self.props.fg).show();
            });
        });

        let clicked = self.clicked;
        self.clicked = false;
        ButtonResponse { clicked }
    }

    fn interest(&self) -> Interest {
        Interest::MOUSE_INSIDE
    }

    fn event(&mut self, _: EventCtx<'_>, event: &Event) -> Handled {
        std::mem::take(&mut self.state);

        self.state = match event {
            Event::MouseEnter => MouseState::Hovering,
            Event::MouseLeave => MouseState::None,
            Event::MouseHeld { .. } if event.is_mouse_primary() => MouseState::MouseDown,
            Event::MouseRelease { .. } if event.is_mouse_primary() => {
                self.clicked = true;
                self.state = MouseState::Hovering;
                self.state
            }
            _ => return Handled::Bubble,
        };

        Handled::Sink
    }
}

pub fn button(label: impl Into<Cow<'static, str>>) -> Response<ButtonResponse> {
    ButtonWidget::show(Button::new(label))
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ButtonStyle {
    pub margin: Margin,
    pub fg: Color,
    pub bg: Color,
}

impl ButtonStyle {
    pub const SUCCESS: Self = Self {
        margin: Margin::symmetric(1, 0),
        fg: Color::Rgb(Rgb::from_u32(0xFFFFFF)),
        bg: Color::Rgb(Rgb::from_u32(0x32CD32)),
    };

    pub const CANCEL: Self = Self {
        margin: Margin::symmetric(1, 0),
        fg: Color::Rgb(Rgb::from_u32(0xFFFFFF)),
        bg: Color::Rgb(Rgb::from_u32(0xB22222)),
    };

    pub const IMPORTANT: Self = Self {
        margin: Margin::symmetric(1, 0),
        fg: Color::Rgb(Rgb::from_u32(0xFFFFFF)),
        bg: Color::Rgb(Rgb::from_u32(0x4169E1)),
    };

    pub const NORMAL: Self = Self {
        margin: Margin::symmetric(2, 1),
        fg: Color::Reset,
        bg: Color::Rgb(Rgb::from_u32(0x333333)),
    };
}
