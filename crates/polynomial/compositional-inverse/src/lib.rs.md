---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution-ntt-friendly/src/lib.rs
    title: crates/convolution/convolution-ntt-friendly/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal-power-series/src/lib.rs
    title: crates/polynomial/formal-power-series/src/lib.rs
  - icon: ':warning:'
    path: crates/polynomial/power-projection/src/lib.rs
    title: crates/polynomial/power-projection/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/compositional_inverse_of_formal_power_series_large/src/main.rs
    title: verify/compositional_inverse_of_formal_power_series_large/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://nyaannyaan.github.io/library/fps/fps-compositional-inverse.hpp
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// https://nyaannyaan.github.io/library/fps/fps-compositional-inverse.hpp\n\
    \nuse formal_power_series::{fps, FormalPowerSeries};\nuse modint::StaticModInt;\n\
    use power_projection::power_projection;\n\n/// f(g(x)) = x \u3092\u6E80\u305F\u3059\
    \ g(x) mod x^d \u3092\u6C42\u3081\u308B\npub fn compositional_inverse<const P:\
    \ u32>(\n    f: &FormalPowerSeries<P>,\n    d: usize,\n) -> FormalPowerSeries<P>\
    \ {\n    assert!(f.len() >= 2 && f[1].val() != 0);\n    if d < 2 {\n        return\
    \ fps![0, f[1].inv()].pre(d);\n    }\n    let n = d - 1;\n    let mut h: FormalPowerSeries<P>\
    \ = power_projection(f, &fps![1], n) * StaticModInt::new(n);\n    for k in 1..=n\
    \ {\n        h[k] /= k;\n    }\n    h.reverse();\n    h *= h[0].inv();\n    let\
    \ mut g = (h.log(d) * (-StaticModInt::new(n)).inv()).exp(d);\n    g *= f[1].inv();\n\
    \    (g << 1).pre(d)\n}\n"
  dependsOn:
  - crates/convolution/convolution-ntt-friendly/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  - crates/polynomial/formal-power-series/src/lib.rs
  - crates/polynomial/power-projection/src/lib.rs
  isVerificationFile: false
  path: crates/polynomial/compositional-inverse/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-12 06:57:08+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/compositional_inverse_of_formal_power_series_large/src/main.rs
documentation_of: crates/polynomial/compositional-inverse/src/lib.rs
layout: document
redirect_from:
- /library/crates/polynomial/compositional-inverse/src/lib.rs
- /library/crates/polynomial/compositional-inverse/src/lib.rs.html
title: crates/polynomial/compositional-inverse/src/lib.rs
---
