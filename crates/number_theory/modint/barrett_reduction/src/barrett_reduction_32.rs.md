---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/number_theory/modint/barrett_reduction/src/barrett_reduction_64.rs
    title: crates/number_theory/modint/barrett_reduction/src/barrett_reduction_64.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/barrett_reduction/src/lib.rs
    title: crates/number_theory/modint/barrett_reduction/src/lib.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/number_theory/modint/barrett_reduction/src/barrett_reduction_64.rs
    title: crates/number_theory/modint/barrett_reduction/src/barrett_reduction_64.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/barrett_reduction/src/lib.rs
    title: crates/number_theory/modint/barrett_reduction/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/dynamic_modint/src/lib.rs
    title: crates/number_theory/modint/dynamic_modint/src/lib.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/dynamic_modint_64/src/lib.rs
    title: crates/number_theory/modint/dynamic_modint_64/src/lib.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "#[derive(Clone, Copy)]\npub struct BarrettReduction32 {\n    m: u32,\n  \
    \  im: u64,\n}\n\nimpl BarrettReduction32 {\n    #[inline]\n    pub fn new(m:\
    \ u32) -> Self {\n        let im = (!0 / m as u64).wrapping_add(1);\n        Self\
    \ { m, im }\n    }\n\n    #[inline]\n    pub fn modulus(&self) -> u32 {\n    \
    \    self.m\n    }\n\n    #[inline]\n    pub fn quo_rem(&self, a: u64) -> (u64,\
    \ u32) {\n        let mut x = (((a as u128) * (self.im as u128)) >> 64) as u64;\n\
    \        let mut r = a.wrapping_sub(x.wrapping_mul(self.m as u64)) as u32;\n \
    \       if self.m <= r {\n            r = r.wrapping_add(self.m);\n          \
    \  x -= 1;\n        }\n        (x, r)\n    }\n\n    #[inline]\n    pub fn quo(&self,\
    \ a: u64) -> u64 {\n        self.quo_rem(a).0\n    }\n\n    #[inline]\n    pub\
    \ fn rem(&self, a: u64) -> u32 {\n        self.quo_rem(a).1\n    }\n\n    #[inline]\n\
    \    pub fn mul(&self, a: u32, b: u32) -> u32 {\n        self.rem(a as u64 * b\
    \ as u64)\n    }\n\n    pub fn pow(&self, a: u32, mut exp: u64) -> u32 {\n   \
    \     let mut a = self.rem(a as u64);\n        let mut r = if self.m == 1 { 0\
    \ } else { 1 };\n        while exp != 0 {\n            if exp & 1 != 0 {\n   \
    \             r = self.mul(r, a);\n            }\n            a = self.mul(a,\
    \ a);\n            exp >>= 1;\n        }\n        r\n    }\n}\n"
  dependsOn:
  - crates/number_theory/modint/barrett_reduction/src/barrett_reduction_64.rs
  - crates/number_theory/modint/barrett_reduction/src/lib.rs
  isVerificationFile: false
  path: crates/number_theory/modint/barrett_reduction/src/barrett_reduction_32.rs
  requiredBy:
  - crates/number_theory/modint/dynamic_modint_64/src/lib.rs
  - crates/number_theory/modint/dynamic_modint/src/lib.rs
  - crates/number_theory/modint/barrett_reduction/src/barrett_reduction_64.rs
  - crates/number_theory/modint/barrett_reduction/src/lib.rs
  timestamp: '2025-05-16 07:00:05+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/number_theory/modint/barrett_reduction/src/barrett_reduction_32.rs
layout: document
redirect_from:
- /library/crates/number_theory/modint/barrett_reduction/src/barrett_reduction_32.rs
- /library/crates/number_theory/modint/barrett_reduction/src/barrett_reduction_32.rs.html
title: crates/number_theory/modint/barrett_reduction/src/barrett_reduction_32.rs
---
