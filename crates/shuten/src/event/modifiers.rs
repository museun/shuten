/// Key modifiers attached to an [`Event`](crate::event::Event)
// TODO this partial eq needs to use 'any' rather than 'all' logic
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Modifiers(pub u8);

#[cfg(feature = "serde")]
impl serde::Serialize for Modifiers {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{self:?}"))
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Modifiers {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error as _;
        <std::borrow::Cow<'_, str>>::deserialize(deserializer)
            .and_then(|data| data.parse().map_err(D::Error::custom))
    }
}

impl std::fmt::Debug for Modifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl Default for Modifiers {
    fn default() -> Self {
        Self::NONE
    }
}

impl Modifiers {
    pub const NONE: Self = Self(0b0000);
    pub const SHIFT: Self = Self(0b0001);
    pub const CTRL: Self = Self(0b0010);
    pub const ALT: Self = Self(0b0100);

    pub const fn is_none(&self) -> bool {
        self.0 == Self::NONE.0
    }

    pub const fn is_shift(&self) -> bool {
        self.0 & 1 == 1
    }

    pub const fn is_ctrl(&self) -> bool {
        (self.0 >> 1) & 1 == 1
    }

    pub const fn is_alt(&self) -> bool {
        (self.0 >> 2) & 1 == 1
    }

    pub const fn is_shift_only(&self) -> bool {
        self.0 == Self::SHIFT.0
    }

    pub const fn is_ctrl_only(&self) -> bool {
        self.0 == Self::CTRL.0
    }

    pub const fn is_alt_only(&self) -> bool {
        self.0 == Self::ALT.0
    }
}

impl std::fmt::Binary for Modifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Display for Modifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut seen = false;
        for (i, repr) in (0..).zip(["Shift", "Ctrl", "Alt"]) {
            if (self.0 >> i) & 1 == 1 {
                if seen {
                    f.write_str(" + ")?;
                }
                f.write_str(repr)?;
                seen |= true
            }
        }
        if !seen {
            f.write_str("None")?;
        }
        Ok(())
    }
}

impl std::str::FromStr for Modifiers {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut modifier = Self::NONE;
        for part in input.split_terminator('+').map(<str>::trim) {
            modifier |= match part {
                s if s.eq_ignore_ascii_case("shift") => Self::SHIFT,
                s if s.eq_ignore_ascii_case("ctrl") => Self::CTRL,
                s if s.eq_ignore_ascii_case("alt") => Self::ALT,
                modifier => return Err(format!("unknown modifier: {modifier}")),
            }
        }
        Ok(modifier)
    }
}

impl std::ops::BitOr for Modifiers {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for Modifiers {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs
    }
}

impl From<crossterm::event::KeyModifiers> for Modifiers {
    fn from(value: crossterm::event::KeyModifiers) -> Self {
        [
            crossterm::event::KeyModifiers::SHIFT,
            crossterm::event::KeyModifiers::CONTROL,
            crossterm::event::KeyModifiers::ALT,
        ]
        .into_iter()
        .fold(Self::NONE, |this, m| this | Self((value & m).bits()))
    }
}
