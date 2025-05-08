---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/constructor.rs
    title: crates/polynomial/formal_power_series/src/constructor.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/convert.rs
    title: crates/polynomial/formal_power_series/src/convert.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/exp.rs
    title: crates/polynomial/formal_power_series/src/exp.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/inv.rs
    title: crates/polynomial/formal_power_series/src/inv.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/lib.rs
    title: crates/polynomial/formal_power_series/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/log.rs
    title: crates/polynomial/formal_power_series/src/log.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/mul.rs
    title: crates/polynomial/formal_power_series/src/mul.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/pow.rs
    title: crates/polynomial/formal_power_series/src/pow.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/sqrt.rs
    title: crates/polynomial/formal_power_series/src/sqrt.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/constructor.rs
    title: crates/polynomial/formal_power_series/src/constructor.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/convert.rs
    title: crates/polynomial/formal_power_series/src/convert.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/exp.rs
    title: crates/polynomial/formal_power_series/src/exp.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/inv.rs
    title: crates/polynomial/formal_power_series/src/inv.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/lib.rs
    title: crates/polynomial/formal_power_series/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/log.rs
    title: crates/polynomial/formal_power_series/src/log.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/mul.rs
    title: crates/polynomial/formal_power_series/src/mul.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/pow.rs
    title: crates/polynomial/formal_power_series/src/pow.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/sqrt.rs
    title: crates/polynomial/formal_power_series/src/sqrt.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/polynomial/exp_of_formal_power_series/src/main.rs
    title: verify/library_checker/polynomial/exp_of_formal_power_series/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/polynomial/exp_of_formal_power_series_not_ntt_friendly/src/main.rs
    title: verify/library_checker/polynomial/exp_of_formal_power_series_not_ntt_friendly/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/polynomial/inv_of_formal_power_series/src/main.rs
    title: verify/library_checker/polynomial/inv_of_formal_power_series/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/polynomial/inv_of_formal_power_series_not_ntt_friendly/src/main.rs
    title: verify/library_checker/polynomial/inv_of_formal_power_series_not_ntt_friendly/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/polynomial/log_of_formal_power_series/src/main.rs
    title: verify/library_checker/polynomial/log_of_formal_power_series/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/polynomial/pow_of_formal_power_series/src/main.rs
    title: verify/library_checker/polynomial/pow_of_formal_power_series/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/polynomial/sqrt_of_formal_power_series/src/main.rs
    title: verify/library_checker/polynomial/sqrt_of_formal_power_series/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{\n    cell::{Cell, RefCell},\n    ops::{Add, AddAssign, Neg, Sub,\
    \ SubAssign},\n};\n\nuse modint::ModInt;\n\nuse crate::FormalPowerSeries;\n\n\
    impl<M: ModInt<Value = u32>> FormalPowerSeries<M> {\n    pub fn diff(&self) ->\
    \ Self {\n        self.iter()\n            .enumerate()\n            .skip(1)\n\
    \            .map(|(i, &v)| M::from_raw(i as _) * v)\n            .collect()\n\
    \    }\n\n    pub fn integral(&self) -> Self {\n        thread_local! {\n    \
    \        static MOD: Cell<u32> = Cell::new(0);\n            static INV: RefCell<Vec<u32>>\
    \ = RefCell::new(vec![0, 1]);\n        }\n\n        MOD.with(|m| {\n         \
    \   if m.get() != M::modulus() {\n                m.set(M::modulus());\n     \
    \           INV.with(|inv| {\n                    let mut inv = inv.borrow_mut();\n\
    \                    inv.resize(2, 1);\n                })\n            }\n  \
    \      });\n\n        INV.with(|inv| {\n            let n = self.len();\n    \
    \        let m = M::modulus();\n            let mut inv = inv.borrow_mut();\n\
    \            let sz = inv.len();\n            let nsz = n + 1;\n            if\
    \ sz < nsz {\n                inv.reserve(nsz);\n                for i in sz..nsz\
    \ {\n                    let t = inv[m as usize % i];\n                    inv.push((-M::from_raw(t)\
    \ * M::from_raw(m / i as u32)).val());\n                }\n            }\n\n \
    \           Self::from_fn(n + 1, |i| {\n                if i == 0 {\n        \
    \            M::from_raw(0)\n                } else {\n                    self[i\
    \ - 1] * M::from_raw(inv[i])\n                }\n            })\n        })\n\
    \    }\n}\n\nimpl<M: ModInt<Value = u32>> Neg for FormalPowerSeries<M> {\n   \
    \ type Output = Self;\n\n    fn neg(mut self) -> Self::Output {\n        self.iter_mut().for_each(|v|\
    \ *v = -*v);\n        self\n    }\n}\n\nimpl<M: ModInt<Value = u32>> Neg for &FormalPowerSeries<M>\
    \ {\n    type Output = FormalPowerSeries<M>;\n\n    fn neg(self) -> Self::Output\
    \ {\n        self.iter().map(|v| -*v).collect()\n    }\n}\n\nimpl<M: ModInt<Value\
    \ = u32>> AddAssign<&FormalPowerSeries<M>> for FormalPowerSeries<M> {\n    fn\
    \ add_assign(&mut self, rhs: &FormalPowerSeries<M>) {\n        if self.len() <\
    \ rhs.len() {\n            self.resize(rhs.len(), M::from_raw(0));\n        }\n\
    \        self.iter_mut().zip(rhs).for_each(|(a, b)| *a += *b);\n    }\n}\n\nimpl<M:\
    \ ModInt<Value = u32>> AddAssign for FormalPowerSeries<M> {\n    fn add_assign(&mut\
    \ self, rhs: Self) {\n        self.add_assign(&rhs);\n    }\n}\n\nimpl<M: ModInt<Value\
    \ = u32>> Add for FormalPowerSeries<M> {\n    type Output = Self;\n\n    fn add(mut\
    \ self, rhs: Self) -> Self::Output {\n        self.add_assign(&rhs);\n       \
    \ self\n    }\n}\n\nimpl<M: ModInt<Value = u32>> Add<&FormalPowerSeries<M>> for\
    \ FormalPowerSeries<M> {\n    type Output = Self;\n\n    fn add(mut self, rhs:\
    \ &FormalPowerSeries<M>) -> Self::Output {\n        self.add_assign(rhs);\n  \
    \      self\n    }\n}\n\nimpl<M: ModInt<Value = u32>> Add<FormalPowerSeries<M>>\
    \ for &FormalPowerSeries<M> {\n    type Output = FormalPowerSeries<M>;\n\n   \
    \ fn add(self, mut rhs: FormalPowerSeries<M>) -> Self::Output {\n        rhs.add_assign(self);\n\
    \        rhs\n    }\n}\n\nimpl<M: ModInt<Value = u32>> Add for &FormalPowerSeries<M>\
    \ {\n    type Output = FormalPowerSeries<M>;\n\n    fn add(self, rhs: &FormalPowerSeries<M>)\
    \ -> Self::Output {\n        self.clone().add(rhs)\n    }\n}\n\nimpl<M: ModInt<Value\
    \ = u32>> SubAssign<&FormalPowerSeries<M>> for FormalPowerSeries<M> {\n    fn\
    \ sub_assign(&mut self, rhs: &FormalPowerSeries<M>) {\n        if self.len() <\
    \ rhs.len() {\n            self.resize(rhs.len(), M::from_raw(0));\n        }\n\
    \        self.iter_mut().zip(rhs).for_each(|(a, b)| *a -= *b);\n    }\n}\n\nimpl<M:\
    \ ModInt<Value = u32>> SubAssign for FormalPowerSeries<M> {\n    fn sub_assign(&mut\
    \ self, rhs: Self) {\n        self.sub_assign(&rhs);\n    }\n}\n\nimpl<M: ModInt<Value\
    \ = u32>> Sub for FormalPowerSeries<M> {\n    type Output = Self;\n\n    fn sub(mut\
    \ self, rhs: Self) -> Self::Output {\n        self.sub_assign(&rhs);\n       \
    \ self\n    }\n}\n\nimpl<M: ModInt<Value = u32>> Sub<&FormalPowerSeries<M>> for\
    \ FormalPowerSeries<M> {\n    type Output = Self;\n\n    fn sub(mut self, rhs:\
    \ &FormalPowerSeries<M>) -> Self::Output {\n        self.sub_assign(rhs);\n  \
    \      self\n    }\n}\n\nimpl<M: ModInt<Value = u32>> Sub<FormalPowerSeries<M>>\
    \ for &FormalPowerSeries<M> {\n    type Output = FormalPowerSeries<M>;\n\n   \
    \ #[allow(clippy::suspicious_arithmetic_impl)]\n    fn sub(self, mut rhs: FormalPowerSeries<M>)\
    \ -> Self::Output {\n        rhs.iter_mut().for_each(|x| *x = -*x);\n        if\
    \ rhs.len() < self.len() {\n            rhs.resize(self.len(), M::from_raw(0));\n\
    \        }\n        rhs.iter_mut().zip(self).for_each(|(a, b)| *a += *b);\n  \
    \      rhs\n    }\n}\n\nimpl<M: ModInt<Value = u32>> Sub for &FormalPowerSeries<M>\
    \ {\n    type Output = FormalPowerSeries<M>;\n\n    fn sub(self, rhs: &FormalPowerSeries<M>)\
    \ -> Self::Output {\n        self.clone().sub(rhs)\n    }\n}\n"
  dependsOn:
  - crates/polynomial/formal_power_series/src/constructor.rs
  - crates/polynomial/formal_power_series/src/convert.rs
  - crates/polynomial/formal_power_series/src/exp.rs
  - crates/polynomial/formal_power_series/src/inv.rs
  - crates/polynomial/formal_power_series/src/lib.rs
  - crates/polynomial/formal_power_series/src/log.rs
  - crates/polynomial/formal_power_series/src/mul.rs
  - crates/polynomial/formal_power_series/src/pow.rs
  - crates/polynomial/formal_power_series/src/sqrt.rs
  isVerificationFile: false
  path: crates/polynomial/formal_power_series/src/ops.rs
  requiredBy:
  - crates/polynomial/formal_power_series/src/lib.rs
  - crates/polynomial/formal_power_series/src/inv.rs
  - crates/polynomial/formal_power_series/src/exp.rs
  - crates/polynomial/formal_power_series/src/mul.rs
  - crates/polynomial/formal_power_series/src/log.rs
  - crates/polynomial/formal_power_series/src/convert.rs
  - crates/polynomial/formal_power_series/src/sqrt.rs
  - crates/polynomial/formal_power_series/src/pow.rs
  - crates/polynomial/formal_power_series/src/constructor.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/polynomial/exp_of_formal_power_series_not_ntt_friendly/src/main.rs
  - verify/library_checker/polynomial/sqrt_of_formal_power_series/src/main.rs
  - verify/library_checker/polynomial/pow_of_formal_power_series/src/main.rs
  - verify/library_checker/polynomial/exp_of_formal_power_series/src/main.rs
  - verify/library_checker/polynomial/log_of_formal_power_series/src/main.rs
  - verify/library_checker/polynomial/inv_of_formal_power_series/src/main.rs
  - verify/library_checker/polynomial/inv_of_formal_power_series_not_ntt_friendly/src/main.rs
documentation_of: crates/polynomial/formal_power_series/src/ops.rs
layout: document
redirect_from:
- /library/crates/polynomial/formal_power_series/src/ops.rs
- /library/crates/polynomial/formal_power_series/src/ops.rs.html
title: crates/polynomial/formal_power_series/src/ops.rs
---
