use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, ShlAssign, ShrAssign, Sub, SubAssign,
    },
};

pub trait PrimitiveNumber:
    Sized
    + Copy
    + Clone
    + Debug
    + Display
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
{
    const ZERO: Self;
    const ONE: Self;
    const MIN_VALUE: Self;
    const MAX_VALUE: Self;
    fn abs(self) -> Self;
}

pub trait PrimitiveSignedNumber: PrimitiveNumber + Neg<Output = Self> {}

pub trait PrimitiveInteger:
    PrimitiveNumber
    + Eq
    + Ord
    + Hash
    + Rem<Output = Self>
    + RemAssign
    + BitXor<Output = Self>
    + BitXorAssign
    + BitAnd<Output = Self>
    + BitAndAssign
    + BitOr<Output = Self>
    + BitOrAssign
    + ShlAssign
    + ShrAssign
{
    const BIT_LEN: usize;
    fn popcount(self) -> usize;
    fn msb_index(self) -> usize;
    fn lsb_index(self) -> usize;
    fn msb(self) -> Self;
    fn lsb(self) -> Self;
}

pub trait PrimitiveFloat: PrimitiveSignedNumber {}

macro_rules! impl_primitive_signed_integer {
    ($($t:ty),*) => {
        $(
            impl PrimitiveNumber for $t {
                const ZERO: Self = 0;
                const ONE: Self = 1;
                const MIN_VALUE: Self = Self::MIN;
                const MAX_VALUE: Self = Self::MAX;
                fn abs(self) -> Self {
                    <$t>::abs(self)
                }
            }

            impl PrimitiveSignedNumber for $t {}

            impl PrimitiveInteger for $t {
                const BIT_LEN: usize = <$t>::BITS as usize;
                fn popcount(self) -> usize {
                    <$t>::count_ones(self) as usize
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
    }
}

impl_primitive_signed_integer!(i8, i16, i32, i64, i128, isize);

macro_rules! impl_primitive_unsigned_integer {
    ($($t:ty),*) => {
        $(
            impl PrimitiveNumber for $t {
                const ZERO: Self = 0;
                const ONE: Self = 1;
                const MIN_VALUE: Self = Self::MIN;
                const MAX_VALUE: Self = Self::MAX;
                fn abs(self) -> Self {
                    self
                }
            }

            impl PrimitiveInteger for $t {
                const BIT_LEN: usize = <$t>::BITS as usize;
                fn popcount(self) -> usize {
                    <$t>::count_ones(self) as usize
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
    }
}

impl_primitive_unsigned_integer!(u8, u16, u32, u64, u128, usize);

macro_rules! impl_primitive_float {
    ($($t:ty),*) => {
        $(
            impl PrimitiveNumber for $t {
                const ZERO: Self = 0.0;
                const ONE: Self = 1.0;
                const MIN_VALUE: Self = Self::MIN;
                const MAX_VALUE: Self = Self::MAX;
                fn abs(self) -> Self {
                    <$t>::abs(self)
                }
            }

            impl PrimitiveSignedNumber for $t {}

            impl PrimitiveFloat for $t {}
        )*
    }
}

impl_primitive_float!(f32, f64);
