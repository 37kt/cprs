---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal-power-series/src/lib.rs
    title: crates/polynomial/formal-power-series/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/polynomial_interpolation/src/main.rs
    title: verify/polynomial_interpolation/src/main.rs
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
  code: "use formal_power_series::{fps, FormalPowerSeries};\nuse modint::StaticModInt;\n\
    \n/// \u591A\u9805\u5F0F\u88DC\u9593  \n///\n/// P(x\\[i\\]) = y\\[i\\] \u3068\
    \u306A\u308B\u591A\u9805\u5F0F P(x) \u3092\u6C42\u3081\u308B\u3002\npub fn polynomial_interpolation<const\
    \ P: u32>(\n    x: &[StaticModInt<P>],\n    y: &[StaticModInt<P>],\n) -> FormalPowerSeries<P>\
    \ {\n    assert_eq!(x.len(), y.len());\n    let m = x.len();\n    if m == 0 {\n\
    \        return fps![];\n    }\n    let m2 = 1 << 64 - (m - 1).leading_zeros();\n\
    \    let mut mul = vec![fps![1]; m2 + m2];\n    for i in 0..m {\n        mul[m2\
    \ + i] = fps![-x[i], 1];\n    }\n    for i in (1..m2).rev() {\n        mul[i]\
    \ = &mul[i << 1 | 0] * &mul[i << 1 | 1];\n    }\n    let mut g = vec![fps![];\
    \ m2 + m2];\n    g[1] = mul[1].differential().div_mod(&mul[1]).1;\n    for i in\
    \ 2..m2 + m {\n        g[i] = g[i >> 1].div_mod(&mul[i]).1;\n    }\n    for i\
    \ in 0..m {\n        g[m2 + i] = fps![y[i] / g[m2 + i][0]];\n    }\n    for i\
    \ in (1..m2).rev() {\n        g[i] = &g[i << 1] * &mul[i << 1 | 1] + &g[i << 1\
    \ | 1] * &mul[i << 1];\n    }\n    g[1].clone()\n}\n"
  dependsOn:
  - crates/number-theory/modint/src/lib.rs
  - crates/polynomial/formal-power-series/src/lib.rs
  isVerificationFile: false
  path: crates/polynomial/polynomial-interpolation/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-30 09:13:10+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/polynomial_interpolation/src/main.rs
documentation_of: crates/polynomial/polynomial-interpolation/src/lib.rs
layout: document
redirect_from:
- /library/crates/polynomial/polynomial-interpolation/src/lib.rs
- /library/crates/polynomial/polynomial-interpolation/src/lib.rs.html
title: crates/polynomial/polynomial-interpolation/src/lib.rs
---
