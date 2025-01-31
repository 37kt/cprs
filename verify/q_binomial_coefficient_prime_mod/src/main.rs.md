---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/q-binomial/src/lib.rs
    title: crates/number-theory/q-binomial/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/q_binomial_coefficient_prime_mod
    links:
    - https://judge.yosupo.jp/problem/q_binomial_coefficient_prime_mod
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/q_binomial_coefficient_prime_mod\n\
    \nuse modint::DynamicModInt as Mint;\nuse proconio::input;\nuse q_binomial::QBinomial;\n\
    \n#[proconio::fastout]\nfn main() {\n    input! {\n        t: usize,\n       \
    \ m: u32,\n    }\n    Mint::set_modulus(m);\n    input! {\n        q: Mint,\n\
    \    }\n    let comb = QBinomial::new(q);\n    for _ in 0..t {\n        input!\
    \ {\n            n: usize,\n            k: usize,\n        }\n        println!(\"\
    {}\", comb.binom(n, k));\n    }\n}\n"
  dependsOn:
  - crates/number-theory/modint/src/lib.rs
  - crates/number-theory/q-binomial/src/lib.rs
  isVerificationFile: true
  path: verify/q_binomial_coefficient_prime_mod/src/main.rs
  requiredBy: []
  timestamp: '2025-01-26 00:19:46+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/q_binomial_coefficient_prime_mod/src/main.rs
layout: document
redirect_from:
- /verify/verify/q_binomial_coefficient_prime_mod/src/main.rs
- /verify/verify/q_binomial_coefficient_prime_mod/src/main.rs.html
title: verify/q_binomial_coefficient_prime_mod/src/main.rs
---
