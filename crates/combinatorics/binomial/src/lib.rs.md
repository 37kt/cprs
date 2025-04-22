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
    path: crates/combinatorics/binomial/src/prime.rs
    title: crates/combinatorics/binomial/src/prime.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/modint/src/lib.rs
    title: crates/number_theory/modint/modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/prime_factorization/src/lib.rs
    title: crates/number_theory/prime_factorization/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/prime_factorization/src/miller_rabin.rs
    title: crates/number_theory/prime_factorization/src/miller_rabin.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/prime_factorization/src/pollard_rho.rs
    title: crates/number_theory/prime_factorization/src/pollard_rho.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/combinatorics/binomial/src/prime.rs
    title: crates/combinatorics/binomial/src/prime.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/enumerative_combinatorics/binomial_coefficient_prime_mod/src/main.rs
    title: verify/library_checker/enumerative_combinatorics/binomial_coefficient_prime_mod/src/main.rs
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
  code: 'pub mod prime;

    pub use prime::*;

    '
  dependsOn:
  - crates/algebra/numeric_traits/src/cast.rs
  - crates/algebra/numeric_traits/src/inf.rs
  - crates/algebra/numeric_traits/src/integer.rs
  - crates/algebra/numeric_traits/src/lib.rs
  - crates/algebra/numeric_traits/src/numeric.rs
  - crates/algebra/numeric_traits/src/signed.rs
  - crates/algebra/numeric_traits/src/zero_one.rs
  - crates/combinatorics/binomial/src/prime.rs
  - crates/number_theory/modint/modint/src/lib.rs
  - crates/number_theory/prime_factorization/src/lib.rs
  - crates/number_theory/prime_factorization/src/miller_rabin.rs
  - crates/number_theory/prime_factorization/src/pollard_rho.rs
  isVerificationFile: false
  path: crates/combinatorics/binomial/src/lib.rs
  requiredBy:
  - crates/combinatorics/binomial/src/prime.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/enumerative_combinatorics/binomial_coefficient_prime_mod/src/main.rs
documentation_of: crates/combinatorics/binomial/src/lib.rs
layout: document
redirect_from:
- /library/crates/combinatorics/binomial/src/lib.rs
- /library/crates/combinatorics/binomial/src/lib.rs.html
title: crates/combinatorics/binomial/src/lib.rs
---
