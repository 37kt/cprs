---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/composition/src/lib.rs
    title: crates/polynomial/composition/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal-power-series/src/lib.rs
    title: crates/polynomial/formal-power-series/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/composition_of_formal_power_series_large
    links:
    - https://judge.yosupo.jp/problem/composition_of_formal_power_series_large
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/composition_of_formal_power_series_large\n\
    \nuse composition::composition;\nuse formal_power_series::FormalPowerSeries;\n\
    use modint::ModInt998244353;\nuse proconio::fastout;\nuse proconio::input;\n\n\
    type Mint = ModInt998244353;\n\n#[fastout]\nfn main() {\n    input! {\n      \
    \  n: usize,\n        a: [Mint; n],\n        b: [Mint; n],\n    }\n    let f =\
    \ FormalPowerSeries(a);\n    let g = FormalPowerSeries(b);\n    let h = composition(&f,\
    \ &g, n);\n    println!(\"{}\", h);\n}\n"
  dependsOn:
  - crates/number-theory/modint/src/lib.rs
  - crates/polynomial/composition/src/lib.rs
  - crates/polynomial/formal-power-series/src/lib.rs
  isVerificationFile: true
  path: verify/composition_of_formal_power_series_large/src/main.rs
  requiredBy: []
  timestamp: '2025-01-12 07:55:53+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/composition_of_formal_power_series_large/src/main.rs
layout: document
redirect_from:
- /verify/verify/composition_of_formal_power_series_large/src/main.rs
- /verify/verify/composition_of_formal_power_series_large/src/main.rs.html
title: verify/composition_of_formal_power_series_large/src/main.rs
---