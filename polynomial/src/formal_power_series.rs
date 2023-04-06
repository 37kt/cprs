use std::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub,
    SubAssign,
};

use ac_library_rs::ModInt998244353 as M;

use crate::convolution;

const P: usize = 998_244_353;

#[derive(Clone)]
pub struct FormalPowerSeries(pub Vec<M>);

impl Deref for FormalPowerSeries {
    type Target = Vec<M>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FormalPowerSeries {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for FormalPowerSeries {
    fn default() -> Self {
        FormalPowerSeries(vec![])
    }
}

impl FormalPowerSeries {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn shrink(&mut self) {
        while self.len() > 0 && self.last().unwrap().val() == 0 {
            self.pop();
        }
    }

    pub fn eval(&self, x: M) -> M {
        let mut r = M::new(0);
        let mut w = M::new(1);
        for &v in &self.0 {
            r += w * v;
            w *= x;
        }
        r
    }

    pub fn pre(&self, sz: usize) -> Self {
        Self(self[0..self.len().min(sz)].to_vec())
    }

    pub fn diff_inplace(&mut self) {
        if self.len() > 0 {
            for i in 1..self.len() {
                self[i - 1] = self[i] * i;
            }
            self.pop();
        }
    }

    pub fn diff(&self) -> Self {
        let mut r = self.clone();
        r.diff_inplace();
        r
    }

    pub fn integral_inplace(&mut self) {
        let n = self.len();
        let mut inv = vec![M::new(1); n + 1];
        for i in 2..=n {
            inv[i] = (inv[P % i] * (P - P / i)).into();
        }
        self.push(0.into());
        for i in (1..=n).rev() {
            self[i] = self[i - 1] * inv[i];
        }
        self[0] = 0.into();
    }

    pub fn integral(&self) -> Self {
        let mut r = self.clone();
        r.integral_inplace();
        r
    }

    pub fn rev_inplace(&mut self, deg: usize) {
        self.resize(deg, M::new(0));
        self.reverse();
    }

    pub fn rev(&self, deg: usize) -> Self {
        let mut r = self.clone();
        r.rev_inplace(deg);
        r
    }

    pub fn dot_inplace(&mut self, o: &Self) {
        self.truncate(o.len());
        for i in 0..self.len() {
            self[i] *= o[i];
        }
    }

    pub fn dot(&self, o: &Self) -> Self {
        let mut r = self.clone();
        r.dot_inplace(o);
        r
    }

    pub fn inv(&self, deg: usize) -> Self {
        assert!(self[0].val() != 0);
        let mut r = FormalPowerSeries(vec![self[0].inv()]);
        for i in 0.. {
            if 1 << i >= deg {
                break;
            }
            let mut a = &r + &r;
            let mut b = &r * &r;
            b *= &self.pre(1 << i + 1);
            a -= &b;
            a.truncate(1 << i + 1);
            r = a;
        }
        r.truncate(deg);
        r
    }
}

impl Neg for &FormalPowerSeries {
    type Output = FormalPowerSeries;
    fn neg(self) -> Self::Output {
        let mut r = self.clone();
        for i in 0..r.len() {
            r[i] = -r[i];
        }
        r
    }
}

impl AddAssign<&FormalPowerSeries> for FormalPowerSeries {
    fn add_assign(&mut self, rhs: &FormalPowerSeries) {
        let n = self.len();
        self.resize(n.max(rhs.len()), M::new(0));
        for i in 0..rhs.len() {
            self[i] += rhs[i];
        }
    }
}

impl Add<&FormalPowerSeries> for &FormalPowerSeries {
    type Output = FormalPowerSeries;
    fn add(self, rhs: &FormalPowerSeries) -> Self::Output {
        let mut r = self.clone();
        r += rhs;
        r
    }
}

impl SubAssign<&FormalPowerSeries> for FormalPowerSeries {
    fn sub_assign(&mut self, rhs: &FormalPowerSeries) {
        let n = self.len();
        self.resize(n.max(rhs.len()), M::new(0));
        for i in 0..rhs.len() {
            self[i] -= rhs[i];
        }
    }
}

impl Sub<&FormalPowerSeries> for &FormalPowerSeries {
    type Output = FormalPowerSeries;
    fn sub(self, rhs: &FormalPowerSeries) -> Self::Output {
        let mut r = self.clone();
        r -= rhs;
        r
    }
}

impl MulAssign<M> for FormalPowerSeries {
    fn mul_assign(&mut self, rhs: M) {
        for i in 0..self.len() {
            self[i] *= rhs;
        }
    }
}

impl Mul<M> for &FormalPowerSeries {
    type Output = FormalPowerSeries;
    fn mul(self, rhs: M) -> Self::Output {
        let mut r = self.clone();
        r *= rhs;
        r
    }
}

impl Mul<&FormalPowerSeries> for &FormalPowerSeries {
    type Output = FormalPowerSeries;
    fn mul(self, rhs: &FormalPowerSeries) -> Self::Output {
        if self.is_empty() || rhs.is_empty() {
            FormalPowerSeries::new()
        } else {
            FormalPowerSeries(convolution(self, rhs))
        }
    }
}

impl MulAssign<&FormalPowerSeries> for FormalPowerSeries {
    fn mul_assign(&mut self, rhs: &FormalPowerSeries) {
        *self = &*self * rhs;
    }
}

impl Div<&FormalPowerSeries> for &FormalPowerSeries {
    type Output = FormalPowerSeries;
    fn div(self, rhs: &FormalPowerSeries) -> Self::Output {
        if self.len() < rhs.len() {
            FormalPowerSeries::new()
        } else {
            let n = self.len() + 1 - rhs.len();
            let mut a = &self.rev(self.len()).pre(n) * &rhs.rev(rhs.len()).inv(n);
            a.truncate(n);
            a.rev_inplace(n);
            a
        }
    }
}

impl DivAssign<&FormalPowerSeries> for FormalPowerSeries {
    fn div_assign(&mut self, rhs: &FormalPowerSeries) {
        *self = &*self / rhs;
    }
}

impl RemAssign<&FormalPowerSeries> for FormalPowerSeries {
    fn rem_assign(&mut self, rhs: &FormalPowerSeries) {
        *self -= &(&(&*self / rhs) * rhs);
    }
}

impl Rem<&FormalPowerSeries> for &FormalPowerSeries {
    type Output = FormalPowerSeries;
    fn rem(self, rhs: &FormalPowerSeries) -> Self::Output {
        let mut r = self.clone();
        r %= rhs;
        r
    }
}
