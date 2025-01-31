---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/quotients/src/lib.rs
    title: crates/number-theory/quotients/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/enumerate_quotients
    links:
    - https://judge.yosupo.jp/problem/enumerate_quotients
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/enumerate_quotients\n\
    \nuse itertools::Itertools;\nuse proconio::input;\nuse quotients::quotients;\n\
    \n#[proconio::fastout]\nfn main() {\n    input! {\n        n: usize,\n    }\n\
    \    let mut v = quotients(n).collect::<Vec<_>>();\n    v.reverse();\n    println!(\"\
    {}\", v.len());\n    println!(\"{}\", v.iter().join(\" \"));\n}\n"
  dependsOn:
  - crates/number-theory/quotients/src/lib.rs
  isVerificationFile: true
  path: verify/enumerate_quotients/src/main.rs
  requiredBy: []
  timestamp: '2024-12-18 03:30:26+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/enumerate_quotients/src/main.rs
layout: document
redirect_from:
- /verify/verify/enumerate_quotients/src/main.rs
- /verify/verify/enumerate_quotients/src/main.rs.html
title: verify/enumerate_quotients/src/main.rs
---
