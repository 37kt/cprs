use std::ops::{Add, BitXor, Div, Rem, Sub};

pub trait Zero {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

pub trait One {
    fn one() -> Self;
    fn is_one(&self) -> bool;
}

macro_rules! impl_zero_one {
    ($($t:ty)*) => {
        $(
            impl $crate::Zero for $t {
                fn zero() -> Self {
                    0
                }
                fn is_zero(&self) -> bool {
                    *self == 0
                }
            }
            impl $crate::One for $t {
                fn one() -> Self {
                    1
                }
                fn is_one(&self) -> bool {
                    *self == 1
                }
            }
        )*
    };
}

impl_zero_one!(usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128);

pub fn div_ceil<T>(a: T, b: T) -> T
where
    T: Copy
        + Zero
        + One
        + Add<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + BitXor<Output = T>
        + PartialOrd,
{
    let zero = T::zero();
    let one = T::one();
    a / b
        + if a ^ b >= zero && a % b != zero {
            one
        } else {
            zero
        }
}

pub fn div_floor<T>(a: T, b: T) -> T
where
    T: Copy
        + Zero
        + One
        + Sub<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + BitXor<Output = T>
        + PartialOrd,
{
    let zero = T::zero();
    let one = T::one();
    a / b
        - if a ^ b < zero && a % b != zero {
            one
        } else {
            zero
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_div_floor() {
        assert_eq!(div_floor(10, 3), 3);
        assert_eq!(div_floor(10, -3), -4);
        assert_eq!(div_floor(-10, 3), -4);
        assert_eq!(div_floor(-10, -3), 3);
    }

    #[test]
    fn test_div_ceil() {
        assert_eq!(div_ceil(10, 3), 4);
        assert_eq!(div_ceil(10, -3), -3);
        assert_eq!(div_ceil(-10, 3), -3);
        assert_eq!(div_ceil(-10, -3), 4);
    }
}
