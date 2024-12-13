---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/partition/src/lib.rs
    title: crates/number-theory/partition/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/partition_function
    links:
    - https://judge.yosupo.jp/problem/partition_function
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/partition_function\n\
    \nuse partition::partition;\nuse proconio::input;\n\n#[proconio::fastout]\nfn\
    \ main() {\n    input! {\n        n: usize,\n    }\n    let p = partition::<998244353>(n\
    \ + 1);\n    println!(\"{}\", p);\n}\n"
  dependsOn:
  - crates/number-theory/partition/src/lib.rs
  isVerificationFile: true
  path: verify/partition_function/src/main.rs
  requiredBy: []
  timestamp: '2024-12-12 07:56:32+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/partition_function/src/main.rs
layout: document
redirect_from:
- /verify/verify/partition_function/src/main.rs
- /verify/verify/partition_function/src/main.rs.html
title: verify/partition_function/src/main.rs
---
