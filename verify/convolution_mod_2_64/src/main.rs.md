---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution-u64/src/lib.rs
    title: crates/convolution/convolution-u64/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/convolution_mod_2_64
    links:
    - https://judge.yosupo.jp/problem/convolution_mod_2_64
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/convolution_mod_2_64\n\
    \nuse convolution_u64::convolution_u64;\nuse itertools::Itertools;\nuse proconio::input;\n\
    \n#[proconio::fastout]\nfn main() {\n    input! {\n        n: usize,\n       \
    \ m: usize,\n        mut a: [u64; n],\n        mut b: [u64; m],\n    }\n    let\
    \ c = convolution_u64(&a, &b);\n    println!(\"{}\", c.iter().join(\" \"));\n\
    }\n"
  dependsOn:
  - crates/convolution/convolution-u64/src/lib.rs
  isVerificationFile: true
  path: verify/convolution_mod_2_64/src/main.rs
  requiredBy: []
  timestamp: '2024-12-25 07:02:27+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/convolution_mod_2_64/src/main.rs
layout: document
redirect_from:
- /verify/verify/convolution_mod_2_64/src/main.rs
- /verify/verify/convolution_mod_2_64/src/main.rs.html
title: verify/convolution_mod_2_64/src/main.rs
---
