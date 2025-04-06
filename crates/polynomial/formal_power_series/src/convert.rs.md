---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/constructor.rs
    title: crates/polynomial/formal_power_series/src/constructor.rs
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
    path: crates/polynomial/formal_power_series/src/ops.rs
    title: crates/polynomial/formal_power_series/src/ops.rs
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
    path: crates/polynomial/formal_power_series/src/ops.rs
    title: crates/polynomial/formal_power_series/src/ops.rs
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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{\n    fmt::Debug,\n    ops::{Deref, DerefMut},\n};\n\nuse modint::ModInt;\n\
    \nuse crate::FormalPowerSeries;\n\nimpl<M: ModInt<Value = u32>> Deref for FormalPowerSeries<M>\
    \ {\n    type Target = Vec<M>;\n\n    fn deref(&self) -> &Self::Target {\n   \
    \     &self.0\n    }\n}\n\nimpl<M: ModInt<Value = u32>> DerefMut for FormalPowerSeries<M>\
    \ {\n    fn deref_mut(&mut self) -> &mut Self::Target {\n        &mut self.0\n\
    \    }\n}\n\nimpl<M: ModInt<Value = u32>> Debug for FormalPowerSeries<M> {\n \
    \   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n   \
    \     write!(f, \"{:?}\", self.deref())\n    }\n}\n\nimpl<M: ModInt<Value = u32>>\
    \ FromIterator<M> for FormalPowerSeries<M> {\n    fn from_iter<T: IntoIterator<Item\
    \ = M>>(iter: T) -> Self {\n        Self(iter.into_iter().collect())\n    }\n\
    }\n\nimpl<'a, M: ModInt<Value = u32> + 'a> FromIterator<&'a M> for FormalPowerSeries<M>\
    \ {\n    fn from_iter<T: IntoIterator<Item = &'a M>>(iter: T) -> Self {\n    \
    \    Self(iter.into_iter().copied().collect())\n    }\n}\n\nimpl<M: ModInt<Value\
    \ = u32>> From<Vec<M>> for FormalPowerSeries<M> {\n    fn from(value: Vec<M>)\
    \ -> Self {\n        Self(value)\n    }\n}\n\nimpl<M: ModInt<Value = u32>> From<&[M]>\
    \ for FormalPowerSeries<M> {\n    fn from(value: &[M]) -> Self {\n        Self(value.to_vec())\n\
    \    }\n}\n\nimpl<M: ModInt<Value = u32>> IntoIterator for FormalPowerSeries<M>\
    \ {\n    type Item = M;\n    type IntoIter = std::vec::IntoIter<Self::Item>;\n\
    \n    fn into_iter(self) -> Self::IntoIter {\n        self.0.into_iter()\n   \
    \ }\n}\n\nimpl<'a, M: ModInt<Value = u32> + 'a> IntoIterator for &'a FormalPowerSeries<M>\
    \ {\n    type Item = &'a M;\n    type IntoIter = std::slice::Iter<'a, M>;\n\n\
    \    fn into_iter(self) -> Self::IntoIter {\n        self.0.iter()\n    }\n}\n"
  dependsOn:
  - crates/polynomial/formal_power_series/src/constructor.rs
  - crates/polynomial/formal_power_series/src/exp.rs
  - crates/polynomial/formal_power_series/src/inv.rs
  - crates/polynomial/formal_power_series/src/lib.rs
  - crates/polynomial/formal_power_series/src/log.rs
  - crates/polynomial/formal_power_series/src/mul.rs
  - crates/polynomial/formal_power_series/src/ops.rs
  - crates/polynomial/formal_power_series/src/pow.rs
  - crates/polynomial/formal_power_series/src/sqrt.rs
  isVerificationFile: false
  path: crates/polynomial/formal_power_series/src/convert.rs
  requiredBy:
  - crates/polynomial/formal_power_series/src/lib.rs
  - crates/polynomial/formal_power_series/src/ops.rs
  - crates/polynomial/formal_power_series/src/constructor.rs
  - crates/polynomial/formal_power_series/src/sqrt.rs
  - crates/polynomial/formal_power_series/src/pow.rs
  - crates/polynomial/formal_power_series/src/exp.rs
  - crates/polynomial/formal_power_series/src/log.rs
  - crates/polynomial/formal_power_series/src/inv.rs
  - crates/polynomial/formal_power_series/src/mul.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/polynomial/sqrt_of_formal_power_series/src/main.rs
  - verify/library_checker/polynomial/exp_of_formal_power_series_not_ntt_friendly/src/main.rs
  - verify/library_checker/polynomial/inv_of_formal_power_series_not_ntt_friendly/src/main.rs
  - verify/library_checker/polynomial/pow_of_formal_power_series/src/main.rs
  - verify/library_checker/polynomial/log_of_formal_power_series/src/main.rs
  - verify/library_checker/polynomial/exp_of_formal_power_series/src/main.rs
  - verify/library_checker/polynomial/inv_of_formal_power_series/src/main.rs
documentation_of: crates/polynomial/formal_power_series/src/convert.rs
layout: document
redirect_from:
- /library/crates/polynomial/formal_power_series/src/convert.rs
- /library/crates/polynomial/formal_power_series/src/convert.rs.html
title: crates/polynomial/formal_power_series/src/convert.rs
---
