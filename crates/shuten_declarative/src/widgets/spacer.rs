use crate::widget::prelude::*;

#[derive(Debug)]
struct SpacerWidget {
    props: u16,
}

impl Default for SpacerWidget {
    fn default() -> Self {
        Self { props: 1 }
    }
}

impl Widget for SpacerWidget {
    type Props<'a> = u16;
    type Response = NoResponse;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn flex(&self) -> (u16, FlexFit) {
        (self.props, FlexFit::Tight)
    }

    fn layout(&self, _: LayoutCtx<'_>, input: Constraints) -> Vec2f {
        input.min
    }

    fn paint(&self, _: PaintCtx<'_, '_>) {}
}

pub fn spacer(flex: u16) -> Response {
    SpacerWidget::show(flex.max(1))
}
