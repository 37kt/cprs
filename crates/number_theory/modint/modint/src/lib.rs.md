---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/combinatorics/binomial/src/lib.rs
    title: crates/combinatorics/binomial/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/lib.rs
    title: crates/convolution/convolution/src/lib.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/barrett_reduction/src/lib.rs
    title: crates/number_theory/modint/barrett_reduction/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/dynamic_modint/src/lib.rs
    title: crates/number_theory/modint/dynamic_modint/src/lib.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/dynamic_modint_64/src/lib.rs
    title: crates/number_theory/modint/dynamic_modint_64/src/lib.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/modint_61/src/lib.rs
    title: crates/number_theory/modint/modint_61/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/static_modint/src/lib.rs
    title: crates/number_theory/modint/static_modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/lib.rs
    title: crates/polynomial/formal_power_series/src/lib.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "pub trait ModInt:\n    Sized\n    + Default\n    + Clone\n    + Copy\n  \
    \  + PartialEq\n    + Eq\n    + std::hash::Hash\n    + std::fmt::Debug\n    +\
    \ std::fmt::Display\n    + std::str::FromStr\n    + From<i8>\n    + From<i16>\n\
    \    + From<i32>\n    + From<i64>\n    + From<i128>\n    + From<isize>\n    +\
    \ From<u8>\n    + From<u16>\n    + From<u32>\n    + From<u64>\n    + From<u128>\n\
    \    + From<usize>\n    + std::ops::Neg<Output = Self>\n    + std::ops::Add<Output\
    \ = Self>\n    + std::ops::Sub<Output = Self>\n    + std::ops::Mul<Output = Self>\n\
    \    + std::ops::Div<Output = Self>\n    + std::ops::AddAssign\n    + std::ops::SubAssign\n\
    \    + std::ops::MulAssign\n    + std::ops::DivAssign\n{\n    type Value;\n\n\
    \    fn new<T: Into<Self>>(val: T) -> Self;\n    fn modulus() -> Self::Value;\n\
    \    fn from_raw(val: Self::Value) -> Self;\n    fn val(self) -> Self::Value;\n\
    \    fn recip(self) -> Self;\n    fn pow(self, exp: usize) -> Self;\n    fn sqrt(self)\
    \ -> Option<Self>;\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/number_theory/modint/modint/src/lib.rs
  requiredBy:
  - crates/number_theory/modint/dynamic_modint_64/src/lib.rs
  - crates/number_theory/modint/modint_61/src/lib.rs
  - crates/number_theory/modint/dynamic_modint/src/lib.rs
  - crates/number_theory/modint/barrett_reduction/src/lib.rs
  - crates/number_theory/modint/static_modint/src/lib.rs
  - crates/convolution/convolution/src/lib.rs
  - crates/combinatorics/binomial/src/lib.rs
  - crates/polynomial/formal_power_series/src/lib.rs
  timestamp: '2025-03-08 06:04:29+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/number_theory/modint/modint/src/lib.rs
layout: document
redirect_from:
- /library/crates/number_theory/modint/modint/src/lib.rs
- /library/crates/number_theory/modint/modint/src/lib.rs.html
title: crates/number_theory/modint/modint/src/lib.rs
---
