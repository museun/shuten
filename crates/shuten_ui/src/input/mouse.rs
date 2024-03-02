use std::collections::HashMap;

use shuten::{event::MouseButton, geom::Pos2f};

use crate::{input::Layered, Interest, WidgetId};

#[derive(Debug, Default)]
pub(crate) struct Mouse {
    pub(crate) pos: Pos2f,
    pub(crate) layered: Layered<Interest>,
    pub(crate) buttons: HashMap<MouseButton, ButtonState>,
}

impl Mouse {
    pub fn push_layer(&mut self, id: WidgetId) {
        self.layered.push_layer(id);
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub(crate) enum ButtonState {
    Held,
    Released,
}

impl ButtonState {
    pub const fn is_down(&self) -> bool {
        matches!(self, Self::Held)
    }
}
