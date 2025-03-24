---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/number_theory/modint/modint_61/src/lib.rs
    title: crates/number_theory/modint/modint_61/src/lib.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/modint_61/src/numeric.rs
    title: crates/number_theory/modint/modint_61/src/numeric.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/number_theory/modint/modint_61/src/lib.rs
    title: crates/number_theory/modint/modint_61/src/lib.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/modint_61/src/numeric.rs
    title: crates/number_theory/modint/modint_61/src/numeric.rs
  - icon: ':heavy_check_mark:'
    path: crates/string/rolling_hash/src/lib.rs
    title: crates/string/rolling_hash/src/lib.rs
  - icon: ':warning:'
    path: verify/sandbox/test/src/main.rs
    title: verify/sandbox/test/src/main.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use crate::{ModInt61, P};\nuse std::fmt::{Debug, Display};\nuse std::num::ParseIntError;\n\
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};\n\
    use std::str::FromStr;\n\nimpl ModInt61 {\n    pub fn new<T: Into<ModInt61>>(x:\
    \ T) -> Self {\n        x.into()\n    }\n\n    pub fn from_raw(x: u64) -> Self\
    \ {\n        Self(x)\n    }\n\n    pub const fn modulus() -> u64 {\n        P\n\
    \    }\n\n    pub fn val(self) -> u64 {\n        self.0\n    }\n\n    pub fn pow(self,\
    \ exp: usize) -> Self {\n        let mut res = Self::from_raw(1);\n        let\
    \ mut base = self;\n        let mut exp = exp;\n        while exp > 0 {\n    \
    \        if exp % 2 == 1 {\n                res *= base;\n            }\n    \
    \        base *= base;\n            exp /= 2;\n        }\n        res\n    }\n\
    \n    pub fn recip(self) -> Self {\n        self.pow(P as usize - 2)\n    }\n\n\
    \    pub fn sqrt(self) -> Option<Self> {\n        let p = Self::modulus() as usize;\n\
    \        if self.0 < 2 {\n            return Some(self);\n        } else if self.pow(p\
    \ - 1 >> 1).val() != 1 {\n            return None;\n        }\n\n        let mut\
    \ b = Self::from_raw(1);\n        while b.pow(p - 1 >> 1).val() == 1 {\n     \
    \       b += 1;\n        }\n\n        let mut e = (p - 1).trailing_zeros() as\
    \ usize;\n        let m = p - 1 >> e;\n        let mut x = self.pow(m - 1 >> 1);\n\
    \        let mut y = self * x * x;\n        x *= self;\n        let mut z = b.pow(m);\n\
    \        while y.val() != 1 {\n            let mut j = 0;\n            let mut\
    \ t = y;\n            while t.val() != 1 {\n                t *= t;\n        \
    \        j += 1;\n            }\n            z = z.pow(1 << e - j - 1);\n    \
    \        x *= z;\n            z *= z;\n            y *= z;\n            e = j;\n\
    \        }\n\n        Some(x)\n    }\n}\n\nimpl From<&ModInt61> for ModInt61 {\n\
    \    fn from(x: &ModInt61) -> Self {\n        *x\n    }\n}\n\nimpl FromStr for\
    \ ModInt61 {\n    type Err = ParseIntError;\n\n    fn from_str(s: &str) -> Result<Self,\
    \ Self::Err> {\n        s.parse::<i64>().map(Self::from)\n    }\n}\n\nimpl Display\
    \ for ModInt61 {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result\
    \ {\n        write!(f, \"{}\", self.0)\n    }\n}\n\nimpl Debug for ModInt61 {\n\
    \    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n  \
    \      write!(f, \"{}\", self.0)\n    }\n}\n\nimpl Neg for ModInt61 {\n    type\
    \ Output = Self;\n\n    fn neg(self) -> Self::Output {\n        if self.0 == 0\
    \ {\n            Self::from_raw(0)\n        } else {\n            Self::from_raw(P\
    \ - self.0)\n        }\n    }\n}\n\nimpl Neg for &ModInt61 {\n    type Output\
    \ = ModInt61;\n\n    fn neg(self) -> Self::Output {\n        -*self\n    }\n}\n\
    \nimpl<T: Into<ModInt61>> Add<T> for ModInt61 {\n    type Output = Self;\n\n \
    \   fn add(self, rhs: T) -> Self::Output {\n        let rhs = rhs.into();\n  \
    \      let mut x = self.0 + rhs.0;\n        if x >= P {\n            x -= P;\n\
    \        }\n        Self::from_raw(x)\n    }\n}\n\nimpl<T: Into<ModInt61>> Sub<T>\
    \ for ModInt61 {\n    type Output = Self;\n\n    fn sub(self, rhs: T) -> Self::Output\
    \ {\n        let rhs = rhs.into();\n        let mut x = self.0 + P - rhs.0;\n\
    \        if x >= P {\n            x -= P;\n        }\n        Self::from_raw(x)\n\
    \    }\n}\n\nimpl<T: Into<ModInt61>> Mul<T> for ModInt61 {\n    type Output =\
    \ Self;\n\n    fn mul(self, rhs: T) -> Self::Output {\n        let rhs = rhs.into();\n\
    \        let t = self.0 as u128 * rhs.0 as u128;\n        let t = (t >> 61) as\
    \ u64 + (t as u64 & P);\n        Self::from_raw(if t >= P { t - P } else { t })\n\
    \    }\n}\n\nimpl<T: Into<ModInt61>> Div<T> for ModInt61 {\n    type Output =\
    \ Self;\n\n    fn div(self, rhs: T) -> Self::Output {\n        let rhs = rhs.into();\n\
    \        self * rhs.recip()\n    }\n}\n\nmacro_rules! impl_from_integer {\n  \
    \  ($(($t1:ty, $t2:ty)),*) => {\n        $(\n            impl From<$t1> for ModInt61\
    \ {\n                fn from(x: $t1) -> Self {\n                    Self::from_raw((x\
    \ as $t2).rem_euclid(Self::modulus() as $t2) as u64)\n                }\n    \
    \        }\n\n            impl From<&$t1> for ModInt61 {\n                fn from(x:\
    \ &$t1) -> Self {\n                    Self::from_raw((*x as $t2).rem_euclid(Self::modulus()\
    \ as $t2) as u64)\n                }\n            }\n        )*\n    };\n}\n\n\
    impl_from_integer! {\n    (i8, i64),\n    (i16, i64),\n    (i32, i64),\n    (i64,\
    \ i64),\n    (isize, i64),\n    (i128, i128),\n    (u8, u64),\n    (u16, u64),\n\
    \    (u32, u64),\n    (u64, u64),\n    (usize, u64),\n    (u128, u128)\n}\n\n\
    macro_rules! impl_ops {\n    ($(\n        $tr:ident,\n        $tr_a:ident,\n \
    \       $f:ident,\n        $f_a:ident,\n    )*) => {$(\n        impl<T: Into<ModInt61>>\
    \ $tr<T> for &ModInt61 {\n            type Output = ModInt61;\n\n            fn\
    \ $f(self, rhs: T) -> Self::Output {\n                (*self).$f(rhs.into())\n\
    \            }\n        }\n\n        impl<T: Into<ModInt61>> $tr_a<T> for ModInt61\
    \ {\n            fn $f_a(&mut self, rhs: T) {\n                *self = (*self).$f(rhs.into());\n\
    \            }\n        }\n    )*};\n}\n\nimpl_ops! {\n    Add, AddAssign, add,\
    \ add_assign,\n    Sub, SubAssign, sub, sub_assign,\n    Mul, MulAssign, mul,\
    \ mul_assign,\n    Div, DivAssign, div, div_assign,\n}\n"
  dependsOn:
  - crates/number_theory/modint/modint_61/src/lib.rs
  - crates/number_theory/modint/modint_61/src/numeric.rs
  isVerificationFile: false
  path: crates/number_theory/modint/modint_61/src/ops.rs
  requiredBy:
  - verify/sandbox/test/src/main.rs
  - crates/number_theory/modint/modint_61/src/lib.rs
  - crates/number_theory/modint/modint_61/src/numeric.rs
  - crates/string/rolling_hash/src/lib.rs
  timestamp: '2025-03-24 01:42:22+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/number_theory/modint/modint_61/src/ops.rs
layout: document
redirect_from:
- /library/crates/number_theory/modint/modint_61/src/ops.rs
- /library/crates/number_theory/modint/modint_61/src/ops.rs.html
title: crates/number_theory/modint/modint_61/src/ops.rs
---
