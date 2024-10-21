---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal-power-series/src/lib.rs
    title: crates/polynomial/formal-power-series/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/division_of_polynomials
    links:
    - https://judge.yosupo.jp/problem/division_of_polynomials
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/division_of_polynomials\n\
    \nuse formal_power_series::FormalPowerSeries;\nuse modint::ModInt998244353;\n\
    use proconio::input;\n\ntype Mint = ModInt998244353;\n\n#[proconio::fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        m: usize,\n        f: [Mint;\
    \ n],\n        g: [Mint; m],\n    }\n    let f = FormalPowerSeries(f);\n    let\
    \ g = FormalPowerSeries(g);\n    let (q, r) = f.div_mod(&g);\n    println!(\"\
    {} {}\", q.len(), r.len());\n    println!(\"{}\", q);\n    println!(\"{}\", r);\n\
    }\n"
  dependsOn:
  - crates/number-theory/modint/src/lib.rs
  - crates/polynomial/formal-power-series/src/lib.rs
  isVerificationFile: true
  path: verify/division_of_polynomials/src/main.rs
  requiredBy: []
  timestamp: '2024-06-13 08:47:29+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/division_of_polynomials/src/main.rs
layout: document
redirect_from:
- /verify/verify/division_of_polynomials/src/main.rs
- /verify/verify/division_of_polynomials/src/main.rs.html
title: verify/division_of_polynomials/src/main.rs
---
