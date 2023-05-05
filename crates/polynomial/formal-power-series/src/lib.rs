use convolution_arbitrary_mod::convolution_arbitrary_mod;
use convolution_ntt_friendly::{convolution_ntt_friendly, ntt, ntt_inv};
use modint::{NttInfo, StaticModInt};
use std::{
    iter::repeat,
    mem::swap,
    ops::{
        Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Shl,
        ShlAssign, Shr, ShrAssign, Sub, SubAssign,
    },
};

#[derive(Default, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct FormalPowerSeries<const P: u32>(pub Vec<StaticModInt<P>>);

#[derive(Default, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct SparseFormalPowerSeries<const P: u32>(pub Vec<(usize, StaticModInt<P>)>);

pub type FormalPowerSeries998244353 = FormalPowerSeries<998_244_353>;
pub type FormalPowerSeries1000000007 = FormalPowerSeries<1_000_000_007>;
pub type SparseFormalPowerSeries998244353 = SparseFormalPowerSeries<998_244_353>;
pub type SparseFormalPowerSeries1000000007 = SparseFormalPowerSeries<1_000_000_007>;

#[macro_export]
macro_rules! fps {
    ($($x:expr), *) => (
        $crate::FormalPowerSeries(vec![$(modint::StaticModInt::from($x)), *])
    );
    ($x:expr; $n:expr) => (
        $crate::FormalPowerSeries(vec![modint::StaticModInt::from($x); $n])
    );
}

#[macro_export]
macro_rules! sfps {
    ($(($d:expr, $x:expr)), *) => (
        $crate::SparseFormalPowerSeries(vec![$(($d, modint::StaticModInt::from($x))), *])
    );
}

impl<const P: u32> FormalPowerSeries<P> {
    pub fn shrink(&mut self) {
        while self.last() == Some(&0.into()) {
            self.pop();
        }
    }

    pub fn pre(mut self, d: usize) -> Self {
        self.truncate(d);
        self
    }

    pub fn eval(&self, x: StaticModInt<P>) -> StaticModInt<P> {
        let mut r = 0.into();
        let mut w = StaticModInt::new(1);
        for &v in &self.0 {
            r += w * v;
            w *= x;
        }
        r
    }

    pub fn differential(&self) -> Self {
        Self(
            self.iter()
                .enumerate()
                .skip(1)
                .map(|(i, v)| v * i)
                .collect(),
        )
    }

    pub fn integral(&self) -> Self {
        let n = self.len();
        let mut res = fps![0; n + 1];
        if n > 0 {
            res[1] = 1.into();
        }
        let m = StaticModInt::<P>::modulus() as usize;
        for i in 2..=n {
            res[i] = -res[m % i] * (m / i);
        }
        for i in 0..n {
            res[i + 1] *= self[i];
        }
        res
    }

    pub fn inv(&self, d: usize) -> Self {
        assert_ne!(self[0].val(), 0);
        if <() as NttInfo<P>>::IS_NTT_FRIENDLY {
            let mut res = fps![0; d];
            res[0] = self[0].inv();
            for k in 0.. {
                let k = 1 << k;
                if k >= d {
                    break;
                }
                let mut f = Self(self.iter().take(k * 2).map(|&x| x).collect());
                f.resize(k * 2, 0.into());
                let mut g = Self(res.iter().take(k).map(|&x| x).collect());
                g.resize(k * 2, 0.into());
                ntt(&mut f);
                ntt(&mut g);
                for (a, b) in f.iter_mut().zip(g.iter()) {
                    *a *= b;
                }
                ntt_inv(&mut f);
                for a in f.iter_mut().take(k) {
                    *a = 0.into();
                }
                ntt(&mut f);
                for (a, b) in f.iter_mut().zip(g.iter()) {
                    *a *= b;
                }
                ntt_inv(&mut f);
                for (a, b) in res.iter_mut().zip(f.iter()).skip(k) {
                    *a = -b;
                }
            }
            res.truncate(d);
            res
        } else {
            let mut res = fps![self[0].inv()];
            for k in 0.. {
                if k >= d {
                    break;
                }
                let k = 1 << k;
                res = (&res + &res - &res * &res * res.pre(k * 2)).pre(k * 2);
            }
            res.truncate(d);
            res
        }
    }

