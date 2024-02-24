use crate::{geom::Rectf, input::Interest, tree::WidgetId};

#[derive(Debug, serde::Serialize)]
pub struct Node {
    pub(crate) rect: Rectf,
    pub(crate) interest: Interest,
    pub(crate) clipping: bool,
    pub(crate) clipped_by: Option<WidgetId>,
    pub(crate) ty: &'static str,
}

impl Node {
    pub const fn rect(&self) -> Rectf {
        self.rect
    }

    pub const fn interest(&self) -> Interest {
        self.interest
    }

    pub fn set_interest(&mut self, interest: Interest) {
        self.interest = interest
    }

    pub const fn type_name(&self) -> &'static str {
        self.ty
    }
}
