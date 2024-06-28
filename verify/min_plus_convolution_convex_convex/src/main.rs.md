---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algorithm/min-plus-convolution/src/lib.rs
    title: crates/algorithm/min-plus-convolution/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/min_plus_convolution_convex_convex
    links:
    - https://judge.yosupo.jp/problem/min_plus_convolution_convex_convex
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.4/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.4/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/min_plus_convolution_convex_convex\n\
    \nuse itertools::Itertools;\nuse min_plus_convolution::min_plus_convolution_convex_convex;\n\
    use proconio::input;\n\n#[proconio::fastout]\nfn main() {\n    input! {\n    \
    \    n: usize,\n        m: usize,\n        a: [i64; n],\n        b: [i64; m],\n\
    \    }\n    let c = min_plus_convolution_convex_convex(&a, &b);\n    println!(\"\
    {}\", c.iter().join(\" \"));\n}\n"
  dependsOn:
  - crates/algorithm/min-plus-convolution/src/lib.rs
  isVerificationFile: true
  path: verify/min_plus_convolution_convex_convex/src/main.rs
  requiredBy: []
  timestamp: '2024-04-07 09:46:12+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/min_plus_convolution_convex_convex/src/main.rs
layout: document
redirect_from:
- /verify/verify/min_plus_convolution_convex_convex/src/main.rs
- /verify/verify/min_plus_convolution_convex_convex/src/main.rs.html
title: verify/min_plus_convolution_convex_convex/src/main.rs
---
