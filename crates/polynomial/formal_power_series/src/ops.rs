use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

use binomial::BinomialPrime;
use modint::ModInt;

use crate::FormalPowerSeries;

impl<M: ModInt<Value = u32>> FormalPowerSeries<M> {
    pub fn diff(&self) -> Self {
        self.iter()
            .enumerate()
            .skip(1)
            .map(|(i, &v)| M::from_raw(i as _) * v)
            .collect()
    }

    pub fn integral(&self) -> Self {
        let n = self.len();

        let mut bin = BinomialPrime::<M>::new();
        bin.expand(n);

        Self::from_fn(n + 1, |i| {
            if i == 0 {
                M::from_raw(0)
            } else {
                bin.inv(i) * self[i - 1]
            }
        })
    }
}

impl<M: ModInt<Value = u32>> Neg for FormalPowerSeries<M> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.iter_mut().for_each(|v| *v = -*v);
        self
    }
}

impl<M: ModInt<Value = u32>> Neg for &FormalPowerSeries<M> {
    type Output = FormalPowerSeries<M>;

    fn neg(self) -> Self::Output {
        self.iter().map(|v| -*v).collect()
    }
}

impl<M: ModInt<Value = u32>> AddAssign<&FormalPowerSeries<M>> for FormalPowerSeries<M> {
    fn add_assign(&mut self, rhs: &FormalPowerSeries<M>) {
        if self.len() < rhs.len() {
            self.resize(rhs.len(), M::from_raw(0));
        }
        self.iter_mut().zip(rhs).for_each(|(a, b)| *a += *b);
    }
}

impl<M: ModInt<Value = u32>> AddAssign for FormalPowerSeries<M> {
    fn add_assign(&mut self, rhs: Self) {
        self.add_assign(&rhs);
    }
}

impl<M: ModInt<Value = u32>> Add for FormalPowerSeries<M> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.add_assign(&rhs);
        self
    }
}

impl<M: ModInt<Value = u32>> Add<&FormalPowerSeries<M>> for FormalPowerSeries<M> {
    type Output = Self;

    fn add(mut self, rhs: &FormalPowerSeries<M>) -> Self::Output {
        self.add_assign(rhs);
        self
    }
}

impl<M: ModInt<Value = u32>> Add<FormalPowerSeries<M>> for &FormalPowerSeries<M> {
    type Output = FormalPowerSeries<M>;

    fn add(self, mut rhs: FormalPowerSeries<M>) -> Self::Output {
        rhs.add_assign(self);
        rhs
    }
}

impl<M: ModInt<Value = u32>> Add for &FormalPowerSeries<M> {
    type Output = FormalPowerSeries<M>;

    fn add(self, rhs: &FormalPowerSeries<M>) -> Self::Output {
        self.clone().add(rhs)
    }
}

impl<M: ModInt<Value = u32>> SubAssign<&FormalPowerSeries<M>> for FormalPowerSeries<M> {
    fn sub_assign(&mut self, rhs: &FormalPowerSeries<M>) {
        if self.len() < rhs.len() {
            self.resize(rhs.len(), M::from_raw(0));
        }
        self.iter_mut().zip(rhs).for_each(|(a, b)| *a -= *b);
    }
}

impl<M: ModInt<Value = u32>> SubAssign for FormalPowerSeries<M> {
    fn sub_assign(&mut self, rhs: Self) {
        self.sub_assign(&rhs);
    }
}

impl<M: ModInt<Value = u32>> Sub for FormalPowerSeries<M> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.sub_assign(&rhs);
        self
    }
}

impl<M: ModInt<Value = u32>> Sub<&FormalPowerSeries<M>> for FormalPowerSeries<M> {
    type Output = Self;

    fn sub(mut self, rhs: &FormalPowerSeries<M>) -> Self::Output {
        self.sub_assign(rhs);
        self
    }
}

impl<M: ModInt<Value = u32>> Sub<FormalPowerSeries<M>> for &FormalPowerSeries<M> {
    type Output = FormalPowerSeries<M>;

    fn sub(self, mut rhs: FormalPowerSeries<M>) -> Self::Output {
        if rhs.len() < self.len() {
            rhs.resize(self.len(), M::from_raw(0));
        }
        rhs.iter_mut().zip(self).for_each(|(a, b)| *a = *b - *a);
        rhs
    }
}

impl<M: ModInt<Value = u32>> Sub for &FormalPowerSeries<M> {
    type Output = FormalPowerSeries<M>;

    fn sub(self, rhs: &FormalPowerSeries<M>) -> Self::Output {
        self.clone().sub(rhs)
    }
}
