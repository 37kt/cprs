---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution-ntt-friendly/src/lib.rs
    title: crates/convolution/convolution-ntt-friendly/src/lib.rs
  - icon: ':question:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal-power-series/src/lib.rs
    title: crates/polynomial/formal-power-series/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/compositional-inverse/src/lib.rs
    title: crates/polynomial/compositional-inverse/src/lib.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://nyaannyaan.github.io/library/fps/pow-enumerate.hpp
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// https://nyaannyaan.github.io/library/fps/pow-enumerate.hpp\n\n#![allow(non_snake_case)]\n\
    \nuse convolution_ntt_friendly::{ntt, ntt_doubling, ntt_inv};\nuse formal_power_series::{fps,\
    \ FormalPowerSeries};\nuse modint::StaticModInt;\n\n/// \\[x^n\\] f(x)^i g(x)\
    \ \u3092 i=0,1,..m \u306B\u3064\u3044\u3066\u5217\u6319\u3059\u308B  \n/// n =\
    \ f.len() - 1\npub fn power_projection<const M: u32>(\n    f: &FormalPowerSeries<M>,\n\
    \    g: &FormalPowerSeries<M>,\n    m: usize,\n) -> FormalPowerSeries<M> {\n \
    \   let mut n = f.len() - 1;\n    let mut k = 1;\n    let mut g = g.clone();\n\
    \    g.resize(n + 1, 0.into());\n    let mut h = 1;\n    while h < n + 1 {\n \
    \       h *= 2;\n    }\n    let mut P = g;\n    let mut Q = -f;\n    Q[0] += 1;\n\
    \    let mut nP = fps![];\n    let mut nQ = fps![];\n\n    let inv2 = StaticModInt::<M>::new(2).inv();\n\
    \    while n > 0 {\n        nP.clear();\n        nQ.clear();\n\n        let mut\
    \ buf = fps![];\n        for i in 0..=n {\n            buf.resize(k, 0.into());\n\
    \            for j in 0..k {\n                buf[j] = P[i * k + j];\n       \
    \     }\n            ntt_doubling(&mut buf);\n            nP.append(&mut buf);\n\
    \n            let mut buf = fps![0; k];\n            for j in 0..k {\n       \
    \         buf[j] = Q[i * k + j];\n            }\n            if i == 0 {\n   \
    \             for j in 0..k {\n                    buf[j] -= 1;\n            \
    \    }\n                ntt_doubling(&mut buf);\n                for j in 0..k\
    \ {\n                    buf[j] += 1;\n                    buf[k + j] -= 1;\n\
    \                }\n            } else {\n                ntt_doubling(&mut buf);\n\
    \            }\n            nQ.append(&mut buf);\n        }\n        nP.resize(2\
    \ * h * 2 * k, 0.into());\n        nQ.resize(2 * h * 2 * k, 0.into());\n     \
    \   let mut p;\n        let mut q;\n\n        let w = StaticModInt::<M>::new(StaticModInt::<M>::G).pow((M\
    \ - 1) as usize / (2 * h));\n        let iw = w.inv();\n        let mut btr =\
    \ vec![];\n        if n % 2 != 0 {\n            btr.resize(h, 0);\n          \
    \  let lg = h.trailing_zeros() as usize;\n            for i in 0..h {\n      \
    \          btr[i] = (btr[i >> 1] >> 1) + ((i & 1) << (lg - 1));\n            }\n\
    \        }\n\n        for j in 0..2 * k {\n            p = fps![0; 2 * h];\n \
    \           q = fps![0; 2 * h];\n            for i in 0..h {\n               \
    \ p[i] = nP[i * 2 * k + j];\n                q[i] = nQ[i * 2 * k + j];\n     \
    \       }\n            ntt(&mut p);\n            ntt(&mut q);\n            for\
    \ i in (0..2 * h).step_by(2) {\n                q.swap(i, i + 1);\n          \
    \  }\n            for i in 0..2 * h {\n                p[i] *= q[i];\n       \
    \     }\n            for i in 0..h {\n                q[i] = q[i * 2] * q[i *\
    \ 2 + 1];\n            }\n            if n % 2 == 0 {\n                for i in\
    \ 0..h {\n                    p[i] = (p[i * 2] + p[i * 2 + 1]) * inv2;\n     \
    \           }\n            } else {\n                let mut c = inv2;\n     \
    \           buf.resize(h, 0.into());\n                for &i in &btr {\n     \
    \               buf[i] = (p[i * 2] - p[i * 2 + 1]) * c;\n                    c\
    \ *= iw;\n                }\n                std::mem::swap(&mut p, &mut buf);\n\
    \            }\n            p.truncate(h);\n            q.truncate(h);\n     \
    \       ntt_inv(&mut p);\n            ntt_inv(&mut q);\n            for i in 0..h\
    \ {\n                nP[i * 2 * k + j] = p[i];\n                nQ[i * 2 * k +\
    \ j] = q[i];\n            }\n        }\n        nP.resize((n / 2 + 1) * 2 * k,\
    \ 0.into());\n        nQ.resize((n / 2 + 1) * 2 * k, 0.into());\n        std::mem::swap(&mut\
    \ P, &mut nP);\n        std::mem::swap(&mut Q, &mut nQ);\n        n /= 2;\n  \
    \      h /= 2;\n        k *= 2;\n    }\n\n    let mut S = fps![0; k];\n    let\
    \ mut T = fps![0; k];\n    for i in 0..k {\n        S[i] = P[i];\n        T[i]\
    \ = Q[i];\n    }\n    ntt_inv(&mut S);\n    ntt_inv(&mut T);\n    T[0] -= 1;\n\
    \    if f[0].val() == 0 {\n        S.reverse();\n        S.pre(m + 1)\n    } else\
    \ {\n        S.reverse();\n        T.push(1.into());\n        T.reverse();\n \
    \       (S * T.inv(m + 1)).pre(m + 1)\n    }\n}\n"
  dependsOn:
  - crates/convolution/convolution-ntt-friendly/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  - crates/polynomial/formal-power-series/src/lib.rs
  isVerificationFile: false
  path: crates/polynomial/power-projection/src/lib.rs
  requiredBy:
  - crates/polynomial/compositional-inverse/src/lib.rs
  timestamp: '2025-01-12 06:57:08+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/polynomial/power-projection/src/lib.rs
layout: document
redirect_from:
- /library/crates/polynomial/power-projection/src/lib.rs
- /library/crates/polynomial/power-projection/src/lib.rs.html
title: crates/polynomial/power-projection/src/lib.rs
---
