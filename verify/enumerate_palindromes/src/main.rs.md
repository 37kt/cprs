---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/string/manacher/src/lib.rs
    title: crates/string/manacher/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/enumerate_palindromes
    links:
    - https://judge.yosupo.jp/problem/enumerate_palindromes
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/enumerate_palindromes\n\
    \nuse itertools::Itertools;\nuse manacher::manacher;\nuse proconio::{input, marker::Bytes};\n\
    \n#[proconio::fastout]\nfn main() {\n    input! {\n        s: Bytes,\n    }\n\
    \    let res = manacher(&s);\n    println!(\"{}\", res[1..res.len() - 1].iter().join(\"\
    \ \"));\n}\n"
  dependsOn:
  - crates/string/manacher/src/lib.rs
  isVerificationFile: true
  path: verify/enumerate_palindromes/src/main.rs
  requiredBy: []
  timestamp: '2024-04-14 08:35:43+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/enumerate_palindromes/src/main.rs
layout: document
redirect_from:
- /verify/verify/enumerate_palindromes/src/main.rs
- /verify/verify/enumerate_palindromes/src/main.rs.html
title: verify/enumerate_palindromes/src/main.rs
---
