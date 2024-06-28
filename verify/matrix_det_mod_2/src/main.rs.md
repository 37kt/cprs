---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/math/matrix-mod2/src/lib.rs
    title: crates/math/matrix-mod2/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/matrix_det_mod_2
    links:
    - https://judge.yosupo.jp/problem/matrix_det_mod_2
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.4/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.4/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/matrix_det_mod_2\n\
    \nuse matrix_mod2::MatrixMod2;\nuse proconio::{input, marker::Bytes};\n\n#[proconio::fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        s: [Bytes; n],\n    }\n\
    \    let mut a = MatrixMod2::new(n, n);\n    for i in 0..n {\n        for j in\
    \ 0..n {\n            if s[i][j] == b'1' {\n                a.set(i, j, true);\n\
    \            }\n        }\n    }\n    println!(\"{}\", a.det() as u8);\n}\n"
  dependsOn:
  - crates/math/matrix-mod2/src/lib.rs
  isVerificationFile: true
  path: verify/matrix_det_mod_2/src/main.rs
  requiredBy: []
  timestamp: '2024-03-21 13:40:37+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/matrix_det_mod_2/src/main.rs
layout: document
redirect_from:
- /verify/verify/matrix_det_mod_2/src/main.rs
- /verify/verify/matrix_det_mod_2/src/main.rs.html
title: verify/matrix_det_mod_2/src/main.rs
---
