---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/math/nimber/src/lib.rs
    title: crates/math/nimber/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/nim_product_64
    links:
    - https://judge.yosupo.jp/problem/nim_product_64
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.4/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.4/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/nim_product_64\n\
    \nuse nimber::Nimber;\nuse proconio::input;\n\n#[proconio::fastout]\nfn main()\
    \ {\n    input! {\n        q: usize,\n    }\n    for _ in 0..q {\n        input!\
    \ {\n            a: Nimber,\n            b: Nimber,\n        }\n        println!(\"\
    {}\", a * b);\n    }\n}\n"
  dependsOn:
  - crates/math/nimber/src/lib.rs
  isVerificationFile: true
  path: verify/nim_product_64/src/main.rs
  requiredBy: []
  timestamp: '2023-05-19 16:25:08+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/nim_product_64/src/main.rs
layout: document
redirect_from:
- /verify/verify/nim_product_64/src/main.rs
- /verify/verify/nim_product_64/src/main.rs.html
title: verify/nim_product_64/src/main.rs
---
