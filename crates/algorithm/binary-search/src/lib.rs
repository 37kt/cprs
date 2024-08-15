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

/// [l..r) で f(x) = false となる最小の x を返す
/// ない場合は r を返す
pub fn binary_search<I>(mut l: I, mut r: I, mut f: impl FnMut(I) -> bool) -> I
where
    I: Copy + Add<Output = I> + Sub<Output = I> + Div<Output = I> + Zero + One + PartialOrd,
{
    let one = I::one();
    let two = one + one;
    if !f(l) {
        return l;
    }
    while l + one < r {
        let m = (l + r) / two;
        if f(m) {
            l = m;
        } else {
            r = m;
        }
    }
    r
}

/// [l..r] で f(x) = false となる最小の x を返す
pub fn binary_search_f64(mut l: f64, mut r: f64, mut f: impl FnMut(f64) -> bool) -> f64 {
    for _ in 0..100 {
        let m = (l + r) / 2.0;
        if f(m) {
            l = m;
        } else {
            r = m;
        }
    }
    r
}
