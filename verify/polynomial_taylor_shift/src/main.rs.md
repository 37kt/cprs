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
    PROBLEM: https://judge.yosupo.jp/problem/polynomial_taylor_shift
    links:
    - https://judge.yosupo.jp/problem/polynomial_taylor_shift
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/polynomial_taylor_shift\n\
    \nuse formal_power_series::FormalPowerSeries998244353 as FPS;\nuse modint::ModInt998244353;\n\
    use proconio::input;\n\ntype Mint = ModInt998244353;\n\n#[proconio::fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        c: Mint,\n        a: [Mint;\
    \ n],\n    }\n    let f: FPS = a.into();\n    let f = f.taylor_shift(c);\n   \
    \ println!(\"{}\", f);\n}\n"
  dependsOn:
  - crates/number-theory/modint/src/lib.rs
  - crates/polynomial/formal-power-series/src/lib.rs
  isVerificationFile: true
  path: verify/polynomial_taylor_shift/src/main.rs
  requiredBy: []
  timestamp: '2024-03-18 03:03:26+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/polynomial_taylor_shift/src/main.rs
layout: document
redirect_from:
- /verify/verify/polynomial_taylor_shift/src/main.rs
- /verify/verify/polynomial_taylor_shift/src/main.rs.html
title: verify/polynomial_taylor_shift/src/main.rs
---
