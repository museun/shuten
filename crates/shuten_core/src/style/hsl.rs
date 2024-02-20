use super::Rgb;

/// HSL color type, this is only provided to convert to an [`Rgb`]
#[derive(Copy, Clone, PartialEq)]
pub struct Hsl(
    /// hue
    pub f32,
    /// saturation
    pub f32,
    /// lightness
    pub f32,
);

impl std::fmt::Debug for Hsl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(h, s, l) = self;
        write!(f, "hsl({h:.02}%,{s:.02}%,{l:.02}%)",)
    }
}

impl Hsl {
    /// Create a new `h`,`s`,`l`
    pub const fn new(h: f32, s: f32, l: f32) -> Self {
        Self(h, s, l)
    }

    /// Get the `hue` channel
    pub const fn hue(&self) -> f32 {
        self.0
    }

    /// Get the `saturation` channel
    pub const fn saturation(&self) -> f32 {
        self.1
    }

    /// Get the `lightness` channel
    pub const fn lightness(&self) -> f32 {
        self.2
    }

    /// Convert this type to an [`Rgb`]
    pub fn to_rgb(&self) -> Rgb {
        self.into()
    }

    /// Darken this HSL by `ratio` (range `0.0 ..= 1.0`)
    pub fn darken(&self, ratio: f32) -> Self {
        let Self(h, s, mut l) = *self;
        l = (l - ratio).max(0.0).min(1.0);
        Self::new(h, s, l)
    }

    /// Lighten this HSL by `ratio` (range `0.0 ..= 1.0`)
    pub fn lighten(&self, ratio: f32) -> Self {
        let Self(h, s, mut l) = *self;
        l = (l + ratio).max(0.0).min(1.0);
        Self::new(h, s, l)
    }

    /// Get the complement color
    pub fn complement(&self) -> Self {
        let Self(h, mut s, l) = *self;
        s = (s + 180.0) % 360.0;
        Self::new(h, s, l)
    }

    /// Mix this color with another color
    ///
    /// - `left` is the ratio of the current color to mix in
    /// - `right` is the ratio of the other color to mix in
    pub fn mix(&self, left: f32, other: Self, right: f32) -> Self {
        let &Self(h1, s1, l1) = self;
        let Self(h2, s2, l2) = other;

        let h = if (h1 - h2).abs() > 180.0 {
            let (a, b) = if h1 < h2 {
                (h1 + 360.0, h2)
            } else {
                (h1, h2 + 360.0)
            };
            left.mul_add(a, right * b) / (left + right) % 360.0
        } else {
            left.mul_add(h1, right * h2) / (left + right)
        };

        let s = left.mul_add(s1, right * s2) / (left + right);
        let l = left.mul_add(l1, right * l2) / (left + right);

        Self(h, s, l)
    }
}

impl From<Rgb> for Hsl {
    fn from(value: Rgb) -> Self {
        let [r, g, b] = value.as_float();
        let min = r.min(g).min(b);
        let max = r.max(g).max(b);

        let l = 0.5 * (max + min);
        if min == max {
            return Self(0.0, 0.0, l);
        }

        let h = match () {
            _ if max == r => 60.0 * (g - b) / (max - min),
            _ if max == g => 60.0 * (b - r) / (max - min) + 120.0,
            _ if max == b => 60.0 * (r - g) / (max - min) + 240.0,
            _ => 0.0,
        };

        let h = (h + 360.0) % 360.0;

        let s = if 0.0 < l && l <= 0.5 {
            (max - min) / (2.0 * l)
        } else {
            (max - min) / 2.0f32.mul_add(-l, 2.0)
        };

        Self(h, s, l)
    }
}

impl From<&Rgb> for Hsl {
    fn from(value: &Rgb) -> Self {
        (*value).into()
    }
}

impl From<Rgb> for crossterm::style::Color {
    #[inline(always)]
    fn from(Rgb(r, g, b): Rgb) -> Self {
        Self::Rgb { r, g, b }
    }
}

impl From<Hsl> for crossterm::style::Color {
    #[inline(always)]
    fn from(hsl: Hsl) -> Self {
        Rgb::from(hsl).into()
    }
}
