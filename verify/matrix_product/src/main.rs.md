---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/math/matrix/src/lib.rs
    title: crates/math/matrix/src/lib.rs
  - icon: ':question:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/matrix_product
    links:
    - https://judge.yosupo.jp/problem/matrix_product
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/matrix_product\n\
    \nuse itertools::Itertools;\nuse matrix::Matrix;\nuse modint::ModInt998244353\
    \ as Mint;\nuse proconio::input;\n\n#[proconio::fastout]\nfn main() {\n    input!\
    \ {\n        n: usize,\n        m: usize,\n        k: usize,\n        a: [[Mint;\
    \ m]; n],\n        b: [[Mint; k]; m],\n    }\n    let a = Matrix::<Mint>::from(a);\n\
    \    let b = Matrix::<Mint>::from(b);\n    let c = &a * &b;\n    for i in 0..n\
    \ {\n        println!(\"{}\", c[i].iter().join(\" \"));\n    }\n}\n"
  dependsOn:
  - crates/math/matrix/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  isVerificationFile: true
  path: verify/matrix_product/src/main.rs
  requiredBy: []
  timestamp: '2024-03-18 01:19:47+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/matrix_product/src/main.rs
layout: document
redirect_from:
- /verify/verify/matrix_product/src/main.rs
- /verify/verify/matrix_product/src/main.rs.html
title: verify/matrix_product/src/main.rs
---