pub trait DigitExt {
    fn width(&self) -> usize;
    fn digits(&self) -> impl Iterator<Item = u8>;
    fn chars(&self) -> impl Iterator<Item = char> {
        self.digits().map(|c| (c + b'0') as char)
    }
}

impl DigitExt for usize {
    fn width(&self) -> usize {
        count_digits(*self)
    }

    fn digits(&self) -> impl Iterator<Item = u8> {
        digits(*self)
    }
}

const fn count_digits(d: usize) -> usize {
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

fn digits(mut d: usize) -> impl Iterator<Item = u8> {
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
