use convolution_arbitrary_mod::convolution_arbitrary_mod;
use convolution_ntt_friendly::{convolution_ntt_friendly, ntt, ntt_inv};
use modint::StaticModInt;
use std::{
    fmt::{Debug, Display},
    iter::repeat,
    mem::swap,
    ops::{
        Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Shl, ShlAssign, Shr,
        ShrAssign, Sub, SubAssign,
    },
};

#[derive(Default, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct FormalPowerSeries<const P: u32>(pub Vec<StaticModInt<P>>);

pub type FormalPowerSeries998244353 = FormalPowerSeries<998_244_353>;
pub type FormalPowerSeries1000000007 = FormalPowerSeries<1_000_000_007>;

#[macro_export]
macro_rules! fps {
    ($($x:expr), *) => (
        $crate::FormalPowerSeries(vec![$(modint::StaticModInt::from($x)), *])
    );
    ($x:expr; $n:expr) => (
        $crate::FormalPowerSeries(vec![modint::StaticModInt::from($x); $n])
    );
}

impl<const P: u32> FormalPowerSeries<P> {
    pub fn shrink(&mut self) {
        while self.last() == Some(&0.into()) {
            self.pop();
        }
    }

    pub fn pre(&self, d: usize) -> Self {
        Self(self.0[0..d.min(self.len())].to_vec())
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

    pub fn count_terms(&self) -> usize {
        self.iter().filter(|&&v| v.val() != 0).count()
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

    pub fn div_mod(&self, g: &Self) -> (Self, Self) {
        assert!(g.last().unwrap().val() != 0);
        if self.len() < g.len() {
            return (fps![], self.clone());
        }
        let mut rf = self.clone();
        let mut rg = g.clone();
        rf.reverse();
        rg.reverse();
        let n = rf.len() - rg.len() + 1;
        rf.resize(n, 0.into());
        rg.resize(n, 0.into());
        let mut q = rf * rg.inv(n);
        q.resize(n, 0.into());
        q.reverse();
        let h = &q * g;
        let mut f = self.clone();
        for i in 0..f.len() {
            f[i] -= h[i];
        }
        f.shrink();
        (q, f)
    }

    pub fn inv(&self, d: usize) -> Self {
        assert_ne!(self[0].val(), 0);
        if StaticModInt::<P>::IS_NTT_FRIENDLY {
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
                let k = 1 << k;
                if k >= d {
                    break;
                }
                res = (&res + &res - &res * &res * self.pre(k * 2)).pre(k * 2);
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
        if StaticModInt::<P>::IS_NTT_FRIENDLY {
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

    pub fn sqrt(&self, d: usize) -> Option<FormalPowerSeries<P>> {
        if self.len() == 0 {
            return Some(fps![0; d]);
        }
        if self[0].val() == 0 {
            if let Some(i) = self.iter().position(|&x| x.val() != 0) {
                if i & 1 != 0 {
                    return None;
                } else if d <= i / 2 {
                    return Some(fps![0; d]);
                }
                let mut res = (self >> i).sqrt(d - i / 2)?;
                res <<= i / 2;
                if res.len() < d {
                    res.resize(d, 0.into());
                }
                return Some(res);
            }
            return Some(fps![0; d]);
        }

        let r = self[0].sqrt()?;
        assert_eq!(r * r, self[0]);
        let mut res = fps![r];
        let inv2 = StaticModInt::new(2).inv();
        for i in 0.. {
            let i = 1 << i;
            if i >= d {
                break;
            }
            res = (&res + self.clone().pre(i << 1) * res.inv(i << 1)) * inv2;
        }
        Some(res.pre(d))
    }

    pub fn multipoint_evaluate(&self, xs: &[StaticModInt<P>]) -> Vec<StaticModInt<P>> {
        let m = xs.len();
        if m == 0 {
            return vec![];
        }
        let m2 = 1 << 64 - (m - 1).leading_zeros();
        let mut g = vec![fps![1]; m2 + m2];
        for i in 0..m {
            g[m2 + i] = fps![-xs[i], 1];
        }
        for i in (1..m2).rev() {
            g[i] = &g[i << 1 | 0] * &g[i << 1 | 1];
        }
        g[1] = self.div_mod(&g[1]).1;
        for i in 2..m2 + m {
            g[i] = g[i >> 1].div_mod(&g[i]).1;
        }
        (m2..m2 + m)
            .map(|i| if g[i].len() == 0 { 0.into() } else { g[i][0] })
            .collect()
    }

    /// f(x + c)
    pub fn taylor_shift(mut self, c: StaticModInt<P>) -> Self {
        if self.len() == 0 {
            return self;
        }
        let n = self.len();
        let mut fact = vec![StaticModInt::new(1); n];
        let mut inv = vec![StaticModInt::new(1); n];
        let mut fact_inv = vec![StaticModInt::new(1); n];
        for i in 1..n {
            fact[i] = fact[i - 1] * i;
        }
        fact_inv[n - 1] = fact[n - 1].inv();
        for i in (1..n).rev() {
            inv[i] = fact_inv[i] * fact[i - 1];
            fact_inv[i - 1] = fact_inv[i] * i;
        }
        for i in 0..n {
            self[i] *= fact[i];
        }
        self.reverse();
        let mut g = fps![1; n];
        for i in 1..n {
            g[i] = g[i - 1] * c * inv[i];
        }
        self = (self * g).pre(n);
        self.reverse();
        for i in 0..n {
            self[i] *= fact_inv[i];
        }
        self
    }
}

// impl<const P: u32> SparseFormalPowerSeries<P> {
//     pub fn normalize(&mut self) {
//         if self.len() == 0 {
//             return;
//         }
//         self.0.sort_by_key(|&(i, _)| i);
//         let mut res = Self(vec![(self[0].0, StaticModInt::new(0))]);
//         for &(i, v) in &self.0 {
//             if res.len() == 0 || res.last().unwrap().0 != i {
//                 res.push((i, v));
//             } else {
//                 res.last_mut().unwrap().1 += v;
//             }
//         }
//         if res.len() != 0 && res.last().unwrap().1.val() == 0 {
//             res.pop();
//         }
//         *self = res;
//     }

//     pub fn differential(&self) -> Self {
//         Self(
//             self.iter()
//                 .filter_map(|&(i, v)| (i > 0).then(|| (i - 1, v * i)))
//                 .collect(),
//         )
//     }

//     pub fn integral(&self) -> Self {
//         Self(self.iter().map(|&(i, v)| (i + 1, v / (i + 1))).collect())
//     }

//     pub fn inv(self, d: usize) -> FormalPowerSeries<P> {
//         let mut f = fps![0; d];
//         f[0] += 1;
//         f /= self;
//         f
//     }

//     pub fn log(self, d: usize) -> FormalPowerSeries<P> {
//         assert!(self[0].0 == 0 && self[0].1.val() == 1);
//         let f = self.differential();
//         let mut res = (self.inv(d) * f).pre(d - 1).integral();
//         res.resize(d, 0.into());
//         res
//     }

//     pub fn exp(&self, d: usize) -> FormalPowerSeries<P> {
//         if self.len() == 0 {
//             let mut res = fps![0; d];
//             if d > 0 {
//                 res[0] = 1.into();
//             }
//             return res;
//         }
//         assert_ne!(self[0].0, 0);
//         let mut res = fps![0; d];
//         if d == 0 {
//             return res;
//         }
//         let mut a = self.differential();
//         for (d, _) in a.iter_mut() {
//             *d += 1;
//         }
//         res[0] = 1.into();
//         let mut inv = vec![StaticModInt::<P>::new(1); d];
//         let m = StaticModInt::<P>::modulus() as usize;
//         for i in 1..d {
//             if i > 1 {
//                 inv[i] = -inv[m % i] * (m / i);
//             }
//             res[i] = a
//                 .iter()
//                 .filter_map(|&(j, v)| (i >= j).then(|| v * res[i - j]))
//                 .sum::<StaticModInt<P>>()
//                 * inv[i];
//         }
//         res
//     }

//     pub fn pow(&self, k: usize, d: usize) -> FormalPowerSeries<P> {
//         let offset = self.iter().position(|&(_, v)| v.val() != 0);
//         let mut res = fps![0; d];
//         if offset.is_none() {
//             if k == 0 {
//                 res[0] += 1;
//             }
//             return res;
//         }
//         let offset = offset.unwrap();
//         if self[offset].0 > 0 {
//             let deg = self[offset].0;
//             if k > (d - 1) / deg {
//                 return res;
//             }
//             let g = Self(
//                 self.iter()
//                     .filter_map(|&(i, v)| (i >= deg).then(|| (i - deg, v)))
//                     .collect(),
//             );
//             let t = g.pow(k, d - k * deg);
//             for i in 0..d - k * deg {
//                 res[k * deg + i] = t[i];
//             }
//             return res;
//         }
//         let mut inv = vec![StaticModInt::<P>::new(1); d + 1];
//         let m = P as usize;
//         for i in 2..=d {
//             inv[i] = -inv[m % i] * (m / i);
//         }
//         res[0] = self[0].1.pow(k);
//         let c = self[0].1.inv();
//         for i in 1..d {
//             for &(j, v) in self.iter().skip(1).filter(|&&(j, _)| i >= j) {
//                 res[i] = res[i] + v * res[i - j] * (StaticModInt::<P>::new(k) * j - (i - j));
//             }
//             res[i] *= inv[i] * c;
//         }
//         res
//     }

//     pub fn sqrt(&self, d: usize) -> Option<FormalPowerSeries<P>> {
//         if self.len() == 0 {
//             return Some(fps![0; d]);
//         }
//         let p = self[0].0;
//         if p & 1 != 0 {
//             return None;
//         } else if p / 2 >= d {
//             return Some(fps![0; d]);
//         }
//         let inv_f0 = self[0].1.inv();
//         let lz = p / 2;
//         let mut g = fps![0; d];
//         g[lz] = self[0].1.sqrt()?;
//         let k = StaticModInt::new(2).inv();
//         let mut inv = vec![StaticModInt::new(1); d];
//         let m = P as usize;
//         for i in 2..d {
//             inv[i] = -inv[m % i] * (m / i);
//         }
//         for i in 1..d - lz {
//             g[lz + i] = self
//                 .iter()
//                 .skip(1)
//                 .filter_map(|&(j, v)| (j - p <= i).then(|| (j - p, v)))
//                 .map(|(j, v)| v * g[lz + i - j] * (k * j - (i - j)))
//                 .sum::<StaticModInt<P>>()
//                 * inv[i]
//                 * inv_f0;
//         }
//         Some(g)
//     }
// }

impl<const P: u32> Debug for FormalPowerSeries<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", &self.0))
    }
}

impl<const P: u32> Display for FormalPowerSeries<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.len() != 0 {
            f.write_fmt(format_args!("{}", self[0]))?;
        }
        for v in self.iter().skip(1) {
            f.write_fmt(format_args!(" {}", v))?;
        }
        Ok(())
    }
}

impl<const P: u32> Deref for FormalPowerSeries<P> {
    type Target = Vec<StaticModInt<P>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const P: u32> DerefMut for FormalPowerSeries<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const P: u32> From<Vec<StaticModInt<P>>> for FormalPowerSeries<P> {
    fn from(v: Vec<StaticModInt<P>>) -> Self {
        Self(v)
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

impl<const P: u32> Neg for &FormalPowerSeries<P> {
    type Output = FormalPowerSeries<P>;
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

impl<const P: u32> DivAssign<StaticModInt<P>> for FormalPowerSeries<P> {
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

impl<const P: u32> SubAssign<Self> for FormalPowerSeries<P> {
    fn sub_assign(&mut self, rhs: Self) {
        if self.len() < rhs.len() {
            self.resize(rhs.len(), 0.into());
        }
        self.iter_mut().zip(rhs.iter()).for_each(|(a, b)| *a -= b);
    }
}

impl<const P: u32> MulAssign<Self> for FormalPowerSeries<P> {
    fn mul_assign(&mut self, rhs: Self) {
        if rhs.count_terms() < 64 {
            let mut v = vec![];
            for i in 0..rhs.len() {
                if rhs[i].val() != 0 {
                    v.push((i, rhs[i]));
                }
            }
            if v.len() == 0 {
                *self = fps![];
                return;
            }
            let n = self.len();
            self.resize(n + v[v.len() - 1].0, 0.into());
            for i in (0..n).rev() {
                for &(j, c) in v.iter().rev() {
                    if j > 0 {
                        self[i + j] = self[i + j] + self[i] * c;
                    } else {
                        self[i] *= c;
                    }
                }
            }
        } else if StaticModInt::<P>::IS_NTT_FRIENDLY {
            let mut a = vec![];
            swap(&mut a, &mut self.0);
            self.0 = convolution_ntt_friendly(a, rhs.0);
        } else {
            self.0 = convolution_arbitrary_mod(&self.0, &rhs.0);
        }
    }
}

impl<const P: u32> DivAssign<Self> for FormalPowerSeries<P> {
    fn div_assign(&mut self, mut g: Self) {
        if g.count_terms() < 64 {
            if g[0].val() != 1 {
                let c = g[0].inv();
                for v in self.iter_mut() {
                    *v *= c;
                }
                for v in g.iter_mut() {
                    *v *= c;
                }
            }
            let mut v = vec![];
            for i in 1..g.len() {
                if g[i].val() != 0 {
                    v.push((i, -g[i]));
                }
            }
            for i in 0..self.len() {
                for &(j, c) in &v {
                    if i >= j {
                        self[i] = self[i] + self[i - j] * c;
                    }
                }
            }
        } else {
            let n = self.len();
            *self *= g.inv(n);
            self.truncate(n);
        }
    }
}

impl<const P: u32> ShlAssign<usize> for FormalPowerSeries<P> {
    fn shl_assign(&mut self, rhs: usize) {
        self.0 = repeat(0.into()).take(rhs).chain(self.0.drain(..)).collect();
    }
}

impl<const P: u32> ShrAssign<usize> for FormalPowerSeries<P> {
    fn shr_assign(&mut self, rhs: usize) {
        self.0 = self.0.drain(rhs..).collect();
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
    FormalPowerSeries<P>, usize, Shl, ShlAssign, shl, shl_assign,
    FormalPowerSeries<P>, usize, Shr, ShrAssign, shr, shr_assign,
}
