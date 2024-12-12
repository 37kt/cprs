---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/number-theory/pentagonal-number-theorem/src/lib.rs
    title: crates/number-theory/pentagonal-number-theorem/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal-power-series/src/lib.rs
    title: crates/polynomial/formal-power-series/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/partition_function/src/main.rs
    title: verify/partition_function/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use formal_power_series::FormalPowerSeries;\nuse pentagonal_number_theorem::pentagonal_number_theorem;\n\
    \npub fn partition<const P: u32>(n: usize) -> FormalPowerSeries<P> {\n    let\
    \ f = FormalPowerSeries(pentagonal_number_theorem(n));\n    f.inv(n)\n}\n"
  dependsOn:
  - crates/number-theory/pentagonal-number-theorem/src/lib.rs
  - crates/polynomial/formal-power-series/src/lib.rs
  isVerificationFile: false
  path: crates/number-theory/partition/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-12 07:56:32+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/partition_function/src/main.rs
documentation_of: crates/number-theory/partition/src/lib.rs
layout: document
redirect_from:
- /library/crates/number-theory/partition/src/lib.rs
- /library/crates/number-theory/partition/src/lib.rs.html
title: crates/number-theory/partition/src/lib.rs
---
