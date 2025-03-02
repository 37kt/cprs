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
