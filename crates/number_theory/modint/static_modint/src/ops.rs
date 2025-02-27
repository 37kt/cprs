use std::{
    iter::{Product, Sum},
    num::ParseIntError,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
};

use crate::{
    mod_arithmetic::{inv_mod, mul_mod, pow_mod},
    StaticModInt,
};

impl<const MOD: u32> StaticModInt<MOD> {
    pub fn new<T: Into<StaticModInt<MOD>>>(x: T) -> Self {
        x.into()
    }

    pub const fn from_raw(x: u32) -> Self {
        Self(x)
    }

    pub const fn modulus() -> u32 {
        MOD
    }

    pub const fn val(self) -> u32 {
        self.0
    }

    pub const fn pow(self, exp: usize) -> Self {
        Self::from_raw(pow_mod(self.0, exp as u32, MOD))
    }

    pub const fn recip(self) -> Self {
        if Self::IS_PRIME {
            self.pow(MOD as usize - 2)
        } else {
            Self::from_raw(inv_mod(self.0, MOD))
        }
    }

    pub fn sqrt(self) -> Option<Self> {
        assert!(Self::IS_PRIME);

        let p = Self::modulus() as usize;
        if self.0 < 2 {
            return Some(self);
        } else if self.pow(p - 1 >> 1).val() != 1 {
            return None;
        }

        let mut b = Self::from_raw(1);
        while b.pow(p - 1 >> 1).val() == 1 {
            b += 1;
        }

        let mut e = (p - 1).trailing_zeros() as usize;
        let m = p - 1 >> e;
        let mut x = self.pow(m - 1 >> 1);
        let mut y = self * x * x;
        x *= self;
        let mut z = b.pow(m);
        while y.val() != 1 {
            let mut j = 0;
            let mut t = y;
            while t.val() != 1 {
                j += 1;
                t *= t;
            }
            z = z.pow(1 << e - j - 1);
            x *= z;
            z *= z;
            y *= z;
            e = j;
        }

        Some(x)
    }
}

impl<const MOD: u32> From<&StaticModInt<MOD>> for StaticModInt<MOD> {
    fn from(x: &StaticModInt<MOD>) -> Self {
        *x
    }
}

impl<const MOD: u32> FromStr for StaticModInt<MOD> {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<i64>().map(Self::from)
    }
}

impl<const MOD: u32> std::fmt::Display for StaticModInt<MOD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const MOD: u32> std::fmt::Debug for StaticModInt<MOD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const MOD: u32> Neg for StaticModInt<MOD> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if self.0 == 0 {
            Self(0)
        } else {
            Self(MOD - self.0)
        }
    }
}

impl<const MOD: u32> Neg for &StaticModInt<MOD> {
    type Output = StaticModInt<MOD>;

    fn neg(self) -> Self::Output {
        -*self
    }
}
impl<const MOD: u32, T: Into<StaticModInt<MOD>>> Add<T> for StaticModInt<MOD> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        let mut x = self.0 + rhs.0;
        if x >= MOD {
            x -= MOD;
        }
        Self(x)
    }
}

impl<const MOD: u32, T: Into<StaticModInt<MOD>>> Sub<T> for StaticModInt<MOD> {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        if self.0 < rhs.0 {
            Self(MOD + self.0 - rhs.0)
        } else {
            Self(self.0 - rhs.0)
        }
    }
}

impl<const MOD: u32, T: Into<StaticModInt<MOD>>> Mul<T> for StaticModInt<MOD> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self(mul_mod(self.0, rhs.0, MOD))
    }
}

impl<const MOD: u32, T: Into<StaticModInt<MOD>>> Div<T> for StaticModInt<MOD> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        self * rhs.into().recip()
    }
}

macro_rules! impl_from_integer {
    ($(($t1:ty, $t2:ty)),*) => {
        $(
            impl<const MOD: u32> From<$t1> for StaticModInt<MOD> {
                fn from(x: $t1) -> Self {
                    Self((x as $t2).rem_euclid(MOD as $t2) as u32)
                }
            }

            impl<const MOD: u32> From<&$t1> for StaticModInt<MOD> {
                fn from(x: &$t1) -> Self {
                    Self((*x as $t2).rem_euclid(MOD as $t2) as u32)
                }
            }
        )*
    };
}

impl_from_integer! {
    (i8, i32),
    (i16, i32),
    (i32, i32),
    (i64, i64),
    (isize, i64),
    (i128, i128),
    (u8, u32),
    (u16, u32),
    (u32, u32),
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
        impl<const MOD: u32, T: Into<StaticModInt<MOD>>> $tr<T> for &StaticModInt<MOD> {
            type Output = StaticModInt<MOD>;

            fn $f(self, rhs: T) -> Self::Output {
                (*self).$f(rhs)
            }
        }

        impl<const MOD: u32, T: Into<StaticModInt<MOD>>> $tr_a<T> for StaticModInt<MOD> {
            fn $f_a(&mut self, rhs: T) {
                *self = (*self).$f(rhs);
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

impl<T: Into<StaticModInt<MOD>>, const MOD: u32> Sum<T> for StaticModInt<MOD> {
    fn sum<I: Iterator<Item = T>>(iter: I) -> Self {
        iter.fold(Self::from_raw(0), |b, x| b + x.into())
    }
}

impl<T: Into<StaticModInt<MOD>>, const MOD: u32> Product<T> for StaticModInt<MOD> {
    fn product<I: Iterator<Item = T>>(iter: I) -> Self {
        iter.fold(Self::new(1), |b, x| b * x.into())
    }
}
