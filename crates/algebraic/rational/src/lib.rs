use std::{
    fmt::{Debug, Display},
    mem::swap,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    sync::atomic::{AtomicBool, Ordering::SeqCst},
};

type Z = i128;

static AUTO_REDUCE: AtomicBool = AtomicBool::new(true);

#[derive(Clone, Copy)]
pub struct Rational {
    pub num: Z,
    pub den: Z,
}

fn gcd(mut a: Z, mut b: Z) -> Z {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        a %= b;
        swap(&mut a, &mut b);
    }
    a
}

impl Default for Rational {
    fn default() -> Self {
        Self { num: 0, den: 1 }
    }
}

impl Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

impl Debug for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

impl From<Z> for Rational {
    fn from(x: Z) -> Self {
        Self { num: x, den: 1 }
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        self.num * other.den == other.num * self.den
    }

    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}

impl Eq for Rational {}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.num * other.den).cmp(&(other.num * self.den))
    }
}

impl Rational {
    pub fn set_auto_reduce(auto_reduce: bool) {
        AUTO_REDUCE.store(auto_reduce, SeqCst);
    }

    pub fn new(num: Z, den: Z) -> Self {
        let mut res = Self { num, den };
        res.normalize();
        res
    }

    pub fn raw(num: Z, den: Z) -> Self {
        Self { num, den }
    }

    pub fn num(&self) -> Z {
        self.num
    }

    pub fn den(&self) -> Z {
        self.den
    }

    pub fn abs(&self) -> Self {
        Self {
            num: self.num.abs(),
            den: self.den,
        }
    }

    pub fn normalize(&mut self) {
        assert!(self.num != 0 || self.den != 0);
        if self.den == 0 {
            self.num = if self.num > 0 { 1 } else { -1 };
            self.den = 0;
            return;
        }
        if self.den < 0 {
            self.num = -self.num;
            self.den = -self.den;
        }
        if self.num == 0 {
            self.den = 1;
        }
        if AUTO_REDUCE.load(SeqCst) {
            self.reduce();
        }
    }

    pub fn reduce(&mut self) {
        let g = gcd(self.num, self.den);
        self.num /= g;
        self.den /= g;
    }
}

impl Neg for Rational {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            num: -self.num,
            den: self.den,
        }
    }
}

impl Neg for &Rational {
    type Output = Rational;
    fn neg(self) -> Self::Output {
        Rational {
            num: -self.num,
            den: self.den,
        }
    }
}

impl AddAssign<Self> for Rational {
    fn add_assign(&mut self, rhs: Self) {
        self.num = self.num * rhs.den + rhs.num * self.den;
        self.den *= rhs.den;
        self.normalize();
    }
}

impl SubAssign<Self> for Rational {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs;
    }
}

impl MulAssign<Self> for Rational {
    fn mul_assign(&mut self, rhs: Self) {
        self.num *= rhs.num;
        self.den *= rhs.den;
        self.normalize();
    }
}

impl DivAssign<Self> for Rational {
    fn div_assign(&mut self, rhs: Self) {
        self.num *= rhs.den;
        self.den *= rhs.num;
        self.normalize();
    }
}

macro_rules! impl_ops {
    ($(
        $trait:ident,
        $trait_assign:ident,
        $fn:ident,
        $fn_assign:ident,
    )*) => {$(
        impl $trait_assign<&Rational> for Rational {
            fn $fn_assign(&mut self, rhs: &Rational) {
                self.$fn_assign(*rhs);
            }
        }
        impl<T: Into<Rational>> $trait<T> for Rational {
            type Output = Rational;
            fn $fn(mut self, rhs: T) -> Self::Output {
                self.$fn_assign(rhs.into());
                self
            }
        }
        impl $trait<&Rational> for Rational {
            type Output = Rational;
            fn $fn(self, rhs: &Rational) -> Self::Output {
                self.$fn(*rhs)
            }
        }
        impl<T: Into<Rational>> $trait<T> for &Rational {
            type Output = Rational;
            fn $fn(self, rhs: T) -> Self::Output {
                (*self).$fn(rhs.into())
            }
        }
        impl $trait<&Rational> for &Rational {
            type Output = Rational;
            fn $fn(self, rhs: &Rational) -> Self::Output {
                (*self).$fn(*rhs)
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