    pub fn log(&self, d: usize) -> Self {
        assert!(self[0].val() == 1);
        (self.differential() * self.inv(d)).pre(d - 1).integral()
    }

    pub fn exp(&self, d: usize) -> Self {
        assert!(self.len() == 0 || self[0].val() == 0);
        if <() as NttInfo<P>>::IS_NTT_FRIENDLY {
            let mut b = fps![1, if self.len() > 1 { self[1] } else { 0.into() }];
            let mut c = fps![1];
            let mut z1;
            let mut z2 = fps![1, 1];
            for m in 1.. {
                let m = 1 << m;
                if m >= d {
                    break;
                }
                let mut y = b.clone();
                y.resize(m * 2, 0.into());
                ntt(&mut y);
                z1 = z2;
                let mut z = Self((0..m).map(|i| y[i] * z1[i]).collect());
                ntt_inv(&mut z);
                for v in z.iter_mut().take(m / 2) {
                    *v = 0.into();
                }
                ntt(&mut z);
                for i in 0..m {
                    z[i] *= -z1[i];
                }
                ntt_inv(&mut z);
                c.append(&mut z.drain(m / 2..).collect());
                z2 = c.clone();
                z2.resize(m * 2, 0.into());
                ntt(&mut z2);
                let mut x: Self = self.clone().pre(m);
                x.resize(m, 0.into());
                x = x.differential();
                x.push(0.into());
                ntt(&mut x);
                for i in 0..m {
                    x[i] *= y[i];
                }
                ntt_inv(&mut x);
                x -= b.differential();
                x.resize(m * 2, 0.into());
                for i in 0..m - 1 {
                    x[m + i] = x[i];
                    x[i] = 0.into();
                }
                ntt(&mut x);
                for i in 0..m * 2 {
                    x[i] *= z2[i];
                }
                ntt_inv(&mut x);
                x.pop();
                x = x.integral();
                for i in m..self.len().min(m * 2) {
                    x[i] += self[i];
                }
                for v in x.iter_mut().take(m) {
                    *v = 0.into();
                }
                ntt(&mut x);
                for i in 0..m * 2 {
                    x[i] *= y[i];
                }
                ntt_inv(&mut x);
                b.append(&mut x.drain(m..).collect());
            }
            b.pre(d)
        } else {
            let mut res = fps![1];
            for i in 0.. {
                let i = 1 << i;
                if i >= d {
                    break;
                }
                let mut t = self.clone().pre(i << 1);
                t[0] += 1;
                t -= res.log(i << 1);
                res = (res * t).pre(i << 1);
            }
            res.pre(d)
        }
    }

    pub fn pow(&self, k: usize, d: usize) -> FormalPowerSeries<P> {
        let n = self.len();
        if k == 0 {
            let mut res = fps![0; d];
            if d > 0 {
                res[0] = 1.into();
            }
            return res;
        }
        for i in 0..n {
            if self[i].val() != 0 {
                let c = self[i].inv();
                let mut res = (((self * c) >> i).log(d) * StaticModInt::new(k)).exp(d);
                res *= self[i].pow(k);
                res = (res << i * k).pre(d);
                if res.len() < d {
                    res.resize(d, 0.into());
                }
                return res;
            }
            if i + 1 >= d / k {
                return fps![0; d];
            }
        }
        fps![0; d]
    }
}

impl<const P: u32> SparseFormalPowerSeries<P> {
    pub fn normalize(&mut self) {
        if self.len() == 0 {
            return;
        }
        self.0.sort_by_key(|&(i, _)| i);
        let mut res = Self(vec![(self[0].0, StaticModInt::new(0))]);
        for &(i, v) in &self.0 {
            if res.len() == 0 || res.last().unwrap().0 != i {
                res.push((i, v));
            } else {
                res.last_mut().unwrap().1 += v;
            }
        }
        if res.len() != 0 && res.last().unwrap().1.val() == 0 {
            res.pop();
        }
        *self = res;
    }

    pub fn differential(&self) -> Self {
        Self(
            self.iter()
                .filter_map(|&(i, v)| (i > 0).then(|| (i - 1, v * i)))
                .collect(),
        )
    }

