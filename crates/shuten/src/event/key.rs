/// Events produced by the keyboard
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[non_exhaustive]
pub enum Key {
    /// A character was pressed
    ///
    /// This interpolates shift-events into a character, if possible
    ///
    /// e.g. This'll be `Char('!')` for `Shift + 1`
    Char(char),
    /// A function key was pressed
    Function(u8),
    /// The left arrow was pressed
    Left,
    /// The right arrow was pressed
    Right,
    /// The up arrow was pressed
    Up,
    /// The down arrow was pressed
    Down,
    /// Page down was pressed
    PageDown,
    /// Page up was pressed
    PageUp,
    /// Backspace was pressed
    Backspace,
    /// De;ete was pressed
    Delete,
    /// Insert was pressed
    Insert,
    /// Home was pressed
    Home,
    /// End was pressed
    End,
    /// Escape was pressed
    Escape,
    /// Tab was pressed
    Tab,
    /// Shift-Tab was pressed
    BackTab,
    /// Enter was pressed
    Enter,
}

impl std::convert::TryFrom<crossterm::event::KeyCode> for Key {
    type Error = crossterm::event::KeyCode;
    fn try_from(input: crossterm::event::KeyCode) -> std::result::Result<Self, Self::Error> {
        use crossterm::event::KeyCode as K;
        let key = match input {
            K::Backspace => Self::Backspace,
            K::Enter => Self::Enter,
            K::Left => Self::Left,
            K::Right => Self::Right,
            K::Up => Self::Up,
            K::Down => Self::Down,
            K::Home => Self::Home,
            K::End => Self::End,
            K::PageUp => Self::PageUp,
            K::PageDown => Self::PageDown,
            K::Tab => Self::Tab,
            K::BackTab => Self::BackTab,
            K::Delete => Self::Delete,
            K::Insert => Self::Insert,
            K::F(f) => Self::Function(f),
            K::Char(c) => Self::Char(c),
            K::Esc => Self::Escape,
            unknown => return Err(unknown),
        };

        Ok(key)
    }
}
