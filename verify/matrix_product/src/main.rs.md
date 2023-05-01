---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/matrix_product
    links:
    - https://judge.yosupo.jp/problem/matrix_product
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/matrix_product\n\
    \nuse ac_library::ModInt998244353 as Mint;\nuse itertools::Itertools;\nuse matrix::{Element,\
    \ Matrix};\nuse proconio::input;\n\n#[derive(Clone)]\nenum E {}\nimpl Element\
    \ for E {\n    type S = Mint;\n    fn zero() -> Self::S {\n        0.into()\n\
    \    }\n    fn one() -> Self::S {\n        1.into()\n    }\n    fn add(a: Self::S,\
    \ b: Self::S) -> Self::S {\n        a + b\n    }\n    fn mul(a: Self::S, b: Self::S)\
    \ -> Self::S {\n        a * b\n    }\n}\n\n#[proconio::fastout]\nfn main() {\n\
    \    input! {\n        n: usize,\n        m: usize,\n        k: usize,\n     \
    \   a: [[Mint; m]; n],\n        b: [[Mint; k]; m],\n    }\n    let a = Matrix::<E>::from(a);\n\
    \    let b = Matrix::<E>::from(b);\n    let c = &a * &b;\n    for i in 0..n {\n\
    \        println!(\"{}\", c[i].iter().join(\" \"));\n    }\n}\n"
  dependsOn: []
  isVerificationFile: true
  path: verify/matrix_product/src/main.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/matrix_product/src/main.rs
layout: document
redirect_from:
- /verify/verify/matrix_product/src/main.rs
- /verify/verify/matrix_product/src/main.rs.html
title: verify/matrix_product/src/main.rs
---
