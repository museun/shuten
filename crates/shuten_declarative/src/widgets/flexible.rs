use crate::{
    geom::FlexFit,
    widget::{Response, Widget},
    WidgetExt,
};

use super::NoResponse;

#[derive(Debug)]
pub struct Flexible {
    flex: u16,
    fit: FlexFit,
}

impl Flexible {
    pub const fn new(flex: u16) -> Self {
        Self {
            flex,
            fit: FlexFit::Loose,
        }
    }

    pub const fn expanded() -> Self {
        Self {
            flex: 1,
            fit: FlexFit::Tight,
        }
    }

    pub fn show(self, children: impl FnOnce()) -> Response {
        FlexibleWidget::show_children(children, self)
    }
}

#[derive(Debug)]
struct FlexibleWidget {
    props: Flexible,
}

impl Default for FlexibleWidget {
    fn default() -> Self {
        Self {
            props: Flexible::new(0),
        }
    }
}

impl Widget for FlexibleWidget {
    type Props<'a> = Flexible;
    type Response = NoResponse;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn flex(&self) -> (u16, FlexFit) {
        (self.props.flex, self.props.fit)
    }
}

pub fn flexible(flex: u16, children: impl FnOnce()) -> Response {
    Flexible::new(flex).show(children)
}

pub fn expanded(children: impl FnOnce()) -> Response {
    Flexible::expanded().show(children)
}
