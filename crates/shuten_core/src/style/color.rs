use super::{Hsl, Rgb};

/// A color type thats used extensively in this crate
///
/// Hint: You'd generally use `rgb.into()` or similar
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Color {
    /// Use an [`Rgb`] color
    Rgb(Rgb),
    /// Reset the color
    Reset,
    /// Reuse the color
    #[default]
    Reuse,
}

impl Color {
    pub fn lighten(self, ratio: f32) -> Self {
        if let Self::Rgb(color) = self {
            return Self::Rgb(color.lighten(ratio));
        }
        self
    }

    pub fn darken(self, ratio: f32) -> Self {
        if let Self::Rgb(color) = self {
            return Self::Rgb(color.darken(ratio));
        }
        self
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Self::Rgb(Rgb::from_u32(value))
    }
}

impl From<&u32> for Color {
    fn from(value: &u32) -> Self {
        Self::from(*value)
    }
}

impl From<Rgb> for Color {
    fn from(value: Rgb) -> Self {
        Self::Rgb(value)
    }
}

impl From<&Rgb> for Color {
    fn from(value: &Rgb) -> Self {
        Self::Rgb(*value)
    }
}

impl From<Option<Rgb>> for Color {
    fn from(value: Option<Rgb>) -> Self {
        value.map(Self::Rgb).unwrap_or(Self::Reset)
    }
}

impl From<Option<Self>> for Color {
    fn from(value: Option<Self>) -> Self {
        value.unwrap_or(Self::Reuse)
    }
}

impl From<&Self> for Color {
    fn from(value: &Self) -> Self {
        *value
    }
}

impl From<Hsl> for Color {
    fn from(value: Hsl) -> Self {
        Self::Rgb(value.into())
    }
}

impl From<&Hsl> for Color {
    fn from(value: &Hsl) -> Self {
        Self::Rgb(value.into())
    }
}

impl From<Option<Hsl>> for Color {
    fn from(value: Option<Hsl>) -> Self {
        value.map(Into::into).map(Self::Rgb).unwrap_or(Self::Reset)
    }
}
