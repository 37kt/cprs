use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use numeric_traits::{Integer, Recip};

use crate::Rational;

impl<const AUTO_REDUCE: bool> Neg for Rational<AUTO_REDUCE> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from_raw(-self.num, self.den)
    }
}

impl<const AUTO_REDUCE: bool> Neg for &Rational<AUTO_REDUCE> {
    type Output = Rational<AUTO_REDUCE>;

    fn neg(self) -> Self::Output {
        -*self
    }
}

impl<const AUTO_REDUCE: bool> Add for Rational<AUTO_REDUCE> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if AUTO_REDUCE {
            let g = self.den.gcd(rhs.den);
            if g == 0 {
                if self.is_nan() || rhs.is_nan() {
                    return Self::nan();
                } else {
                    return Self::from_raw((self.num.signum() + rhs.num.signum()).signum(), 0);
                }
            }
            let num = self.num * (rhs.den / g) + rhs.num * (self.den / g);
            let den = self.den / g * rhs.den;
            Self::new(num, den)
        } else {
            let num = self.num * rhs.den + rhs.num * self.den;
            let den = self.den * rhs.den;
            Self::from_raw(num, den)
        }
    }
}

impl<const AUTO_REDUCE: bool> Sub for Rational<AUTO_REDUCE> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl<const AUTO_REDUCE: bool> Mul for Rational<AUTO_REDUCE> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let (g1, g2) = if AUTO_REDUCE {
            (self.num.gcd(rhs.den), self.den.gcd(rhs.num))
        } else {
            (1, 1)
        };
        if g1 == 0 || g2 == 0 {
            return Self::nan();
        }
        let num = (self.num / g1) * (rhs.num / g2);
        let den = (self.den / g2) * (rhs.den / g1);
        Self::from_raw(num, den)
    }
}

impl<const AUTO_REDUCE: bool> Div for Rational<AUTO_REDUCE> {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.recip()
    }
}

impl<const AUTO_REDUCE: bool> PartialEq for Rational<AUTO_REDUCE> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl<const AUTO_REDUCE: bool> Eq for Rational<AUTO_REDUCE> {}

// https://misawa.github.io/others/avoid_errors/compare_fractions.html
fn compare<const AUTO_REDUCE: bool>(
    mut a: Rational<AUTO_REDUCE>,
    mut b: Rational<AUTO_REDUCE>,
) -> Ordering {
    if a.num <= 0 || b.num <= 0 {
        if a.num == 0 || b.num == 0 || ((a.num < 0) ^ (b.num < 0)) {
            return a.num.cmp(&b.num);
        } else {
            return compare(-b, -a);
        }
    }
    let ord = (a.num / a.den).cmp(&(b.num / b.den));
    if !ord.is_eq() {
        return ord;
    }
    a.num %= a.den;
    b.num %= b.den;
    if a.num == 0 || b.num == 0 {
        (a.num * b.den).cmp(&(a.den * b.num))
    } else {
        compare(b.recip(), a.recip())
    }
}

impl<const AUTO_REDUCE: bool> PartialOrd for Rational<AUTO_REDUCE> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_nan() || other.is_nan() {
            return None;
        }
        let sgn = |x: Rational<AUTO_REDUCE>| {
            if x.is_inf() {
                1
            } else if x.is_neg_inf() {
                -1
            } else {
                0
            }
        };
        let sgn_l = sgn(*self);
        let sgn_r = sgn(*other);
        if sgn_l == 0 && sgn_r == 0 {
            return Some(compare(*self, *other));
        }
        match sgn_l.cmp(&sgn_r) {
            Ordering::Equal => None,
            res => Some(res),
        }
    }
}

impl<const AUTO_REDUCE: bool> Ord for Rational<AUTO_REDUCE> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

macro_rules! impl_ops {
    ($(
        $tr:ident,
        $tr_a:ident,
        $f:ident,
        $f_a:ident,
    )*) => {$(
        impl<const AUTO_REDUCE: bool> $tr<&Rational<AUTO_REDUCE>> for Rational<AUTO_REDUCE> {
            type Output = Rational<AUTO_REDUCE>;

            fn $f(self, rhs: &Rational<AUTO_REDUCE>) -> Self::Output {
                self.$f(*rhs)
            }
        }

        impl<const AUTO_REDUCE: bool> $tr<Rational<AUTO_REDUCE>> for &Rational<AUTO_REDUCE> {
            type Output = Rational<AUTO_REDUCE>;

            fn $f(self, rhs: Rational<AUTO_REDUCE>) -> Self::Output {
                (*self).$f(rhs)
            }
        }

        impl<const AUTO_REDUCE: bool> $tr<&Rational<AUTO_REDUCE>> for &Rational<AUTO_REDUCE> {
            type Output = Rational<AUTO_REDUCE>;

            fn $f(self, rhs: &Rational<AUTO_REDUCE>) -> Self::Output {
                (*self).$f(*rhs)
            }
        }

        impl<const AUTO_REDUCE: bool> $tr_a<Rational<AUTO_REDUCE>> for Rational<AUTO_REDUCE> {
            fn $f_a(&mut self, rhs: Rational<AUTO_REDUCE>) {
                *self = (*self).$f(rhs);
            }
        }

        impl<const AUTO_REDUCE: bool> $tr_a<&Rational<AUTO_REDUCE>> for Rational<AUTO_REDUCE> {
            fn $f_a(&mut self, rhs: &Rational<AUTO_REDUCE>) {
                *self = (*self).$f(*rhs);
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
