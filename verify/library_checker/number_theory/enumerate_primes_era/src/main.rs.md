---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/eratosthenes/src/lib.rs
    title: crates/number_theory/eratosthenes/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/enumerate_primes
    links:
    - https://judge.yosupo.jp/problem/enumerate_primes
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/enumerate_primes\n\
    \nuse eratosthenes::Eratosthenes;\nuse proconio::{fastout, input};\n\n#[fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        a: usize,\n        b: usize,\n\
    \    }\n    let sieve = Eratosthenes::new(n);\n    let primes = sieve.primes().collect::<Vec<_>>();\n\
    \    let pi = primes.len();\n    let res = primes\n        .iter()\n        .skip(b)\n\
    \        .step_by(a)\n        .copied()\n        .collect::<Vec<_>>();\n    println!(\"\
    {} {}\", pi, res.len());\n    for i in 0..res.len() {\n        print!(\"{}\",\
    \ res[i]);\n        if i < res.len() - 1 {\n            print!(\" \");\n     \
    \   } else {\n            println!();\n        }\n    }\n}\n"
  dependsOn:
  - crates/number_theory/eratosthenes/src/lib.rs
  isVerificationFile: true
  path: verify/library_checker/number_theory/enumerate_primes_era/src/main.rs
  requiredBy: []
  timestamp: '2025-03-04 07:34:26+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/number_theory/enumerate_primes_era/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/number_theory/enumerate_primes_era/src/main.rs
- /verify/verify/library_checker/number_theory/enumerate_primes_era/src/main.rs.html
title: verify/library_checker/number_theory/enumerate_primes_era/src/main.rs
---
