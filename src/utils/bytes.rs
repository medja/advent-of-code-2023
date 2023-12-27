use std::ops::{Add, Mul, Neg, Shl};

pub trait Bytes<'a> {
    fn parse_dec<T>(self) -> T
    where
        T: Copy + Add<T, Output = T> + Mul<T, Output = T> + From<u8>;

    fn parse_signed_dec<T>(self) -> T
    where
        T: Copy + Add<T, Output = T> + Mul<T, Output = T> + Neg<Output = T> + From<u8>;

    fn parse_hex<T>(self) -> T
    where
        T: Copy + Add<T, Output = T> + Shl<T, Output = T> + From<u8>;

    fn parse_signed_hex<T>(self) -> T
    where
        T: Copy + Add<T, Output = T> + Shl<T, Output = T> + Neg<Output = T> + From<u8>;
}

impl<'a, I: IntoIterator<Item = &'a u8>> Bytes<'a> for I {
    fn parse_dec<T>(self) -> T
    where
        T: Copy + Add<T, Output = T> + Mul<T, Output = T> + From<u8>,
    {
        parse_dec(T::from(0), self.into_iter().cloned())
    }

    fn parse_signed_dec<T>(self) -> T
    where
        T: Copy + Add<T, Output = T> + Mul<T, Output = T> + Neg<Output = T> + From<u8>,
    {
        let mut bytes = self.into_iter().cloned();

        match bytes.next() {
            Some(b'-') => -parse_dec::<T>(T::from(0), bytes),
            Some(initial) => parse_dec::<T>(T::from(initial - b'0'), bytes),
            None => T::from(0),
        }
    }

    fn parse_hex<T>(self) -> T
    where
        T: Copy + Add<T, Output = T> + Shl<T, Output = T> + From<u8>,
    {
        parse_hex(T::from(0), self.into_iter().cloned())
    }

    fn parse_signed_hex<T>(self) -> T
    where
        T: Copy + Add<T, Output = T> + Shl<T, Output = T> + Neg<Output = T> + From<u8>,
    {
        let mut bytes = self.into_iter().cloned();

        match bytes.next() {
            Some(b'-') => -parse_hex::<T>(T::from(0), bytes),
            Some(initial) => parse_hex::<T>(T::from(parse_hex_digit(initial)), bytes),
            None => T::from(0),
        }
    }
}

fn parse_dec<T>(initial: T, bytes: impl Iterator<Item = u8>) -> T
where
    T: Copy + Add<T, Output = T> + Mul<T, Output = T> + From<u8>,
{
    let radix = T::from(10);
    bytes.fold(initial, |acc, value| acc * radix + (value - b'0').into())
}

#[allow(dead_code)]
fn parse_hex<T>(initial: T, bytes: impl Iterator<Item = u8>) -> T
where
    T: Copy + Add<T, Output = T> + Shl<T, Output = T> + From<u8>,
{
    let shift = T::from(4);
    
    bytes.fold(initial, |acc, value| {
        (acc << shift) + parse_hex_digit(value).into()
    })
}

fn parse_hex_digit(value: u8) -> u8 {
    if value >= b'a' {
        value - b'a' + 10
    } else if value >= b'A' {
        value - b'A' + 10
    } else {
        value - b'0'
    }
}
