use std::{
    fmt::Debug,
    ops::{
        Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Shl,
        ShlAssign, Shr, ShrAssign, Sub, SubAssign,
    },
};

use ac_library::{convolution, ModInt998244353 as M};

const P: usize = 998_244_353;

fn sqrt(a: M) -> Option<M> {
    if a.val() <= 1 {
        return Some(a);
    } else if a.pow(499122176).val() == 998244352 {
        return None;
    }
    let mut c = a.pow(119);
    let d = M::new(15311432);
    let mut m = 0;
    for i in 0..23 {
        if c.pow(1 << 22 - i).val() == 998244352 {
            c *= d.pow(1 << i);
            m += 1 << i;
        }
    }
    Some(a.pow(60) * d.pow(m >> 1))
}

#[derive(Clone)]
#[repr(transparent)]
pub struct FPS(pub Vec<M>);

#[macro_export]
macro_rules! fps {
    ($($x:expr), *) => (
        $crate::FPS(vec![$(ac_library::ModInt998244353::from($x)), *])
    );
    ($x:expr; $n:expr) => (
        $crate::FPS(vec![ac_library::ModInt998244353::from($x); $n])
    );
}

impl Debug for FPS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl FPS {
    pub fn pre(&self, d: usize) -> FPS {
        Self(self[0..self.len().min(d)].to_vec())
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

    pub fn diff(&self) -> FPS {
        let n = self.len();
        if n == 0 {
            fps![]
        } else {
            let mut r = fps![0; n - 1];
            for i in 1..n {
                r[i - 1] = self[i] * i;
            }
            r
        }
    }

    pub fn integral(&self) -> FPS {
        let n = self.len();
        let mut inv = vec![M::new(1); n + 1];
        let mut r = fps![0; n + 1];
        for i in 1..=n {
            if i >= 2 {
                inv[i] = (inv[P % i] * (P - P / i)).into();
            }
            r[i] = self[i - 1] * inv[i];
        }
        r
    }

    pub fn inv(&self, d: usize) -> FPS {
        let mut g = fps![self[0].inv()];
        let mut k = 1;
        while k < d {
            k *= 2;
            g = &(&(&(-&self.pre(k)) * &g) + &fps![2]) * &g;
            g.resize(k, M::new(0));
        }
        g.truncate(d);
        g
    }

    pub fn log(&self, d: usize) -> FPS {
        let mut f = self.clone();
        f.resize(d, M::new(0));
        (&f.diff() / &f).integral()
    }

    pub fn exp(&self, d: usize) -> FPS {
        let mut g = fps![1];
        let mut k = 1;
        while k < d {
            k *= 2;
            g = &(&(&self.pre(k) - &g.log(k)) + &fps![1]) * &g;
            g.resize(k, M::new(0));
        }
        g.truncate(d);
        g
    }

    pub fn pow(&self, k: usize, d: usize) -> FPS {
        if k == 0 {
            let mut r = fps![0; d];
            if d > 0 {
                r[0] = M::new(1);
            }
            return r;
        }
        for i in 0..d {
            if i * k > d {
                return fps![0; d];
            }
            if self[i].val() == 0 {
                continue;
            }
            let inv = self[i].inv();
            let mut r = (&(&(self * inv) >> i).log(d) * M::new(k)).exp(d);
            r *= self[i].pow(k as u64);
            r = (&r << i * k).pre(d);
            if r.len() < d {
                r.resize(d, M::new(0));
            }
            return r;
        }
        fps![0; d]
    }

    pub fn sqrt(&self, d: usize) -> Option<FPS> {
        let n = self.len();
        if n == 0 {
            return Some(fps![0; d]);
        }
        if self[0].val() == 0 {
            for i in 1..n {
                if self[i].val() != 0 {
                    if i & 1 == 1 {
                        return None;
                    }
                    if d <= i / 2 {
                        break;
                    }
                    if let Some(mut r) = (self >> i).sqrt(d - i / 2) {
                        r <<= i / 2;
                        if r.len() < d {
                            return None;
                        }
                        return Some(r);
                    } else {
                        return None;
                    }
                }
            }
            return Some(fps![0; d]);
        }
        if let Some(sqr) = sqrt(self[0]) {
            let mut g = fps![sqr];
            let inv2 = M::new(2).inv();
            let mut k = 1;
            while k < d {
                k *= 2;
                g = &(&g + &(&self.pre(k) * &g.inv(k))) * inv2;
            }
            return Some(g.pre(d));
        } else {
            return None;
        }
    }

    pub fn divmod(&self, g: &FPS) -> (FPS, FPS) {
        let mut f = self.clone();
        let mut g = g.clone();
        f.shrink();
        g.shrink();
        let n = f.len();
        let m = g.len();
        if n < m {
            return (fps![], f);
        }
        let mut q = f.clone();
        q.reverse();
        g.reverse();
        q /= &g;
        q.resize(n + 1 - m, M::new(0));
        q.reverse();
        g.reverse();
        let mut r = &f - &(&g * &q);
        r.shrink();
        (q, r)
    }
}

impl Default for FPS {
    fn default() -> Self {
        fps![]
    }
}

impl Deref for FPS {
    type Target = Vec<M>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FPS {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Neg for &FPS {
    type Output = FPS;
    fn neg(self) -> FPS {
        let mut r = self.clone();
        for i in 0..r.len() {
            r[i] = -r[i];
        }
        r
    }
}

impl AddAssign<&FPS> for FPS {
    fn add_assign(&mut self, rhs: &FPS) {
        let n = self.len();
        self.resize(n.max(rhs.len()), M::new(0));
        for i in 0..rhs.len() {
            self[i] += rhs[i];
        }
    }
}

impl Add<&FPS> for &FPS {
    type Output = FPS;
    fn add(self, rhs: &FPS) -> FPS {
        let mut r = self.clone();
        r += rhs;
        r
    }
}

impl SubAssign<&FPS> for FPS {
    fn sub_assign(&mut self, rhs: &FPS) {
        let n = self.len();
        self.resize(n.max(rhs.len()), M::new(0));
        for i in 0..rhs.len() {
            self[i] -= rhs[i];
        }
    }
}

impl Sub<&FPS> for &FPS {
    type Output = FPS;
    fn sub(self, rhs: &FPS) -> FPS {
        let mut r = self.clone();
        r -= rhs;
        r
    }
}

impl MulAssign<M> for FPS {
    fn mul_assign(&mut self, rhs: M) {
        for i in 0..self.len() {
            self[i] *= rhs;
        }
    }
}

impl Mul<M> for &FPS {
    type Output = FPS;
    fn mul(self, rhs: M) -> FPS {
        let mut r = self.clone();
        r *= rhs;
        r
    }
}

impl DivAssign<M> for FPS {
    fn div_assign(&mut self, rhs: M) {
        *self *= rhs.inv();
    }
}

impl Div<M> for &FPS {
    type Output = FPS;
    fn div(self, rhs: M) -> FPS {
        self * rhs.inv()
    }
}

impl Mul<&FPS> for &FPS {
    type Output = FPS;
    fn mul(self, rhs: &FPS) -> FPS {
        FPS(convolution(&self, &rhs))
    }
}

impl MulAssign<&FPS> for FPS {
    fn mul_assign(&mut self, rhs: &FPS) {
        *self = &*self * rhs;
    }
}

impl Div<&FPS> for &FPS {
    type Output = FPS;
    fn div(self, rhs: &FPS) -> FPS {
        if self.len() < rhs.len() {
            return FPS::default();
        }
        let n = self.len() - rhs.len() + 1;
        let mut a = self.clone();
        a.reverse();
        a.truncate(n);
        let mut b = rhs.clone();
        b.reverse();
        b.truncate(n);
        let mut c = &a * &b;
        c.truncate(n);
        c.reverse();
        c
    }
}

impl DivAssign<&FPS> for FPS {
    fn div_assign(&mut self, rhs: &FPS) {
        *self = &*self / rhs;
    }
}

impl RemAssign<&FPS> for FPS {
    fn rem_assign(&mut self, rhs: &FPS) {
        *self -= &(&(&*self / rhs) * rhs);
        self.shrink();
    }
}

impl Rem<&FPS> for &FPS {
    type Output = FPS;
    fn rem(self, rhs: &FPS) -> FPS {
        let mut r = self.clone();
        r %= rhs;
        r
    }
}

impl Shl<usize> for &FPS {
    type Output = FPS;
    fn shl(self, rhs: usize) -> FPS {
        let mut r = fps![0; rhs];
        r.append(&mut self.clone());
        r
    }
}

impl ShlAssign<usize> for FPS {
    fn shl_assign(&mut self, rhs: usize) {
        *self = &*self << rhs;
    }
}

impl Shr<usize> for &FPS {
    type Output = FPS;
    fn shr(self, rhs: usize) -> FPS {
        if self.len() <= rhs {
            FPS::default()
        } else {
            FPS(self[rhs..].to_vec())
        }
    }
}

impl ShrAssign<usize> for FPS {
    fn shr_assign(&mut self, rhs: usize) {
        *self = &*self >> rhs;
    }
}
