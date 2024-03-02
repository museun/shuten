use crate::{NoResponse, Ui, Widget};

#[derive(Default, Debug)]
pub(crate) struct Placeholder;

impl Widget for Placeholder {
    type Response = NoResponse;
    type Props<'a> = ();

    fn update(&mut self, _: &Ui, _: Self::Props<'_>) -> Self::Response {}
}
