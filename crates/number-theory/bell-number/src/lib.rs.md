---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/combination/src/lib.rs
    title: crates/number-theory/combination/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal-power-series/src/lib.rs
    title: crates/polynomial/formal-power-series/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/bell_number/src/main.rs
    title: verify/bell_number/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use combination::Combination;\nuse formal_power_series::{fps, FormalPowerSeries};\n\
    use modint::StaticModInt;\n\n/// \u30D9\u30EB\u6570  \n/// b(n) := \u5927\u304D\
    \u3055 n \u306E\u96C6\u5408\u3092\u5206\u5272\u3059\u308B\u65B9\u6CD5\u306E\u6570\
    \n/// b(0), b(1), ..., b(n) \u3092\u8FD4\u3059\npub fn bell_number<const P: u32>(n:\
    \ usize) -> FormalPowerSeries<P> {\n    // \u30D9\u30EB\u6570\u306E EGF \u306F\
    \ exp(exp(x) - 1)\n\n    let comb = Combination::<StaticModInt<P>>::new();\n \
    \   comb.expand(n);\n\n    let mut f = fps![0; n + 1];\n    for i in 1..=n {\n\
    \        f[i] = comb.fact_inv(i);\n    }\n\n    let mut f = f.exp(n + 1);\n  \
    \  for i in 0..=n {\n        f[i] *= comb.fact(i);\n    }\n\n    f\n}\n"
  dependsOn:
  - crates/number-theory/combination/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  - crates/polynomial/formal-power-series/src/lib.rs
  isVerificationFile: false
  path: crates/number-theory/bell-number/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-26 00:19:46+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/bell_number/src/main.rs
documentation_of: crates/number-theory/bell-number/src/lib.rs
layout: document
redirect_from:
- /library/crates/number-theory/bell-number/src/lib.rs
- /library/crates/number-theory/bell-number/src/lib.rs.html
title: crates/number-theory/bell-number/src/lib.rs
---
