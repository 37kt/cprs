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
    path: verify/stirling_number_of_the_first_kind/src/main.rs
    title: verify/stirling_number_of_the_first_kind/src/main.rs
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
    \n/// \u7B2C 1 \u7A2E\u30B9\u30BF\u30FC\u30EA\u30F3\u30B0\u6570\n/// (1, 2, ...,\
    \ n) \u306E\u9806\u5217\u3092\u3061\u3087\u3046\u3069 k \u500B\u306E\u5DE1\u56DE\
    \u7F6E\u63DB\u306B\u5206\u89E3\u3059\u308B\u65B9\u6CD5\u306E\u6570\npub fn stirling_first<const\
    \ P: u32>(n: usize) -> FormalPowerSeries<P> {\n    if n == 0 {\n        return\
    \ fps![1];\n    } else if n == 1 {\n        return fps![0, 1];\n    }\n\n    let\
    \ log = 63 - n.leading_zeros() as usize;\n    let mut f = fps![0, 1];\n    for\
    \ i in (0..log).rev() {\n        let m = n >> i;\n        f *= f.clone().taylor_shift((m\
    \ >> 1).into());\n        if m & 1 == 1 {\n            f = (&f << 1) + f * StaticModInt::new(m\
    \ - 1);\n        }\n    }\n\n    f\n}\n\n/// \u7B26\u53F7\u4ED8\u304D\u7B2C 1\
    \ \u7A2E\u30B9\u30BF\u30FC\u30EA\u30F3\u30B0\u6570\npub fn signed_stirling_first<const\
    \ P: u32>(n: usize) -> FormalPowerSeries<P> {\n    let mut f = stirling_first::<P>(n);\n\
    \    for i in 0..=n {\n        f[i] = if (n - i) % 2 == 0 { f[i] } else { -f[i]\
    \ };\n    }\n    f\n}\n"
  dependsOn:
  - crates/number-theory/modint/src/lib.rs
  - crates/polynomial/formal-power-series/src/lib.rs
  isVerificationFile: false
  path: crates/number-theory/stirling-first/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-30 09:13:10+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/stirling_number_of_the_first_kind/src/main.rs
documentation_of: crates/number-theory/stirling-first/src/lib.rs
layout: document
redirect_from:
- /library/crates/number-theory/stirling-first/src/lib.rs
- /library/crates/number-theory/stirling-first/src/lib.rs.html
title: crates/number-theory/stirling-first/src/lib.rs
---
