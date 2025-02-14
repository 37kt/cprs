use std::{
    convert::TryInto,
    fmt::Debug,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    sync::atomic::{AtomicU64, Ordering::SeqCst},
};

use algebraic::{One, Zero};

struct Montgomery {
    m: AtomicU64,
    r: AtomicU64,
    n2: AtomicU64,
}

impl Montgomery {
    const fn new() -> Self {
        Self {
            m: AtomicU64::new(0),
            r: AtomicU64::new(0),
            n2: AtomicU64::new(0),
        }
    }

    fn set(&self, m: u64) {
        assert!(m < 1 << 62);
        assert!(m & 1 != 0);
        if self.m.load(SeqCst) == m {
            return;
        }
        let n2 = ((m as u128).wrapping_neg() % m as u128) as u64;
        let mut r = m;
        for _ in 0..5 {
            r = r.wrapping_mul(2u64.wrapping_sub(m.wrapping_mul(r)));
        }
        assert!(r.wrapping_mul(m) == 1);
        self.m.store(m, SeqCst);
        self.r.store(r, SeqCst);
        self.n2.store(n2, SeqCst);
    }

    fn reduce(&self, x: u128) -> u64 {
        let r = self.r.load(SeqCst);
        let m = self.m.load(SeqCst);
        (x.wrapping_add(
            ((x as u64).wrapping_mul(r.wrapping_neg()) as u128).wrapping_mul(m as u128),
        ) >> 64) as u64
    }
}

static MONTGOMERY: Montgomery = Montgomery::new();

#[derive(Default, Clone, Copy)]
pub struct MontgomeryModInt(u64);

impl MontgomeryModInt {
    pub fn set_modulus(m: u64) {
        MONTGOMERY.set(m);
    }

    pub fn modulus() -> u64 {
        MONTGOMERY.m.load(SeqCst)
    }

    pub fn new(x: u64) -> Self {
        Self(
            MONTGOMERY.reduce(
                (x as u128)
                    .wrapping_add(Self::modulus() as u128)
                    .wrapping_mul(MONTGOMERY.n2.load(SeqCst) as u128),
            ),
        )
    }

    pub fn pow(mut self, k: impl TryInto<u128, Error = impl Debug>) -> Self {
        let mut k: u128 = k.try_into().unwrap();
        let mut r = Self::new(1);
        while k > 0 {
            if k & 1 != 0 {
                r *= self;
            }
            self *= self;
            k >>= 1;
        }
        r
    }

    pub fn val(self) -> u64 {
        let x = MONTGOMERY.reduce(self.0 as u128);
        let m = Self::modulus();
        if x >= m {
            x - m
        } else {
            x
        }
    }
}

impl Neg for MontgomeryModInt {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(0) - self
    }
}

impl AddAssign<Self> for MontgomeryModInt {
    fn add_assign(&mut self, rhs: Self) {
        let m = Self::modulus();
        self.0 = self.0.wrapping_add(rhs.0.wrapping_sub(m * 2));
        if (self.0 as i64) < 0 {
            self.0 = self.0.wrapping_add(m * 2);
        }
    }
}

impl SubAssign<Self> for MontgomeryModInt {
    fn sub_assign(&mut self, rhs: Self) {
        let m = Self::modulus();
        self.0 = self.0.wrapping_sub(rhs.0);
        if (self.0 as i64) < 0 {
            self.0 = self.0.wrapping_add(m * 2);
        }
    }
}

impl MulAssign<Self> for MontgomeryModInt {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = MONTGOMERY.reduce(self.0 as u128 * rhs.0 as u128);
    }
}

impl Add<Self> for MontgomeryModInt {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl Sub<Self> for MontgomeryModInt {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl Mul<Self> for MontgomeryModInt {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl PartialEq for MontgomeryModInt {
    fn eq(&self, other: &Self) -> bool {
        let m = Self::modulus();
        (if self.0 >= m { self.0 - m } else { self.0 })
            == (if other.0 >= m { other.0 - m } else { other.0 })
    }
}

impl Eq for MontgomeryModInt {}

impl Zero for MontgomeryModInt {
    fn zero() -> Self {
        Self(0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl One for MontgomeryModInt {
    fn one() -> Self {
        Self(1)
    }

    fn is_one(&self) -> bool {
        self == &Self::new(1)
    }
}
