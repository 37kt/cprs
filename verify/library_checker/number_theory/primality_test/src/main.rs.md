---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/prime_factorization/src/lib.rs
    title: crates/number_theory/prime_factorization/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/prime_factorization/src/miller_rabin.rs
    title: crates/number_theory/prime_factorization/src/miller_rabin.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/prime_factorization/src/pollard_rho.rs
    title: crates/number_theory/prime_factorization/src/pollard_rho.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/primality_test
    links:
    - https://judge.yosupo.jp/problem/primality_test
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/primality_test\n\
    \nuse prime_factorization::is_prime;\nuse proconio::{fastout, input};\n\n#[fastout]\n\
    fn main() {\n    input! {\n        q: usize,\n    }\n    for _ in 0..q {\n   \
    \     input! {\n            n: usize,\n        }\n        if is_prime(n) {\n \
    \           println!(\"Yes\");\n        } else {\n            println!(\"No\"\
    );\n        }\n    }\n}\n"
  dependsOn:
  - crates/number_theory/prime_factorization/src/lib.rs
  - crates/number_theory/prime_factorization/src/miller_rabin.rs
  - crates/number_theory/prime_factorization/src/pollard_rho.rs
  isVerificationFile: true
  path: verify/library_checker/number_theory/primality_test/src/main.rs
  requiredBy: []
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/number_theory/primality_test/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/number_theory/primality_test/src/main.rs
- /verify/verify/library_checker/number_theory/primality_test/src/main.rs.html
title: verify/library_checker/number_theory/primality_test/src/main.rs
---