    pub fn integral(&self) -> Self {
        Self(self.iter().map(|&(i, v)| (i + 1, v / (i + 1))).collect())
    }

    pub fn inv(self, d: usize) -> FormalPowerSeries<P> {
        let mut f = fps![0; d];
        f[0] += 1;
        f /= self;
        f
    }

    pub fn log(self, d: usize) -> FormalPowerSeries<P> {
        assert!(self[0].0 == 0 && self[0].1.val() == 1);
        let f = self.differential();
        let mut res = (self.inv(d) * f).pre(d - 1).integral();
        res.resize(d, 0.into());
        res
    }

    pub fn exp(&self, d: usize) -> FormalPowerSeries<P> {
        if self.len() == 0 {
            let mut res = fps![0; d];
            if d > 0 {
                res[0] = 1.into();
            }
            return res;
        }
        assert_ne!(self[0].0, 0);
        let mut res = fps![0; d];
        if d == 0 {
            return res;
        }
        let mut a = self.differential();
        for (d, _) in a.iter_mut() {
            *d += 1;
        }
        res[0] = 1.into();
        let mut inv = vec![StaticModInt::<P>::new(1); d];
        let m = StaticModInt::<P>::modulus() as usize;
        for i in 1..d {
            if i > 1 {
                inv[i] = -inv[m % i] * (m / i);
            }
            res[i] = a
                .iter()
                .filter_map(|&(j, v)| (i >= j).then(|| v * res[i - j]))
                .sum::<StaticModInt<P>>()
                * inv[i];
        }
        res
    }

    pub fn pow(&self, k: usize, d: usize) -> FormalPowerSeries<P> {
        let offset = self.iter().position(|&(_, v)| v.val() != 0);
        let mut res = fps![0; d];
        if offset.is_none() {
            if k == 0 {
                res[0] += 1;
            }
            return res;
        }
        let offset = offset.unwrap();
        if self[offset].0 > 0 {
            let deg = self[offset].0;
            if k > (d - 1) / deg {
                return res;
            }
            let g = Self(
                self.iter()
                    .filter_map(|&(i, v)| (i >= deg).then(|| (i - deg, v)))
                    .collect(),
            );
            let t = g.pow(k, d - k * deg);
            for i in 0..d - k * deg {
                res[k * deg + i] = t[i];
            }
            return res;
        }
        let mut inv = vec![StaticModInt::<P>::new(1); d + 1];
        let m = P as usize;
        for i in 2..=d {
            inv[i] = -inv[m % i] * (m / i);
        }
        res[0] = self[0].1.pow(k);
        let c = self[0].1.inv();
        for i in 1..d {
            for &(j, v) in self.iter().skip(1).filter(|&&(j, _)| i >= j) {
                res[i] = res[i] + v * res[i - j] * (StaticModInt::<P>::new(k) * j - (i - j));
            }
            res[i] *= inv[i] * c;
        }
        res
    }
}

impl<const P: u32> Deref for FormalPowerSeries<P> {
    type Target = Vec<StaticModInt<P>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const P: u32> Deref for SparseFormalPowerSeries<P> {
    type Target = Vec<(usize, StaticModInt<P>)>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const P: u32> DerefMut for FormalPowerSeries<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const P: u32> DerefMut for SparseFormalPowerSeries<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const P: u32> From<Vec<StaticModInt<P>>> for FormalPowerSeries<P> {
    fn from(v: Vec<StaticModInt<P>>) -> Self {
        Self(v)
    }
}

impl<const P: u32> From<Vec<(usize, StaticModInt<P>)>> for SparseFormalPowerSeries<P> {
    fn from(v: Vec<(usize, StaticModInt<P>)>) -> Self {
        Self(v)
    }
}

impl<const P: u32> From<SparseFormalPowerSeries<P>> for FormalPowerSeries<P> {
    fn from(f: SparseFormalPowerSeries<P>) -> Self {
        if f.len() == 0 {
            return fps![];
        }
        let mut g = FormalPowerSeries(vec![0.into(); f.last().unwrap().0 + 1]);
        for (i, v) in f.0 {
            g[i] += v;
        }
        g
    }
}

impl<const P: u32> From<FormalPowerSeries<P>> for SparseFormalPowerSeries<P> {
    fn from(f: FormalPowerSeries<P>) -> Self {
        Self(f.0.into_iter().enumerate().collect())
    }
}

impl<const P: u32> Neg for FormalPowerSeries<P> {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        for v in self.iter_mut() {
            *v = -*v;
        }
        self
    }
}

impl<const P: u32> Neg for SparseFormalPowerSeries<P> {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        for (_, v) in self.iter_mut() {
            *v = -*v;
        }
        self
    }
}

