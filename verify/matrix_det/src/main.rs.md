---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/math/matrix/src/lib.rs
    title: crates/math/matrix/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/matrix_det
    links:
    - https://judge.yosupo.jp/problem/matrix_det
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/matrix_det\n\
    \nuse ac_library::ModInt998244353 as Mint;\nuse matrix::Matrix;\nuse proconio::input;\n\
    \n#[proconio::fastout]\nfn main() {\n    input! {\n        n: usize,\n       \
    \ a: [[Mint; n]; n],\n    }\n    let a = Matrix::<Mint>::from(a);\n    let (_,\
    \ _, det) = a.gauss_elimination();\n    println!(\"{}\", det.unwrap());\n}\n"
  dependsOn:
  - crates/math/matrix/src/lib.rs
  isVerificationFile: true
  path: verify/matrix_det/src/main.rs
  requiredBy: []
  timestamp: '2023-06-13 17:39:04+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/matrix_det/src/main.rs
layout: document
redirect_from:
- /verify/verify/matrix_det/src/main.rs
- /verify/verify/matrix_det/src/main.rs.html
title: verify/matrix_det/src/main.rs
---
