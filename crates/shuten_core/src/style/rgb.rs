use std::str::FromStr;

use super::Hsl;

/// The main color type
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Rgb(
    /// The red channel
    pub u8,
    /// The green channel
    pub u8,
    /// The blue channel
    pub u8,
);

impl From<u32> for Rgb {
    fn from(value: u32) -> Self {
        Self::from_u32(value)
    }
}

impl std::fmt::Debug for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(r, g, b) = self;
        write!(f, "rgb({r}, {g}, {b})")
    }
}

impl std::fmt::LowerHex for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(r, g, b) = self;
        write!(f, "#{r:02x}{g:02x}{b:02x}")
    }
}

impl std::fmt::UpperHex for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(r, g, b) = self;
        write!(f, "#{r:02X}{g:02X}{b:02X}")
    }
}

impl Rgb {
    /// Create a new `r`,`g`,`b`
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }

    /// Get the red channel
    pub const fn red(&self) -> u8 {
        self.0
    }

    /// Get the green channel
    pub const fn green(&self) -> u8 {
        self.1
    }

    /// Get the blue channel
    pub const fn blue(&self) -> u8 {
        self.2
    }

    /// This allows you to parse a `#rrggbb` or `#rgb` `&static str` at compile time
    ///
    /// ```rust
    /// use shuten_core::style::Rgb;
    /// const RED: Rgb = Rgb::from_static("#FF0000");
    /// const BLUE: Rgb = Rgb::from_static("#00F");
    /// ```
    #[track_caller]
    pub const fn from_static(input: &'static str) -> Self {
        #[track_caller]
        const fn to_digit(d: u8) -> u8 {
            assert!(d.is_ascii_hexdigit(), "invalid hex digit");
            match d.wrapping_sub(b'0') {
                d if d < 10 => d,
                _ => d.to_ascii_lowercase().saturating_sub(b'a') + 10,
            }
        }
        const fn pack(high: u8, low: u8) -> u8 {
            high << 4 | low
        }

        #[allow(clippy::match_ref_pats)]
        let (r, g, b) = match input.as_bytes() {
            &[b'#', rh, rl, gh, gl, bh, bl] => {
                let (rh, gh, bh) = (to_digit(rh), to_digit(gh), to_digit(bh));
                let (rl, gl, bl) = (to_digit(rl), to_digit(gl), to_digit(bl));
                (pack(rh, rl), pack(gh, gl), pack(bh, bl))
            }
            &[b'#', r, g, b] => {
                let (r, g, b) = (to_digit(r), to_digit(g), to_digit(b));
                (pack(r, r), pack(g, g), pack(b, b))
            }
            _ => panic!("invalid hex string color"),
        };
        Self(r, g, b)
    }

    /// Create an RGB from a provided `rrggbb` `u32`
    ///
    /// ```rust
    /// use shuten_core::style::Rgb;
    /// const RED: Rgb = Rgb::from_u32(0xFF0000);
    /// const BLUE: Rgb = Rgb::from_u32(255);
    /// ```
    pub const fn from_u32(rrggbb: u32) -> Self {
        Self::new(
            ((rrggbb >> 16) & 0xFF) as u8,
            ((rrggbb >> 8) & 0xFF) as u8,
            (rrggbb & 0xFF) as u8,
        )
    }

    /// Create an RGB from a provided `rgb` `u16`
    ///
    /// ```rust
    /// use shuten_core::style::Rgb;
    /// const RED: Rgb = Rgb::from_u16(0xF00);
    /// const BLUE: Rgb = Rgb::from_u16(15);
    /// ```
    pub const fn from_u16(rgb: u16) -> Self {
        let (r, g, b) = (
            ((rgb >> 8) & 0xF) as u8,
            ((rgb >> 4) & 0xF) as u8,
            (rgb & 0xF) as u8,
        );

        Self::new((r << 4) | r, (g << 4) | g, (b << 4) | b)
    }

    /// Mix this color with another color
    ///
    /// - `left` is the ratio of current color to mix in
    /// - `right` is the ratio of other color to mix in
    pub fn mix(&self, left: f32, other: Self, right: f32) -> Self {
        let [r1, g1, b1] = self.as_float();
        let [r2, g2, b2] = other.as_float();

        Self::from_float([
            left.mul_add(r1, right * r2) / (left + right),
            left.mul_add(g1, right * g2) / (left + right),
            left.mul_add(b1, right * b2) / (left + right),
        ])
    }

    /// Blend this color other another color, at a fixed ratio
    ///
    /// This differents from [`Rgb::blend`] in:
    ///
    /// This does an sRGB blend
    pub fn blend_flat(&self, other: Self, mix: f32) -> Self {
        let [r1, g1, b1] = self.as_float();
        let [r2, g2, b2] = other.as_float();

        Self::from_float([
            (r2 - r1).mul_add(mix, r1),
            (g2 - g1).mul_add(mix, g1),
            (b2 - b1).mul_add(mix, b1),
        ])
    }

    /// Blend this color other another color, at a fixed ratio
    ///
    /// This differents from [`Rgb::blend`] in:
    ///
    /// This does a perceptual blend
    pub fn blend(&self, other: Self, mix: f32) -> Self {
        self.mix(mix, other, mix)
    }

    /// Blend this color other another color, at a fixed ratio
    ///
    /// This differents from [`Rgb::blend`] in:
    ///
    /// It does a linear blend, so an sRGB->linear rgb conversion is done
    pub fn blend_linear(&self, other: Self, mix: f32) -> Self {
        let mixed @ Hsl(h, ..) = Hsl::from(self.blend(other, mix));
        let Hsl(_, s, l) = Hsl::from(*self)
            .mix(mix, Hsl::from(other), mix)
            .mix(0.5, mixed, 0.5);
        Hsl(h, s, l).into()
    }

    /// Get a float array of this type
    ///
    /// Format: `[r,g,b]`
    pub fn as_float(&self) -> [f32; 3] {
        let Self(r, g, b) = *self;
        let scale = |d| (d as f32 / 256.0);
        [scale(r), scale(g), scale(b)]
    }

    /// Produce an [`Rgb`] from a float array
    ///
    /// Format: `[r,g,b]`
    pub fn from_float([r, g, b]: [f32; 3]) -> Self {
        let scale = |d| (255.0 * d) as u8;
        Self(scale(r), scale(g), scale(b))
    }

    /// Get a the complement color for this color
    pub fn complement(&self) -> Self {
        Hsl::from(self).complement().into()
    }

    /// Darken this color by a ratio (in the range `0.0 ..= 1.0`)
    pub fn darken(&self, ratio: f32) -> Self {
        Hsl::from(self).darken(ratio).into()
    }

    /// Lighten this color by a ratio (in the range `0.0 ..= 1.0`)
    pub fn lighten(&self, ratio: f32) -> Self {
        Hsl::from(self).lighten(ratio).into()
    }
}

