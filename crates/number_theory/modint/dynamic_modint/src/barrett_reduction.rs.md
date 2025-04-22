---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/dynamic_modint/src/lib.rs
    title: crates/number_theory/modint/dynamic_modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/dynamic_modint/src/numeric.rs
    title: crates/number_theory/modint/dynamic_modint/src/numeric.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/dynamic_modint/src/ops.rs
    title: crates/number_theory/modint/dynamic_modint/src/ops.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/dynamic_modint/src/lib.rs
    title: crates/number_theory/modint/dynamic_modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/dynamic_modint/src/numeric.rs
    title: crates/number_theory/modint/dynamic_modint/src/numeric.rs
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
  code: "use std::cell::Cell;\n\nuse barrett_reduction::BarrettReduction32;\n\n#[allow(clippy::extra_unused_type_parameters)]\n\
    pub(crate) fn barrett_reduction<Id, Ret>(f: impl FnOnce(&Cell<BarrettReduction32>)\
    \ -> Ret) -> Ret {\n    thread_local! {\n        static BARRETT_REDUCTION: Cell<BarrettReduction32>\
    \ = Cell::new(BarrettReduction32::new(1_000_000_009));\n    }\n\n    BARRETT_REDUCTION.with(|br|\
    \ f(br))\n}\n"
  dependsOn:
  - crates/number_theory/modint/dynamic_modint/src/lib.rs
  - crates/number_theory/modint/dynamic_modint/src/numeric.rs
  - crates/number_theory/modint/dynamic_modint/src/ops.rs
  isVerificationFile: false
  path: crates/number_theory/modint/dynamic_modint/src/barrett_reduction.rs
  requiredBy:
  - crates/polynomial/formal_power_series/src/lib.rs
  - crates/number_theory/modint/dynamic_modint/src/lib.rs
  - crates/number_theory/modint/dynamic_modint/src/numeric.rs
  - crates/number_theory/modint/dynamic_modint/src/ops.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/enumerative_combinatorics/binomial_coefficient_prime_mod/src/main.rs
  - verify/library_checker/polynomial/exp_of_formal_power_series_not_ntt_friendly/src/main.rs
  - verify/library_checker/polynomial/inv_of_formal_power_series_not_ntt_friendly/src/main.rs
  - verify/library_checker/number_theory/sqrt_mod/src/main.rs
documentation_of: crates/number_theory/modint/dynamic_modint/src/barrett_reduction.rs
layout: document
redirect_from:
- /library/crates/number_theory/modint/dynamic_modint/src/barrett_reduction.rs
- /library/crates/number_theory/modint/dynamic_modint/src/barrett_reduction.rs.html
title: crates/number_theory/modint/dynamic_modint/src/barrett_reduction.rs
---
