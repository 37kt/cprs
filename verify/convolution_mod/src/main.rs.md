---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution-ntt-friendly/src/lib.rs
    title: crates/convolution/convolution-ntt-friendly/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/convolution_mod
    links:
    - https://judge.yosupo.jp/problem/convolution_mod
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.3/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.3/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/convolution_mod\n\
    \nuse convolution_ntt_friendly::convolution_ntt_friendly;\nuse itertools::Itertools;\n\
    use modint::ModInt998244353 as Mint;\nuse proconio::input;\n\n#[proconio::fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        m: usize,\n        mut a:\
    \ [Mint; n],\n        mut b: [Mint; m],\n    }\n    let c = convolution_ntt_friendly(a,\
    \ b);\n    println!(\"{}\", c.iter().join(\" \"));\n}\n"
  dependsOn:
  - crates/convolution/convolution-ntt-friendly/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  isVerificationFile: true
  path: verify/convolution_mod/src/main.rs
  requiredBy: []
  timestamp: '2024-03-18 01:19:47+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/convolution_mod/src/main.rs
layout: document
redirect_from:
- /verify/verify/convolution_mod/src/main.rs
- /verify/verify/convolution_mod/src/main.rs.html
title: verify/convolution_mod/src/main.rs
---
