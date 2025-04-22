---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_traits/src/lib.rs
    title: crates/algebra/algebraic_traits/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_traits/src/macros.rs
    title: crates/algebra/algebraic_traits/src/macros.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_traits/src/traits.rs
    title: crates/algebra/algebraic_traits/src/traits.rs
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
    path: crates/convolution/convolution/src/arbitrary_mod.rs
    title: crates/convolution/convolution/src/arbitrary_mod.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/mod_2_64.rs
    title: crates/convolution/convolution/src/mod_2_64.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/naive.rs
    title: crates/convolution/convolution/src/naive.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/ntt.rs
    title: crates/convolution/convolution/src/ntt.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/ntt_friendly.rs
    title: crates/convolution/convolution/src/ntt_friendly.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/modint/src/lib.rs
    title: crates/number_theory/modint/modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/static_modint/src/lib.rs
    title: crates/number_theory/modint/static_modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/static_modint/src/mod_arithmetic.rs
    title: crates/number_theory/modint/static_modint/src/mod_arithmetic.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/static_modint/src/ntt_precalc.rs
    title: crates/number_theory/modint/static_modint/src/ntt_precalc.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/static_modint/src/numeric.rs
    title: crates/number_theory/modint/static_modint/src/numeric.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/static_modint/src/ops.rs
    title: crates/number_theory/modint/static_modint/src/ops.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/arbitrary_mod.rs
    title: crates/convolution/convolution/src/arbitrary_mod.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/mod_2_64.rs
    title: crates/convolution/convolution/src/mod_2_64.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/naive.rs
    title: crates/convolution/convolution/src/naive.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/ntt.rs
    title: crates/convolution/convolution/src/ntt.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/ntt_friendly.rs
    title: crates/convolution/convolution/src/ntt_friendly.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/lib.rs
    title: crates/polynomial/formal_power_series/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/convolution/convolution_mod/src/main.rs
    title: verify/library_checker/convolution/convolution_mod/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/convolution/convolution_mod_1000000007/src/main.rs
    title: verify/library_checker/convolution/convolution_mod_1000000007/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/convolution/convolution_mod_2_64/src/main.rs
    title: verify/library_checker/convolution/convolution_mod_2_64/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/frequency_table_of_tree_distance/src/main.rs
    title: verify/library_checker/tree/frequency_table_of_tree_distance/src/main.rs
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
  code: "mod naive;\npub use naive::*;\n\nmod ntt;\npub use ntt::*;\n\nmod ntt_friendly;\n\
    pub use ntt_friendly::*;\n\nmod arbitrary_mod;\npub use arbitrary_mod::*;\n\n\
    mod mod_2_64;\npub use mod_2_64::*;\n\npub(crate) const fn pow(x: u32, mut exp:\
    \ u32, m: u32) -> u32 {\n    let m = m as u64;\n    let mut res = 1u64;\n    let\
    \ mut x = x as u64;\n    while exp != 0 {\n        if exp & 1 != 0 {\n       \
    \     res = res * x % m;\n        }\n        x = x * x % m;\n        exp >>= 1;\n\
    \    }\n    res as u32\n}\n\npub(crate) const fn inv(x: u32, m: u32) -> u32 {\n\
    \    pow(x, m - 2, m)\n}\n"
  dependsOn:
  - crates/algebra/algebraic_traits/src/lib.rs
  - crates/algebra/algebraic_traits/src/macros.rs
  - crates/algebra/algebraic_traits/src/traits.rs
  - crates/algebra/numeric_traits/src/cast.rs
  - crates/algebra/numeric_traits/src/inf.rs
  - crates/algebra/numeric_traits/src/integer.rs
  - crates/algebra/numeric_traits/src/lib.rs
  - crates/algebra/numeric_traits/src/numeric.rs
  - crates/algebra/numeric_traits/src/signed.rs
  - crates/algebra/numeric_traits/src/zero_one.rs
  - crates/convolution/convolution/src/arbitrary_mod.rs
  - crates/convolution/convolution/src/mod_2_64.rs
  - crates/convolution/convolution/src/naive.rs
  - crates/convolution/convolution/src/ntt.rs
  - crates/convolution/convolution/src/ntt_friendly.rs
  - crates/number_theory/modint/modint/src/lib.rs
  - crates/number_theory/modint/static_modint/src/lib.rs
  - crates/number_theory/modint/static_modint/src/mod_arithmetic.rs
  - crates/number_theory/modint/static_modint/src/ntt_precalc.rs
  - crates/number_theory/modint/static_modint/src/numeric.rs
  - crates/number_theory/modint/static_modint/src/ops.rs
  isVerificationFile: false
  path: crates/convolution/convolution/src/lib.rs
  requiredBy:
  - crates/polynomial/formal_power_series/src/lib.rs
  - crates/convolution/convolution/src/arbitrary_mod.rs
  - crates/convolution/convolution/src/ntt.rs
  - crates/convolution/convolution/src/naive.rs
  - crates/convolution/convolution/src/ntt_friendly.rs
  - crates/convolution/convolution/src/mod_2_64.rs
  timestamp: '2025-04-22 06:09:15+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/tree/frequency_table_of_tree_distance/src/main.rs
  - verify/library_checker/convolution/convolution_mod_1000000007/src/main.rs
  - verify/library_checker/convolution/convolution_mod_2_64/src/main.rs
  - verify/library_checker/convolution/convolution_mod/src/main.rs
documentation_of: crates/convolution/convolution/src/lib.rs
layout: document
redirect_from:
- /library/crates/convolution/convolution/src/lib.rs
- /library/crates/convolution/convolution/src/lib.rs.html
title: crates/convolution/convolution/src/lib.rs
---
