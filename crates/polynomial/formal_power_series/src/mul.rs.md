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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ops::{Div, DivAssign, Mul, MulAssign};\n\nuse convolution::{convolution_arbitrary_mod,\
    \ convolution_ntt_friendly};\nuse dynamic_modint::DynamicModInt;\nuse modint::ModInt;\n\
    use numeric_traits::Numeric;\nuse static_modint::StaticModInt;\n\nuse crate::{fps,\
    \ FormalPowerSeries};\n\nimpl<M: ModInt<Value = u32> + FpsMul> Mul<FormalPowerSeries<M>>\
    \ for FormalPowerSeries<M> {\n    type Output = FormalPowerSeries<M>;\n\n    fn\
    \ mul(self, rhs: FormalPowerSeries<M>) -> Self::Output {\n        FpsMul::mul(&self,\
    \ &rhs)\n    }\n}\n\nimpl<M: ModInt<Value = u32> + FpsMul> Mul<FormalPowerSeries<M>>\
    \ for &FormalPowerSeries<M> {\n    type Output = FormalPowerSeries<M>;\n\n   \
    \ fn mul(self, rhs: FormalPowerSeries<M>) -> Self::Output {\n        FpsMul::mul(self,\
    \ &rhs)\n    }\n}\n\nimpl<M: ModInt<Value = u32> + FpsMul> Mul<&FormalPowerSeries<M>>\
    \ for FormalPowerSeries<M> {\n    type Output = FormalPowerSeries<M>;\n\n    fn\
    \ mul(self, rhs: &FormalPowerSeries<M>) -> Self::Output {\n        FpsMul::mul(&self,\
    \ rhs)\n    }\n}\n\nimpl<M: ModInt<Value = u32> + FpsMul> Mul<&FormalPowerSeries<M>>\
    \ for &FormalPowerSeries<M> {\n    type Output = FormalPowerSeries<M>;\n\n   \
    \ fn mul(self, rhs: &FormalPowerSeries<M>) -> Self::Output {\n        FpsMul::mul(self,\
    \ rhs)\n    }\n}\n\nimpl<M: ModInt<Value = u32> + FpsMul> MulAssign for FormalPowerSeries<M>\
    \ {\n    fn mul_assign(&mut self, rhs: Self) {\n        *self = &*self * &rhs;\n\
    \    }\n}\n\nimpl<M: ModInt<Value = u32> + FpsMul> MulAssign<&FormalPowerSeries<M>>\
    \ for FormalPowerSeries<M> {\n    fn mul_assign(&mut self, rhs: &FormalPowerSeries<M>)\
    \ {\n        *self = &*self * rhs;\n    }\n}\n\nimpl<M: ModInt<Value = u32>> MulAssign<M>\
    \ for FormalPowerSeries<M> {\n    fn mul_assign(&mut self, rhs: M) {\n       \
    \ self.iter_mut().for_each(|x| *x *= rhs);\n    }\n}\n\nimpl<M: ModInt<Value =\
    \ u32>> Mul<M> for FormalPowerSeries<M> {\n    type Output = FormalPowerSeries<M>;\n\
    \n    fn mul(mut self, rhs: M) -> Self::Output {\n        self *= rhs;\n     \
    \   self\n    }\n}\n\nimpl<M: ModInt<Value = u32>> Mul<M> for &FormalPowerSeries<M>\
    \ {\n    type Output = FormalPowerSeries<M>;\n\n    fn mul(self, rhs: M) -> Self::Output\
    \ {\n        self.iter().map(|&x| x * rhs).collect()\n    }\n}\n\nimpl<M: ModInt<Value\
    \ = u32>> DivAssign<M> for FormalPowerSeries<M> {\n    fn div_assign(&mut self,\
    \ rhs: M) {\n        *self *= rhs.recip();\n    }\n}\n\nimpl<M: ModInt<Value =\
    \ u32>> Div<M> for FormalPowerSeries<M> {\n    type Output = FormalPowerSeries<M>;\n\
    \n    fn div(mut self, rhs: M) -> Self::Output {\n        self *= rhs.recip();\n\
    \        self\n    }\n}\n\nimpl<M: ModInt<Value = u32>> Div<M> for &FormalPowerSeries<M>\
    \ {\n    type Output = FormalPowerSeries<M>;\n\n    fn div(self, rhs: M) -> Self::Output\
    \ {\n        self * rhs.recip()\n    }\n}\n\n#[doc(hidden)]\npub trait FpsMul:\
    \ ModInt<Value = u32> + Numeric {\n    fn mul(f: &FormalPowerSeries<Self>, g:\
    \ &FormalPowerSeries<Self>) -> FormalPowerSeries<Self>;\n}\n\nconst MUL_THRESHOLD_NTT_FRIENDLY:\
    \ usize = 128;\nconst MUL_THRESHOLD_ARBITRARY: usize = 512;\n\nfn mul_naive<M:\
    \ ModInt<Value = u32> + Numeric>(\n    f: &FormalPowerSeries<M>,\n    g: &FormalPowerSeries<M>,\n\
    ) -> FormalPowerSeries<M> {\n    if f.is_empty() || g.is_empty() {\n        return\
    \ fps![];\n    }\n\n    let mut h = fps![0; f.len() + g.len() - 1];\n    for (i,\
    \ &x) in f.iter().enumerate() {\n        if x.val() == 0 {\n            continue;\n\
    \        }\n        for (j, &y) in g.iter().enumerate() {\n            h[i + j]\
    \ += x * y;\n        }\n    }\n    h\n}\n\nimpl<const MOD: u32> FpsMul for StaticModInt<MOD>\
    \ {\n    fn mul<'a>(\n        mut f: &'a FormalPowerSeries<Self>,\n        mut\
    \ g: &'a FormalPowerSeries<Self>,\n    ) -> FormalPowerSeries<Self> {\n      \
    \  let mut fc = f.count_terms();\n        let mut gc = g.count_terms();\n    \
    \    if fc > gc {\n            std::mem::swap(&mut f, &mut g);\n            std::mem::swap(&mut\
    \ fc, &mut gc);\n        }\n\n        if StaticModInt::<MOD>::IS_NTT_FRIENDLY\
    \ {\n            if fc <= MUL_THRESHOLD_NTT_FRIENDLY {\n                mul_naive(f,\
    \ g)\n            } else {\n                FormalPowerSeries(convolution_ntt_friendly(f,\
    \ g))\n            }\n        } else {\n            if fc <= MUL_THRESHOLD_ARBITRARY\
    \ {\n                mul_naive(f, g)\n            } else {\n                FormalPowerSeries(convolution_arbitrary_mod(f,\
    \ g))\n            }\n        }\n    }\n}\n\nimpl<Id> FpsMul for DynamicModInt<Id>\
    \ {\n    fn mul<'a>(\n        mut f: &'a FormalPowerSeries<Self>,\n        mut\
    \ g: &'a FormalPowerSeries<Self>,\n    ) -> FormalPowerSeries<Self> {\n      \
    \  let mut fc = f.count_terms();\n        let mut gc = g.count_terms();\n    \
    \    if fc > gc {\n            std::mem::swap(&mut f, &mut g);\n            std::mem::swap(&mut\
    \ fc, &mut gc);\n        }\n\n        if fc <= MUL_THRESHOLD_ARBITRARY {\n   \
    \         mul_naive(f, g)\n        } else {\n            FormalPowerSeries(convolution_arbitrary_mod(f,\
    \ g))\n        }\n    }\n}\n"
  dependsOn:
  - crates/polynomial/formal_power_series/src/constructor.rs
  - crates/polynomial/formal_power_series/src/convert.rs
  - crates/polynomial/formal_power_series/src/exp.rs
  - crates/polynomial/formal_power_series/src/inv.rs
  - crates/polynomial/formal_power_series/src/lib.rs
  - crates/polynomial/formal_power_series/src/log.rs
  - crates/polynomial/formal_power_series/src/ops.rs
  - crates/polynomial/formal_power_series/src/pow.rs
  - crates/polynomial/formal_power_series/src/sqrt.rs
  isVerificationFile: false
  path: crates/polynomial/formal_power_series/src/mul.rs
  requiredBy:
  - crates/polynomial/formal_power_series/src/lib.rs
  - crates/polynomial/formal_power_series/src/convert.rs
  - crates/polynomial/formal_power_series/src/ops.rs
  - crates/polynomial/formal_power_series/src/constructor.rs
  - crates/polynomial/formal_power_series/src/sqrt.rs
  - crates/polynomial/formal_power_series/src/pow.rs
  - crates/polynomial/formal_power_series/src/exp.rs
  - crates/polynomial/formal_power_series/src/log.rs
  - crates/polynomial/formal_power_series/src/inv.rs
  timestamp: '2025-03-29 09:22:56+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/polynomial/sqrt_of_formal_power_series/src/main.rs
  - verify/library_checker/polynomial/exp_of_formal_power_series_not_ntt_friendly/src/main.rs
  - verify/library_checker/polynomial/inv_of_formal_power_series_not_ntt_friendly/src/main.rs
  - verify/library_checker/polynomial/pow_of_formal_power_series/src/main.rs
  - verify/library_checker/polynomial/log_of_formal_power_series/src/main.rs
  - verify/library_checker/polynomial/exp_of_formal_power_series/src/main.rs
  - verify/library_checker/polynomial/inv_of_formal_power_series/src/main.rs
documentation_of: crates/polynomial/formal_power_series/src/mul.rs
layout: document
redirect_from:
- /library/crates/polynomial/formal_power_series/src/mul.rs
- /library/crates/polynomial/formal_power_series/src/mul.rs.html
title: crates/polynomial/formal_power_series/src/mul.rs
---
