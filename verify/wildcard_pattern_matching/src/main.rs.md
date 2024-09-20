---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/wildcard_pattern_matching
    links:
    - https://judge.yosupo.jp/problem/wildcard_pattern_matching
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/wildcard_pattern_matching\n\
    \nuse itertools::Itertools;\nuse proconio::{input, marker::Bytes};\nuse wildcard_pattern_matching::wildcard_pattern_matching;\n\
    \n#[proconio::fastout]\nfn main() {\n    input! {\n        s: Bytes,\n       \
    \ t: Bytes,\n    }\n    let res = wildcard_pattern_matching(&s, &t, b'*');\n \
    \   println!(\"{}\", res.iter().map(|&f| f as u8).join(\"\"));\n}\n"
  dependsOn: []
  isVerificationFile: true
  path: verify/wildcard_pattern_matching/src/main.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/wildcard_pattern_matching/src/main.rs
layout: document
redirect_from:
- /verify/verify/wildcard_pattern_matching/src/main.rs
- /verify/verify/wildcard_pattern_matching/src/main.rs.html
title: verify/wildcard_pattern_matching/src/main.rs
---
