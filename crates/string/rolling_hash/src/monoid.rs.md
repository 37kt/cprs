---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/string/rolling_hash/src/lib.rs
    title: crates/string/rolling_hash/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/string/rolling_hash/src/sequence.rs
    title: crates/string/rolling_hash/src/sequence.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/string/rolling_hash/src/lib.rs
    title: crates/string/rolling_hash/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/string/rolling_hash/src/sequence.rs
    title: crates/string/rolling_hash/src/sequence.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/string/zalgorithm_rh/src/main.rs
    title: verify/library_checker/string/zalgorithm_rh/src/main.rs
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
  code: "use algebraic_traits::{Algebraic, Associative, Magma, Pow, Unital};\nuse\
    \ modint_61::ModInt61;\n\nuse crate::RollingHash;\n\npub enum RollingHashOperator\
    \ {}\n\nimpl Algebraic for RollingHashOperator {\n    type Value = RollingHash;\n\
    }\n\nimpl Magma for RollingHashOperator {\n    fn op(a: &RollingHash, b: &RollingHash)\
    \ -> RollingHash {\n        RollingHash {\n            hash: a.hash + b.hash *\
    \ a.base_pow,\n            base_pow: a.base_pow * b.base_pow,\n        }\n   \
    \ }\n}\n\nimpl Unital for RollingHashOperator {\n    fn unit() -> RollingHash\
    \ {\n        RollingHash {\n            hash: ModInt61::from_raw(0),\n       \
    \     base_pow: ModInt61::from_raw(1),\n        }\n    }\n}\n\nimpl Associative\
    \ for RollingHashOperator {}\n\nimpl Pow for RollingHashOperator {}\n"
  dependsOn:
  - crates/string/rolling_hash/src/lib.rs
  - crates/string/rolling_hash/src/sequence.rs
  isVerificationFile: false
  path: crates/string/rolling_hash/src/monoid.rs
  requiredBy:
  - crates/string/rolling_hash/src/lib.rs
  - crates/string/rolling_hash/src/sequence.rs
  timestamp: '2025-03-24 01:42:22+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/string/zalgorithm_rh/src/main.rs
documentation_of: crates/string/rolling_hash/src/monoid.rs
layout: document
redirect_from:
- /library/crates/string/rolling_hash/src/monoid.rs
- /library/crates/string/rolling_hash/src/monoid.rs.html
title: crates/string/rolling_hash/src/monoid.rs
---
