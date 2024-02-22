/// Attributes to apply to text
#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct Attribute(pub u8);

#[cfg(feature = "serde")]
impl serde::Serialize for Attribute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{self:?}"))
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Attribute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error as _;
        <std::borrow::Cow<'_, str>>::deserialize(deserializer)
            .and_then(|data| data.parse().map_err(D::Error::custom))
    }
}

impl Attribute {
    pub const BOLD: Self = Self(0b00000001);
    pub const FAINT: Self = Self(0b00000010);
    pub const ITALIC: Self = Self(0b00000100);
    pub const UNDERLINE: Self = Self(0b00001000);
    pub const BLINK: Self = Self(0b00010000);
    pub const REVERSE: Self = Self(0b00100000);
    pub const STRIKE_OUT: Self = Self(0b01000000);
}

impl Attribute {
    pub const fn is_bold(&self) -> bool {
        self.0 & (1 << 0) != 0
    }

    pub const fn is_faint(&self) -> bool {
        self.0 & (1 << 1) != 0
    }

    pub const fn is_italic(&self) -> bool {
        self.0 & (1 << 2) != 0
    }

    pub const fn is_underline(&self) -> bool {
        self.0 & (1 << 3) != 0
    }

    pub const fn is_blink(&self) -> bool {
        self.0 & (1 << 4) != 0
    }

    pub const fn is_reverse(&self) -> bool {
        self.0 & (1 << 5) != 0
    }

    pub const fn is_strike_out(&self) -> bool {
        self.0 & (1 << 6) != 0
    }
}

impl std::ops::BitOr for Attribute {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for Attribute {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs
    }
}

impl std::ops::BitAnd for Attribute {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign for Attribute {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs
    }
}

impl std::ops::Not for Attribute {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl std::fmt::Binary for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08b}", self.0)
    }
}

impl std::fmt::Debug for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const FIELDS: [&str; 7] = [
            "Bold",
            "Faint",
            "Italic",
            "Underline",
            "Blink",
            "Reverse",
            "StrikeOut",
        ];

        let mut seen = false;
        for (flag, repr) in (0..).zip(FIELDS) {
            if (self.0 >> flag) & 1 == 1 {
                if seen {
                    f.write_str(" + ")?;
                }
                f.write_str(repr)?;
                seen |= true
            }
        }

        if !seen {
            f.write_str("Reset")?
        }
        Ok(())
    }
}

impl std::str::FromStr for Attribute {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut attribute = Self(0);
        for part in input.split_terminator('+').map(<str>::trim) {
            attribute |= match part {
                s if s.eq_ignore_ascii_case("bold") => Self::BOLD,
                s if s.eq_ignore_ascii_case("faint") => Self::FAINT,
                s if s.eq_ignore_ascii_case("italic") => Self::ITALIC,
                s if s.eq_ignore_ascii_case("underline") => Self::UNDERLINE,
                s if s.eq_ignore_ascii_case("blink") => Self::BLINK,
                s if s.eq_ignore_ascii_case("reverse") => Self::REVERSE,
                s if s.eq_ignore_ascii_case("strikeout") => Self::STRIKE_OUT,
                attr => return Err(format!("unknown attribute: {attr}")),
            }
        }
        Ok(attribute)
    }
}
