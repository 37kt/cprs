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
                    1 << self.msb_index()
                }

                fn lsb(self) -> Self {
                    self & self.wrapping_neg()
                }
            }
        )*
    };
}

impl_integer! {
    u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize
}
