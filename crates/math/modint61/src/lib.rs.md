---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/string/rolling-hash/src/lib.rs
    title: crates/string/rolling-hash/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/zalgorithm_rolling_hash/src/main.rs
    title: verify/zalgorithm_rolling_hash/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.4/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.4/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{\n    fmt::{Debug, Display},\n    ops::{Add, AddAssign, Div, DivAssign,\
    \ Mul, MulAssign, Neg, Sub, SubAssign},\n};\n\nuse algebraic::{One, Zero};\n\n\
    const P: u64 = (1 << 61) - 1;\n\n#[derive(Clone, Copy, Default, PartialEq, Eq,\
    \ Hash)]\npub struct ModInt61(u64);\n\nimpl ModInt61 {\n    #[inline]\n    pub\
    \ fn new(x: impl Into<ModInt61>) -> Self {\n        x.into()\n    }\n\n    #[inline]\n\
    \    pub fn raw(val: u64) -> Self {\n        Self(val)\n    }\n\n    #[inline]\n\
    \    pub fn val(self) -> u64 {\n        self.0\n    }\n\n    #[inline]\n    pub\
    \ fn modulus() -> u64 {\n        P\n    }\n\n    pub fn pow(mut self, mut k: usize)\
    \ -> Self {\n        let mut res = Self::raw(1);\n        while k != 0 {\n   \
    \         if k & 1 != 0 {\n                res *= self;\n            }\n     \
    \       k >>= 1;\n            self *= self;\n        }\n        res\n    }\n\n\
    \    pub fn inv(self) -> Self {\n        self.pow(P as usize - 2)\n    }\n}\n\n\
    impl Display for ModInt61 {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)\
    \ -> std::fmt::Result {\n        write!(f, \"{}\", self.0)\n    }\n}\n\nimpl Debug\
    \ for ModInt61 {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result\
    \ {\n        write!(f, \"{}\", self.0)\n    }\n}\n\nimpl Neg for ModInt61 {\n\
    \    type Output = Self;\n    fn neg(self) -> Self::Output {\n        if self.0\
    \ == 0 {\n            self\n        } else {\n            Self(P - self.0)\n \
    \       }\n    }\n}\n\nimpl Neg for &ModInt61 {\n    type Output = ModInt61;\n\
    \    fn neg(self) -> Self::Output {\n        -*self\n    }\n}\n\nimpl AddAssign\
    \ for ModInt61 {\n    fn add_assign(&mut self, rhs: Self) {\n        self.0 +=\
    \ rhs.0;\n        if self.0 >= P {\n            self.0 -= P;\n        }\n    }\n\
    }\n\nimpl SubAssign for ModInt61 {\n    fn sub_assign(&mut self, rhs: Self) {\n\
    \        *self += -rhs;\n    }\n}\n\nimpl MulAssign for ModInt61 {\n    fn mul_assign(&mut\
    \ self, rhs: Self) {\n        let t = self.0 as u128 * rhs.0 as u128;\n      \
    \  let t = (t >> 61) as u64 + (t as u64 & P);\n        self.0 = if t >= P { t\
    \ - P } else { t }\n    }\n}\n\nimpl DivAssign for ModInt61 {\n    fn div_assign(&mut\
    \ self, rhs: Self) {\n        *self *= rhs.inv();\n    }\n}\n\nmacro_rules! impl_from_integer\
    \ {\n    ($(($t1:ty, $t2:ty)),*) => {\n        $(\n            impl From<$t1>\
    \ for ModInt61 {\n                fn from(x: $t1) -> Self {\n                \
    \    Self((x as $t2).rem_euclid(P as $t2) as u64)\n                }\n       \
    \     }\n        )*\n    };\n}\n\nimpl_from_integer!(\n    (i8, i64),\n    (i16,\
    \ i64),\n    (i32, i64),\n    (i64, i64),\n    (isize, i64),\n    (i128, i128),\n\
    \    (u8, u64),\n    (u16, u64),\n    (u32, u64),\n    (u64, u64),\n    (usize,\
    \ u64),\n    (u128, u128)\n);\n\nmacro_rules! impl_ops {\n    ($(\n        $trait:ident,\n\
    \        $trait_assign:ident,\n        $fn:ident,\n        $fn_assign:ident,\n\
    \    )*) => {$(\n        impl $trait_assign<&ModInt61> for ModInt61 {\n      \
    \      fn $fn_assign(&mut self, rhs: &ModInt61) {\n                self.$fn_assign(*rhs);\n\
    \            }\n        }\n        impl $trait<&ModInt61> for ModInt61 {\n   \
    \         type Output = ModInt61;\n            fn $fn(mut self, rhs: &ModInt61)\
    \ -> Self::Output {\n                self.$fn_assign(*rhs);\n                self\n\
    \            }\n        }\n        impl $trait<ModInt61> for &ModInt61 {\n   \
    \         type Output = ModInt61;\n            fn $fn(self, rhs: ModInt61) ->\
    \ Self::Output {\n                (*self).$fn(rhs)\n            }\n        }\n\
    \        impl $trait<ModInt61> for ModInt61 {\n            type Output = ModInt61;\n\
    \            fn $fn(mut self, rhs: ModInt61) -> Self::Output {\n             \
    \   self.$fn_assign(rhs);\n                self\n            }\n        }\n  \
    \      impl $trait<&ModInt61> for &ModInt61 {\n            type Output = ModInt61;\n\
    \            fn $fn(self, rhs: &ModInt61) -> Self::Output {\n                (*self).$fn(&*rhs)\n\
    \            }\n        }\n    )*};\n}\n\nimpl_ops! {\n    Add, AddAssign, add,\
    \ add_assign,\n    Sub, SubAssign, sub, sub_assign,\n    Mul, MulAssign, mul,\
    \ mul_assign,\n    Div, DivAssign, div, div_assign,\n}\n\nimpl Zero for ModInt61\
    \ {\n    fn zero() -> Self {\n        Self::raw(0)\n    }\n\n    fn is_zero(&self)\
    \ -> bool {\n        self.0 == 0\n    }\n}\n\nimpl One for ModInt61 {\n    fn\
    \ one() -> Self {\n        Self::raw(1)\n    }\n\n    fn is_one(&self) -> bool\
    \ {\n        self.0 == 1\n    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  isVerificationFile: false
  path: crates/math/modint61/src/lib.rs
  requiredBy:
  - crates/string/rolling-hash/src/lib.rs
  timestamp: '2024-03-18 01:19:47+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/zalgorithm_rolling_hash/src/main.rs
documentation_of: crates/math/modint61/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/modint61/src/lib.rs
- /library/crates/math/modint61/src/lib.rs.html
title: crates/math/modint61/src/lib.rs
---
