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
    path: verify/kth_term_of_linearly_recurrent_sequence/src/main.rs
    title: verify/kth_term_of_linearly_recurrent_sequence/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::mem::swap;\n\nuse convolution_ntt_friendly::{ntt, ntt_doubling,\
    \ ntt_inv};\nuse formal_power_series::{fps, FormalPowerSeries};\nuse modint::StaticModInt;\n\
    \npub fn bostan_mori<const P: u32>(\n    mut p: FormalPowerSeries<P>,\n    mut\
    \ q: FormalPowerSeries<P>,\n    mut k: usize,\n) -> StaticModInt<P> {\n    q.shrink();\n\
    \    let mut res = 0.into();\n    if p.len() >= q.len() {\n        let r = &p\
    \ / &q;\n        p -= &r * &q;\n        p.shrink();\n        if k < r.len() {\n\
    \            res += r[k];\n        }\n    }\n    if p.len() == 0 {\n        return\
    \ res;\n    }\n\n    if StaticModInt::<P>::IS_NTT_FRIENDLY {\n        let logn\
    \ = 64 - (q.len() - 1).leading_zeros() as usize;\n        let n = 1 << logn;\n\
    \        p.resize(n * 2, 0.into());\n        q.resize(n * 2, 0.into());\n    \
    \    ntt(&mut p);\n        ntt(&mut q);\n        let mut s = fps![0; n * 2];\n\
    \        let mut t = fps![0; n * 2];\n        let mut btr = vec![0; n];\n    \
    \    for i in 0..n {\n            btr[i] = (btr[i >> 1] >> 1) + ((i & 1) << logn\
    \ - 1);\n        }\n        let dw = StaticModInt::new(StaticModInt::<P>::G)\n\
    \            .inv()\n            .pow((P as usize - 1) / (n * 2));\n        while\
    \ k != 0 {\n            let mut inv2 = StaticModInt::new(2).inv();\n         \
    \   t.resize(n, 0.into());\n            for i in 0..n {\n                t[i]\
    \ = q[i << 1 | 0] * q[i << 1 | 1];\n            }\n            s.resize(n, 0.into());\n\
    \            if k & 1 != 0 {\n                for &i in &btr {\n             \
    \       s[i] = (p[i << 1 | 0] * q[i << 1 | 1] - p[i << 1 | 1] * q[i << 1 | 0])\
    \ * inv2;\n                    inv2 *= dw;\n                }\n            } else\
    \ {\n                for i in 0..n {\n                    s[i] = (p[i << 1 | 0]\
    \ * q[i << 1 | 1] + p[i << 1 | 1] * q[i << 1 | 0]) * inv2;\n                }\n\
    \            }\n            swap(&mut p, &mut s);\n            swap(&mut q, &mut\
    \ t);\n            k >>= 1;\n            if k < n {\n                break;\n\
    \            }\n            ntt_doubling(&mut p);\n            ntt_doubling(&mut\
    \ q);\n        }\n        ntt_inv(&mut p);\n        ntt_inv(&mut q);\n       \
    \ res + (p * q.inv(q.len()))[k]\n    } else {\n        p.resize(q.len() - 1, 0.into());\n\
    \        while k != 0 {\n            let mut q2 = q.clone();\n            for\
    \ v in q2.iter_mut().skip(1).step_by(2) {\n                *v = -*v;\n       \
    \     }\n            let s = &p * &q2;\n            let t = &q * q2;\n       \
    \     for i in (k & 1..s.len()).step_by(2) {\n                p[i >> 1] = s[i];\n\
    \            }\n            for i in (0..t.len()).step_by(2) {\n             \
    \   q[i >> 1] = t[i];\n            }\n            k >>= 1;\n        }\n      \
    \  res + p[0]\n    }\n}\n"
  dependsOn:
  - crates/convolution/convolution-ntt-friendly/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  - crates/polynomial/formal-power-series/src/lib.rs
  isVerificationFile: false
  path: crates/polynomial/bostan-mori/src/lib.rs
  requiredBy: []
  timestamp: '2024-06-13 08:47:29+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/kth_term_of_linearly_recurrent_sequence/src/main.rs
documentation_of: crates/polynomial/bostan-mori/src/lib.rs
layout: document
redirect_from:
- /library/crates/polynomial/bostan-mori/src/lib.rs
- /library/crates/polynomial/bostan-mori/src/lib.rs.html
title: crates/polynomial/bostan-mori/src/lib.rs
---
