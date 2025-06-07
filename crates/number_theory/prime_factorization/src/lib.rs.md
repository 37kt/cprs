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
  - icon: ':warning:'
    path: crates/number_theory/modint/dynamic_modint_64/src/barrett_reduction.rs
    title: crates/number_theory/modint/dynamic_modint_64/src/barrett_reduction.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/dynamic_modint_64/src/lib.rs
    title: crates/number_theory/modint/dynamic_modint_64/src/lib.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/dynamic_modint_64/src/numeric.rs
    title: crates/number_theory/modint/dynamic_modint_64/src/numeric.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/dynamic_modint_64/src/ops.rs
    title: crates/number_theory/modint/dynamic_modint_64/src/ops.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/prime_factorization/src/miller_rabin.rs
    title: crates/number_theory/prime_factorization/src/miller_rabin.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/prime_factorization/src/pollard_rho.rs
    title: crates/number_theory/prime_factorization/src/pollard_rho.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/combinatorics/binomial/src/lib.rs
    title: crates/combinatorics/binomial/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/prime_factorization/src/miller_rabin.rs
    title: crates/number_theory/prime_factorization/src/miller_rabin.rs
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
  code: "use std::ops::Mul;\n\nuse dynamic_modint_64::DynamicModInt64;\n\nmod miller_rabin;\n\
    mod pollard_rho;\n\nuse miller_rabin::miller_rabin;\nuse pollard_rho::pollard_rho;\n\
    \npub fn is_prime(n: impl TryInto<u64, Error = impl std::fmt::Debug>) -> bool\
    \ {\n    let n: u64 = n.try_into().expect(\"failed to convert to u64\");\n   \
    \ if n & 1 == 0 {\n        n == 2\n    } else if n <= 1 {\n        false\n   \
    \ } else if n < 1 << 30 {\n        miller_rabin(n, &[2, 7, 61])\n    } else {\n\
    \        miller_rabin(n, &[2, 325, 9375, 28178, 450775, 9780504, 1795265022])\n\
    \    }\n}\n\n/// n \u306E\u7D20\u56E0\u6570\u3092\u6607\u9806\u3067\u8FD4\u3059\
    \npub fn prime_factors<T>(n: T) -> Vec<T>\nwhere\n    T: TryInto<u64> + TryFrom<u64>,\n\
    \    <T as TryInto<u64>>::Error: std::fmt::Debug,\n    <T as TryFrom<u64>>::Error:\
    \ std::fmt::Debug,\n{\n    let n: u64 = n.try_into().expect(\"failed to convert\
    \ to u64\");\n    assert!(n > 0, \"n must be positive\");\n\n    let mut res =\
    \ vec![];\n    pollard_rho(n, &mut res);\n    res.sort_unstable();\n    res.into_iter().map(|x|\
    \ x.try_into().unwrap()).collect()\n}\n\n/// n \u306E\u7D20\u56E0\u6570\u5206\u89E3\
    \u3092 \\[(\u7D20\u56E0\u6570, \u6307\u6570), ...\\] \u306E\u5F62\u3067\u6607\u9806\
    \u3067\u8FD4\u3059\npub fn prime_factorization<T>(n: T) -> Vec<(T, usize)>\nwhere\n\
    \    T: TryInto<u64> + TryFrom<u64> + Eq,\n    <T as TryInto<u64>>::Error: std::fmt::Debug,\n\
    \    <T as TryFrom<u64>>::Error: std::fmt::Debug,\n{\n    let factors = prime_factors(n);\n\
    \    if factors.is_empty() {\n        return vec![];\n    }\n    let mut res =\
    \ vec![];\n    for p in factors {\n        if matches!(res.last(), Some((last,\
    \ _)) if last == &p) {\n            res.last_mut().unwrap().1 += 1;\n        }\
    \ else {\n            res.push((p, 1));\n        }\n    }\n    res\n}\n\npub fn\
    \ divisors_unordered<T>(n: T) -> Vec<T>\nwhere\n    T: TryInto<u64> + TryFrom<u64>\
    \ + Eq + Mul<Output = T> + Copy,\n    <T as TryInto<u64>>::Error: std::fmt::Debug,\n\
    \    <T as TryFrom<u64>>::Error: std::fmt::Debug,\n{\n    let pe = prime_factorization(n);\n\
    \    let mut res = vec![T::try_from(1).unwrap()];\n    for (p, e) in pe {\n  \
    \      for i in 0..res.len() {\n            let mut x = res[i];\n            for\
    \ _ in 0..e {\n                x = x * p;\n                res.push(x);\n    \
    \        }\n        }\n    }\n    res\n}\n\n/// n \u306E\u7D04\u6570\u3092\u6607\
    \u9806\u3067\u8FD4\u3059\npub fn divisors<T>(n: T) -> Vec<T>\nwhere\n    T: TryInto<u64>\
    \ + TryFrom<u64> + Ord + Mul<Output = T> + Copy,\n    <T as TryInto<u64>>::Error:\
    \ std::fmt::Debug,\n    <T as TryFrom<u64>>::Error: std::fmt::Debug,\n{\n    let\
    \ mut res = divisors_unordered(n);\n    res.sort_unstable();\n    res\n}\n\nenum\
    \ Id {}\ntype Mint = DynamicModInt64<Id>;\n"
  dependsOn:
  - crates/algebra/numeric_traits/src/cast.rs
  - crates/algebra/numeric_traits/src/inf.rs
  - crates/algebra/numeric_traits/src/integer.rs
  - crates/algebra/numeric_traits/src/lib.rs
  - crates/algebra/numeric_traits/src/numeric.rs
  - crates/algebra/numeric_traits/src/signed.rs
  - crates/algebra/numeric_traits/src/zero_one.rs
  - crates/number_theory/modint/dynamic_modint_64/src/barrett_reduction.rs
  - crates/number_theory/modint/dynamic_modint_64/src/lib.rs
  - crates/number_theory/modint/dynamic_modint_64/src/numeric.rs
  - crates/number_theory/modint/dynamic_modint_64/src/ops.rs
  - crates/number_theory/prime_factorization/src/miller_rabin.rs
  - crates/number_theory/prime_factorization/src/pollard_rho.rs
  isVerificationFile: false
  path: crates/number_theory/prime_factorization/src/lib.rs
  requiredBy:
  - crates/combinatorics/binomial/src/lib.rs
  - crates/number_theory/prime_factorization/src/miller_rabin.rs
  - crates/number_theory/prime_factorization/src/pollard_rho.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/number_theory/factorize/src/main.rs
  - verify/library_checker/number_theory/primality_test/src/main.rs
documentation_of: crates/number_theory/prime_factorization/src/lib.rs
layout: document
redirect_from:
- /library/crates/number_theory/prime_factorization/src/lib.rs
- /library/crates/number_theory/prime_factorization/src/lib.rs.html
title: crates/number_theory/prime_factorization/src/lib.rs
---
