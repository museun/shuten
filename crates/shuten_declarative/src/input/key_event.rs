pub use crate::input::mouse::ButtonState;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct KeyEvent {
    pub state: ButtonState,
    pub kind: KeyEventKind,
}

#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum KeyEventKind {
    Char(char),
    Function(u8),
    Left,
    Right,
    Up,
    Down,
    PageDown,
    PageUp,
    Backspace,
    Delete,
    Insert,
    Home,
    End,
    Escape,
    Tab,
    BackTab,
    Enter,
}

pub trait TranslateKeyEvent {
    fn translate(&self) -> Option<KeyEvent>;
}

impl TranslateKeyEvent for shuten::event::Key {
    fn translate(&self) -> Option<KeyEvent> {
        use KeyEventKind as E;
        let kind = match *self {
            Self::Char(char) => E::Char(char),
            Self::Function(func) => E::Function(func),
            Self::Left => E::Left,
            Self::Right => E::Right,
            Self::Up => E::Up,
            Self::Down => E::Down,
            Self::PageDown => E::PageDown,
            Self::PageUp => E::PageUp,
            Self::Backspace => E::Backspace,
            Self::Delete => E::Delete,
            Self::Insert => E::Insert,
            Self::Home => E::Home,
            Self::End => E::End,
            Self::Escape => E::Escape,
            Self::Tab => E::Tab,
            Self::BackTab => E::BackTab,
            Self::Enter => E::Enter,
            _ => return None,
        };

        Some(KeyEvent {
            state: ButtonState::Released,
            kind,
        })
    }
}
