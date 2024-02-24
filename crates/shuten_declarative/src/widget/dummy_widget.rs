use crate::widget::prelude::*;

#[derive(Debug, Default)]
pub struct DummyWidget;

impl Widget for DummyWidget {
    type Props<'a> = ();
    type Response = NoResponse;
    fn update(&mut self, _: Self::Props<'_>) -> Self::Response {}
}
