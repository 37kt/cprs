use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

const P: u64 = (1 << 61) - 1;

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct ModInt61(u64);

impl ModInt61 {
    #[inline]
    pub fn new(x: impl Into<ModInt61>) -> Self {
        x.into()
    }

    #[inline]
    pub fn raw(val: u64) -> Self {
        Self(val)
    }

    #[inline]
    pub fn val(self) -> u64 {
        self.0
    }

    #[inline]
    pub fn modulus() -> u64 {
        P
    }

    pub fn pow(mut self, mut k: usize) -> Self {
        let mut res = Self::raw(1);
        while k != 0 {
            if k & 1 != 0 {
                res *= self;
            }
            k >>= 1;
            self *= self;
        }
        res
    }

    pub fn inv(self) -> Self {
        self.pow(P as usize - 2)
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
            self
        } else {
            Self(P - self.0)
        }
    }
}

impl Neg for &ModInt61 {
    type Output = ModInt61;
    fn neg(self) -> Self::Output {
        -*self
    }
}

impl AddAssign for ModInt61 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        if self.0 >= P {
            self.0 -= P;
        }
    }
}

impl SubAssign for ModInt61 {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs;
    }
}

impl MulAssign for ModInt61 {
    fn mul_assign(&mut self, rhs: Self) {
        let t = self.0 as u128 * rhs.0 as u128;
        let t = (t >> 61) as u64 + (t as u64 & P);
        self.0 = if t >= P { t - P } else { t }
    }
}

impl DivAssign for ModInt61 {
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.inv();
    }
}

macro_rules! impl_from_integer {
    ($(($t1:ty, $t2:ty)),*) => {
        $(
            impl From<$t1> for ModInt61 {
                fn from(x: $t1) -> Self {
                    Self((x as $t2).rem_euclid(P as $t2) as u64)
                }
            }
        )*
    };
}

impl_from_integer!(
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
);

macro_rules! impl_ops {
    ($(
        $trait:ident,
        $trait_assign:ident,
        $fn:ident,
        $fn_assign:ident,
    )*) => {$(
        impl $trait_assign<&ModInt61> for ModInt61 {
            fn $fn_assign(&mut self, rhs: &ModInt61) {
                self.$fn_assign(*rhs);
            }
        }
        impl $trait<&ModInt61> for ModInt61 {
            type Output = ModInt61;
            fn $fn(mut self, rhs: &ModInt61) -> Self::Output {
                self.$fn_assign(*rhs);
                self
            }
        }
        impl $trait<ModInt61> for &ModInt61 {
            type Output = ModInt61;
            fn $fn(self, rhs: ModInt61) -> Self::Output {
                (*self).$fn(rhs)
            }
        }
        impl $trait<ModInt61> for ModInt61 {
            type Output = ModInt61;
            fn $fn(mut self, rhs: ModInt61) -> Self::Output {
                self.$fn_assign(rhs);
                self
            }
        }
        impl $trait<&ModInt61> for &ModInt61 {
            type Output = ModInt61;
            fn $fn(self, rhs: &ModInt61) -> Self::Output {
                (*self).$fn(&*rhs)
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
