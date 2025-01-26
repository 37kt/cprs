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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use formal_power_series::FormalPowerSeries;\nuse pentagonal_number_theorem::pentagonal_number_theorem;\n\
    \n/// \u5206\u5272\u6570 p(0), p(1), ... p(n) \u3092\u5217\u6319\u3059\u308B \
    \ \n/// \u5206\u5272\u6570 p(n) \u306F\u3001 n \u3092\u9806\u756A\u306E\u9055\u3044\
    \u3092\u9664\u3044\u3066\u81EA\u7136\u6570\u306E\u548C\u3068\u3057\u3066\u8868\
    \u3059\u65B9\u6CD5\u306E\u6570\npub fn partition<const P: u32>(n: usize) -> FormalPowerSeries<P>\
    \ {\n    let f = FormalPowerSeries(pentagonal_number_theorem(n));\n    f.inv(n\
    \ + 1)\n}\n"
  dependsOn:
  - crates/number-theory/pentagonal-number-theorem/src/lib.rs
  - crates/polynomial/formal-power-series/src/lib.rs
  isVerificationFile: false
  path: crates/number-theory/partition/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-26 00:01:59+00:00'
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
