use crate::{barrett_reduction, DynamicModInt};
use std::{
    marker::PhantomData,
    num::ParseIntError,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
};

impl<ID> DynamicModInt<ID> {
    pub fn new<T: Into<DynamicModInt<ID>>>(x: T) -> Self {
        x.into()
    }

    pub fn from_raw(x: u32) -> Self {
        Self(x, PhantomData::default())
    }

    pub fn set_modulus(m: u32) {
        barrett_reduction::barrett_reduction::<ID>().set_modulus(m);
    }

    pub fn modulus() -> u32 {
        barrett_reduction::barrett_reduction::<ID>().modulus()
    }

    pub fn val(self) -> u32 {
        self.0
    }

    pub fn pow(self, exp: usize) -> Self {
        let mut res = Self::from(1);
        let mut base = self;
        let mut exp = exp;
        while exp > 0 {
            if exp & 1 == 1 {
                res *= base;
            }
            base *= base;
            exp >>= 1;
        }
        res
    }

    pub fn recip(self) -> Self {
        Self::from_raw(inv_mod(self.0, Self::modulus()))
    }

    /// 制約: `MOD` は素数
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

impl<ID> From<&DynamicModInt<ID>> for DynamicModInt<ID> {
    fn from(x: &DynamicModInt<ID>) -> Self {
        *x
    }
}

impl<ID> FromStr for DynamicModInt<ID> {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<i64>().map(Self::from)
    }
}

impl<ID> std::fmt::Display for DynamicModInt<ID> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<ID> std::fmt::Debug for DynamicModInt<ID> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<ID> Neg for DynamicModInt<ID> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if self.0 == 0 {
            Self(0, PhantomData::default())
        } else {
            Self(Self::modulus() - self.0, PhantomData::default())
        }
    }
}

impl<ID> Neg for &DynamicModInt<ID> {
    type Output = DynamicModInt<ID>;

    fn neg(self) -> Self::Output {
        -*self
    }
}

impl<ID, T: Into<DynamicModInt<ID>>> Add<T> for DynamicModInt<ID> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        let mut x = self.0 + rhs.0;
        if x >= Self::modulus() {
            x -= Self::modulus();
        }
        Self(x, PhantomData::default())
    }
}

impl<ID, T: Into<DynamicModInt<ID>>> Sub<T> for DynamicModInt<ID> {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        if self.0 < rhs.0 {
            Self(Self::modulus() + self.0 - rhs.0, PhantomData::default())
        } else {
            Self(self.0 - rhs.0, PhantomData::default())
        }
    }
}

impl<ID, T: Into<DynamicModInt<ID>>> Mul<T> for DynamicModInt<ID> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        let barrett = barrett_reduction::barrett_reduction::<ID>();
        Self(barrett.mul(self.0, rhs.0), PhantomData::default())
    }
}

impl<ID, T: Into<DynamicModInt<ID>>> Div<T> for DynamicModInt<ID> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        self * rhs.into().recip()
    }
}

macro_rules! impl_from_integer {
    ($(($t1:ty, $t2:ty)),*) => {
        $(
            impl<ID> From<$t1> for DynamicModInt<ID> {
                fn from(x: $t1) -> Self {
                    Self((x as $t2).rem_euclid(Self::modulus() as $t2) as u32, PhantomData::default())
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
        impl<ID, T: Into<DynamicModInt<ID>>> $tr<T> for &DynamicModInt<ID> {
            type Output = DynamicModInt<ID>;

            fn $f(self, rhs: T) -> Self::Output {
                (*self).$f(rhs.into())
            }
        }

        impl<ID, T: Into<DynamicModInt<ID>>> $tr_a<T> for DynamicModInt<ID> {
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

const fn inv_mod(x: u32, m: u32) -> u32 {
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
