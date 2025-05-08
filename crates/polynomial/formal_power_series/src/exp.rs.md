---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/constructor.rs
    title: crates/polynomial/formal_power_series/src/constructor.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/convert.rs
    title: crates/polynomial/formal_power_series/src/convert.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/inv.rs
    title: crates/polynomial/formal_power_series/src/inv.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/lib.rs
    title: crates/polynomial/formal_power_series/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/log.rs
    title: crates/polynomial/formal_power_series/src/log.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/mul.rs
    title: crates/polynomial/formal_power_series/src/mul.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/ops.rs
    title: crates/polynomial/formal_power_series/src/ops.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/pow.rs
    title: crates/polynomial/formal_power_series/src/pow.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/sqrt.rs
    title: crates/polynomial/formal_power_series/src/sqrt.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/constructor.rs
    title: crates/polynomial/formal_power_series/src/constructor.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/convert.rs
    title: crates/polynomial/formal_power_series/src/convert.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/inv.rs
    title: crates/polynomial/formal_power_series/src/inv.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/lib.rs
    title: crates/polynomial/formal_power_series/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/log.rs
    title: crates/polynomial/formal_power_series/src/log.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/mul.rs
    title: crates/polynomial/formal_power_series/src/mul.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/ops.rs
    title: crates/polynomial/formal_power_series/src/ops.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/pow.rs
    title: crates/polynomial/formal_power_series/src/pow.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/sqrt.rs
    title: crates/polynomial/formal_power_series/src/sqrt.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/polynomial/exp_of_formal_power_series/src/main.rs
    title: verify/library_checker/polynomial/exp_of_formal_power_series/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/polynomial/exp_of_formal_power_series_not_ntt_friendly/src/main.rs
    title: verify/library_checker/polynomial/exp_of_formal_power_series_not_ntt_friendly/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/polynomial/inv_of_formal_power_series/src/main.rs
    title: verify/library_checker/polynomial/inv_of_formal_power_series/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/polynomial/inv_of_formal_power_series_not_ntt_friendly/src/main.rs
    title: verify/library_checker/polynomial/inv_of_formal_power_series_not_ntt_friendly/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/polynomial/log_of_formal_power_series/src/main.rs
    title: verify/library_checker/polynomial/log_of_formal_power_series/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/polynomial/pow_of_formal_power_series/src/main.rs
    title: verify/library_checker/polynomial/pow_of_formal_power_series/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/polynomial/sqrt_of_formal_power_series/src/main.rs
    title: verify/library_checker/polynomial/sqrt_of_formal_power_series/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// TODO: sparse \u306E\u3068\u304D\n\nuse convolution::{ntt, ntt_inv};\n\
    use dynamic_modint::DynamicModInt;\nuse modint::ModInt;\nuse numeric_traits::Integer;\n\
    use static_modint::StaticModInt;\n\nuse crate::{fps, FormalPowerSeries, FpsInv,\
    \ FpsMul};\n\nimpl<M: ModInt<Value = u32> + FpsExp> FormalPowerSeries<M> {\n \
    \   pub fn exp(&self, d: usize) -> Self {\n        FpsExp::exp(self, d)\n    }\n\
    }\n\n#[doc(hidden)]\npub trait FpsExp: ModInt<Value = u32> {\n    fn exp(f: &FormalPowerSeries<Self>,\
    \ d: usize) -> FormalPowerSeries<Self>;\n}\n\nimpl<const MOD: u32> FpsExp for\
    \ StaticModInt<MOD> {\n    fn exp(f: &FormalPowerSeries<Self>, d: usize) -> FormalPowerSeries<Self>\
    \ {\n        assert_eq!(f[0].val(), 0);\n\n        if !Self::IS_NTT_FRIENDLY {\n\
    \            return exp_not_ntt_friendly(f, d);\n        }\n\n        let mut\
    \ b = fps![1, *f.get(1).unwrap_or(&0.into())];\n        let mut c = fps![1];\n\
    \        let mut z1: FormalPowerSeries<Self>;\n        let mut z2 = fps![1, 1];\n\
    \        for w in (1..).map(|i| 1 << i).take_while(|&w| w < d) {\n           \
    \ let mut y = b.clone();\n            y.resize(w * 2, Self::from_raw(0));\n  \
    \          ntt(&mut y);\n\n            z1 = z2;\n            let mut z: Vec<_>\
    \ = y.iter().zip(&z1).map(|(x, y)| x * y).collect();\n            ntt_inv(&mut\
    \ z);\n            z[..w / 2].fill(Self::from_raw(0));\n            ntt(&mut z);\n\
    \            z.iter_mut().zip(&z1).for_each(|(x, y)| *x *= -*y);\n           \
    \ ntt_inv(&mut z);\n\n            c.extend(&z[w / 2..]);\n            z2 = c.clone();\n\
    \            z2.resize(w * 2, Self::from_raw(0));\n            ntt(&mut z2);\n\
    \n            let mut x = f.prefix(w).diff();\n            x.push(Self::from_raw(0));\n\
    \            ntt(&mut x);\n            x.iter_mut().zip(&y).for_each(|(x, y)|\
    \ *x *= *y);\n            ntt_inv(&mut x);\n            x -= b.diff();\n     \
    \       x.resize(w * 2, Self::from_raw(0));\n            let (xl, xr) = x.split_at_mut(w);\n\
    \            xl.iter_mut().zip(xr).take(w - 1).for_each(|(x, y)| {\n         \
    \       *y = *x;\n                *x = Self::from_raw(0);\n            });\n \
    \           ntt(&mut x);\n            x.iter_mut().zip(&z2).for_each(|(x, y)|\
    \ *x *= *y);\n            ntt_inv(&mut x);\n            x.pop();\n           \
    \ x = x.integral();\n            x.iter_mut().zip(f).skip(w).for_each(|(x, y)|\
    \ *x += *y);\n            x[..w].fill(Self::from_raw(0));\n            ntt(&mut\
    \ x);\n            x.iter_mut().zip(&y).for_each(|(x, y)| *x *= *y);\n       \
    \     ntt_inv(&mut x);\n            b.extend(&x[w..]);\n        }\n\n        b.truncate(d);\n\
    \        b\n    }\n}\n\nimpl<Id> FpsExp for DynamicModInt<Id> {\n    fn exp(f:\
    \ &FormalPowerSeries<Self>, d: usize) -> FormalPowerSeries<Self> {\n        assert_eq!(f[0].val(),\
    \ 0);\n        exp_not_ntt_friendly(f, d)\n    }\n}\n\nfn exp_not_ntt_friendly<M:\
    \ ModInt<Value = u32> + FpsMul + FpsInv>(\n    f: &FormalPowerSeries<M>,\n   \
    \ d: usize,\n) -> FormalPowerSeries<M> {\n    let sz = d.ceil_pow2();\n    let\
    \ h = f.prefix(sz);\n    let dh = h.diff();\n\n    let mut f = fps![1];\n    let\
    \ mut g = fps![1];\n    for w in (0..).map(|i| 1 << i).take_while(|&w| w < d)\
    \ {\n        let mut p = &f * &g;\n        p.resize(w, M::from_raw(0));\n    \
    \    p *= &g;\n        p.resize(w, M::from_raw(0));\n        g.resize(w, M::from_raw(0));\n\
    \        g.iter_mut().zip(&p).for_each(|(x, y)| *x += *x - *y);\n\n        p =\
    \ &dh[..w - 1].into() * &f;\n        p.resize(w * 2 - 1, M::from_raw(0));\n  \
    \      p.iter_mut().for_each(|x| *x = -*x);\n        p.iter_mut()\n          \
    \  .zip(f.iter().enumerate().skip(1))\n            .for_each(|(x, (i, y))| {\n\
    \                *x += *y * M::from_raw(i as u32);\n            });\n        p\
    \ *= &g;\n        p.resize(w * 2 - 1, M::from_raw(0));\n        p.iter_mut()\n\
    \            .zip(&dh)\n            .take(w - 1)\n            .for_each(|(x, y)|\
    \ *x += *y);\n        p = p.integral();\n        p.iter_mut().zip(&h).for_each(|(x,\
    \ y)| *x = *y - *x);\n        p[0] += M::from_raw(1);\n        f *= &p;\n    \
    \    f.resize(w * 2, M::from_raw(0));\n    }\n\n    f.truncate(d);\n    f\n}\n"
  dependsOn:
  - crates/polynomial/formal_power_series/src/constructor.rs
  - crates/polynomial/formal_power_series/src/convert.rs
  - crates/polynomial/formal_power_series/src/inv.rs
  - crates/polynomial/formal_power_series/src/lib.rs
  - crates/polynomial/formal_power_series/src/log.rs
  - crates/polynomial/formal_power_series/src/mul.rs
  - crates/polynomial/formal_power_series/src/ops.rs
  - crates/polynomial/formal_power_series/src/pow.rs
  - crates/polynomial/formal_power_series/src/sqrt.rs
  isVerificationFile: false
  path: crates/polynomial/formal_power_series/src/exp.rs
  requiredBy:
  - crates/polynomial/formal_power_series/src/lib.rs
  - crates/polynomial/formal_power_series/src/inv.rs
  - crates/polynomial/formal_power_series/src/mul.rs
  - crates/polynomial/formal_power_series/src/log.rs
  - crates/polynomial/formal_power_series/src/convert.rs
  - crates/polynomial/formal_power_series/src/sqrt.rs
  - crates/polynomial/formal_power_series/src/pow.rs
  - crates/polynomial/formal_power_series/src/constructor.rs
  - crates/polynomial/formal_power_series/src/ops.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/polynomial/exp_of_formal_power_series_not_ntt_friendly/src/main.rs
  - verify/library_checker/polynomial/sqrt_of_formal_power_series/src/main.rs
  - verify/library_checker/polynomial/pow_of_formal_power_series/src/main.rs
  - verify/library_checker/polynomial/exp_of_formal_power_series/src/main.rs
  - verify/library_checker/polynomial/log_of_formal_power_series/src/main.rs
  - verify/library_checker/polynomial/inv_of_formal_power_series/src/main.rs
  - verify/library_checker/polynomial/inv_of_formal_power_series_not_ntt_friendly/src/main.rs
documentation_of: crates/polynomial/formal_power_series/src/exp.rs
layout: document
redirect_from:
- /library/crates/polynomial/formal_power_series/src/exp.rs
- /library/crates/polynomial/formal_power_series/src/exp.rs.html
title: crates/polynomial/formal_power_series/src/exp.rs
---
