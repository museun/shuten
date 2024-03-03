use shuten::geom::FlexFit;

use crate::{NoResponse, Response, Ui, Widget, WidgetExt};

#[derive(Debug, Default)]
pub struct Flex {
    flex: u16,
    fit: FlexFit,
}

impl Flex {
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
}

#[derive(Debug, Default)]
pub struct FlexWidget {
    props: Flex,
}

impl Widget for FlexWidget {
    type Response = NoResponse;
    type Props<'a> = Flex;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn flex(&self) -> (u16, FlexFit) {
        (self.props.flex, self.props.fit)
    }
}

pub fn flex<R>(ui: &Ui, factor: u16, show: impl FnOnce(&Ui) -> R) -> Response {
    FlexWidget::show_children(ui, Flex::new(factor), show)
}

pub fn expanded<R>(ui: &Ui, show: impl FnOnce(&Ui) -> R) -> Response {
    FlexWidget::show_children(ui, Flex::expanded(), show)
}

pub fn spacer(ui: &Ui) -> Response {
    FlexWidget::show(ui, Flex::expanded())
}
