use std::collections::HashMap;

use crate::tree::WidgetId;
use shuten::{event::MouseButton, geom::Pos2f};

#[derive(Debug, Default, serde::Serialize)]
pub struct Mouse {
    pub pos: Pos2f,
    pub buttons: HashMap<MouseButton, ButtonState>,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, serde::Serialize)]
pub enum ButtonState {
    Down, // just pressed
    Held, // been pressed for some frames

    Up,       // just released
    Released, // been released for some frames
}

impl ButtonState {
    pub const fn is_down(self) -> bool {
        matches!(self, Self::Down | Self::Held)
    }

    pub fn interpolate(&mut self) {
        match self {
            Self::Down => *self = Self::Held,
            Self::Up => *self = Self::Released,
            _ => {}
        }
    }
}

#[derive(Debug, Default, serde::Serialize)]
pub struct Intersections {
    pub hit: Vec<WidgetId>,
    pub entered: Vec<WidgetId>,
    pub entered_and_sunk: Vec<WidgetId>,
    pub pressed: HashMap<MouseButton, Vec<WidgetId>>,
}
