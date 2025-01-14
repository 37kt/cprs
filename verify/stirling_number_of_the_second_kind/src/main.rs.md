---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/stirling-second/src/lib.rs
    title: crates/number-theory/stirling-second/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/stirling_number_of_the_second_kind
    links:
    - https://judge.yosupo.jp/problem/stirling_number_of_the_second_kind
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/stirling_number_of_the_second_kind\n\
    \nuse proconio::input;\nuse stirling_second::stirling_second;\n\n#[proconio::fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n    }\n    let st = stirling_second::<998244353>(n);\n\
    \    println!(\"{}\", st);\n}\n"
  dependsOn:
  - crates/number-theory/modint/src/lib.rs
  - crates/number-theory/stirling-second/src/lib.rs
  isVerificationFile: true
  path: verify/stirling_number_of_the_second_kind/src/main.rs
  requiredBy: []
  timestamp: '2024-12-30 09:13:10+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/stirling_number_of_the_second_kind/src/main.rs
layout: document
redirect_from:
- /verify/verify/stirling_number_of_the_second_kind/src/main.rs
- /verify/verify/stirling_number_of_the_second_kind/src/main.rs.html
title: verify/stirling_number_of_the_second_kind/src/main.rs
---
