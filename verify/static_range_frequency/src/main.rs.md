---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/wavelet-matrix/src/lib.rs
    title: crates/data-structure/wavelet-matrix/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/static_range_frequency
    links:
    - https://judge.yosupo.jp/problem/static_range_frequency
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_frequency\n\
    \nuse proconio::input;\nuse wavelet_matrix::WaveletMatrix;\n\n#[proconio::fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        q: usize,\n        a: [usize;\
    \ n],\n    }\n    let wm = WaveletMatrix::new(a);\n    for _ in 0..q {\n     \
    \   input! {\n            l: usize,\n            r: usize,\n            x: usize,\n\
    \        }\n        println!(\"{}\", wm.range_freq(l..r, x..=x));\n    }\n}\n"
  dependsOn:
  - crates/data-structure/wavelet-matrix/src/lib.rs
  isVerificationFile: true
  path: verify/static_range_frequency/src/main.rs
  requiredBy: []
  timestamp: '2023-04-22 21:59:33+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/static_range_frequency/src/main.rs
layout: document
redirect_from:
- /verify/verify/static_range_frequency/src/main.rs
- /verify/verify/static_range_frequency/src/main.rs.html
title: verify/static_range_frequency/src/main.rs
---
