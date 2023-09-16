---
data:
  _extendedDependsOn:
  - icon: ':x:'
    path: crates/string/z-algorithm/src/lib.rs
    title: crates/string/z-algorithm/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/zalgorithm
    links:
    - https://judge.yosupo.jp/problem/zalgorithm
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.5/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.5/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/zalgorithm\n\
    \nuse itertools::Itertools;\nuse proconio::{input, marker::Bytes};\nuse z_algorithm::z_algorithm;\n\
    \n#[proconio::fastout]\nfn main() {\n    input! {\n        s: Bytes,\n    }\n\
    \    let z = z_algorithm(&s);\n    println!(\"{}\", z.iter().join(\" \"));\n}\n"
  dependsOn:
  - crates/string/z-algorithm/src/lib.rs
  isVerificationFile: true
  path: verify/zalgorithm/src/main.rs
  requiredBy: []
  timestamp: '2023-04-27 14:07:11+09:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: verify/zalgorithm/src/main.rs
layout: document
redirect_from:
- /verify/verify/zalgorithm/src/main.rs
- /verify/verify/zalgorithm/src/main.rs.html
title: verify/zalgorithm/src/main.rs
---
