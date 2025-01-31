---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/bell-number/src/lib.rs
    title: crates/number-theory/bell-number/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/bell_number
    links:
    - https://judge.yosupo.jp/problem/bell_number
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/bell_number\n\
    \nuse bell_number::bell_number;\nuse proconio::fastout;\nuse proconio::input;\n\
    \n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n    }\n    let b =\
    \ bell_number::<998244353>(n);\n    println!(\"{}\", b);\n}\n"
  dependsOn:
  - crates/number-theory/bell-number/src/lib.rs
  isVerificationFile: true
  path: verify/bell_number/src/main.rs
  requiredBy: []
  timestamp: '2025-01-26 00:19:46+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/bell_number/src/main.rs
layout: document
redirect_from:
- /verify/verify/bell_number/src/main.rs
- /verify/verify/bell_number/src/main.rs.html
title: verify/bell_number/src/main.rs
---
