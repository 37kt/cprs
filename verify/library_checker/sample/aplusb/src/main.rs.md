---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/aplusb
    links:
    - https://judge.yosupo.jp/problem/aplusb
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/aplusb\n\n\
    use proconio::{fastout, input};\n\n#[fastout]\nfn main() {\n    input! {\n   \
    \     a: i32,\n        b: i32,\n    }\n    println!(\"{}\", a + b);\n}\n"
  dependsOn: []
  isVerificationFile: true
  path: verify/library_checker/sample/aplusb/src/main.rs
  requiredBy: []
  timestamp: '2025-03-12 07:36:02+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/sample/aplusb/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/sample/aplusb/src/main.rs
- /verify/verify/library_checker/sample/aplusb/src/main.rs.html
title: verify/library_checker/sample/aplusb/src/main.rs
---
