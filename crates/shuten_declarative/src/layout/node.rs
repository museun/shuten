use crate::{geom::Rectf, input::Interest, tree::WidgetId};

#[derive(Debug)]
pub struct Node {
    pub(crate) rect: Rectf,
    pub(crate) interest: Interest,
    pub(crate) clipping: bool,
    pub(crate) clipped_by: Option<WidgetId>,
}
