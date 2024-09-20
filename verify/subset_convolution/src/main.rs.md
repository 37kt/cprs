---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/subset_convolution
    links:
    - https://judge.yosupo.jp/problem/subset_convolution
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/subset_convolution\n\
    \nuse itertools::Itertools;\nuse modint::ModInt998244353 as Mint;\nuse proconio::input;\n\
    use subset_convolution::subset_convolution;\n\n#[proconio::fastout]\nfn main()\
    \ {\n    input! {\n        n: usize,\n        a: [Mint; 1 << n],\n        b: [Mint;\
    \ 1 << n],\n    }\n    let c = subset_convolution(&a, &b);\n    println!(\"{}\"\
    , c.iter().join(\" \"));\n}\n"
  dependsOn: []
  isVerificationFile: true
  path: verify/subset_convolution/src/main.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/subset_convolution/src/main.rs
layout: document
redirect_from:
- /verify/verify/subset_convolution/src/main.rs
- /verify/verify/subset_convolution/src/main.rs.html
title: verify/subset_convolution/src/main.rs
---
