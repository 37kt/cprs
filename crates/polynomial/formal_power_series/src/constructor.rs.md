---
data:
  _extendedDependsOn:
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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use modint::ModInt;\n\nuse crate::FormalPowerSeries;\n\n/// fps![]\n#[macro_export]\n\
    macro_rules! fps {\n    ($($x:expr), *) => (\n        $crate::FormalPowerSeries(vec![$(modint::ModInt::new($x)),\
    \ *])\n    );\n    ($x:expr; $n:expr) => (\n        $crate::FormalPowerSeries(vec![modint::ModInt::new($x);\
    \ $n])\n    );\n}\n\nimpl<M: ModInt<Value = u32>> FormalPowerSeries<M> {\n   \
    \ pub fn new() -> Self {\n        Self(vec![])\n    }\n\n    pub fn from_fn(n:\
    \ usize, f: impl FnMut(usize) -> M) -> Self {\n        (0..n).map(f).collect()\n\
    \    }\n}\n"
  dependsOn:
  - crates/polynomial/formal_power_series/src/convert.rs
  - crates/polynomial/formal_power_series/src/exp.rs
  - crates/polynomial/formal_power_series/src/inv.rs
  - crates/polynomial/formal_power_series/src/lib.rs
  - crates/polynomial/formal_power_series/src/log.rs
  - crates/polynomial/formal_power_series/src/mul.rs
  - crates/polynomial/formal_power_series/src/ops.rs
  - crates/polynomial/formal_power_series/src/pow.rs
  - crates/polynomial/formal_power_series/src/sqrt.rs
  isVerificationFile: false
  path: crates/polynomial/formal_power_series/src/constructor.rs
  requiredBy:
  - crates/polynomial/formal_power_series/src/inv.rs
  - crates/polynomial/formal_power_series/src/ops.rs
  - crates/polynomial/formal_power_series/src/lib.rs
  - crates/polynomial/formal_power_series/src/mul.rs
  - crates/polynomial/formal_power_series/src/log.rs
  - crates/polynomial/formal_power_series/src/pow.rs
  - crates/polynomial/formal_power_series/src/convert.rs
  - crates/polynomial/formal_power_series/src/sqrt.rs
  - crates/polynomial/formal_power_series/src/exp.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/polynomial/sqrt_of_formal_power_series/src/main.rs
  - verify/library_checker/polynomial/pow_of_formal_power_series/src/main.rs
  - verify/library_checker/polynomial/inv_of_formal_power_series_not_ntt_friendly/src/main.rs
  - verify/library_checker/polynomial/exp_of_formal_power_series_not_ntt_friendly/src/main.rs
  - verify/library_checker/polynomial/inv_of_formal_power_series/src/main.rs
  - verify/library_checker/polynomial/exp_of_formal_power_series/src/main.rs
  - verify/library_checker/polynomial/log_of_formal_power_series/src/main.rs
documentation_of: crates/polynomial/formal_power_series/src/constructor.rs
layout: document
redirect_from:
- /library/crates/polynomial/formal_power_series/src/constructor.rs
- /library/crates/polynomial/formal_power_series/src/constructor.rs.html
title: crates/polynomial/formal_power_series/src/constructor.rs
---
