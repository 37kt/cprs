---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/dynamic_modint/src/barrett_reduction.rs
    title: crates/number_theory/modint/dynamic_modint/src/barrett_reduction.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/dynamic_modint/src/lib.rs
    title: crates/number_theory/modint/dynamic_modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/dynamic_modint/src/ops.rs
    title: crates/number_theory/modint/dynamic_modint/src/ops.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/dynamic_modint/src/barrett_reduction.rs
    title: crates/number_theory/modint/dynamic_modint/src/barrett_reduction.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/dynamic_modint/src/lib.rs
    title: crates/number_theory/modint/dynamic_modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/dynamic_modint/src/ops.rs
    title: crates/number_theory/modint/dynamic_modint/src/ops.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/lib.rs
    title: crates/polynomial/formal_power_series/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/enumerative_combinatorics/binomial_coefficient_prime_mod/src/main.rs
    title: verify/library_checker/enumerative_combinatorics/binomial_coefficient_prime_mod/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/number_theory/sqrt_mod/src/main.rs
    title: verify/library_checker/number_theory/sqrt_mod/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/polynomial/exp_of_formal_power_series_not_ntt_friendly/src/main.rs
    title: verify/library_checker/polynomial/exp_of_formal_power_series_not_ntt_friendly/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/polynomial/inv_of_formal_power_series_not_ntt_friendly/src/main.rs
    title: verify/library_checker/polynomial/inv_of_formal_power_series_not_ntt_friendly/src/main.rs
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
  code: "use numeric_traits::{Cast, Numeric, One, Recip, Zero};\n\nuse crate::DynamicModInt;\n\
    \nimpl<Id> Zero for DynamicModInt<Id> {\n    fn zero() -> Self {\n        Self::from_raw(0)\n\
    \    }\n}\n\nimpl<Id> One for DynamicModInt<Id> {\n    fn one() -> Self {\n  \
    \      Self::from(1)\n    }\n}\n\nimpl<Id> Numeric for DynamicModInt<Id> {}\n\n\
    impl<Id> Recip for DynamicModInt<Id> {\n    fn recip(self) -> Self {\n       \
    \ self.recip()\n    }\n}\n\nmacro_rules! impl_cast {\n    ($($t:ty),*) => {\n\
    \        $(impl<Id> Cast<DynamicModInt<Id>> for $t {\n            fn cast(self)\
    \ -> DynamicModInt<Id> {\n                DynamicModInt::from(self)\n        \
    \    }\n        })*\n    }\n}\n\nimpl_cast! {\n    u8, u16, u32, u64, u128, i8,\
    \ i16, i32, i64, i128, usize, isize\n}\n"
  dependsOn:
  - crates/number_theory/modint/dynamic_modint/src/barrett_reduction.rs
  - crates/number_theory/modint/dynamic_modint/src/lib.rs
  - crates/number_theory/modint/dynamic_modint/src/ops.rs
  isVerificationFile: false
  path: crates/number_theory/modint/dynamic_modint/src/numeric.rs
  requiredBy:
  - crates/number_theory/modint/dynamic_modint/src/lib.rs
  - crates/number_theory/modint/dynamic_modint/src/ops.rs
  - crates/number_theory/modint/dynamic_modint/src/barrett_reduction.rs
  - crates/polynomial/formal_power_series/src/lib.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/number_theory/sqrt_mod/src/main.rs
  - verify/library_checker/enumerative_combinatorics/binomial_coefficient_prime_mod/src/main.rs
  - verify/library_checker/polynomial/exp_of_formal_power_series_not_ntt_friendly/src/main.rs
  - verify/library_checker/polynomial/inv_of_formal_power_series_not_ntt_friendly/src/main.rs
documentation_of: crates/number_theory/modint/dynamic_modint/src/numeric.rs
layout: document
redirect_from:
- /library/crates/number_theory/modint/dynamic_modint/src/numeric.rs
- /library/crates/number_theory/modint/dynamic_modint/src/numeric.rs.html
title: crates/number_theory/modint/dynamic_modint/src/numeric.rs
---
