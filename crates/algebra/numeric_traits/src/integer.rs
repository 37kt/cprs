use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Rem, RemAssign, Shl, ShlAssign,
    Shr, ShrAssign,
};

use crate::Numeric;

pub trait Integer:
    Numeric
    + Eq
    + Ord
    + std::hash::Hash
    + Rem<Output = Self>
    + RemAssign
    + BitXor<Output = Self>
    + BitXorAssign
    + BitAnd<Output = Self>
    + BitAndAssign
    + BitOr<Output = Self>
    + BitOrAssign
    + Shl<usize, Output = Self>
    + ShlAssign<usize>
    + Shr<usize, Output = Self>
    + ShrAssign<usize>
{
    fn popcount(self) -> usize;
    fn msb_index(self) -> usize;
    fn lsb_index(self) -> usize;
    fn msb(self) -> Self;
    fn lsb(self) -> Self;
    fn ceil_pow2(self) -> Self;
    fn floor_pow2(self) -> Self;
    fn ceil_log2(self) -> usize;
    fn floor_log2(self) -> usize;
    fn checked_msb_index(self) -> Option<usize>;
    fn checked_lsb_index(self) -> Option<usize>;
    fn checked_ceil_pow2(self) -> Option<Self>;
    fn checked_floor_pow2(self) -> Option<Self>;
    fn checked_ceil_log2(self) -> Option<usize>;
    fn checked_floor_log2(self) -> Option<usize>;
    fn floor_div(self, other: Self) -> Self;
    fn ceil_div(self, other: Self) -> Self;
    fn gcd(self, other: Self) -> Self;
    fn lcm(self, other: Self) -> Self;
}

macro_rules! impl_integer {
    ($($t:ty),*) => {
        $(
            impl Integer for $t {
                fn popcount(self) -> usize {
                    self.count_ones() as usize
                }

                fn msb_index(self) -> usize {
                    (<$t>::BITS - 1 - self.leading_zeros()) as usize
                }

                fn lsb_index(self) -> usize {
                    self.trailing_zeros() as usize
                }

                fn msb(self) -> Self {
                    if self == 0 {
                        0
                    } else {
                        1 << self.msb_index()
                    }
                }

                fn lsb(self) -> Self {
                    self & self.wrapping_neg()
                }

                fn ceil_pow2(self) -> Self {
                    1 << self.ceil_log2()
                }

                fn floor_pow2(self) -> Self {
                    assert!(self > 0);
                    self.msb()
                }

                fn ceil_log2(self) -> usize {
                    assert!(self > 0);
                    (<$t>::BITS - (self - 1).leading_zeros()) as usize
                }

                fn floor_log2(self) -> usize {
                    assert!(self > 0);
                    self.msb_index()
                }

                fn checked_msb_index(self) -> Option<usize> {
                    if self == 0 {
                        None
                    } else {
                        Some(self.msb_index())
                    }
                }

                fn checked_lsb_index(self) -> Option<usize> {
                    if self == 0 {
                        None
                    } else {
                        Some(self.lsb_index())
                    }
                }

                fn checked_ceil_pow2(self) -> Option<Self> {
                    if self <= 0 {
                        None
                    } else {
                        Some(self.ceil_pow2())
                    }
                }

                fn checked_floor_pow2(self) -> Option<Self> {
                    if self <= 0 {
                        None
                    } else {
                        Some(self.floor_pow2())
                    }
                }

                fn checked_ceil_log2(self) -> Option<usize> {
                    if self <= 0 {
                        None
                    } else {
                        Some(self.ceil_log2())
                    }
                }

                fn checked_floor_log2(self) -> Option<usize> {
                    if self <= 0 {
                        None
                    } else {
                        Some(self.floor_log2())
                    }
                }

                #[allow(unused_comparisons)]
                fn floor_div(self, other: Self) -> Self {
                    self / other - if self ^ other < 0 && self % other != 0 {
                        1
                    } else {
                        0
                    }
                }

                #[allow(unused_comparisons)]
                fn ceil_div(self, other: Self) -> Self {
                    self / other + if self ^ other >= 0 && self % other != 0 {
                        1
                    } else {
                        0
                    }
                }

                #[allow(unused_comparisons)]
                fn gcd(self, other: Self) -> Self {
                    let mut x = if self < 0 { 0 - self } else { self };
                    let mut y = if other < 0 { 0 - other } else { other };
                    if x == 0 || y == 0 {
                        return x | y;
                    }
                    let n = x.trailing_zeros();
                    let m = y.trailing_zeros();
                    x >>= n;
                    y >>= m;
                    while x != y {
                        if x > y {
                            x -= y;
                            x >>= x.trailing_zeros();
                        } else {
                            y -= x;
                            y >>= y.trailing_zeros();
                        }
                    }
                    x << n.min(m)
                }

                fn lcm(self, other: Self) -> Self {
                    self / self.gcd(other) * other
                }
            }
        )*
    };
}

impl_integer! {
    u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize
}
