use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::zero_one;

pub trait Numeric:
    Sized
    + Clone
    + Copy
    + std::fmt::Debug
    + std::fmt::Display
    + PartialEq
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + zero_one::Zero
    + zero_one::One
{
}

macro_rules! impl_numeric {
    ($($t:ty),*) => {
        $(impl Numeric for $t {})*
    };
}

impl_numeric! {
    u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize, f32, f64
}
