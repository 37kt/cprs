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
    path: crates/number_theory/rational/src/lib.rs
    title: crates/number_theory/rational/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/rational/src/numeric.rs
    title: crates/number_theory/rational/src/numeric.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/rational/src/ops.rs
    title: crates/number_theory/rational/src/ops.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/aplusb
    links:
    - https://judge.yosupo.jp/problem/aplusb
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/aplusb\n\n\
    use numeric_traits::{Inf, NegInf};\nuse proconio::input;\nuse rand::Rng;\nuse\
    \ rational::Rational;\n\nfn main() {\n    input! {\n        a: u32,\n        b:\
    \ u32,\n    }\n    println!(\"{}\", a + b);\n\n    type Q1 = Rational<false>;\n\
    \    const M: i64 = 1 << 60;\n    let mut rng = rand::thread_rng();\n    for _\
    \ in 0..200000 {\n        let a = Q1::new(rng.gen_range(-M..=M), rng.gen_range(-M..=M));\n\
    \        let b = Q1::new(rng.gen_range(-M..=M), rng.gen_range(-M..=M));\n    \
    \    let x: f64 = a.into();\n        let y: f64 = b.into();\n        assert_eq!(a.cmp(&b),\
    \ x.partial_cmp(&y).unwrap());\n    }\n\n    type Q2 = Rational<true>;\n    let\
    \ mut rng = rand::thread_rng();\n    for _ in 0..200000 {\n        let a = Q2::new(rng.gen_range(-M..=M),\
    \ rng.gen_range(-M..=M));\n        let b = Q2::new(rng.gen_range(-M..=M), rng.gen_range(-M..=M));\n\
    \        let x: f64 = a.into();\n        let y: f64 = b.into();\n        assert_eq!(a.cmp(&b),\
    \ x.partial_cmp(&y).unwrap());\n    }\n\n    assert!(Q1::inf() > Q1::from_raw(10,\
    \ 1));\n    assert!(Q1::neg_inf() < Q1::from_raw(-10, 1));\n    assert!(Q1::neg_inf()\
    \ < Q1::inf());\n    assert!((Q1::inf() + Q1::neg_inf()).is_nan());\n    assert!((Q1::inf()\
    \ - Q1::inf()).is_nan());\n    assert!((Q1::neg_inf() - Q1::neg_inf()).is_nan());\n\
    \n    eprintln!(\"OK\");\n}\n"
  dependsOn:
  - crates/algebra/numeric_traits/src/cast.rs
  - crates/algebra/numeric_traits/src/inf.rs
  - crates/algebra/numeric_traits/src/integer.rs
  - crates/algebra/numeric_traits/src/lib.rs
  - crates/algebra/numeric_traits/src/numeric.rs
  - crates/algebra/numeric_traits/src/signed.rs
  - crates/algebra/numeric_traits/src/zero_one.rs
  - crates/number_theory/rational/src/lib.rs
  - crates/number_theory/rational/src/numeric.rs
  - crates/number_theory/rational/src/ops.rs
  isVerificationFile: true
  path: verify/stress_test/number_theory/comparing_rational/src/main.rs
  requiredBy: []
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/stress_test/number_theory/comparing_rational/src/main.rs
layout: document
redirect_from:
- /verify/verify/stress_test/number_theory/comparing_rational/src/main.rs
- /verify/verify/stress_test/number_theory/comparing_rational/src/main.rs.html
title: verify/stress_test/number_theory/comparing_rational/src/main.rs
---
