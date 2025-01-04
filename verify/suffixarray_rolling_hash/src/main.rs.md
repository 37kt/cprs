---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/string/rolling-hash/src/lib.rs
    title: crates/string/rolling-hash/src/lib.rs
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
    \nuse itertools::Itertools;\nuse proconio::{input, marker::Bytes};\nuse rolling_hash::RollingHash;\n\
    \n#[proconio::fastout]\nfn main() {\n    input! {\n        s: Bytes,\n    }\n\
    \    let rh = RollingHash::new();\n    let rhs = rh.build_table(&s);\n    let\
    \ mut res = (0..s.len()).sorted_by(|i, j| rhs.compare(i.., &rhs, j..));\n    println!(\"\
    {}\", res.join(\" \"));\n}\n"
  dependsOn:
  - crates/string/rolling-hash/src/lib.rs
  isVerificationFile: true
  path: verify/suffixarray_rolling_hash/src/main.rs
  requiredBy: []
  timestamp: '2025-01-04 02:49:00+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/suffixarray_rolling_hash/src/main.rs
layout: document
redirect_from:
- /verify/verify/suffixarray_rolling_hash/src/main.rs
- /verify/verify/suffixarray_rolling_hash/src/main.rs.html
title: verify/suffixarray_rolling_hash/src/main.rs
---
