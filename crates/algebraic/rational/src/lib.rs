use std::{
    fmt::{Debug, Display},
    mem::swap,
    ops::{AddAssign, DivAssign, MulAssign, Neg, SubAssign},
    sync::atomic::{AtomicBool, Ordering::SeqCst},
};

type Z = i64;

static AUTO_REDUCE: AtomicBool = AtomicBool::new(true);

#[derive(Clone, Copy)]
pub struct Rational {
    num: Z,
    den: Z,
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

impl Rational {
    pub fn set_auto_reduce(auto_reduce: bool) {
        AUTO_REDUCE.store(auto_reduce, SeqCst);
    }

    pub fn new(mut num: Z, mut den: Z) -> Self {
        if den < 0 {
            num = -num;
            den = -den;
        }
        let mut res = Self { num, den };
        if AUTO_REDUCE.load(SeqCst) {
            res.reduce();
        }
        res
    }

    pub fn num(&self) -> Z {
        self.num
    }

    pub fn den(&self) -> Z {
        self.den
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
        if AUTO_REDUCE.load(SeqCst) {
            self.reduce();
        }
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
        if AUTO_REDUCE.load(SeqCst) {
            self.reduce();
        }
    }
}

impl DivAssign<Self> for Rational {
    fn div_assign(&mut self, rhs: Self) {
        self.num *= rhs.den;
        self.den *= rhs.num;
        if self.den < 0 {
            self.num = -self.num;
            self.den = -self.den;
        }
        if AUTO_REDUCE.load(SeqCst) {
            self.reduce();
        }
    }
}
