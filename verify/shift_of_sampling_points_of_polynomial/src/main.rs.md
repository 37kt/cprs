---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal-power-series/src/lib.rs
    title: crates/polynomial/formal-power-series/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/shift-of-sampling-points/src/lib.rs
    title: crates/polynomial/shift-of-sampling-points/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/shift_of_sampling_points_of_polynomial
    links:
    - https://judge.yosupo.jp/problem/shift_of_sampling_points_of_polynomial
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/shift_of_sampling_points_of_polynomial\n\
    \nuse itertools::Itertools;\nuse modint::ModInt998244353;\nuse proconio::input;\n\
    use shift_of_sampling_points::shift_of_sampling_points;\n\ntype Mint = ModInt998244353;\n\
    \n#[proconio::fastout]\nfn main() {\n    input! {\n        n: usize,\n       \
    \ m: usize,\n        c: Mint,\n        ys: [Mint; n],\n    }\n    let res = shift_of_sampling_points(&ys,\
    \ c, m);\n    println!(\"{}\", res.iter().join(\" \"));\n}\n"
  dependsOn:
  - crates/number-theory/modint/src/lib.rs
  - crates/polynomial/formal-power-series/src/lib.rs
  - crates/polynomial/shift-of-sampling-points/src/lib.rs
  isVerificationFile: true
  path: verify/shift_of_sampling_points_of_polynomial/src/main.rs
  requiredBy: []
  timestamp: '2024-06-13 08:47:29+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/shift_of_sampling_points_of_polynomial/src/main.rs
layout: document
redirect_from:
- /verify/verify/shift_of_sampling_points_of_polynomial/src/main.rs
- /verify/verify/shift_of_sampling_points_of_polynomial/src/main.rs.html
title: verify/shift_of_sampling_points_of_polynomial/src/main.rs
---
