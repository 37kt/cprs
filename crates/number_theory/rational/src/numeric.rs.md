---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/rational/src/lib.rs
    title: crates/number_theory/rational/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/rational/src/ops.rs
    title: crates/number_theory/rational/src/ops.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/rational/src/lib.rs
    title: crates/number_theory/rational/src/lib.rs
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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use numeric_traits::{Inf, NegInf, One, Recip, Signed, Zero};\n\nuse crate::Rational;\n\
    \nimpl<const AUTO_REDUCE: bool> Inf for Rational<AUTO_REDUCE> {\n    fn inf()\
    \ -> Self {\n        Self::from_raw(1, 0)\n    }\n}\n\nimpl<const AUTO_REDUCE:\
    \ bool> NegInf for Rational<AUTO_REDUCE> {\n    fn neg_inf() -> Self {\n     \
    \   Self::from_raw(-1, 0)\n    }\n}\n\nimpl<const AUTO_REDUCE: bool> Zero for\
    \ Rational<AUTO_REDUCE> {\n    fn zero() -> Self {\n        Self::from_raw(0,\
    \ 1)\n    }\n}\n\nimpl<const AUTO_REDUCE: bool> One for Rational<AUTO_REDUCE>\
    \ {\n    fn one() -> Self {\n        Self::from_raw(1, 1)\n    }\n}\n\nimpl<const\
    \ AUTO_REDUCE: bool> Recip for Rational<AUTO_REDUCE> {\n    fn recip(mut self)\
    \ -> Self {\n        std::mem::swap(&mut self.num, &mut self.den);\n        if\
    \ self.den < 0 {\n            self.num = -self.num;\n            self.den = -self.den;\n\
    \        }\n        self\n    }\n}\n\nimpl<const AUTO_REDUCE: bool> Signed for\
    \ Rational<AUTO_REDUCE> {\n    fn signum(self) -> Self {\n        if self.num\
    \ < 0 {\n            Self::from_raw(-1, 1)\n        } else if self.num > 0 {\n\
    \            Self::from_raw(1, 1)\n        } else if self.den == 0 {\n       \
    \     Self::from_raw(0, 0)\n        } else {\n            Self::from_raw(0, 1)\n\
    \        }\n    }\n}\n"
  dependsOn:
  - crates/number_theory/rational/src/lib.rs
  - crates/number_theory/rational/src/ops.rs
  isVerificationFile: false
  path: crates/number_theory/rational/src/numeric.rs
  requiredBy:
  - crates/number_theory/rational/src/ops.rs
  - crates/number_theory/rational/src/lib.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/stress_test/number_theory/comparing_rational/src/main.rs
documentation_of: crates/number_theory/rational/src/numeric.rs
layout: document
redirect_from:
- /library/crates/number_theory/rational/src/numeric.rs
- /library/crates/number_theory/rational/src/numeric.rs.html
title: crates/number_theory/rational/src/numeric.rs
---
