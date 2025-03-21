---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/prime_factorization/src/lib.rs
    title: crates/number_theory/prime_factorization/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/prime_factorization/src/miller_rabin.rs
    title: crates/number_theory/prime_factorization/src/miller_rabin.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/combinatorics/binomial/src/lib.rs
    title: crates/combinatorics/binomial/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/prime_factorization/src/lib.rs
    title: crates/number_theory/prime_factorization/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/prime_factorization/src/miller_rabin.rs
    title: crates/number_theory/prime_factorization/src/miller_rabin.rs
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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use numeric_traits::Integer;\n\nuse crate::{is_prime, Mint};\n\npub(crate)\
    \ fn pollard_rho(n: u64, res: &mut Vec<u64>) {\n    if n == 1 {\n        return;\n\
    \    } else if n & 1 == 0 {\n        res.push(2);\n        pollard_rho(n >> 1,\
    \ res);\n        return;\n    } else if is_prime(n) {\n        res.push(n);\n\
    \        return;\n    }\n    let m = 1 << (n.floor_log2() + 1) / 8;\n    let mut\
    \ c = Mint::from_raw(1);\n    loop {\n        let mut x = Mint::from_raw(1);\n\
    \        let mut y = Mint::from_raw(2);\n        let mut ys = Mint::from_raw(1);\n\
    \        let mut q = Mint::from_raw(1);\n        let mut r = 1;\n        let mut\
    \ g = 1;\n        while g == 1 {\n            x = y;\n            for _ in 0..r\
    \ {\n                y = y * y + c;\n            }\n            for k in (0..r).step_by(m)\
    \ {\n                if g != 1 {\n                    break;\n               \
    \ }\n                ys = y;\n                for _ in 0..m.min(r - k) {\n   \
    \                 y = y * y + c;\n                    q *= x - y;\n          \
    \      }\n                g = n.gcd(q.val());\n            }\n            r <<=\
    \ 1;\n        }\n        if g == n {\n            g = 1;\n            while g\
    \ == 1 {\n                ys = ys * ys + c;\n                g = n.gcd((x - ys).val());\n\
    \            }\n        }\n        if g < n {\n            pollard_rho(g, res);\n\
    \            pollard_rho(n / g, res);\n            return;\n        }\n      \
    \  c += 1;\n    }\n}\n"
  dependsOn:
  - crates/number_theory/prime_factorization/src/lib.rs
  - crates/number_theory/prime_factorization/src/miller_rabin.rs
  isVerificationFile: false
  path: crates/number_theory/prime_factorization/src/pollard_rho.rs
  requiredBy:
  - crates/combinatorics/binomial/src/lib.rs
  - crates/number_theory/prime_factorization/src/lib.rs
  - crates/number_theory/prime_factorization/src/miller_rabin.rs
  timestamp: '2025-03-01 09:08:13+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/number_theory/primality_test/src/main.rs
  - verify/library_checker/number_theory/factorize/src/main.rs
documentation_of: crates/number_theory/prime_factorization/src/pollard_rho.rs
layout: document
redirect_from:
- /library/crates/number_theory/prime_factorization/src/pollard_rho.rs
- /library/crates/number_theory/prime_factorization/src/pollard_rho.rs.html
title: crates/number_theory/prime_factorization/src/pollard_rho.rs
---