impl From<&Hsl> for Rgb {
    fn from(value: &Hsl) -> Self {
        (*value).into()
    }
}

impl From<Hsl> for Rgb {
    #[allow(clippy::many_single_char_names)]
    fn from(value: Hsl) -> Self {
        let Hsl(mut h, s, l) = value;

        if s == 0.0 {
            return Self::from_float([l, l, l]);
        }

        h /= 360.0;

        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            s.mul_add(-l, l + s)
        };

        let p = 2.0f32.mul_add(l, -q);

        let r = hue(p, q, h + (1.0 / 3.0));
        let g = hue(p, q, h);
        let b = hue(p, q, h - (1.0 / 3.0));

        Self::from_float([r, g, b])
    }
}

fn hue(p: f32, q: f32, mut t: f32) -> f32 {
    if t < 0.0 {
        t += 1.0
    }
    if t > 1.0 {
        t -= 1.0;
    }
    if 6.0 * t < 1.0 {
        ((p + (q - p)) * 6.0 * t).clamp(0.0, 1.0)
    } else if 2.0 * t < 1.0 {
        q
    } else if 3.0 * t < 2.0 {
        ((q - p) * ((2.0 / 3.0) - t))
            .mul_add(6.0, p)
            .clamp(0.0, 1.0)
    } else {
        p
    }
}

impl FromStr for Rgb {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(s) = s.strip_prefix('#') {
            let digits = u16::from_str_radix(s, 16).map_err(|_| "invalid hex digits")?;
            return match s.len() {
                3 => Ok(Self::from_u16(digits)),
                6 => Ok(Self::from_u16(digits)),
                _ => Err("invalid hex-string, should be #rrggbb or #rgb"),
            };
        }

        if s.starts_with("rgb(") && s.ends_with(')') {
            let s = &s[4..s.len() - 1];
            let mut iter = s.split_terminator(',').flat_map(|s| s.trim().parse());
            let r = iter.next().ok_or("invalid red channel")?;
            let g = iter.next().ok_or("invalid green channel")?;
            let b = iter.next().ok_or("invalid blue channel")?;
            if iter.next().is_some() {
                return Err("rgb(r,g,b) must be a triplet");
            }
            return Ok(Self::new(r, g, b));
        }

        Err("rgb color must be in the form of rgb(r,g,b) or #rrggbb or #rgb")
    }
}
