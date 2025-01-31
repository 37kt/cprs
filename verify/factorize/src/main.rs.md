---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/math/fast-factorize/src/lib.rs
    title: crates/math/fast-factorize/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/factorize
    links:
    - https://judge.yosupo.jp/problem/factorize
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/factorize\n\
    \nuse fast_factorize::factorize;\nuse proconio::input;\n\n#[proconio::fastout]\n\
    fn main() {\n    input! {\n        q: usize,\n    }\n    for _ in 0..q {\n   \
    \     input! {\n            a: u64,\n        }\n        let x = factorize(a);\n\
    \        print!(\"{}\", x.len());\n        for x in x {\n            print!(\"\
    \ {}\", x);\n        }\n        println!();\n    }\n}\n"
  dependsOn:
  - crates/math/fast-factorize/src/lib.rs
  isVerificationFile: true
  path: verify/factorize/src/main.rs
  requiredBy: []
  timestamp: '2024-12-30 09:13:10+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/factorize/src/main.rs
layout: document
redirect_from:
- /verify/verify/factorize/src/main.rs
- /verify/verify/factorize/src/main.rs.html
title: verify/factorize/src/main.rs
---
