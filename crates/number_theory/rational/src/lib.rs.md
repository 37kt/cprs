---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/cast.rs
    title: crates/algebra/numeric_traits/src/cast.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/inf.rs
    title: crates/algebra/numeric_traits/src/inf.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/integer.rs
    title: crates/algebra/numeric_traits/src/integer.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/lib.rs
    title: crates/algebra/numeric_traits/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/numeric.rs
    title: crates/algebra/numeric_traits/src/numeric.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/signed.rs
    title: crates/algebra/numeric_traits/src/signed.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/zero_one.rs
    title: crates/algebra/numeric_traits/src/zero_one.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/rational/src/numeric.rs
    title: crates/number_theory/rational/src/numeric.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/rational/src/ops.rs
    title: crates/number_theory/rational/src/ops.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/rational/src/numeric.rs
    title: crates/number_theory/rational/src/numeric.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/rational/src/ops.rs
    title: crates/number_theory/rational/src/ops.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/stress_test/number_theory/comparing_rational/src/main.rs
    title: verify/stress_test/number_theory/comparing_rational/src/main.rs
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
  code: "use std::fmt::Debug;\n\nuse numeric_traits::Integer;\n\nmod numeric;\nmod\
    \ ops;\n\n#[derive(Clone, Copy)]\npub struct Rational<const AUTO_REDUCE: bool>\
    \ {\n    pub num: i64,\n    pub den: i64,\n}\n\nimpl<const AUTO_REDUCE: bool>\
    \ Default for Rational<AUTO_REDUCE> {\n    fn default() -> Self {\n        Self\
    \ { num: 0, den: 1 }\n    }\n}\n\nimpl<const AUTO_REDUCE: bool> Debug for Rational<AUTO_REDUCE>\
    \ {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n\
    \        write!(f, \"{}/{}\", self.num, self.den)\n    }\n}\n\nimpl From<Rational<true>>\
    \ for Rational<false> {\n    fn from(value: Rational<true>) -> Self {\n      \
    \  Self::new(value.num, value.den)\n    }\n}\n\nimpl From<Rational<false>> for\
    \ Rational<true> {\n    fn from(value: Rational<false>) -> Self {\n        Self::new(value.num,\
    \ value.den)\n    }\n}\n\nimpl<const AUTO_REDUCE: bool> From<i64> for Rational<AUTO_REDUCE>\
    \ {\n    fn from(value: i64) -> Self {\n        Self::new(value, 1)\n    }\n}\n\
    \nimpl<const AUTO_REDUCE: bool> From<Rational<AUTO_REDUCE>> for f64 {\n    fn\
    \ from(r: Rational<AUTO_REDUCE>) -> Self {\n        r.num as f64 / r.den as f64\n\
    \    }\n}\n\nimpl<const AUTO_REDUCE: bool> Rational<AUTO_REDUCE> {\n    pub fn\
    \ new(mut num: i64, mut den: i64) -> Self {\n        if den < 0 {\n          \
    \  num = -num;\n            den = -den;\n        }\n        let mut res = Self\
    \ { num, den };\n        if AUTO_REDUCE {\n            res.reduce();\n       \
    \ }\n        res\n    }\n\n    pub fn from_raw(num: i64, den: i64) -> Self {\n\
    \        Self { num, den }\n    }\n\n    pub fn reduce(&mut self) {\n        let\
    \ g = self.num.gcd(self.den);\n        self.num /= g;\n        self.den /= g;\n\
    \    }\n\n    pub fn floor(&self) -> i64 {\n        self.num.floor_div(self.den)\n\
    \    }\n\n    pub fn ceil(&self) -> i64 {\n        self.num.ceil_div(self.den)\n\
    \    }\n\n    pub fn is_nan(&self) -> bool {\n        self.num == 0 && self.den\
    \ == 0\n    }\n\n    pub fn is_inf(&self) -> bool {\n        self.num > 0 && self.den\
    \ == 0\n    }\n\n    pub fn is_neg_inf(&self) -> bool {\n        self.num < 0\
    \ && self.den == 0\n    }\n\n    pub fn nan() -> Self {\n        Self::from_raw(0,\
    \ 0)\n    }\n}\n"
  dependsOn:
  - crates/algebra/numeric_traits/src/cast.rs
  - crates/algebra/numeric_traits/src/inf.rs
  - crates/algebra/numeric_traits/src/integer.rs
  - crates/algebra/numeric_traits/src/lib.rs
  - crates/algebra/numeric_traits/src/numeric.rs
  - crates/algebra/numeric_traits/src/signed.rs
  - crates/algebra/numeric_traits/src/zero_one.rs
  - crates/number_theory/rational/src/numeric.rs
  - crates/number_theory/rational/src/ops.rs
  isVerificationFile: false
  path: crates/number_theory/rational/src/lib.rs
  requiredBy:
  - crates/number_theory/rational/src/numeric.rs
  - crates/number_theory/rational/src/ops.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/stress_test/number_theory/comparing_rational/src/main.rs
documentation_of: crates/number_theory/rational/src/lib.rs
layout: document
redirect_from:
- /library/crates/number_theory/rational/src/lib.rs
- /library/crates/number_theory/rational/src/lib.rs.html
title: crates/number_theory/rational/src/lib.rs
---
