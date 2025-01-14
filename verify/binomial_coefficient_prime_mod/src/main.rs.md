---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/combination/src/lib.rs
    title: crates/number-theory/combination/src/lib.rs
  - icon: ':question:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/binomial_coefficient_prime_mod
    links:
    - https://judge.yosupo.jp/problem/binomial_coefficient_prime_mod
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/binomial_coefficient_prime_mod\n\
    \nuse combination::Combination;\nuse modint::DynamicModInt as Mint;\nuse proconio::input;\n\
    \n#[proconio::fastout]\nfn main() {\n    input! {\n        t: usize,\n       \
    \ m: u32,\n    }\n    Mint::set_modulus(m);\n    let comb = Combination::<Mint>::new();\n\
    \    for _ in 0..t {\n        input! {\n            n: usize,\n            k:\
    \ usize,\n        }\n        println!(\"{}\", comb.nck(n, k));\n    }\n}\n"
  dependsOn:
  - crates/number-theory/combination/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  isVerificationFile: true
  path: verify/binomial_coefficient_prime_mod/src/main.rs
  requiredBy: []
  timestamp: '2024-12-30 09:13:10+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/binomial_coefficient_prime_mod/src/main.rs
layout: document
redirect_from:
- /verify/verify/binomial_coefficient_prime_mod/src/main.rs
- /verify/verify/binomial_coefficient_prime_mod/src/main.rs.html
title: verify/binomial_coefficient_prime_mod/src/main.rs
---
