---
data:
  _extendedDependsOn:
  - icon: ':x:'
    path: crates/combinatorics/binomial/src/lib.rs
    title: crates/combinatorics/binomial/src/lib.rs
  - icon: ':x:'
    path: crates/combinatorics/binomial/src/prime.rs
    title: crates/combinatorics/binomial/src/prime.rs
  - icon: ':question:'
    path: crates/number_theory/modint/dynamic_modint/src/barrett_reduction.rs
    title: crates/number_theory/modint/dynamic_modint/src/barrett_reduction.rs
  - icon: ':question:'
    path: crates/number_theory/modint/dynamic_modint/src/lib.rs
    title: crates/number_theory/modint/dynamic_modint/src/lib.rs
  - icon: ':question:'
    path: crates/number_theory/modint/dynamic_modint/src/numeric.rs
    title: crates/number_theory/modint/dynamic_modint/src/numeric.rs
  - icon: ':question:'
    path: crates/number_theory/modint/dynamic_modint/src/ops.rs
    title: crates/number_theory/modint/dynamic_modint/src/ops.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/binomial_coefficient_prime_mod
    links:
    - https://judge.yosupo.jp/problem/binomial_coefficient_prime_mod
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/binomial_coefficient_prime_mod\n\
    \nuse binomial::BinomialPrime;\nuse dynamic_modint::DefaultDynamicModInt;\nuse\
    \ proconio::{fastout, input};\n\ntype Mint = DefaultDynamicModInt;\n\n#[fastout]\n\
    fn main() {\n    input! {\n        t: usize,\n        m: u32,\n    }\n    Mint::set_modulus(m);\n\
    \    let mut binom = BinomialPrime::new();\n    binom.expand(10_000_000);\n  \
    \  for _ in 0..t {\n        input! {\n            n: usize,\n            k: usize,\n\
    \        }\n        let res: Mint = binom.nck(n, k);\n        println!(\"{}\"\
    , res);\n    }\n}\n"
  dependsOn:
  - crates/combinatorics/binomial/src/lib.rs
  - crates/combinatorics/binomial/src/prime.rs
  - crates/number_theory/modint/dynamic_modint/src/barrett_reduction.rs
  - crates/number_theory/modint/dynamic_modint/src/lib.rs
  - crates/number_theory/modint/dynamic_modint/src/numeric.rs
  - crates/number_theory/modint/dynamic_modint/src/ops.rs
  isVerificationFile: true
  path: verify/library_checker/enumerative_combinatorics/binomial_coefficient_prime_mod/src/main.rs
  requiredBy: []
  timestamp: '2025-05-12 06:37:24+00:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: verify/library_checker/enumerative_combinatorics/binomial_coefficient_prime_mod/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/enumerative_combinatorics/binomial_coefficient_prime_mod/src/main.rs
- /verify/verify/library_checker/enumerative_combinatorics/binomial_coefficient_prime_mod/src/main.rs.html
title: verify/library_checker/enumerative_combinatorics/binomial_coefficient_prime_mod/src/main.rs
---
