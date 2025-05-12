---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/prime_factorization/src/lib.rs
    title: crates/number_theory/prime_factorization/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/prime_factorization/src/pollard_rho.rs
    title: crates/number_theory/prime_factorization/src/pollard_rho.rs
  _extendedRequiredBy:
  - icon: ':x:'
    path: crates/combinatorics/binomial/src/lib.rs
    title: crates/combinatorics/binomial/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/prime_factorization/src/lib.rs
    title: crates/number_theory/prime_factorization/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/prime_factorization/src/pollard_rho.rs
    title: crates/number_theory/prime_factorization/src/pollard_rho.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/number_theory/factorize/src/main.rs
    title: verify/library_checker/number_theory/factorize/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/number_theory/primality_test/src/main.rs
    title: verify/library_checker/number_theory/primality_test/src/main.rs
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
  code: "use crate::Mint;\n\npub(crate) fn miller_rabin(n: u64, a: &[u64]) -> bool\
    \ {\n    Mint::set_modulus(n);\n    let d = (n - 1) >> (n - 1).trailing_zeros();\n\
    \    let e = Mint::from_raw(1);\n    let r = Mint::from_raw(n - 1);\n    for &a\
    \ in a {\n        if n <= a {\n            break;\n        }\n        let mut\
    \ t = d;\n        let mut y = Mint::new(a).pow(t as _);\n        while t != n\
    \ - 1 && y != e && y != r {\n            y *= y;\n            t *= 2;\n      \
    \  }\n        if y != r && t % 2 == 0 {\n            return false;\n        }\n\
    \    }\n    true\n}\n"
  dependsOn:
  - crates/number_theory/prime_factorization/src/lib.rs
  - crates/number_theory/prime_factorization/src/pollard_rho.rs
  isVerificationFile: false
  path: crates/number_theory/prime_factorization/src/miller_rabin.rs
  requiredBy:
  - crates/combinatorics/binomial/src/lib.rs
  - crates/number_theory/prime_factorization/src/lib.rs
  - crates/number_theory/prime_factorization/src/pollard_rho.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/number_theory/factorize/src/main.rs
  - verify/library_checker/number_theory/primality_test/src/main.rs
documentation_of: crates/number_theory/prime_factorization/src/miller_rabin.rs
layout: document
redirect_from:
- /library/crates/number_theory/prime_factorization/src/miller_rabin.rs
- /library/crates/number_theory/prime_factorization/src/miller_rabin.rs.html
title: crates/number_theory/prime_factorization/src/miller_rabin.rs
---
