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
    path: crates/polynomial/formal_power_series/src/exp.rs
    title: crates/polynomial/formal_power_series/src/exp.rs
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
    path: crates/polynomial/formal_power_series/src/exp.rs
    title: crates/polynomial/formal_power_series/src/exp.rs
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
  code: "// TODO: sparse \u306E\u3068\u304D\n\nuse modint::ModInt;\nuse numeric_traits::Integer;\n\
    \nuse crate::{fps, FormalPowerSeries, FpsExp, FpsInv, FpsMul};\n\nimpl<M: ModInt<Value\
    \ = u32> + FpsInv + FpsMul + FpsExp> FormalPowerSeries<M> {\n    pub fn pow(&self,\
    \ exp: usize, d: usize) -> Self {\n        if exp == 0 {\n            let mut\
    \ res = fps![0; d];\n            res[0] = M::from(1);\n            return res;\n\
    \        }\n\n        let Some(l) = self\n            .iter()\n            .enumerate()\n\
    \            .find(|&(_, x)| x.val() != 0)\n            .map(|(i, _)| i) else\
    \ {\n                return fps![0; d];\n            };\n        if l >= d.ceil_div(exp)\
    \ {\n            return fps![0; d];\n        }\n\n        let offset = l * exp;\n\
    \        let c = self[l];\n        let recip_c = c.recip();\n        // let g\
    \ = Self::from_fn(d - offset, |i| self[l + i] * recip_c);\n        let g = Self::from_fn(d\
    \ - offset, |i| {\n            *self.get(l + i).unwrap_or(&0.into()) * recip_c\n\
    \        });\n        let mut log_g = g.log(g.len());\n        log_g *= M::from(exp);\n\
    \        let mut g = log_g.exp(g.len());\n        g *= c.pow(exp);\n        let\
    \ mut res = fps![0; d];\n        res[offset..].copy_from_slice(&g);\n        res\n\
    \    }\n}\n"
  dependsOn:
  - crates/polynomial/formal_power_series/src/constructor.rs
  - crates/polynomial/formal_power_series/src/convert.rs
  - crates/polynomial/formal_power_series/src/exp.rs
  - crates/polynomial/formal_power_series/src/inv.rs
  - crates/polynomial/formal_power_series/src/lib.rs
  - crates/polynomial/formal_power_series/src/log.rs
  - crates/polynomial/formal_power_series/src/mul.rs
  - crates/polynomial/formal_power_series/src/ops.rs
  - crates/polynomial/formal_power_series/src/sqrt.rs
  isVerificationFile: false
  path: crates/polynomial/formal_power_series/src/pow.rs
  requiredBy:
  - crates/polynomial/formal_power_series/src/inv.rs
  - crates/polynomial/formal_power_series/src/ops.rs
  - crates/polynomial/formal_power_series/src/lib.rs
  - crates/polynomial/formal_power_series/src/mul.rs
  - crates/polynomial/formal_power_series/src/log.rs
  - crates/polynomial/formal_power_series/src/constructor.rs
  - crates/polynomial/formal_power_series/src/convert.rs
  - crates/polynomial/formal_power_series/src/sqrt.rs
  - crates/polynomial/formal_power_series/src/exp.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/polynomial/sqrt_of_formal_power_series/src/main.rs
  - verify/library_checker/polynomial/pow_of_formal_power_series/src/main.rs
  - verify/library_checker/polynomial/inv_of_formal_power_series_not_ntt_friendly/src/main.rs
  - verify/library_checker/polynomial/exp_of_formal_power_series_not_ntt_friendly/src/main.rs
  - verify/library_checker/polynomial/inv_of_formal_power_series/src/main.rs
  - verify/library_checker/polynomial/exp_of_formal_power_series/src/main.rs
  - verify/library_checker/polynomial/log_of_formal_power_series/src/main.rs
documentation_of: crates/polynomial/formal_power_series/src/pow.rs
layout: document
redirect_from:
- /library/crates/polynomial/formal_power_series/src/pow.rs
- /library/crates/polynomial/formal_power_series/src/pow.rs.html
title: crates/polynomial/formal_power_series/src/pow.rs
---
