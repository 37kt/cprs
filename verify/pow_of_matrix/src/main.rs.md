---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/math/matrix/src/lib.rs
    title: crates/math/matrix/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/pow_of_matrix
    links:
    - https://judge.yosupo.jp/problem/pow_of_matrix
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.3/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.3/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/pow_of_matrix\n\
    \nuse itertools::Itertools;\nuse matrix::Matrix;\nuse modint::ModInt998244353\
    \ as Mint;\nuse proconio::input;\n\n#[proconio::fastout]\nfn main() {\n    input!\
    \ {\n        n: usize,\n        k: usize,\n        a: [[Mint; n]; n],\n    }\n\
    \    let a = Matrix::<Mint>::from(a);\n    let b = a.pow(k);\n    for i in 0..n\
    \ {\n        println!(\"{}\", b[i].iter().join(\" \"));\n    }\n}\n"
  dependsOn:
  - crates/math/matrix/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  isVerificationFile: true
  path: verify/pow_of_matrix/src/main.rs
  requiredBy: []
  timestamp: '2024-03-18 01:19:47+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/pow_of_matrix/src/main.rs
layout: document
redirect_from:
- /verify/verify/pow_of_matrix/src/main.rs
- /verify/verify/pow_of_matrix/src/main.rs.html
title: verify/pow_of_matrix/src/main.rs
---