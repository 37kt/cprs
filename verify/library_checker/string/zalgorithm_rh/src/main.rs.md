---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/string/rolling_hash/src/lib.rs
    title: crates/string/rolling_hash/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/string/rolling_hash/src/monoid.rs
    title: crates/string/rolling_hash/src/monoid.rs
  - icon: ':heavy_check_mark:'
    path: crates/string/rolling_hash/src/sequence.rs
    title: crates/string/rolling_hash/src/sequence.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/zalgorithm
    links:
    - https://judge.yosupo.jp/problem/zalgorithm
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/zalgorithm\n\
    \nuse proconio::{fastout, input, marker::Bytes};\nuse rolling_hash::RollingHashSequence;\n\
    \n#[fastout]\nfn main() {\n    input! {\n        s: Bytes,\n    }\n\n    let rh\
    \ = RollingHashSequence::from_iter(s);\n    for i in 0..rh.len() {\n        print!(\"\
    {} \", rh.lcp(.., &rh, i..));\n    }\n    println!();\n}\n"
  dependsOn:
  - crates/string/rolling_hash/src/lib.rs
  - crates/string/rolling_hash/src/monoid.rs
  - crates/string/rolling_hash/src/sequence.rs
  isVerificationFile: true
  path: verify/library_checker/string/zalgorithm_rh/src/main.rs
  requiredBy: []
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/string/zalgorithm_rh/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/string/zalgorithm_rh/src/main.rs
- /verify/verify/library_checker/string/zalgorithm_rh/src/main.rs.html
title: verify/library_checker/string/zalgorithm_rh/src/main.rs
---
