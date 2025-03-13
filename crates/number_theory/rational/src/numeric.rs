use numeric_traits::{Inf, NegInf, One, Recip, Signed, Zero};

use crate::Rational;

impl<const AUTO_REDUCE: bool> Inf for Rational<AUTO_REDUCE> {
    fn inf() -> Self {
        Self::from_raw(1, 0)
    }
}

impl<const AUTO_REDUCE: bool> NegInf for Rational<AUTO_REDUCE> {
    fn neg_inf() -> Self {
        Self::from_raw(-1, 0)
    }
}

impl<const AUTO_REDUCE: bool> Zero for Rational<AUTO_REDUCE> {
    fn zero() -> Self {
        Self::from_raw(0, 1)
    }
}

impl<const AUTO_REDUCE: bool> One for Rational<AUTO_REDUCE> {
    fn one() -> Self {
        Self::from_raw(1, 1)
    }
}

impl<const AUTO_REDUCE: bool> Recip for Rational<AUTO_REDUCE> {
    fn recip(mut self) -> Self {
        std::mem::swap(&mut self.num, &mut self.den);
        if self.den < 0 {
            self.num = -self.num;
            self.den = -self.den;
        }
        self
    }
}

impl<const AUTO_REDUCE: bool> Signed for Rational<AUTO_REDUCE> {
    fn signum(self) -> Self {
        if self.num < 0 {
            Self::from_raw(-1, 1)
        } else if self.num > 0 {
            Self::from_raw(1, 1)
        } else if self.den == 0 {
            Self::from_raw(0, 0)
        } else {
            Self::from_raw(0, 1)
        }
    }
}
