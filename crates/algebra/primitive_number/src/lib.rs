use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Rem, RemAssign, ShlAssign, ShrAssign, Sub, SubAssign,
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
    const MIN: Self;
    const MAX: Self;
    fn abs(self) -> Self;
}

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
    const BITS: usize;
}

pub trait PrimitiveFloat: PrimitiveNumber {}

macro_rules! impl_primitive_signed_integer {
    ($($t:ty),*) => {
        $(
            impl PrimitiveNumber for $t {
                const ZERO: Self = 0;
                const ONE: Self = 1;
                const MIN: Self = Self::MIN;
                const MAX: Self = Self::MAX;
                fn abs(self) -> Self {
                    <$t>::abs(self)
                }
            }

            impl PrimitiveInteger for $t {
                const BITS: usize = <$t>::BITS as usize;
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
                const MIN: Self = Self::MIN;
                const MAX: Self = Self::MAX;
                fn abs(self) -> Self {
                    self
                }
            }

            impl PrimitiveInteger for $t {
                const BITS: usize = <$t>::BITS as usize;
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
                const MIN: Self = Self::MIN;
                const MAX: Self = Self::MAX;
                fn abs(self) -> Self {
                    <$t>::abs(self)
                }
            }

            impl PrimitiveFloat for $t {}
        )*
    }
}

impl_primitive_float!(f32, f64);
