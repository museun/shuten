use crate::{NoResponse, Widget};

#[derive(Default, Debug)]
pub(crate) struct Placeholder;

impl Widget for Placeholder {
    type Response = NoResponse;
    type Props<'a> = ();

    fn update(&mut self, _: Self::Props<'_>) -> Self::Response {}
}
