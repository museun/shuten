use shuten::geom::FlexFit;

use crate::{NoResponse, Widget};

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
