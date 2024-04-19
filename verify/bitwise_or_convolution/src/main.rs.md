---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/convolution/or-convolution/src/lib.rs
    title: crates/convolution/or-convolution/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/bitwise_and_convolution
    links:
    - https://judge.yosupo.jp/problem/bitwise_and_convolution
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.3/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.3/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/bitwise_and_convolution\n\
    \nuse itertools::Itertools;\nuse modint::ModInt998244353 as Mint;\nuse or_convolution::or_convolution;\n\
    use proconio::input;\n\n#[proconio::fastout]\nfn main() {\n    input! {\n    \
    \    n: usize,\n        mut a: [Mint; 1 << n],\n        mut b: [Mint; 1 << n],\n\
    \    }\n    a.reverse();\n    b.reverse();\n    let c = or_convolution(a, b);\n\
    \    println!(\"{}\", c.iter().rev().join(\" \"));\n}\n"
  dependsOn:
  - crates/convolution/or-convolution/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  isVerificationFile: true
  path: verify/bitwise_or_convolution/src/main.rs
  requiredBy: []
  timestamp: '2024-03-18 01:19:47+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/bitwise_or_convolution/src/main.rs
layout: document
redirect_from:
- /verify/verify/bitwise_or_convolution/src/main.rs
- /verify/verify/bitwise_or_convolution/src/main.rs.html
title: verify/bitwise_or_convolution/src/main.rs
---
