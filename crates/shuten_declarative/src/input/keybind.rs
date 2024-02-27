use crate::Modifiers;

use super::Key;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Keybind {
    pub key: Key,
    pub modifiers: Modifiers,
}

impl Keybind {
    pub const fn new(key: Key, modifiers: Modifiers) -> Self {
        Self { key, modifiers }
    }

    pub const fn key(key: Key) -> Self {
        Self {
            key,
            modifiers: Modifiers::NONE,
        }
    }

    pub const fn char(char: char) -> Self {
        Self::key(Key::Char(char))
    }

    pub const fn shift(mut self) -> Self {
        self.modifiers.0 |= Modifiers::SHIFT.0;
        self
    }

    pub const fn ctrl(mut self) -> Self {
        self.modifiers.0 |= Modifiers::CTRL.0;
        self
    }

    pub const fn alt(mut self) -> Self {
        self.modifiers.0 |= Modifiers::ALT.0;
        self
    }
}

impl From<Key> for Keybind {
    fn from(value: Key) -> Self {
        Self::key(value)
    }
}

impl From<char> for Keybind {
    fn from(value: char) -> Self {
        Self::key(Key::Char(value))
    }
}
