use std::ops::{Add, Div, Sub};

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

/// [l..r] で最小値をとる x と f(x) を返す
pub fn ternary_search<I, T>(mut l: I, mut r: I, mut f: impl FnMut(I) -> T) -> (I, T)
where
    I: Copy + Add<Output = I> + Sub<Output = I> + Div<Output = I> + Zero + One + PartialOrd,
    T: Copy + PartialOrd,
{
    assert!(l <= r);
    let one = I::one();
    let two = one + one;
    let three = two + one;
    while l + two < r {
        let m1 = (l + l + r) / three;
        let m2 = (l + r + r) / three;
        if f(m1) < f(m2) {
            r = m2;
        } else {
            l = m1;
        }
    }
    let mut mn = f(l);
    let mut i = l;
    if l + one <= r && f(l + one) < mn {
        mn = f(l + one);
        i = l + one;
    }
    if l + two <= r && f(l + two) < mn {
        mn = f(l + two);
        i = l + two;
    }
    (i, mn)
}

/// [l..r] で最小値をとる x と f(x) を返す
pub fn ternary_search_f64<T>(mut l: f64, mut r: f64, mut f: impl FnMut(f64) -> T) -> (f64, T)
where
    T: Copy + PartialOrd,
{
    for _ in 0..100 {
        let m1 = (l * 2.0 + r) / 3.0;
        let m2 = (l + 2.0 * r) / 3.0;
        if f(m1) < f(m2) {
            r = m2;
        } else {
            l = m1;
        }
    }
    (l, f(l))
}
