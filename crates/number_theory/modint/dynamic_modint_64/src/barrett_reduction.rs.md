---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/number_theory/modint/dynamic_modint_64/src/lib.rs
    title: crates/number_theory/modint/dynamic_modint_64/src/lib.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/dynamic_modint_64/src/numeric.rs
    title: crates/number_theory/modint/dynamic_modint_64/src/numeric.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/dynamic_modint_64/src/ops.rs
    title: crates/number_theory/modint/dynamic_modint_64/src/ops.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/number_theory/modint/dynamic_modint_64/src/lib.rs
    title: crates/number_theory/modint/dynamic_modint_64/src/lib.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/dynamic_modint_64/src/numeric.rs
    title: crates/number_theory/modint/dynamic_modint_64/src/numeric.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/dynamic_modint_64/src/ops.rs
    title: crates/number_theory/modint/dynamic_modint_64/src/ops.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/prime_factorization/src/lib.rs
    title: crates/number_theory/prime_factorization/src/lib.rs
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
  code: "use std::cell::Cell;\n\nuse barrett_reduction::BarrettReduction64;\n\n#[allow(clippy::extra_unused_type_parameters)]\n\
    pub(crate) fn barrett_reduction<Id, Ret>(f: impl FnOnce(&Cell<BarrettReduction64>)\
    \ -> Ret) -> Ret {\n    thread_local! {\n        static BARRETT_REDUCTION: Cell<BarrettReduction64>\
    \ = Cell::new(BarrettReduction64::new(1_000_000_009));\n    }\n\n    BARRETT_REDUCTION.with(|br|\
    \ f(br))\n}\n"
  dependsOn:
  - crates/number_theory/modint/dynamic_modint_64/src/lib.rs
  - crates/number_theory/modint/dynamic_modint_64/src/numeric.rs
  - crates/number_theory/modint/dynamic_modint_64/src/ops.rs
  isVerificationFile: false
  path: crates/number_theory/modint/dynamic_modint_64/src/barrett_reduction.rs
  requiredBy:
  - crates/number_theory/modint/dynamic_modint_64/src/numeric.rs
  - crates/number_theory/modint/dynamic_modint_64/src/ops.rs
  - crates/number_theory/modint/dynamic_modint_64/src/lib.rs
  - crates/number_theory/prime_factorization/src/lib.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/number_theory/modint/dynamic_modint_64/src/barrett_reduction.rs
layout: document
redirect_from:
- /library/crates/number_theory/modint/dynamic_modint_64/src/barrett_reduction.rs
- /library/crates/number_theory/modint/dynamic_modint_64/src/barrett_reduction.rs.html
title: crates/number_theory/modint/dynamic_modint_64/src/barrett_reduction.rs
---
