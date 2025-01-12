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
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/composition_of_formal_power_series_large/src/main.rs
    title: verify/composition_of_formal_power_series_large/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://nyaannyaan.github.io/library/fps/fps-composition.hpp
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// https://nyaannyaan.github.io/library/fps/fps-composition.hpp\n\n#![allow(non_snake_case)]\n\
    \nuse convolution_ntt_friendly::{ntt, ntt_inv};\nuse formal_power_series::{fps,\
    \ FormalPowerSeries};\n\n/// f(g(x)) mod x^d \u3092\u6C42\u3081\u308B\npub fn\
    \ composition<const M: u32>(\n    f: &FormalPowerSeries<M>,\n    g: &FormalPowerSeries<M>,\n\
    \    d: usize,\n) -> FormalPowerSeries<M> {\n    let mut f = f.clone();\n    let\
    \ mut g = g.clone();\n    f.resize(d, 0.into());\n    g.resize(d, 0.into());\n\
    \    let n = d - 1;\n    let k = 1;\n    let mut h = 1;\n    while h < n + 1 {\n\
    \        h *= 2;\n    }\n    let mut Q = fps![0; h * k];\n    for i in 0..=n {\n\
    \        Q[i] = -g[i];\n    }\n    let mut P = dfs(Q, &f, n, h, k).pre(d);\n \
    \   P.reverse();\n    P\n}\n\nfn dfs<const M: u32>(\n    mut Q: FormalPowerSeries<M>,\n\
    \    f: &FormalPowerSeries<M>,\n    n: usize,\n    h: usize,\n    k: usize,\n\
    ) -> FormalPowerSeries<M> {\n    if n == 0 {\n        let mut T = fps![0; k +\
    \ 1];\n        for i in 0..k {\n            T[i] = Q[i];\n        }\n        T[k]\
    \ += 1;\n        T.reverse();\n        let mut T = T.inv(k + 1);\n        T.reverse();\n\
    \        let u = f * T;\n        let mut P = fps![0; h * k];\n        for i in\
    \ 0..f.len() {\n            P[k - 1 - i] = u[i + k];\n        }\n        return\
    \ P;\n    }\n    let mut nQ = fps![0; 4 * h * k];\n    let mut nR = fps![0; 2\
    \ * h * k];\n    for i in 0..k {\n        for j in 0..=n {\n            nQ[i *\
    \ 2 * h + j] = Q[i * h + j];\n        }\n    }\n    nQ[k * 2 * h] += 1;\n    ntt(&mut\
    \ nQ);\n    for i in (0..4 * h * k).step_by(2) {\n        nQ.swap(i, i + 1);\n\
    \    }\n    for i in 0..2 * h * k {\n        nR[i] = nQ[i * 2] * nQ[i * 2 + 1];\n\
    \    }\n    ntt_inv(&mut nR);\n    nR[0] -= 1;\n    Q = fps![0; h * k];\n    for\
    \ i in 0..2 * k {\n        for j in 0..=n / 2 {\n            Q[i * h / 2 + j]\
    \ = nR[i * h + j];\n        }\n    }\n    let P = dfs(Q, f, n / 2, h / 2, k *\
    \ 2);\n    let mut nP = fps![0; 4 * h * k];\n    for i in 0..2 * k {\n       \
    \ for j in 0..=n / 2 {\n            nP[i * 2 * h + j * 2 + n % 2] = P[i * h /\
    \ 2 + j];\n        }\n    }\n    ntt(&mut nP);\n    let mut i = 1;\n    while\
    \ i < 4 * h * k {\n        nQ[i..i * 2].reverse();\n        i *= 2;\n    }\n \
    \   for i in 0..4 * h * k {\n        nP[i] *= nQ[i];\n    }\n    ntt_inv(&mut\
    \ nP);\n    let mut P = fps![0; h * k];\n    for i in 0..k {\n        for j in\
    \ 0..=n {\n            P[i * h + j] = nP[i * 2 * h + j];\n        }\n    }\n \
    \   P\n}\n"
  dependsOn:
  - crates/convolution/convolution-ntt-friendly/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  - crates/polynomial/formal-power-series/src/lib.rs
  isVerificationFile: false
  path: crates/polynomial/composition/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-12 07:55:53+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/composition_of_formal_power_series_large/src/main.rs
documentation_of: crates/polynomial/composition/src/lib.rs
layout: document
redirect_from:
- /library/crates/polynomial/composition/src/lib.rs
- /library/crates/polynomial/composition/src/lib.rs.html
title: crates/polynomial/composition/src/lib.rs
---
