//! Geometry and general math types

use std::ops::{Add, Div, Mul, RangeInclusive, Rem, Sub};

mod pos2;
pub use pos2::{pos2, Pos2};

mod vec2;
pub use vec2::{vec2, Vec2};

mod rect;
pub use rect::{rect, Rect};

mod pos2f;
pub use pos2f::{pos2f, Pos2f};

mod vec2f;
pub use vec2f::{vec2f, Vec2f};

mod rectf;
pub use rectf::Rectf;

mod offset;
pub use offset::{offset, Offset};

mod align;
pub use align::{Align, Align2};

mod margin;
pub use margin::Margin;

mod constraints;
pub use constraints::Constraints;

mod flex;
pub use flex::{
    CrossAxisAlignment, FlexFit, Flow, MainAxisAlignItems, MainAxisAlignment, MainAxisSize,
};

mod dimension;
pub use dimension::{Dimension, Dimension2};

/// How should a [`Rect`] be resized?
///
/// You'll add/subtract this type to a [`Rect`]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum ResizeDelta {
    Left(u16),
    Right(u16),
    Up(u16),
    Down(u16),
}

/// Remap a `from` range to a `to` range and the interpolate `x` from the `from` range to the `to` range
///
/// ```rust
/// use shuten_core::geom::{remap, almost_eq};
/// assert!(almost_eq(remap(0.3, 0.0..=1.0, 0.0..=50.0), 15.0))
/// ```
pub fn remap<N>(x: N, from: impl Into<RangeInclusive<N>>, to: impl Into<RangeInclusive<N>>) -> N
where
    N: Sub<Output = N> + Mul<Output = N> + Add<Output = N> + Div<Output = N> + Copy + Num,
{
    let from = from.into();
    let t = (x - *from.start()) / (*from.end() - *from.start());
    lerp(to.into(), t)
}

/// Remap a `from` range to a `to` range and the interpolate `x` from the `from` range to the `to` range, clamping the value to the `to` range
///
/// ```rust
/// use shuten_core::geom::{remap_clamp, almost_eq};
/// assert!(almost_eq(remap_clamp(1.3, 0.0..=1.0, 0.0..=50.0), 50.0))
/// ```
pub fn remap_clamp<N>(
    x: N,
    from: impl Into<RangeInclusive<N>>,
    to: impl Into<RangeInclusive<N>>,
) -> N
where
    N: Num,
{
    let from = from.into();
    let to = to.into();
    if *from.end() < *from.start() {
        return remap_clamp(x, *from.end()..=*from.start(), *to.end()..=*to.start());
    }

    if x <= *from.start() {
        return *to.start();
    }
    if *from.end() <= x {
        return *to.end();
    }

    let t = (x - *from.start()) / (*from.end() - *from.start());
    if N::ONE <= t {
        *to.end()
    } else {
        lerp(to, t)
    }
}

/// Linearly interpolate `t` to fit in `range`
///
/// ```rust
/// use shuten_core::geom::{lerp, almost_eq};
/// assert_eq!(lerp(0.0..=10.0, 0.5), 5.0)
/// ```
pub fn lerp<N>(range: impl Into<RangeInclusive<N>>, t: N) -> N
where
    N: Num,
{
    let range = range.into();
    let (l, r) = (*range.start(), *range.end());
    (N::ONE - t) * l + t * r
}

/// Inverse linear interpolation for `t` to fit in the inverse `range`
///
/// ```rust
/// use shuten_core::geom::{inverse_lerp, almost_eq};
/// assert_eq!(inverse_lerp(0.0..=10.0, 0.5).unwrap(), 0.05)
/// ```
pub fn inverse_lerp<N>(range: impl Into<RangeInclusive<N>>, t: N) -> Option<N>
where
    N: Num,
{
    let range = range.into();
    let (min, max) = (*range.start(), *range.end());
    if min == max {
        return None;
    }
    Some((t - min) / (max - min))
}

/// Compares to floats to see if they are almost equal
pub fn almost_eq(a: f32, b: f32) -> bool {
    if a == b {
        return true;
    }

    let abs = a.abs().max(b.abs());
    abs <= f32::EPSILON || ((a - b).abs() / abs) <= f32::EPSILON
}

/// Numbers used by this module
pub trait Num:
    PartialOrd
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Add<Output = Self>
    + Div<Output = Self>
    + Copy
{
    const ONE: Self;
}

impl Num for i32 {
    const ONE: Self = 1;
}

impl Num for u16 {
    const ONE: Self = 1;
}

impl Num for f32 {
    const ONE: Self = 1.0;
}

/// Divide `d` by `n` and round up
pub fn div_round<N>(n: N, d: N) -> N
where
    N: Num + Rem<Output = N>,
{
    (n / d) + (n % d)
}
