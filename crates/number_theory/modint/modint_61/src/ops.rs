use crate::{ModInt61, P};
use std::fmt::{Debug, Display};
use std::num::ParseIntError;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::str::FromStr;

impl ModInt61 {
    pub fn new<T: Into<ModInt61>>(x: T) -> Self {
        x.into()
    }

    pub fn from_raw(x: u64) -> Self {
        Self(x)
    }

    pub const fn modulus() -> u64 {
        P
    }

    pub fn val(self) -> u64 {
        self.0
    }

    pub fn pow(self, exp: usize) -> Self {
        let mut res = Self::from_raw(1);
        let mut base = self;
        let mut exp = exp;
        while exp > 0 {
            if exp % 2 == 1 {
                res *= base;
            }
            base *= base;
            exp /= 2;
        }
        res
    }

    pub fn recip(self) -> Self {
        self.pow(P as usize - 2)
    }

    pub fn sqrt(self) -> Option<Self> {
        let p = Self::modulus() as usize;
        if self.0 < 2 {
            return Some(self);
        } else if self.pow((p - 1) >> 1).val() != 1 {
            return None;
        }

        let mut b = Self::from_raw(1);
        while b.pow((p - 1) >> 1).val() == 1 {
            b += 1;
        }

        let mut e = (p - 1).trailing_zeros() as usize;
        let m = (p - 1) >> e;
        let mut x = self.pow((m - 1) >> 1);
        let mut y = self * x * x;
        x *= self;
        let mut z = b.pow(m);
        while y.val() != 1 {
            let mut j = 0;
            let mut t = y;
            while t.val() != 1 {
                t *= t;
                j += 1;
            }
            z = z.pow(1 << (e - j - 1));
            x *= z;
            z *= z;
            y *= z;
            e = j;
        }

        Some(x)
    }
}

impl From<&ModInt61> for ModInt61 {
    fn from(x: &ModInt61) -> Self {
        *x
    }
}

impl FromStr for ModInt61 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<i64>().map(Self::from)
    }
}

impl Display for ModInt61 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for ModInt61 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Neg for ModInt61 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if self.0 == 0 {
            Self::from_raw(0)
        } else {
            Self::from_raw(P - self.0)
        }
    }
}

impl Neg for &ModInt61 {
    type Output = ModInt61;

    fn neg(self) -> Self::Output {
        -*self
    }
}

impl<T: Into<ModInt61>> Add<T> for ModInt61 {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        let mut x = self.0 + rhs.0;
        if x >= P {
            x -= P;
        }
        Self::from_raw(x)
    }
}

impl<T: Into<ModInt61>> Sub<T> for ModInt61 {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        let mut x = self.0 + P - rhs.0;
        if x >= P {
            x -= P;
        }
        Self::from_raw(x)
    }
}

impl<T: Into<ModInt61>> Mul<T> for ModInt61 {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        let t = self.0 as u128 * rhs.0 as u128;
        let t = (t >> 61) as u64 + (t as u64 & P);
        Self::from_raw(if t >= P { t - P } else { t })
    }
}

impl<T: Into<ModInt61>> Div<T> for ModInt61 {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        self * rhs.recip()
    }
}

macro_rules! impl_from_integer {
    ($(($t1:ty, $t2:ty)),*) => {
        $(
            impl From<$t1> for ModInt61 {
                fn from(x: $t1) -> Self {
                    Self::from_raw((x as $t2).rem_euclid(Self::modulus() as $t2) as u64)
                }
            }

            impl From<&$t1> for ModInt61 {
                fn from(x: &$t1) -> Self {
                    Self::from_raw((*x as $t2).rem_euclid(Self::modulus() as $t2) as u64)
                }
            }
        )*
    };
}

impl_from_integer! {
    (i8, i64),
    (i16, i64),
    (i32, i64),
    (i64, i64),
    (isize, i64),
    (i128, i128),
    (u8, u64),
    (u16, u64),
    (u32, u64),
    (u64, u64),
    (usize, u64),
    (u128, u128)
}

macro_rules! impl_ops {
    ($(
        $tr:ident,
        $tr_a:ident,
        $f:ident,
        $f_a:ident,
    )*) => {$(
        impl<T: Into<ModInt61>> $tr<T> for &ModInt61 {
            type Output = ModInt61;

            fn $f(self, rhs: T) -> Self::Output {
                (*self).$f(rhs.into())
            }
        }

        impl<T: Into<ModInt61>> $tr_a<T> for ModInt61 {
            fn $f_a(&mut self, rhs: T) {
                *self = (*self).$f(rhs.into());
            }
        }
    )*};
}

impl_ops! {
    Add, AddAssign, add, add_assign,
    Sub, SubAssign, sub, sub_assign,
    Mul, MulAssign, mul, mul_assign,
    Div, DivAssign, div, div_assign,
}