impl<const P: u32> Neg for &FormalPowerSeries<P> {
    type Output = FormalPowerSeries<P>;
    fn neg(self) -> Self::Output {
        -self.clone()
    }
}

impl<const P: u32> Neg for &SparseFormalPowerSeries<P> {
    type Output = SparseFormalPowerSeries<P>;
    fn neg(self) -> Self::Output {
        -self.clone()
    }
}

impl<const P: u32> MulAssign<StaticModInt<P>> for FormalPowerSeries<P> {
    fn mul_assign(&mut self, rhs: StaticModInt<P>) {
        for v in self.iter_mut() {
            *v *= rhs;
        }
    }
}

impl<const P: u32> MulAssign<StaticModInt<P>> for SparseFormalPowerSeries<P> {
    fn mul_assign(&mut self, rhs: StaticModInt<P>) {
        for (_, v) in self.iter_mut() {
            *v *= rhs;
        }
    }
}

impl<const P: u32> DivAssign<StaticModInt<P>> for FormalPowerSeries<P> {
    fn div_assign(&mut self, rhs: StaticModInt<P>) {
        *self *= rhs.inv();
    }
}

impl<const P: u32> DivAssign<StaticModInt<P>> for SparseFormalPowerSeries<P> {
    fn div_assign(&mut self, rhs: StaticModInt<P>) {
        *self *= rhs.inv();
    }
}

impl<const P: u32> AddAssign<Self> for FormalPowerSeries<P> {
    fn add_assign(&mut self, rhs: Self) {
        if self.len() < rhs.len() {
            self.resize(rhs.len(), 0.into());
        }
        self.iter_mut().zip(rhs.iter()).for_each(|(a, b)| *a += b);
    }
}

impl<const P: u32> AddAssign<Self> for SparseFormalPowerSeries<P> {
    fn add_assign(&mut self, rhs: Self) {
        let mut res = sfps![];
        let n = self.len();
        let m = rhs.len();
        let mut i = 0;
        let mut j = 0;
        while i < n && j < m {
            if j == m {
                res.push(self[i]);
                i += 1;
            } else if i == n {
                res.push(rhs[j]);
                j += 1;
            } else if self[i].0 == rhs[j].0 {
                res.push((self[i].0, self[i].1 + rhs[j].1));
                i += 1;
                j += 1;
            } else if self[i].0 < rhs[j].0 {
                res.push(self[i]);
                i += 1;
            } else {
                res.push(rhs[j]);
                j += 1;
            }
        }
        *self = res;
    }
}

impl<const P: u32> AddAssign<SparseFormalPowerSeries<P>> for FormalPowerSeries<P> {
    fn add_assign(&mut self, rhs: SparseFormalPowerSeries<P>) {
        if rhs.len() == 0 {
            return;
        }
        let m = rhs.last().unwrap().0 + 1;
        if self.len() < m {
            self.resize(m, 0.into());
        }
        for (i, v) in rhs.0 {
            self[i] += v;
        }
    }
}

impl<const P: u32> SubAssign<Self> for FormalPowerSeries<P> {
    fn sub_assign(&mut self, rhs: Self) {
        if self.len() < rhs.len() {
            self.resize(rhs.len(), 0.into());
        }
        self.iter_mut().zip(rhs.iter()).for_each(|(a, b)| *a -= b);
    }
}

impl<const P: u32> SubAssign<Self> for SparseFormalPowerSeries<P> {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs;
    }
}

impl<const P: u32> SubAssign<SparseFormalPowerSeries<P>> for FormalPowerSeries<P> {
    fn sub_assign(&mut self, rhs: SparseFormalPowerSeries<P>) {
        *self += -rhs;
    }
}

