---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/xor-convolution/src/lib.rs
    title: crates/convolution/xor-convolution/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/bitwise_xor_convolution
    links:
    - https://judge.yosupo.jp/problem/bitwise_xor_convolution
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/bitwise_xor_convolution\n\
    \nuse ac_library::ModInt998244353 as Mint;\nuse itertools::Itertools;\nuse proconio::input;\n\
    use xor_convolution::xor_convolution;\n\n#[proconio::fastout]\nfn main() {\n \
    \   input! {\n        n: usize,\n        a: [Mint; 1 << n],\n        b: [Mint;\
    \ 1 << n],\n    }\n    let c = xor_convolution(a, b);\n    println!(\"{}\", c.iter().join(\"\
    \ \"));\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/convolution/xor-convolution/src/lib.rs
  isVerificationFile: true
  path: verify/bitwise_xor_convolution/src/main.rs
  requiredBy: []
  timestamp: '2025-01-04 02:49:00+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/bitwise_xor_convolution/src/main.rs
layout: document
redirect_from:
- /verify/verify/bitwise_xor_convolution/src/main.rs
- /verify/verify/bitwise_xor_convolution/src/main.rs.html
title: verify/bitwise_xor_convolution/src/main.rs
---
