---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/convolution/lcm-convolution/src/lib.rs
    title: crates/convolution/lcm-convolution/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/lcm_convolution
    links:
    - https://judge.yosupo.jp/problem/lcm_convolution
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/lcm_convolution\n\
    \nuse ac_library::ModInt998244353 as Mint;\nuse itertools::Itertools;\nuse lcm_convolution::lcm_convolution;\n\
    use proconio::input;\n\n#[proconio::fastout]\nfn main() {\n    input! {\n    \
    \    n: usize,\n        mut a: [Mint; n],\n        mut b: [Mint; n],\n    }\n\
    \    a.insert(0, 0.into());\n    b.insert(0, 0.into());\n    let c = lcm_convolution(a,\
    \ b);\n    println!(\"{}\", c[1..].iter().join(\" \"));\n}\n"
  dependsOn:
  - crates/convolution/lcm-convolution/src/lib.rs
  isVerificationFile: true
  path: verify/lcm_convolution/src/main.rs
  requiredBy: []
  timestamp: '2023-04-21 13:06:23+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/lcm_convolution/src/main.rs
layout: document
redirect_from:
- /verify/verify/lcm_convolution/src/main.rs
- /verify/verify/lcm_convolution/src/main.rs.html
title: verify/lcm_convolution/src/main.rs
---