impl<const P: u32> MulAssign<Self> for FormalPowerSeries<P> {
    fn mul_assign(&mut self, rhs: Self) {
        if <() as NttInfo<P>>::IS_NTT_FRIENDLY {
            let mut a = vec![];
            swap(&mut a, &mut self.0);
            self.0 = convolution_ntt_friendly(a, rhs.0);
        } else {
            self.0 = convolution_arbitrary_mod(&self.0, &rhs.0);
        }
    }
}

impl<const P: u32> MulAssign<Self> for SparseFormalPowerSeries<P> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = self
            .0
            .drain(..)
            .map(|(i, x)| rhs.iter().map(move |(j, y)| (i + j, x * y)))
            .flatten()
            .collect::<Vec<_>>();
        self.normalize();
    }
}

impl<const P: u32> MulAssign<SparseFormalPowerSeries<P>> for FormalPowerSeries<P> {
    fn mul_assign(&mut self, rhs: SparseFormalPowerSeries<P>) {
        if self.len() == 0 || rhs.len() == 0 {
            self.clear();
            return;
        }
        let n = self.len();
        self.resize(n + rhs.last().unwrap().0, 0.into());
        let c = if rhs[0].0 == 0 { rhs[0].1 } else { 0.into() };
        for i in (0..n).rev() {
            for &(j, v) in rhs.iter().rev() {
                if j == 0 {
                    continue;
                }
                self[i + j] = self[i + j] + self[i] * v;
            }
            self[i] *= c;
        }
    }
}

impl<const P: u32> DivAssign<Self> for FormalPowerSeries<P> {
    fn div_assign(&mut self, mut g: Self) {
        if self.len() < g.len() {
            self.clear();
            return;
        }
        let n = self.len() + 1 - g.len();
        if g.len() <= 64 {
            let mut f = FormalPowerSeries(vec![]);
            swap(&mut f.0, &mut self.0);
            g.shrink();
            let coef = g.last().unwrap().inv();
            for x in g.iter_mut() {
                *x *= coef;
            }
            let d = f.len() + 1 - g.len();
            let m = g.len();
            let mut quo = FormalPowerSeries(vec![StaticModInt::new(0); d]);
            for i in (0..d).rev() {
                quo[i] = f[i + m - 1];
                for j in 0..m {
                    f[i + j] -= quo[i] * g[j];
                }
            }
            *self = quo * coef;
            self.resize(n, 0.into());
        } else {
            self.reverse();
            self.truncate(n);
            g.reverse();
            *self *= g.inv(n);
            self.truncate(n);
            self.reverse();
        }
    }
}

impl<const P: u32> DivAssign<SparseFormalPowerSeries<P>> for FormalPowerSeries<P> {
    fn div_assign(&mut self, mut rhs: SparseFormalPowerSeries<P>) {
        assert_eq!(rhs[0].0, 0);
        let c = rhs[0].1.inv();
        self.iter_mut().for_each(|v| *v *= c);
        rhs.iter_mut().for_each(|(_, v)| *v *= c);
        for i in 0..self.len() {
            for &(j, v) in rhs.iter().filter(|&&(j, _)| j > 0 && i >= j) {
                self[i] = self[i] - self[i - j] * v;
            }
        }
    }
}

impl<const P: u32> RemAssign<Self> for FormalPowerSeries<P> {
    fn rem_assign(&mut self, rhs: Self) {
        *self -= self.clone() / &rhs * &rhs;
        self.shrink();
    }
}

impl<const P: u32> ShlAssign<usize> for FormalPowerSeries<P> {
    fn shl_assign(&mut self, rhs: usize) {
        self.0 = repeat(0.into()).take(rhs).chain(self.0.drain(..)).collect();
    }
}

impl<const P: u32> ShlAssign<usize> for SparseFormalPowerSeries<P> {
    fn shl_assign(&mut self, rhs: usize) {
        self.iter_mut().for_each(|(i, _)| *i += rhs);
    }
}

impl<const P: u32> ShrAssign<usize> for FormalPowerSeries<P> {
    fn shr_assign(&mut self, rhs: usize) {
        self.0 = self.0.drain(rhs..).collect();
    }
}

