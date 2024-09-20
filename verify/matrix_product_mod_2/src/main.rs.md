---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/matrix_product_mod_2
    links:
    - https://judge.yosupo.jp/problem/matrix_product_mod_2
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/matrix_product_mod_2\n\
    \nuse matrix_mod2::MatrixMod2;\nuse proconio::{input, marker::Bytes};\n\n#[proconio::fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        m: usize,\n        k: usize,\n\
    \        s: [Bytes; n],\n        t: [Bytes; m],\n    }\n    let mut a = MatrixMod2::new(n,\
    \ m);\n    let mut b = MatrixMod2::new(m, k);\n    for i in 0..n {\n        for\
    \ j in 0..m {\n            if s[i][j] == b'1' {\n                a.set(i, j, true);\n\
    \            }\n        }\n    }\n    for i in 0..m {\n        for j in 0..k {\n\
    \            if t[i][j] == b'1' {\n                b.set(i, j, true);\n      \
    \      }\n        }\n    }\n    let c = &a * &b;\n    print!(\"{}\", c);\n}\n"
  dependsOn: []
  isVerificationFile: true
  path: verify/matrix_product_mod_2/src/main.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/matrix_product_mod_2/src/main.rs
layout: document
redirect_from:
- /verify/verify/matrix_product_mod_2/src/main.rs
- /verify/verify/matrix_product_mod_2/src/main.rs.html
title: verify/matrix_product_mod_2/src/main.rs
---
