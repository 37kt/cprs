---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal-power-series/src/lib.rs
    title: crates/polynomial/formal-power-series/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/polynomial-interpolation/src/lib.rs
    title: crates/polynomial/polynomial-interpolation/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/polynomial_interpolation
    links:
    - https://judge.yosupo.jp/problem/polynomial_interpolation
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/polynomial_interpolation\n\
    \nuse modint::ModInt998244353;\nuse polynomial_interpolation::polynomial_interpolation;\n\
    use proconio::input;\n\ntype Mint = ModInt998244353;\n\n#[proconio::fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        x: [Mint; n],\n        y:\
    \ [Mint; n],\n    }\n    let f = polynomial_interpolation(&x, &y);\n    println!(\"\
    {}\", f);\n}\n"
  dependsOn:
  - crates/number-theory/modint/src/lib.rs
  - crates/polynomial/formal-power-series/src/lib.rs
  - crates/polynomial/polynomial-interpolation/src/lib.rs
  isVerificationFile: true
  path: verify/polynomial_interpolation/src/main.rs
  requiredBy: []
  timestamp: '2024-12-30 09:13:10+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/polynomial_interpolation/src/main.rs
layout: document
redirect_from:
- /verify/verify/polynomial_interpolation/src/main.rs
- /verify/verify/polynomial_interpolation/src/main.rs.html
title: verify/polynomial_interpolation/src/main.rs
---
