use shuten::{
    event::{Key, Modifiers, MouseButton},
    geom::{Pos2f, Vec2f},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KeyPressed {
    pub key: Key,
    pub modifiers: Modifiers,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MouseMove {
    pub pos: Pos2f,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MouseClick {
    pub pos: Pos2f,
    pub button: MouseButton,
    pub modifiers: Modifiers,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MouseHeld {
    pub pos: Pos2f,
    pub button: MouseButton,
    pub modifiers: Modifiers,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MouseDrag {
    pub released: bool,
    pub origin: Pos2f,
    pub pos: Pos2f,
    pub delta: Vec2f,
    pub button: MouseButton,
    pub modifiers: Modifiers,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MouseScroll {
    pub pos: Pos2f,
    pub delta: Vec2f,
    pub modifiers: Modifiers,
}
