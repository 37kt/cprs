---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/quotients-array/src/lib.rs
    title: crates/data-structure/quotients-array/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/math/lucy-dp/src/lib.rs
    title: crates/math/lucy-dp/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/math/min_25-sieve/src/lib.rs
    title: crates/math/min_25-sieve/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/sum_of_multiplicative_function
    links:
    - https://judge.yosupo.jp/problem/sum_of_multiplicative_function
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/sum_of_multiplicative_function\n\
    \nuse lucy_dp::lucy_dp;\nuse min_25_sieve::min_25_sieve;\nuse modint::StaticModInt;\n\
    use proconio::input;\nuse quotients_array::QuotientsArray;\n\ntype Mint = StaticModInt<469762049>;\n\
    \n#[proconio::fastout]\nfn main() {\n    input! {\n        t: usize,\n    }\n\
    \    for _ in 0..t {\n        input! {\n            n: usize,\n            a:\
    \ Mint,\n            b: Mint,\n        }\n        let inv_2 = Mint::new(2).inv();\n\
    \        let sum_fa = QuotientsArray::from_fn(n, |i| Mint::new(i) * a);\n    \
    \    let sum_fb = QuotientsArray::from_fn(n, |i| Mint::new(i) * (i + 1) * inv_2\
    \ * b);\n        let qs = sum_fa.quotients().to_vec();\n        let sum_fap =\
    \ lucy_dp(&sum_fa, |x, _| x);\n        let sum_fbp = lucy_dp(&sum_fb, |x, p| x\
    \ * p);\n        let mut sum_fp = sum_fap;\n        for &q in &qs {\n        \
    \    sum_fp[q] += sum_fbp[q];\n        }\n        let sum_f = min_25_sieve(&sum_fp,\
    \ |p, e| a * e + b * p);\n        println!(\"{}\", sum_f[n]);\n    }\n}\n"
  dependsOn:
  - crates/data-structure/quotients-array/src/lib.rs
  - crates/math/lucy-dp/src/lib.rs
  - crates/math/min_25-sieve/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  isVerificationFile: true
  path: verify/sum_of_multiplicative_function/src/main.rs
  requiredBy: []
  timestamp: '2024-12-30 09:13:10+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/sum_of_multiplicative_function/src/main.rs
layout: document
redirect_from:
- /verify/verify/sum_of_multiplicative_function/src/main.rs
- /verify/verify/sum_of_multiplicative_function/src/main.rs.html
title: verify/sum_of_multiplicative_function/src/main.rs
---
