---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  - icon: ':question:'
    path: crates/polynomial/formal-power-series/src/lib.rs
    title: crates/polynomial/formal-power-series/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/sqrt_of_formal_power_series
    links:
    - https://judge.yosupo.jp/problem/sqrt_of_formal_power_series
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/sqrt_of_formal_power_series\n\
    \nuse formal_power_series::FormalPowerSeries;\nuse itertools::Itertools;\nuse\
    \ modint::ModInt998244353;\nuse proconio::input;\n\ntype Mint = ModInt998244353;\n\
    \n#[proconio::fastout]\nfn main() {\n    input! {\n        n: usize,\n       \
    \ a: [Mint; n],\n    }\n    let a = FormalPowerSeries(a);\n    if let Some(b)\
    \ = a.sqrt(n) {\n        println!(\"{}\", b.iter().join(\" \"));\n    } else {\n\
    \        println!(\"-1\");\n    }\n}\n"
  dependsOn:
  - crates/number-theory/modint/src/lib.rs
  - crates/polynomial/formal-power-series/src/lib.rs
  isVerificationFile: true
  path: verify/sqrt_of_formal_power_series/src/main.rs
  requiredBy: []
  timestamp: '2024-03-18 01:19:47+09:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: verify/sqrt_of_formal_power_series/src/main.rs
layout: document
redirect_from:
- /verify/verify/sqrt_of_formal_power_series/src/main.rs
- /verify/verify/sqrt_of_formal_power_series/src/main.rs.html
title: verify/sqrt_of_formal_power_series/src/main.rs
---