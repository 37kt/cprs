---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/many_aplusb
    links:
    - https://judge.yosupo.jp/problem/many_aplusb
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/many_aplusb\n\
    \nuse proconio::{fastout, input};\n\n#[fastout]\nfn main() {\n    input! {\n \
    \       t: usize,\n    }\n    for _ in 0..t {\n        input! {\n            a:\
    \ u64,\n            b: u64,\n        }\n        println!(\"{}\", a + b);\n   \
    \ }\n}\n"
  dependsOn: []
  isVerificationFile: true
  path: verify/library_checker/sample/many_aplusb/src/main.rs
  requiredBy: []
  timestamp: '2025-03-09 11:49:32+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/sample/many_aplusb/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/sample/many_aplusb/src/main.rs
- /verify/verify/library_checker/sample/many_aplusb/src/main.rs.html
title: verify/library_checker/sample/many_aplusb/src/main.rs
---
