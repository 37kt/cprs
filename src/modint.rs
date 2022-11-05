use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::str::FromStr;

pub trait Modulus: Copy + Clone + PartialEq {
    fn modulus() -> u32;
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Mod1000000007;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Mod998244353;

impl Modulus for Mod1000000007 {
    fn modulus() -> u32 {
        1_000_000_007
    }
}

impl Modulus for Mod998244353 {
    fn modulus() -> u32 {
        998_244_353
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ModInt<M: Modulus> {
    v: u32,
    _marker: PhantomData<fn() -> M>,
}

pub type ModInt1000000007 = ModInt<Mod1000000007>;
pub type ModInt998244353 = ModInt<Mod998244353>;

impl<M: Modulus> ModInt<M> {
    pub fn modulus() -> u32 {
        M::modulus()
    }
}

impl<M: Modulus> ModInt<M> {
    pub fn new(v: u32) -> Self {
        Self {
            v: v % Self::modulus(),
            _marker: PhantomData::default(),
        }
    }

    pub fn val(&self) -> u32 {
        self.v
    }

    pub fn pow(&self, exp: usize) -> Self {
        num_traits::pow(*self, exp)
    }

    // modulus が素数であることを前提にしている
    pub fn inv(&self) -> Self {
        assert_ne!(self.v, 0, "divide by zero");
        self.pow((Self::modulus() - 2) as usize)
    }
}

impl<M: Modulus> FromStr for ModInt<M> {
    type Err = <i128 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<i128>().map(|v| Self::from(v))
    }
}

impl<M: Modulus> Display for ModInt<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.val().to_string())
    }
}

macro_rules! impl_from_small_unsigned {
    ($($t:ty)*) => ($(
        impl<M:Modulus> From<$t> for ModInt<M> {
            fn from(v: $t) -> Self {
                Self::new(v as u32)
            }
        }
    )*)
}

macro_rules! impl_from_large_unsigned {
    ($($t:ty)*) => ($(
        impl<M:Modulus> From<$t> for ModInt<M> {
            fn from(v: $t) -> Self {
                Self::new((v % (Self::modulus() as $t)) as u32)
            }
        }
    )*)
}

macro_rules! impl_from_small_signed {
    ($($t:ty)*) => ($(
        impl<M:Modulus> From<$t> for ModInt<M> {
            fn from(v: $t) -> Self {
                if v < 0 {
                    -Self::new(v.abs() as u32)
                } else {
                    Self::new(v as u32)
                }
            }
        }
    )*)
}

macro_rules! impl_from_large_signed {
    ($($t:ty)*) => ($(
        impl<M:Modulus> From<$t> for ModInt<M> {
            fn from(v: $t) -> Self {
                if v < 0 {
                    -Self::from(v.abs() as u128)
                } else {
                    Self::from(v as u128)
                }
            }
        }
    )*)
}

impl_from_small_unsigned!(u8 u16 u32);
impl_from_large_unsigned!(u64 u128 usize);
impl_from_small_signed!(i8 i16 i32);
impl_from_large_signed!(i64 i128 isize);

impl<M: Modulus> std::default::Default for ModInt<M> {
    fn default() -> Self {
        Self::new(0)
    }
}

impl<M: Modulus> num_traits::Zero for ModInt<M> {
    fn is_zero(&self) -> bool {
        self.v == 0
    }

    fn set_zero(&mut self) {
        self.v = 0
    }

    fn zero() -> Self {
        Self::new(0)
    }
}

impl<M: Modulus> num_traits::One for ModInt<M> {
    fn is_one(&self) -> bool {
        self.v == 1
    }

    fn set_one(&mut self) {
        self.v = 1
    }

    fn one() -> Self {
        Self::new(1)
    }
}

impl<M: Modulus> Neg for ModInt<M> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        if self.v == 0 {
            self
        } else {
            Self::new(Self::modulus() - self.v)
        }
    }
}

impl<M: Modulus> Add for ModInt<M> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<M: Modulus> AddAssign for ModInt<M> {
    fn add_assign(&mut self, rhs: Self) {
        self.v += rhs.v;
        if self.v >= Self::modulus() {
            self.v -= Self::modulus();
        }
    }
}

impl<M: Modulus> Sub for ModInt<M> {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<M: Modulus> SubAssign for ModInt<M> {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs;
    }
}

impl<M: Modulus> Mul for ModInt<M> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(((self.v as u64) * (rhs.v as u64) % (Self::modulus() as u64)) as u32)
    }
}

impl<M: Modulus> MulAssign for ModInt<M> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<M: Modulus> Div for ModInt<M> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

impl<M: Modulus> DivAssign for ModInt<M> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

macro_rules! impl_op {
    ($t:ty, $b:ident, $f:ident) => {
        impl<M: Modulus> $b<$t> for ModInt<M> {
            type Output = Self;
            fn $f(self, rhs: $t) -> Self::Output {
                self.$f(Self::from(rhs))
            }
        }
    };
}

macro_rules! impl_op_assign {
    ($t:ty, $b:ident, $f:ident) => {
        impl<M: Modulus> $b<$t> for ModInt<M> {
            fn $f(&mut self, rhs: $t) {
                self.$f(Self::from(rhs))
            }
        }
    };
}

macro_rules! impl_op_prim {
    ($($t:ty)*) => ($(
        impl_op!($t, Add, add);
        impl_op!($t, Sub, sub);
        impl_op!($t, Mul, mul);
        impl_op!($t, Div, div);
        impl_op_assign!($t, AddAssign, add_assign);
        impl_op_assign!($t, SubAssign, sub_assign);
        impl_op_assign!($t, MulAssign, mul_assign);
        impl_op_assign!($t, DivAssign, div_assign);
    )*)
}

impl_op_prim!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_i8_test() {
        let x: i8 = -21;
        let y = ModInt998244353::from(x);
        assert_eq!(y.v, 998244332);
    }

    #[test]
    fn pow_test() {
        let x = ModInt1000000007::new(2);
        assert_eq!(x.pow(90000000).v, 99049511);
    }

    #[test]
    fn operator_primitive_test() {
        let mut x = ModInt998244353::new(5);
        x += 2_i128;
        assert_eq!(x.v, 7);
    }
}
