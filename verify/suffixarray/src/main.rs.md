---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/string/suffix-array/src/lib.rs
    title: crates/string/suffix-array/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/suffixarray
    links:
    - https://judge.yosupo.jp/problem/suffixarray
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/suffixarray\n\
    \nuse itertools::Itertools;\nuse proconio::{input, marker::Bytes};\nuse suffix_array::SuffixArray;\n\
    \n#[proconio::fastout]\nfn main() {\n    input! {\n        s: Bytes,\n    }\n\
    \    let sa = SuffixArray::build(&s);\n    println!(\"{}\", sa.suffix_array().iter().join(\"\
    \ \"));\n}\n"
  dependsOn:
  - crates/string/suffix-array/src/lib.rs
  isVerificationFile: true
  path: verify/suffixarray/src/main.rs
  requiredBy: []
  timestamp: '2024-12-25 08:18:46+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/suffixarray/src/main.rs
layout: document
redirect_from:
- /verify/verify/suffixarray/src/main.rs
- /verify/verify/suffixarray/src/main.rs.html
title: verify/suffixarray/src/main.rs
---
