use algebraic::{One, Zero};
use std::ops::{Add, BitXor, Div, Rem, Sub};

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
