use std::fmt::Debug;

use numeric_traits::Integer;

mod numeric;
mod ops;

#[derive(Clone, Copy)]
pub struct Rational<const AUTO_REDUCE: bool> {
    pub num: i64,
    pub den: i64,
}

impl<const AUTO_REDUCE: bool> Default for Rational<AUTO_REDUCE> {
    fn default() -> Self {
        Self { num: 0, den: 1 }
    }
}

impl<const AUTO_REDUCE: bool> Debug for Rational<AUTO_REDUCE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

impl From<Rational<true>> for Rational<false> {
    fn from(value: Rational<true>) -> Self {
        Self::new(value.num, value.den)
    }
}

impl From<Rational<false>> for Rational<true> {
    fn from(value: Rational<false>) -> Self {
        Self::new(value.num, value.den)
    }
}

impl<const AUTO_REDUCE: bool> From<i64> for Rational<AUTO_REDUCE> {
    fn from(value: i64) -> Self {
        Self::new(value, 1)
    }
}

impl<const AUTO_REDUCE: bool> From<Rational<AUTO_REDUCE>> for f64 {
    fn from(r: Rational<AUTO_REDUCE>) -> Self {
        r.num as f64 / r.den as f64
    }
}

impl<const AUTO_REDUCE: bool> Rational<AUTO_REDUCE> {
    pub fn new(mut num: i64, mut den: i64) -> Self {
        if den < 0 {
            num = -num;
            den = -den;
        }
        let mut res = Self { num, den };
        if AUTO_REDUCE {
            res.reduce();
        }
        res
    }

    pub fn from_raw(num: i64, den: i64) -> Self {
        Self { num, den }
    }

    pub fn reduce(&mut self) {
        let g = self.num.gcd(self.den);
        self.num /= g;
        self.den /= g;
    }

    pub fn floor(&self) -> i64 {
        self.num.floor_div(self.den)
    }

    pub fn ceil(&self) -> i64 {
        self.num.ceil_div(self.den)
    }

    pub fn is_nan(&self) -> bool {
        self.num == 0 && self.den == 0
    }

    pub fn is_inf(&self) -> bool {
        self.num > 0 && self.den == 0
    }

    pub fn is_neg_inf(&self) -> bool {
        self.num < 0 && self.den == 0
    }

    pub fn nan() -> Self {
        Self::from_raw(0, 0)
    }
}
