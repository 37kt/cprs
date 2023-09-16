---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':question:'
    path: crates/math/fast-factorize/src/lib.rs
    title: crates/math/fast-factorize/src/lib.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.5/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.5/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{\n    convert::TryInto,\n    fmt::Debug,\n    ops::{Add, AddAssign,\
    \ Mul, MulAssign, Neg, Sub, SubAssign},\n    sync::atomic::{AtomicU64, Ordering::SeqCst},\n\
    };\n\nstruct Montgomery {\n    m: AtomicU64,\n    r: AtomicU64,\n    n2: AtomicU64,\n\
    }\n\nimpl Montgomery {\n    const fn new() -> Self {\n        Self {\n       \
    \     m: AtomicU64::new(0),\n            r: AtomicU64::new(0),\n            n2:\
    \ AtomicU64::new(0),\n        }\n    }\n\n    fn set(&self, m: u64) {\n      \
    \  assert!(m < 1 << 62);\n        assert!(m & 1 != 0);\n        if self.m.load(SeqCst)\
    \ == m {\n            return;\n        }\n        let n2 = ((m as u128).wrapping_neg()\
    \ % m as u128) as u64;\n        let mut r = m;\n        for _ in 0..5 {\n    \
    \        r = r.wrapping_mul(2u64.wrapping_sub(m.wrapping_mul(r)));\n        }\n\
    \        assert!(r.wrapping_mul(m) == 1);\n        self.m.store(m, SeqCst);\n\
    \        self.r.store(r, SeqCst);\n        self.n2.store(n2, SeqCst);\n    }\n\
    \n    fn reduce(&self, x: u128) -> u64 {\n        let r = self.r.load(SeqCst);\n\
    \        let m = self.m.load(SeqCst);\n        (x.wrapping_add(\n            ((x\
    \ as u64).wrapping_mul(r.wrapping_neg()) as u128).wrapping_mul(m as u128),\n \
    \       ) >> 64) as u64\n    }\n}\n\nstatic MONTGOMERY: Montgomery = Montgomery::new();\n\
    \n#[derive(Default, Clone, Copy)]\npub struct MontgomeryModInt(u64);\n\nimpl MontgomeryModInt\
    \ {\n    pub fn set_modulus(m: u64) {\n        MONTGOMERY.set(m);\n    }\n\n \
    \   pub fn modulus() -> u64 {\n        MONTGOMERY.m.load(SeqCst)\n    }\n\n  \
    \  pub fn new(x: u64) -> Self {\n        Self(\n            MONTGOMERY.reduce(\n\
    \                (x as u128)\n                    .wrapping_add(Self::modulus()\
    \ as u128)\n                    .wrapping_mul(MONTGOMERY.n2.load(SeqCst) as u128),\n\
    \            ),\n        )\n    }\n\n    pub fn pow(mut self, k: impl TryInto<u128,\
    \ Error = impl Debug>) -> Self {\n        let mut k: u128 = k.try_into().unwrap();\n\
    \        let mut r = Self::new(1);\n        while k > 0 {\n            if k &\
    \ 1 != 0 {\n                r *= self;\n            }\n            self *= self;\n\
    \            k >>= 1;\n        }\n        r\n    }\n\n    pub fn val(self) ->\
    \ u64 {\n        let x = MONTGOMERY.reduce(self.0 as u128);\n        let m = Self::modulus();\n\
    \        if x >= m {\n            x - m\n        } else {\n            x\n   \
    \     }\n    }\n}\n\nimpl Neg for MontgomeryModInt {\n    type Output = Self;\n\
    \    fn neg(self) -> Self::Output {\n        Self::new(0) - self\n    }\n}\n\n\
    impl AddAssign<Self> for MontgomeryModInt {\n    fn add_assign(&mut self, rhs:\
    \ Self) {\n        let m = Self::modulus();\n        self.0 = self.0.wrapping_add(rhs.0.wrapping_sub(m\
    \ * 2));\n        if (self.0 as i64) < 0 {\n            self.0 = self.0.wrapping_add(m\
    \ * 2);\n        }\n    }\n}\n\nimpl SubAssign<Self> for MontgomeryModInt {\n\
    \    fn sub_assign(&mut self, rhs: Self) {\n        let m = Self::modulus();\n\
    \        self.0 = self.0.wrapping_sub(rhs.0);\n        if (self.0 as i64) < 0\
    \ {\n            self.0 = self.0.wrapping_add(m * 2);\n        }\n    }\n}\n\n\
    impl MulAssign<Self> for MontgomeryModInt {\n    fn mul_assign(&mut self, rhs:\
    \ Self) {\n        self.0 = MONTGOMERY.reduce(self.0 as u128 * rhs.0 as u128);\n\
    \    }\n}\n\nimpl Add<Self> for MontgomeryModInt {\n    type Output = Self;\n\
    \    fn add(mut self, rhs: Self) -> Self::Output {\n        self += rhs;\n   \
    \     self\n    }\n}\n\nimpl Sub<Self> for MontgomeryModInt {\n    type Output\
    \ = Self;\n    fn sub(mut self, rhs: Self) -> Self::Output {\n        self -=\
    \ rhs;\n        self\n    }\n}\n\nimpl Mul<Self> for MontgomeryModInt {\n    type\
    \ Output = Self;\n    fn mul(mut self, rhs: Self) -> Self::Output {\n        self\
    \ *= rhs;\n        self\n    }\n}\n\nimpl PartialEq for MontgomeryModInt {\n \
    \   fn eq(&self, other: &Self) -> bool {\n        let m = Self::modulus();\n \
    \       (if self.0 >= m { self.0 - m } else { self.0 })\n            == (if other.0\
    \ >= m { other.0 - m } else { other.0 })\n    }\n}\n\nimpl Eq for MontgomeryModInt\
    \ {}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/math/montgomery-modint/src/lib.rs
  requiredBy:
  - crates/math/fast-factorize/src/lib.rs
  timestamp: '2023-05-18 17:32:10+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/math/montgomery-modint/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/montgomery-modint/src/lib.rs
- /library/crates/math/montgomery-modint/src/lib.rs.html
title: crates/math/montgomery-modint/src/lib.rs
---
