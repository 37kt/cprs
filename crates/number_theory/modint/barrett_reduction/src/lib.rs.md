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
  - icon: ':warning:'
    path: crates/number_theory/modint/barrett_reduction/src/barrett_reduction_32.rs
    title: crates/number_theory/modint/barrett_reduction/src/barrett_reduction_32.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/barrett_reduction/src/barrett_reduction_64.rs
    title: crates/number_theory/modint/barrett_reduction/src/barrett_reduction_64.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/modint/src/lib.rs
    title: crates/number_theory/modint/modint/src/lib.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/number_theory/modint/barrett_reduction/src/barrett_reduction_32.rs
    title: crates/number_theory/modint/barrett_reduction/src/barrett_reduction_32.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/barrett_reduction/src/barrett_reduction_64.rs
    title: crates/number_theory/modint/barrett_reduction/src/barrett_reduction_64.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/dynamic_modint/src/lib.rs
    title: crates/number_theory/modint/dynamic_modint/src/lib.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/dynamic_modint_64/src/lib.rs
    title: crates/number_theory/modint/dynamic_modint_64/src/lib.rs
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
  code: 'pub mod barrett_reduction_32;

    pub use barrett_reduction_32::*;

    pub mod barrett_reduction_64;

    pub use barrett_reduction_64::*;

    '
  dependsOn:
  - crates/algebra/numeric_traits/src/cast.rs
  - crates/algebra/numeric_traits/src/inf.rs
  - crates/algebra/numeric_traits/src/integer.rs
  - crates/algebra/numeric_traits/src/lib.rs
  - crates/algebra/numeric_traits/src/numeric.rs
  - crates/algebra/numeric_traits/src/signed.rs
  - crates/algebra/numeric_traits/src/zero_one.rs
  - crates/number_theory/modint/barrett_reduction/src/barrett_reduction_32.rs
  - crates/number_theory/modint/barrett_reduction/src/barrett_reduction_64.rs
  - crates/number_theory/modint/modint/src/lib.rs
  isVerificationFile: false
  path: crates/number_theory/modint/barrett_reduction/src/lib.rs
  requiredBy:
  - crates/number_theory/modint/dynamic_modint_64/src/lib.rs
  - crates/number_theory/modint/dynamic_modint/src/lib.rs
  - crates/number_theory/modint/barrett_reduction/src/barrett_reduction_64.rs
  - crates/number_theory/modint/barrett_reduction/src/barrett_reduction_32.rs
  timestamp: '2025-05-16 07:00:05+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/number_theory/modint/barrett_reduction/src/lib.rs
layout: document
redirect_from:
- /library/crates/number_theory/modint/barrett_reduction/src/lib.rs
- /library/crates/number_theory/modint/barrett_reduction/src/lib.rs.html
title: crates/number_theory/modint/barrett_reduction/src/lib.rs
---
