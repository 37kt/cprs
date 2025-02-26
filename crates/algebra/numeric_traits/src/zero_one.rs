use std::ops::{Add, Mul};

pub trait Zero: Sized + Add<Output = Self> {
    fn zero() -> Self;
}

pub trait One: Sized + Mul<Output = Self> {
    fn one() -> Self;
}

macro_rules! impl_zero_one {
    ($($t:ty),*) => {
        $(
            impl Zero for $t {
                fn zero() -> Self {
                    0 as $t
                }
            }

            impl One for $t {
                fn one() -> Self {
                    1 as $t
                }
            }
        )*
    }
}

impl_zero_one!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
