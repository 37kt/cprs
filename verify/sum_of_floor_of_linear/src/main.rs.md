---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/math/floor-sum/src/lib.rs
    title: crates/math/floor-sum/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/sum_of_floor_of_linear
    links:
    - https://judge.yosupo.jp/problem/sum_of_floor_of_linear
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/sum_of_floor_of_linear\n\
    \nuse floor_sum::floor_sum;\nuse proconio::input;\n\n#[proconio::fastout]\nfn\
    \ main() {\n    input! {\n        t: usize,\n    }\n    for _ in 0..t {\n    \
    \    input! {\n            n: u64,\n            m: u64,\n            a: i64,\n\
    \            b: i64,\n        }\n        println!(\"{}\", floor_sum(n, m, a, b));\n\
    \    }\n}\n"
  dependsOn:
  - crates/math/floor-sum/src/lib.rs
  isVerificationFile: true
  path: verify/sum_of_floor_of_linear/src/main.rs
  requiredBy: []
  timestamp: '2023-04-27 15:09:25+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/sum_of_floor_of_linear/src/main.rs
layout: document
redirect_from:
- /verify/verify/sum_of_floor_of_linear/src/main.rs
- /verify/verify/sum_of_floor_of_linear/src/main.rs.html
title: verify/sum_of_floor_of_linear/src/main.rs
---
