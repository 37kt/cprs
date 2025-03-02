use std::{
    marker::PhantomData,
    num::ParseIntError,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
};

use barrett_reduction::BarrettReduction64;

use crate::{barrett_reduction::barrett_reduction, DynamicModInt64};

impl<Id> DynamicModInt64<Id> {
    pub fn new<T: Into<DynamicModInt64<Id>>>(x: T) -> Self {
        x.into()
    }

    pub fn from_raw(x: u64) -> Self {
        Self(x, PhantomData::default())
    }

    pub fn set_modulus(m: u64) {
        barrett_reduction::<Id, _>(|br| {
            br.replace(BarrettReduction64::new(m));
        });
    }

    pub fn modulus() -> u64 {
        barrett_reduction::<Id, _>(|br| br.get().modulus())
    }

    pub fn val(self) -> u64 {
        self.0
    }

    pub fn pow(self, exp: usize) -> Self {
        let mut res = Self::from(1);
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
        Self::from_raw(inv_mod(self.0, Self::modulus()))
    }

    pub fn sqrt(self) -> Option<Self> {
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
                t *= t;
                j += 1;
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

impl<Id> From<&DynamicModInt64<Id>> for DynamicModInt64<Id> {
    fn from(x: &DynamicModInt64<Id>) -> Self {
        *x
    }
}

impl<Id> FromStr for DynamicModInt64<Id> {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<i64>().map(Self::from)
    }
}

impl<Id> std::fmt::Display for DynamicModInt64<Id> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<Id> std::fmt::Debug for DynamicModInt64<Id> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<Id> Neg for DynamicModInt64<Id> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if self.0 == 0 {
            Self::from_raw(0)
        } else {
            Self::from_raw(Self::modulus() - self.0)
        }
    }
}

impl<Id> Neg for &DynamicModInt64<Id> {
    type Output = DynamicModInt64<Id>;

    fn neg(self) -> Self::Output {
        -*self
    }
}

impl<Id, T: Into<DynamicModInt64<Id>>> Add<T> for DynamicModInt64<Id> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        let mut x = self.0 + rhs.0;
        if x >= Self::modulus() {
            x -= Self::modulus();
        }
        Self::from_raw(x)
    }
}

impl<Id, T: Into<DynamicModInt64<Id>>> Sub<T> for DynamicModInt64<Id> {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        let mut x = self.0 + Self::modulus() - rhs.0;
        if x >= Self::modulus() {
            x -= Self::modulus();
        }
        Self::from_raw(x)
    }
}

impl<Id, T: Into<DynamicModInt64<Id>>> Mul<T> for DynamicModInt64<Id> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        barrett_reduction::<Id, _>(|br| Self::from_raw(br.get().mul(self.0, rhs.0)))
    }
}

impl<Id, T: Into<DynamicModInt64<Id>>> Div<T> for DynamicModInt64<Id> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        self * rhs.recip()
    }
}

macro_rules! impl_from_integer {
    ($(($t1:ty, $t2:ty)),*) => {
        $(
            impl<Id> From<$t1> for DynamicModInt64<Id> {
                fn from(x: $t1) -> Self {
                    Self::from_raw((x as $t2).rem_euclid(Self::modulus() as $t2) as u64)
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
        impl<Id, T: Into<DynamicModInt64<Id>>> $tr<T> for &DynamicModInt64<Id> {
            type Output = DynamicModInt64<Id>;

            fn $f(self, rhs: T) -> Self::Output {
                (*self).$f(rhs.into())
            }
        }

        impl<Id, T: Into<DynamicModInt64<Id>>> $tr_a<T> for DynamicModInt64<Id> {
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

const fn inv_mod(x: u64, m: u64) -> u64 {
    let (mut a, mut b, mut x, mut y) = (1, 0, x, m);
    if m == 1 {
        return 0;
    }

    loop {
        match x {
            0 => panic!("gcd(x, m) is not 1."),
            1 => return a,
            _ => {}
        }
        b += a * (y / x);
        y %= x;

        match y {
            0 => panic!("gcd(x, m) is not 1."),
            1 => return m - b,
            _ => {}
        }
        a += b * (x / y);
        x %= y;
    }
}
