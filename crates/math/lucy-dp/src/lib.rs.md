---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/quotients-array/src/lib.rs
    title: crates/data-structure/quotients-array/src/lib.rs
  - icon: ':warning:'
    path: crates/math/prime-sieve/src/lib.rs
    title: crates/math/prime-sieve/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/sum_of_multiplicative_function/src/main.rs
    title: verify/sum_of_multiplicative_function/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ops::{Add, Neg};\n\nuse prime_sieve::PrimeSieve;\nuse quotients_array::QuotientsArray;\n\
    \n/// sum_f(n) = \u03A3i\u2208\\[1..=n\\] f(i)  \n/// mul(f(x), p) = f(px)  \n\
    /// return \u03A3p\u2208\\[1..=n | p is prime\\] f(p)\npub fn lucy_dp<T>(\n  \
    \  sum_f: &QuotientsArray<T>,\n    mut mul: impl FnMut(T, usize) -> T,\n) -> QuotientsArray<T>\n\
    where\n    T: Copy + Add<Output = T> + Neg<Output = T>,\n{\n    let sqrt_n = sum_f.sqrt_n();\n\
    \    let qs = sum_f.quotients().to_vec();\n    let ps = PrimeSieve::new(sqrt_n).primes();\n\
    \n    let mut dp = sum_f.clone();\n    let f1 = dp[1];\n    for &i in &qs {\n\
    \        dp[i] = dp[i] + -f1;\n    }\n\n    for &p in &ps {\n        let dpr =\
    \ dp[p - 1];\n        let p2 = p * p;\n        for &q in &qs {\n            if\
    \ q < p2 {\n                break;\n            }\n            let dpl = dp[q\
    \ / p];\n            dp[q] = dp[q] + -mul(dpl + -dpr, p);\n        }\n    }\n\n\
    \    dp\n}\n"
  dependsOn:
  - crates/data-structure/quotients-array/src/lib.rs
  - crates/math/prime-sieve/src/lib.rs
  isVerificationFile: false
  path: crates/math/lucy-dp/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-30 09:13:10+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/sum_of_multiplicative_function/src/main.rs
documentation_of: crates/math/lucy-dp/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/lucy-dp/src/lib.rs
- /library/crates/math/lucy-dp/src/lib.rs.html
title: crates/math/lucy-dp/src/lib.rs
---
