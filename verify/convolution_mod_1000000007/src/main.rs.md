---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution-arbitrary-mod/src/lib.rs
    title: crates/convolution/convolution-arbitrary-mod/src/lib.rs
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
    PROBLEM: https://judge.yosupo.jp/problem/convolution_mod_1000000007
    links:
    - https://judge.yosupo.jp/problem/convolution_mod_1000000007
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/convolution_mod_1000000007\n\
    \nuse convolution_arbitrary_mod::convolution_arbitrary_mod;\nuse itertools::Itertools;\n\
    use modint::ModInt1000000007 as Mint;\nuse proconio::input;\n\n#[proconio::fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        m: usize,\n        mut a:\
    \ [Mint; n],\n        mut b: [Mint; m],\n    }\n    let c = convolution_arbitrary_mod(&a,\
    \ &b);\n    println!(\"{}\", c.iter().join(\" \"));\n}\n"
  dependsOn:
  - crates/convolution/convolution-arbitrary-mod/src/lib.rs
  - crates/convolution/convolution-ntt-friendly/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  isVerificationFile: true
  path: verify/convolution_mod_1000000007/src/main.rs
  requiredBy: []
  timestamp: '2024-06-13 08:47:29+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/convolution_mod_1000000007/src/main.rs
layout: document
redirect_from:
- /verify/verify/convolution_mod_1000000007/src/main.rs
- /verify/verify/convolution_mod_1000000007/src/main.rs.html
title: verify/convolution_mod_1000000007/src/main.rs
---
