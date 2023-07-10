---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/division_of_polynomials/src/main.rs
    title: verify/division_of_polynomials/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/exp_of_formal_power_series/src/main.rs
    title: verify/exp_of_formal_power_series/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/inv_of_formal_power_series/src/main.rs
    title: verify/inv_of_formal_power_series/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/log_of_formal_power_series/src/main.rs
    title: verify/log_of_formal_power_series/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/pow_of_formal_power_series/src/main.rs
    title: verify/pow_of_formal_power_series/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/sqrt_of_formal_power_series/src/main.rs
    title: verify/sqrt_of_formal_power_series/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{\n    fmt::Debug,\n    ops::{\n        Add, AddAssign, Deref, DerefMut,\
    \ Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Shl,\n        ShlAssign,\
    \ Shr, ShrAssign, Sub, SubAssign,\n    },\n};\n\nuse ac_library::{convolution,\
    \ ModInt998244353 as M};\n\nconst P: usize = 998_244_353;\n\nfn sqrt(a: M) ->\
    \ Option<M> {\n    if a.val() <= 1 {\n        return Some(a);\n    } else if a.pow(499122176).val()\
    \ == 998244352 {\n        return None;\n    }\n    let mut c = a.pow(119);\n \
    \   let d = M::new(15311432);\n    let mut m = 0;\n    for i in 0..23 {\n    \
    \    if c.pow(1 << 22 - i).val() == 998244352 {\n            c *= d.pow(1 << i);\n\
    \            m += 1 << i;\n        }\n    }\n    Some(a.pow(60) * d.pow(m >> 1))\n\
    }\n\n#[derive(Clone)]\n#[repr(transparent)]\npub struct FPS(pub Vec<M>);\n\n#[macro_export]\n\
    macro_rules! fps {\n    ($($x:expr), *) => (\n        $crate::FPS(vec![$(ac_library::ModInt998244353::from($x)),\
    \ *])\n    );\n    ($x:expr; $n:expr) => (\n        $crate::FPS(vec![ac_library::ModInt998244353::from($x);\
    \ $n])\n    );\n}\n\nimpl Debug for FPS {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)\
    \ -> std::fmt::Result {\n        self.0.fmt(f)\n    }\n}\n\nimpl FPS {\n    pub\
    \ fn pre(&self, d: usize) -> FPS {\n        Self(self[0..self.len().min(d)].to_vec())\n\
    \    }\n\n    pub fn shrink(&mut self) {\n        while self.len() > 0 && self.last().unwrap().val()\
    \ == 0 {\n            self.pop();\n        }\n    }\n\n    pub fn eval(&self,\
    \ x: M) -> M {\n        let mut r = M::new(0);\n        let mut w = M::new(1);\n\
    \        for &v in &self.0 {\n            r += w * v;\n            w *= x;\n \
    \       }\n        r\n    }\n\n    pub fn diff(&self) -> FPS {\n        let n\
    \ = self.len();\n        if n == 0 {\n            fps![]\n        } else {\n \
    \           let mut r = fps![0; n - 1];\n            for i in 1..n {\n       \
    \         r[i - 1] = self[i] * i;\n            }\n            r\n        }\n \
    \   }\n\n    pub fn integral(&self) -> FPS {\n        let n = self.len();\n  \
    \      let mut inv = vec![M::new(1); n + 1];\n        let mut r = fps![0; n +\
    \ 1];\n        for i in 1..=n {\n            if i >= 2 {\n                inv[i]\
    \ = (inv[P % i] * (P - P / i)).into();\n            }\n            r[i] = self[i\
    \ - 1] * inv[i];\n        }\n        r\n    }\n\n    pub fn inv(&self, d: usize)\
    \ -> FPS {\n        let mut g = fps![self[0].inv()];\n        let mut k = 1;\n\
    \        while k < d {\n            k *= 2;\n            g = &(&(&(-&self.pre(k))\
    \ * &g) + &fps![2]) * &g;\n            g.resize(k, M::new(0));\n        }\n  \
    \      g.truncate(d);\n        g\n    }\n\n    pub fn log(&self, d: usize) ->\
    \ FPS {\n        assert!(self[0].val() == 1);\n        (&self.diff() * &self.inv(d)).pre(d\
    \ - 1).integral()\n    }\n\n    pub fn exp(&self, d: usize) -> FPS {\n       \
    \ let mut g = fps![1];\n        let mut k = 1;\n        while k < d {\n      \
    \      k *= 2;\n            g = &(&(&self.pre(k) - &g.log(k)) + &fps![1]) * &g;\n\
    \            g.resize(k, M::new(0));\n        }\n        g.truncate(d);\n    \
    \    g\n    }\n\n    pub fn pow(&self, k: usize, d: usize) -> FPS {\n        if\
    \ k == 0 {\n            let mut r = fps![0; d];\n            if d > 0 {\n    \
    \            r[0] = M::new(1);\n            }\n            return r;\n       \
    \ }\n        for i in 0..d {\n            if i * k > d {\n                return\
    \ fps![0; d];\n            }\n            if self[i].val() == 0 {\n          \
    \      continue;\n            }\n            let inv = self[i].inv();\n      \
    \      let mut r = (&(&(self * inv) >> i).log(d) * M::new(k)).exp(d);\n      \
    \      r *= self[i].pow(k as u64);\n            r = (&r << i * k).pre(d);\n  \
    \          if r.len() < d {\n                r.resize(d, M::new(0));\n       \
    \     }\n            return r;\n        }\n        fps![0; d]\n    }\n\n    pub\
    \ fn sqrt(&self, d: usize) -> Option<FPS> {\n        let n = self.len();\n   \
    \     if n == 0 {\n            return Some(fps![0; d]);\n        }\n        if\
    \ self[0].val() == 0 {\n            for i in 1..n {\n                if self[i].val()\
    \ != 0 {\n                    if i & 1 == 1 {\n                        return\
    \ None;\n                    }\n                    if d <= i / 2 {\n        \
    \                break;\n                    }\n                    if let Some(mut\
    \ r) = (self >> i).sqrt(d - i / 2) {\n                        r <<= i / 2;\n \
    \                       if r.len() < d {\n                            return None;\n\
    \                        }\n                        return Some(r);\n        \
    \            } else {\n                        return None;\n                \
    \    }\n                }\n            }\n            return Some(fps![0; d]);\n\
    \        }\n        if let Some(sqr) = sqrt(self[0]) {\n            let mut g\
    \ = fps![sqr];\n            let inv2 = M::new(2).inv();\n            let mut k\
    \ = 1;\n            while k < d {\n                k *= 2;\n                g\
    \ = &(&g + &(&self.pre(k) * &g.inv(k))) * inv2;\n            }\n            return\
    \ Some(g.pre(d));\n        } else {\n            return None;\n        }\n   \
    \ }\n}\n\nimpl Default for FPS {\n    fn default() -> Self {\n        fps![]\n\
    \    }\n}\n\nimpl Deref for FPS {\n    type Target = Vec<M>;\n    fn deref(&self)\
    \ -> &Self::Target {\n        &self.0\n    }\n}\n\nimpl DerefMut for FPS {\n \
    \   fn deref_mut(&mut self) -> &mut Self::Target {\n        &mut self.0\n    }\n\
    }\n\nimpl Neg for &FPS {\n    type Output = FPS;\n    fn neg(self) -> FPS {\n\
    \        let mut r = self.clone();\n        for i in 0..r.len() {\n          \
    \  r[i] = -r[i];\n        }\n        r\n    }\n}\n\nimpl AddAssign<&FPS> for FPS\
    \ {\n    fn add_assign(&mut self, rhs: &FPS) {\n        let n = self.len();\n\
    \        self.resize(n.max(rhs.len()), M::new(0));\n        for i in 0..rhs.len()\
    \ {\n            self[i] += rhs[i];\n        }\n    }\n}\n\nimpl Add<&FPS> for\
    \ &FPS {\n    type Output = FPS;\n    fn add(self, rhs: &FPS) -> FPS {\n     \
    \   let mut r = self.clone();\n        r += rhs;\n        r\n    }\n}\n\nimpl\
    \ SubAssign<&FPS> for FPS {\n    fn sub_assign(&mut self, rhs: &FPS) {\n     \
    \   let n = self.len();\n        self.resize(n.max(rhs.len()), M::new(0));\n \
    \       for i in 0..rhs.len() {\n            self[i] -= rhs[i];\n        }\n \
    \   }\n}\n\nimpl Sub<&FPS> for &FPS {\n    type Output = FPS;\n    fn sub(self,\
    \ rhs: &FPS) -> FPS {\n        let mut r = self.clone();\n        r -= rhs;\n\
    \        r\n    }\n}\n\nimpl MulAssign<M> for FPS {\n    fn mul_assign(&mut self,\
    \ rhs: M) {\n        for i in 0..self.len() {\n            self[i] *= rhs;\n \
    \       }\n    }\n}\n\nimpl Mul<M> for &FPS {\n    type Output = FPS;\n    fn\
    \ mul(self, rhs: M) -> FPS {\n        let mut r = self.clone();\n        r *=\
    \ rhs;\n        r\n    }\n}\n\nimpl DivAssign<M> for FPS {\n    fn div_assign(&mut\
    \ self, rhs: M) {\n        *self *= rhs.inv();\n    }\n}\n\nimpl Div<M> for &FPS\
    \ {\n    type Output = FPS;\n    fn div(self, rhs: M) -> FPS {\n        self *\
    \ rhs.inv()\n    }\n}\n\nimpl Mul<&FPS> for &FPS {\n    type Output = FPS;\n \
    \   fn mul(self, rhs: &FPS) -> FPS {\n        FPS(convolution(&self, &rhs))\n\
    \    }\n}\n\nimpl MulAssign<&FPS> for FPS {\n    fn mul_assign(&mut self, rhs:\
    \ &FPS) {\n        *self = &*self * rhs;\n    }\n}\n\nimpl Div<&FPS> for &FPS\
    \ {\n    type Output = FPS;\n    fn div(self, rhs: &FPS) -> FPS {\n        if\
    \ self.len() < rhs.len() {\n            return FPS::default();\n        }\n  \
    \      let n = self.len() - rhs.len() + 1;\n        let mut a = self.clone();\n\
    \        a.reverse();\n        a.truncate(n);\n        let mut b = rhs.clone();\n\
    \        b.reverse();\n        b = b.inv(n);\n        let mut c = &a * &b;\n \
    \       c.truncate(n);\n        c.reverse();\n        c\n    }\n}\n\nimpl DivAssign<&FPS>\
    \ for FPS {\n    fn div_assign(&mut self, rhs: &FPS) {\n        *self = &*self\
    \ / rhs;\n    }\n}\n\nimpl RemAssign<&FPS> for FPS {\n    fn rem_assign(&mut self,\
    \ rhs: &FPS) {\n        *self -= &(&(&*self / rhs) * rhs);\n        self.shrink();\n\
    \    }\n}\n\nimpl Rem<&FPS> for &FPS {\n    type Output = FPS;\n    fn rem(self,\
    \ rhs: &FPS) -> FPS {\n        let mut r = self.clone();\n        r %= rhs;\n\
    \        r\n    }\n}\n\nimpl Shl<usize> for &FPS {\n    type Output = FPS;\n \
    \   fn shl(self, rhs: usize) -> FPS {\n        let mut r = fps![0; rhs];\n   \
    \     r.append(&mut self.clone());\n        r\n    }\n}\n\nimpl ShlAssign<usize>\
    \ for FPS {\n    fn shl_assign(&mut self, rhs: usize) {\n        *self = &*self\
    \ << rhs;\n    }\n}\n\nimpl Shr<usize> for &FPS {\n    type Output = FPS;\n  \
    \  fn shr(self, rhs: usize) -> FPS {\n        if self.len() <= rhs {\n       \
    \     FPS::default()\n        } else {\n            FPS(self[rhs..].to_vec())\n\
    \        }\n    }\n}\n\nimpl ShrAssign<usize> for FPS {\n    fn shr_assign(&mut\
    \ self, rhs: usize) {\n        *self = &*self >> rhs;\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/polynomial/formal-power-series/src/lib.rs
  requiredBy: []
  timestamp: '2023-07-10 17:06:44+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/inv_of_formal_power_series/src/main.rs
  - verify/exp_of_formal_power_series/src/main.rs
  - verify/log_of_formal_power_series/src/main.rs
  - verify/division_of_polynomials/src/main.rs
  - verify/sqrt_of_formal_power_series/src/main.rs
  - verify/pow_of_formal_power_series/src/main.rs
documentation_of: crates/polynomial/formal-power-series/src/lib.rs
layout: document
redirect_from:
- /library/crates/polynomial/formal-power-series/src/lib.rs
- /library/crates/polynomial/formal-power-series/src/lib.rs.html
title: crates/polynomial/formal-power-series/src/lib.rs
---
