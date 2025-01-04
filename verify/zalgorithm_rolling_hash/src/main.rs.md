---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/math/modint61/src/lib.rs
    title: crates/math/modint61/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/string/rolling-hash/src/lib.rs
    title: crates/string/rolling-hash/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/zalgorithm
    links:
    - https://judge.yosupo.jp/problem/zalgorithm
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/zalgorithm\n\
    \nuse itertools::Itertools;\nuse proconio::{input, marker::Bytes};\nuse rolling_hash::RollingHash;\n\
    \n#[proconio::fastout]\nfn main() {\n    input! {\n        s: Bytes,\n    }\n\
    \    let rh = RollingHash::new();\n    let rhs = rh.build_table(&s);\n    let\
    \ res = (0..s.len()).map(|i| rhs.lcp(.., &rhs, i..)).collect_vec();\n    println!(\"\
    {}\", res.iter().join(\" \"));\n}\n"
  dependsOn:
  - crates/math/modint61/src/lib.rs
  - crates/string/rolling-hash/src/lib.rs
  isVerificationFile: true
  path: verify/zalgorithm_rolling_hash/src/main.rs
  requiredBy: []
  timestamp: '2025-01-04 02:49:00+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/zalgorithm_rolling_hash/src/main.rs
layout: document
redirect_from:
- /verify/verify/zalgorithm_rolling_hash/src/main.rs
- /verify/verify/zalgorithm_rolling_hash/src/main.rs.html
title: verify/zalgorithm_rolling_hash/src/main.rs
---
