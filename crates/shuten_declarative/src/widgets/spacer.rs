use crate::widget::prelude::*;

#[derive(Debug)]
pub struct Spacer {
    flex_factor: u16,
}

impl Spacer {
    pub fn new(flex_factor: u16) -> Self {
        Self {
            flex_factor: flex_factor.max(1),
        }
    }

    pub fn show(self) -> Response {
        SpacerWidget::show(self)
    }
}

#[derive(Debug)]
struct SpacerWidget {
    props: Spacer,
}

impl Default for SpacerWidget {
    fn default() -> Self {
        Self {
            props: Spacer::new(1),
        }
    }
}

impl Widget for SpacerWidget {
    type Props<'a> = Spacer;
    type Response = NoResponse;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn flex(&self) -> (u16, FlexFit) {
        (self.props.flex_factor, FlexFit::Tight)
    }

    fn layout(&self, _: LayoutCtx<'_>, input: Constraints) -> Vec2f {
        input.min
    }

    fn paint(&self, _: PaintCtx<'_, '_>) {}
}

pub fn flex_spacer(flex: u16) -> Response {
    Spacer::new(flex).show()
}
