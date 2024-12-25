---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ops::{Add, Mul, Neg};\n\nuse algebraic::One;\nuse prime_sieve::PrimeSieve;\n\
    use quotients_array::QuotientsArray;\n\n/// f \u306F\u4E57\u6CD5\u7684\u95A2\u6570\
    \n/// sum_fp(n) = \u03A3p\u2208\\[1..=n | p is prime\\] f(p)  \n/// f(p, e) =\
    \ f(p^e)  \n/// return \u03A3i\u2208\\[1..=n\\] f(i)  \npub fn min_25_sieve<T>(\n\
    \    sum_fp: &QuotientsArray<T>,\n    mut f: impl FnMut(usize, usize) -> T,\n\
    ) -> QuotientsArray<T>\nwhere\n    T: Copy + Add<Output = T> + Neg<Output = T>\
    \ + Mul<Output = T> + One,\n{\n    let sqrt_n = sum_fp.sqrt_n();\n    let ps =\
    \ PrimeSieve::new(sqrt_n).primes();\n    let qs = sum_fp.quotients().to_vec();\n\
    \n    let mut dp = sum_fp.clone();\n    for &p in ps.iter().rev() {\n        for\
    \ &q in &qs {\n            let mut pc = p;\n            if pc * p > q {\n    \
    \            break;\n            }\n            let mut c = 1;\n            while\
    \ q / p >= pc {\n                let d = dp[q / pc] + -sum_fp[p];\n          \
    \      dp[q] = dp[q] + f(p, c) * d + f(p, c + 1);\n                pc *= p;\n\
    \                c += 1;\n            }\n        }\n    }\n    for &q in &qs {\n\
    \        dp[q] = dp[q] + T::one();\n    }\n\n    dp\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/data-structure/quotients-array/src/lib.rs
  - crates/math/prime-sieve/src/lib.rs
  isVerificationFile: false
  path: crates/math/min_25-sieve/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-25 03:34:39+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/sum_of_multiplicative_function/src/main.rs
documentation_of: crates/math/min_25-sieve/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/min_25-sieve/src/lib.rs
- /library/crates/math/min_25-sieve/src/lib.rs.html
title: crates/math/min_25-sieve/src/lib.rs
---
