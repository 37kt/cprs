---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/string/z_algorithm/src/lib.rs
    title: crates/string/z_algorithm/src/lib.rs
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
    \nuse proconio::{fastout, input, marker::Bytes};\nuse z_algorithm::z_algorithm;\n\
    \n#[fastout]\nfn main() {\n    input! {\n        s: Bytes,\n    }\n\n    let z\
    \ = z_algorithm(&s);\n    for &x in &z {\n        print!(\"{} \", x);\n    }\n\
    \    println!();\n}\n"
  dependsOn:
  - crates/string/z_algorithm/src/lib.rs
  isVerificationFile: true
  path: verify/library_checker/string/zalgorithm/src/main.rs
  requiredBy: []
  timestamp: '2025-03-16 01:29:32+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/string/zalgorithm/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/string/zalgorithm/src/main.rs
- /verify/verify/library_checker/string/zalgorithm/src/main.rs.html
title: verify/library_checker/string/zalgorithm/src/main.rs
---
