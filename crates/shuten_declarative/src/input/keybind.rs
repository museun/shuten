use crate::Modifiers;

use super::KeyEventKind;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Keybind {
    pub key: KeyEventKind,
    pub modifiers: Modifiers,
}

impl Keybind {
    pub const fn new(key: KeyEventKind, modifiers: Modifiers) -> Self {
        Self { key, modifiers }
    }

    pub const fn key(key: KeyEventKind) -> Self {
        Self {
            key,
            modifiers: Modifiers::NONE,
        }
    }

    pub const fn char(char: char) -> Self {
        Self::key(KeyEventKind::Char(char))
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

impl From<KeyEventKind> for Keybind {
    fn from(value: KeyEventKind) -> Self {
        Self::key(value)
    }
}

impl From<char> for Keybind {
    fn from(value: char) -> Self {
        Self::key(KeyEventKind::Char(value))
    }
}
