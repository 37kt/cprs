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
    path: verify/stirling_number_of_the_second_kind/src/main.rs
    title: verify/stirling_number_of_the_second_kind/src/main.rs
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
    use modint::StaticModInt;\n\n/// \u7B2C 2 \u7A2E\u30B9\u30BF\u30FC\u30EA\u30F3\
    \u30B0\u6570  \n/// n \u500B\u306E\u533A\u5225\u3067\u304D\u308B\u3082\u306E\u3092\
    \ k \u500B\u306E\u533A\u5225\u3067\u304D\u306A\u3044\u7BB1\u306B\u5206\u5272\u3059\
    \u308B\u65B9\u6CD5\u306E\u6570\n/// S(0, k), S(1, k), ..., S(n, k) \u3092\u8FD4\
    \u3059\npub fn stirling_second<const P: u32>(n: usize) -> FormalPowerSeries<P>\
    \ {\n    let comb = Combination::<StaticModInt<P>>::new();\n    let mut f = fps![0;\
    \ n + 1];\n    let mut g = fps![0; n + 1];\n    for i in 0..=n {\n        f[i]\
    \ = StaticModInt::from(i).pow(n) * comb.fact_inv(i);\n        g[i] = if i & 1\
    \ == 1 {\n            -comb.fact_inv(i)\n        } else {\n            comb.fact_inv(i)\n\
    \        };\n    }\n    (f * g).pre(n + 1)\n}\n"
  dependsOn:
  - crates/number-theory/combination/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  - crates/polynomial/formal-power-series/src/lib.rs
  isVerificationFile: false
  path: crates/number-theory/stirling-second/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-24 03:04:37+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/stirling_number_of_the_second_kind/src/main.rs
documentation_of: crates/number-theory/stirling-second/src/lib.rs
layout: document
redirect_from:
- /library/crates/number-theory/stirling-second/src/lib.rs
- /library/crates/number-theory/stirling-second/src/lib.rs.html
title: crates/number-theory/stirling-second/src/lib.rs
---
