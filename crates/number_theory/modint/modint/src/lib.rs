pub trait ModInt:
    Sized
    + Default
    + Clone
    + Copy
    + PartialEq
    + Eq
    + std::hash::Hash
    + std::fmt::Debug
    + std::fmt::Display
    + std::str::FromStr
    + From<i8>
    + From<i16>
    + From<i32>
    + From<i64>
    + From<i128>
    + From<isize>
    + From<u8>
    + From<u16>
    + From<u32>
    + From<u64>
    + From<u128>
    + From<usize>
    + std::ops::Neg<Output = Self>
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::AddAssign
    + std::ops::SubAssign
    + std::ops::MulAssign
    + std::ops::DivAssign
{
    type Value;

    fn modulus() -> Self::Value;
    fn from_raw(val: Self::Value) -> Self;
    fn val(self) -> Self::Value;
    fn recip(self) -> Self;
    fn pow(self, exp: usize) -> Self;
    fn sqrt(self) -> Option<Self>;
}

pub trait ModInt32: ModInt<Value = u32> {
    fn modulus() -> u32 {
        <Self as ModInt>::modulus()
    }

    fn from_raw(val: u32) -> Self {
        <Self as ModInt>::from_raw(val)
    }

    fn val(self) -> u32 {
        <Self as ModInt>::val(self)
    }

    fn recip(self) -> Self {
        <Self as ModInt>::recip(self)
    }

    fn pow(self, exp: usize) -> Self {
        <Self as ModInt>::pow(self, exp)
    }

    fn sqrt(self) -> Option<Self> {
        <Self as ModInt>::sqrt(self)
    }
}

impl<T: ModInt<Value = u32>> ModInt32 for T {}

pub trait ModInt64: ModInt<Value = u64> {
    fn modulus() -> u64 {
        <Self as ModInt>::modulus()
    }

    fn from_raw(val: u64) -> Self {
        <Self as ModInt>::from_raw(val)
    }

    fn val(self) -> u64 {
        <Self as ModInt>::val(self)
    }

    fn recip(self) -> Self {
        <Self as ModInt>::recip(self)
    }

    fn pow(self, exp: usize) -> Self {
        <Self as ModInt>::pow(self, exp)
    }

    fn sqrt(self) -> Option<Self> {
        <Self as ModInt>::sqrt(self)
    }
}

impl<T: ModInt<Value = u64>> ModInt64 for T {}
