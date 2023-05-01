---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/sum_of_floor_of_linear
    links:
    - https://judge.yosupo.jp/problem/sum_of_floor_of_linear
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/sum_of_floor_of_linear\n\
    \nuse floor_sum::floor_sum;\nuse proconio::input;\n\n#[proconio::fastout]\nfn\
    \ main() {\n    input! {\n        t: usize,\n    }\n    for _ in 0..t {\n    \
    \    input! {\n            n: u64,\n            m: u64,\n            a: i64,\n\
    \            b: i64,\n        }\n        println!(\"{}\", floor_sum(n, m, a, b));\n\
    \    }\n}\n"
  dependsOn: []
  isVerificationFile: true
  path: verify/sum_of_floor_of_linear/src/main.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: verify/sum_of_floor_of_linear/src/main.rs
layout: document
redirect_from:
- /verify/verify/sum_of_floor_of_linear/src/main.rs
- /verify/verify/sum_of_floor_of_linear/src/main.rs.html
title: verify/sum_of_floor_of_linear/src/main.rs
---
