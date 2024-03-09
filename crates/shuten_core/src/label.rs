use std::borrow::Cow;

mod digits;
use self::digits::{count_digits_f32, count_leading, digits, split, DigitExt};

mod styled;
pub use styled::Styled;

pub trait Label: std::fmt::Debug {
    type Static: Label + 'static + Sized;
    fn into_static(self) -> Self::Static;
    fn width(&self) -> u16;
    fn chars(&self) -> impl Iterator<Item = char>;
}

impl<T: Label> Label for &T
where
    T: Clone,
{
    type Static = T::Static;

    fn into_static(self) -> Self::Static {
        self.clone().into_static()
    }

    fn width(&self) -> u16 {
        <T as Label>::width(self)
    }

    fn chars(&self) -> impl Iterator<Item = char> {
        <T as Label>::chars(self)
    }
}

impl<'a> Label for Cow<'a, str> {
    type Static = Cow<'static, str>;

    fn into_static(self) -> Self::Static {
        match self {
            Cow::Borrowed(s) => Cow::Owned(s.to_owned()),
            Cow::Owned(s) => Cow::Owned(s),
        }
    }

    fn width(&self) -> u16 {
        self.as_ref().width()
    }

    fn chars(&self) -> impl Iterator<Item = char> {
        self.as_ref().chars()
    }
}

impl Label for String {
    type Static = String;

    fn into_static(self) -> Self::Static {
        self
    }

    fn width(&self) -> u16 {
        self.as_str().width()
    }

    fn chars(&self) -> impl Iterator<Item = char> {
        self.as_str().chars()
    }
}

impl Label for &str {
    type Static = String;
    fn into_static(self) -> Self::Static {
        self.to_string()
    }

    fn width(&self) -> u16 {
        self.len() as _
    }

    fn chars(&self) -> impl Iterator<Item = char> {
        (*self).chars()
    }
}

impl Label for char {
    type Static = Self;

    fn into_static(self) -> Self::Static {
        self
    }

    fn width(&self) -> u16 {
        1
    }

    fn chars(&self) -> impl Iterator<Item = char> {
        std::iter::once(*self)
    }
}

impl Label for () {
    type Static = Self;
    fn into_static(self) -> Self::Static {
        self
    }
    fn width(&self) -> u16 {
        0
    }
    fn chars(&self) -> impl Iterator<Item = char> {
        std::iter::empty()
    }
}

macro_rules! impl_for_unsigned {
    ($($ty:ty)*) => {
        $(impl Label for $ty {
            type Static = Self;
            fn into_static(self) -> Self::Static {
                self
            }

            fn width(&self) -> u16 {
                <Self as DigitExt>::width(self) as u16
            }

            fn chars(&self) -> impl Iterator<Item = char> {
                <Self as DigitExt>::chars(self)
            }
        })*
    };
}

impl_for_unsigned! {
    u8 u16 u32 u64 usize
}

impl Label for i32 {
    type Static = Self;
    fn into_static(self) -> Self::Static {
        self
    }

    fn width(&self) -> u16 {
        let signed = if self.is_negative() { 1 } else { 0 };
        let len = digits::count_digits(self.unsigned_abs() as usize);
        (len + signed) as u16
    }

    fn chars(&self) -> impl Iterator<Item = char> {
        (self.is_negative())
            .then_some('-')
            .into_iter()
            .chain(digits::digits(self.unsigned_abs() as usize).map(|c| (c + b'0') as char))
    }
}

impl Label for f32 {
    type Static = Self;
    fn into_static(self) -> Self::Static {
        self
    }

    fn width(&self) -> u16 {
        count_digits_f32(*self) as _
    }

    fn chars(&self) -> impl Iterator<Item = char> {
        let (head, tail) = split(*self);

        digits(head)
            .map(|c| (c + b'0') as char)
            .chain(Some('.'))
            .chain(
                digits(tail)
                    .take(count_leading(tail) + 1)
                    .map(|c| (c + b'0') as char),
            )
    }
}
