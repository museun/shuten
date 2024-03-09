pub trait DigitExt {
    fn width(&self) -> usize;
    fn digits(&self) -> impl Iterator<Item = u8>;
    fn chars(&self) -> impl Iterator<Item = char> {
        self.digits().map(|c| (c + b'0') as char)
    }
}

macro_rules! impl_for_unsigned {
    ($($ty:ty)*) => {
        $(impl DigitExt for $ty {
            fn width(&self) -> usize {
                count_digits(*self as usize)
            }
            fn digits(&self) -> impl Iterator<Item = u8> {
                digits(*self as usize)
            }
        })*
    };
}

impl_for_unsigned! {
    u8 u16 u32 u64 usize
}

pub(crate) const fn count_digits(d: usize) -> usize {
    let (mut len, mut n) = (1, 1);
    while len < 20 {
        n *= 10;
        if n > d {
            return len;
        }
        len += 1;
    }
    len
}

pub(crate) fn digits(mut d: usize) -> impl Iterator<Item = u8> {
    let x = count_digits(d) as u32 - 1;
    let mut mag = 10usize.pow(x);
    if d < mag {
        mag /= 10;
    }

    let mut is_zero = d == 0;
    std::iter::from_fn(move || {
        if std::mem::take(&mut is_zero) {
            return Some(0);
        }

        if mag == 0 {
            return None;
        }

        let n = d / mag;
        d %= mag;
        mag /= 10;
        Some(n as u8)
    })
}

pub(crate) fn split(f: f32) -> (usize, usize) {
    let head = f.trunc() as usize;
    let tail = (f.fract() * 1e2) as usize;
    (head, tail)
}

pub(crate) fn count_leading(f: usize) -> usize {
    let mut p = 0;

    f.digits()
        .enumerate()
        .take_while(|&(i, c)| {
            p += (i > 0 && (c == 0)) as usize;
            p < 1
        })
        .map(|(s, _)| s)
        .last()
        .unwrap_or(0)
}

pub(crate) fn count_digits_f32(f: f32) -> usize {
    let (head, tail) = split(f);
    count_digits(head) + 2 + count_leading(tail)
}
