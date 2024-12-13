---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/bostan-mori/src/lib.rs
    title: crates/polynomial/bostan-mori/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal-power-series/src/lib.rs
    title: crates/polynomial/formal-power-series/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/kth_term_of_linearly_recurrent_sequence
    links:
    - https://judge.yosupo.jp/problem/kth_term_of_linearly_recurrent_sequence
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/kth_term_of_linearly_recurrent_sequence\n\
    \nuse bostan_mori::bostan_mori;\nuse formal_power_series::{fps, FormalPowerSeries};\n\
    use modint::ModInt998244353;\nuse proconio::input;\n\ntype Mint = ModInt998244353;\n\
    \n#[proconio::fastout]\nfn main() {\n    input! {\n        d: usize,\n       \
    \ k: usize,\n        a: [Mint; d],\n        c: [Mint; d],\n    }\n    let a =\
    \ FormalPowerSeries(a);\n    let c = FormalPowerSeries(c);\n    let c = fps![1]\
    \ - (c << 1);\n    let res = bostan_mori((&a * &c).pre(d), c, k);\n    println!(\"\
    {}\", res);\n}\n"
  dependsOn:
  - crates/number-theory/modint/src/lib.rs
  - crates/polynomial/bostan-mori/src/lib.rs
  - crates/polynomial/formal-power-series/src/lib.rs
  isVerificationFile: true
  path: verify/kth_term_of_linearly_recurrent_sequence/src/main.rs
  requiredBy: []
  timestamp: '2024-11-25 09:47:32+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/kth_term_of_linearly_recurrent_sequence/src/main.rs
layout: document
redirect_from:
- /verify/verify/kth_term_of_linearly_recurrent_sequence/src/main.rs
- /verify/verify/kth_term_of_linearly_recurrent_sequence/src/main.rs.html
title: verify/kth_term_of_linearly_recurrent_sequence/src/main.rs
---