impl<const P: u32> ShrAssign<usize> for SparseFormalPowerSeries<P> {
    fn shr_assign(&mut self, rhs: usize) {
        self.0 = self
            .0
            .drain(..)
            .filter_map(|(i, v)| (i >= rhs).then(|| (i - rhs, v)))
            .collect();
    }
}

macro_rules! impl_ops {
    ($(
        $ty_l:ty,
        $ty_r:ty,
        $trait:ident,
        $trait_assign:ident,
        $fn:ident,
        $fn_assign:ident,
    )*) => {$(
        impl<const P: u32> $trait_assign<&$ty_r> for $ty_l {
            fn $fn_assign(&mut self, rhs: &$ty_r) {
                self.$fn_assign(rhs.clone());
            }
        }
        impl<const P: u32> $trait<$ty_r> for $ty_l {
            type Output = $ty_l;
            fn $fn(mut self, rhs: $ty_r) -> $ty_l {
                self.$fn_assign(rhs);
                self
            }
        }
        impl<const P: u32> $trait<$ty_r> for &$ty_l {
            type Output = $ty_l;
            fn $fn(self, rhs: $ty_r) -> $ty_l {
                let mut r = self.clone();
                r.$fn_assign(rhs);
                r
            }
        }
        impl<const P: u32> $trait<&$ty_r> for $ty_l {
            type Output = $ty_l;
            fn $fn(mut self, rhs: &$ty_r) -> $ty_l {
                self.$fn_assign(rhs.clone());
                self
            }
        }
        impl<const P: u32> $trait<&$ty_r> for &$ty_l {
            type Output = $ty_l;
            fn $fn(self, rhs: &$ty_r) -> $ty_l {
                let mut r = self.clone();
                r.$fn_assign(rhs.clone());
                r
            }
        }
    )*};
}

impl_ops! {
    FormalPowerSeries<P>, StaticModInt<P>, Mul, MulAssign, mul, mul_assign,
    FormalPowerSeries<P>, StaticModInt<P>, Div, DivAssign, div, div_assign,
    FormalPowerSeries<P>, FormalPowerSeries<P>, Add, AddAssign, add, add_assign,
    FormalPowerSeries<P>, FormalPowerSeries<P>, Sub, SubAssign, sub, sub_assign,
    FormalPowerSeries<P>, FormalPowerSeries<P>, Mul, MulAssign, mul, mul_assign,
    FormalPowerSeries<P>, FormalPowerSeries<P>, Div, DivAssign, div, div_assign,
    FormalPowerSeries<P>, FormalPowerSeries<P>, Rem, RemAssign, rem, rem_assign,
    FormalPowerSeries<P>, usize, Shl, ShlAssign, shl, shl_assign,
    FormalPowerSeries<P>, usize, Shr, ShrAssign, shr, shr_assign,
    SparseFormalPowerSeries<P>, StaticModInt<P>, Mul, MulAssign, mul, mul_assign,
    SparseFormalPowerSeries<P>, StaticModInt<P>, Div, DivAssign, div, div_assign,
    SparseFormalPowerSeries<P>, SparseFormalPowerSeries<P>, Add, AddAssign, add, add_assign,
    SparseFormalPowerSeries<P>, SparseFormalPowerSeries<P>, Sub, SubAssign, sub, sub_assign,
    SparseFormalPowerSeries<P>, SparseFormalPowerSeries<P>, Mul, MulAssign, mul, mul_assign,
    SparseFormalPowerSeries<P>, usize, Shl, ShlAssign, shl, shl_assign,
    SparseFormalPowerSeries<P>, usize, Shr, ShrAssign, shr, shr_assign,
    FormalPowerSeries<P>, SparseFormalPowerSeries<P>, Add, AddAssign, add, add_assign,
    FormalPowerSeries<P>, SparseFormalPowerSeries<P>, Sub, SubAssign, sub, sub_assign,
    FormalPowerSeries<P>, SparseFormalPowerSeries<P>, Mul, MulAssign, mul, mul_assign,
    FormalPowerSeries<P>, SparseFormalPowerSeries<P>, Div, DivAssign, div, div_assign,
}
