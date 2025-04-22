---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/number_theory/modint/dynamic_modint_64/src/barrett_reduction.rs
    title: crates/number_theory/modint/dynamic_modint_64/src/barrett_reduction.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/dynamic_modint_64/src/lib.rs
    title: crates/number_theory/modint/dynamic_modint_64/src/lib.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/dynamic_modint_64/src/numeric.rs
    title: crates/number_theory/modint/dynamic_modint_64/src/numeric.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/number_theory/modint/dynamic_modint_64/src/barrett_reduction.rs
    title: crates/number_theory/modint/dynamic_modint_64/src/barrett_reduction.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/dynamic_modint_64/src/lib.rs
    title: crates/number_theory/modint/dynamic_modint_64/src/lib.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/dynamic_modint_64/src/numeric.rs
    title: crates/number_theory/modint/dynamic_modint_64/src/numeric.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/prime_factorization/src/lib.rs
    title: crates/number_theory/prime_factorization/src/lib.rs
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
  code: "use std::{\n    marker::PhantomData,\n    num::ParseIntError,\n    ops::{Add,\
    \ AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},\n    str::FromStr,\n\
    };\n\nuse barrett_reduction::BarrettReduction64;\n\nuse crate::{barrett_reduction::barrett_reduction,\
    \ DynamicModInt64};\n\nimpl<Id> DynamicModInt64<Id> {\n    pub fn new<T: Into<DynamicModInt64<Id>>>(x:\
    \ T) -> Self {\n        x.into()\n    }\n\n    pub fn from_raw(x: u64) -> Self\
    \ {\n        Self(x, PhantomData::default())\n    }\n\n    pub fn set_modulus(m:\
    \ u64) {\n        barrett_reduction::<Id, _>(|br| {\n            br.replace(BarrettReduction64::new(m));\n\
    \        });\n    }\n\n    pub fn modulus() -> u64 {\n        barrett_reduction::<Id,\
    \ _>(|br| br.get().modulus())\n    }\n\n    pub fn val(self) -> u64 {\n      \
    \  self.0\n    }\n\n    pub fn pow(self, exp: usize) -> Self {\n        let mut\
    \ res = Self::from(1);\n        let mut base = self;\n        let mut exp = exp;\n\
    \        while exp > 0 {\n            if exp % 2 == 1 {\n                res *=\
    \ base;\n            }\n            base *= base;\n            exp /= 2;\n   \
    \     }\n        res\n    }\n\n    pub fn recip(self) -> Self {\n        Self::from_raw(inv_mod(self.0,\
    \ Self::modulus()))\n    }\n\n    pub fn sqrt(self) -> Option<Self> {\n      \
    \  let p = Self::modulus() as usize;\n        if self.0 < 2 {\n            return\
    \ Some(self);\n        } else if self.pow((p - 1) >> 1).val() != 1 {\n       \
    \     return None;\n        }\n\n        let mut b = Self::from_raw(1);\n    \
    \    while b.pow((p - 1) >> 1).val() == 1 {\n            b += 1;\n        }\n\n\
    \        let mut e = (p - 1).trailing_zeros() as usize;\n        let m = (p -\
    \ 1) >> e;\n        let mut x = self.pow((m - 1) >> 1);\n        let mut y = self\
    \ * x * x;\n        x *= self;\n        let mut z = b.pow(m);\n        while y.val()\
    \ != 1 {\n            let mut j = 0;\n            let mut t = y;\n           \
    \ while t.val() != 1 {\n                t *= t;\n                j += 1;\n   \
    \         }\n            z = z.pow(1 << (e - j - 1));\n            x *= z;\n \
    \           z *= z;\n            y *= z;\n            e = j;\n        }\n\n  \
    \      Some(x)\n    }\n}\n\nimpl<Id> From<&DynamicModInt64<Id>> for DynamicModInt64<Id>\
    \ {\n    fn from(x: &DynamicModInt64<Id>) -> Self {\n        *x\n    }\n}\n\n\
    impl<Id> FromStr for DynamicModInt64<Id> {\n    type Err = ParseIntError;\n\n\
    \    fn from_str(s: &str) -> Result<Self, Self::Err> {\n        s.parse::<i64>().map(Self::from)\n\
    \    }\n}\n\nimpl<Id> std::fmt::Display for DynamicModInt64<Id> {\n    fn fmt(&self,\
    \ f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n        write!(f, \"\
    {}\", self.0)\n    }\n}\n\nimpl<Id> std::fmt::Debug for DynamicModInt64<Id> {\n\
    \    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n  \
    \      write!(f, \"{}\", self.0)\n    }\n}\n\nimpl<Id> Neg for DynamicModInt64<Id>\
    \ {\n    type Output = Self;\n\n    fn neg(self) -> Self::Output {\n        if\
    \ self.0 == 0 {\n            Self::from_raw(0)\n        } else {\n           \
    \ Self::from_raw(Self::modulus() - self.0)\n        }\n    }\n}\n\nimpl<Id> Neg\
    \ for &DynamicModInt64<Id> {\n    type Output = DynamicModInt64<Id>;\n\n    fn\
    \ neg(self) -> Self::Output {\n        -*self\n    }\n}\n\nimpl<Id, T: Into<DynamicModInt64<Id>>>\
    \ Add<T> for DynamicModInt64<Id> {\n    type Output = Self;\n\n    fn add(self,\
    \ rhs: T) -> Self::Output {\n        let rhs = rhs.into();\n        let mut x\
    \ = self.0 + rhs.0;\n        if x >= Self::modulus() {\n            x -= Self::modulus();\n\
    \        }\n        Self::from_raw(x)\n    }\n}\n\nimpl<Id, T: Into<DynamicModInt64<Id>>>\
    \ Sub<T> for DynamicModInt64<Id> {\n    type Output = Self;\n\n    fn sub(self,\
    \ rhs: T) -> Self::Output {\n        let rhs = rhs.into();\n        let mut x\
    \ = self.0 + Self::modulus() - rhs.0;\n        if x >= Self::modulus() {\n   \
    \         x -= Self::modulus();\n        }\n        Self::from_raw(x)\n    }\n\
    }\n\nimpl<Id, T: Into<DynamicModInt64<Id>>> Mul<T> for DynamicModInt64<Id> {\n\
    \    type Output = Self;\n\n    fn mul(self, rhs: T) -> Self::Output {\n     \
    \   let rhs = rhs.into();\n        barrett_reduction::<Id, _>(|br| Self::from_raw(br.get().mul(self.0,\
    \ rhs.0)))\n    }\n}\n\nimpl<Id, T: Into<DynamicModInt64<Id>>> Div<T> for DynamicModInt64<Id>\
    \ {\n    type Output = Self;\n\n    #[allow(clippy::suspicious_arithmetic_impl)]\n\
    \    fn div(self, rhs: T) -> Self::Output {\n        let rhs = rhs.into();\n \
    \       self * rhs.recip()\n    }\n}\n\nmacro_rules! impl_from_integer {\n   \
    \ ($(($t1:ty, $t2:ty)),*) => {\n        $(\n            impl<Id> From<$t1> for\
    \ DynamicModInt64<Id> {\n                fn from(x: $t1) -> Self {\n         \
    \           Self::from_raw((x as $t2).rem_euclid(Self::modulus() as $t2) as u64)\n\
    \                }\n            }\n        )*\n    };\n}\n\nimpl_from_integer!\
    \ {\n    (i8, i64),\n    (i16, i64),\n    (i32, i64),\n    (i64, i64),\n    (isize,\
    \ i64),\n    (i128, i128),\n    (u8, u64),\n    (u16, u64),\n    (u32, u64),\n\
    \    (u64, u64),\n    (usize, u64),\n    (u128, u128)\n}\n\nmacro_rules! impl_ops\
    \ {\n    ($(\n        $tr:ident,\n        $tr_a:ident,\n        $f:ident,\n  \
    \      $f_a:ident,\n    )*) => {$(\n        impl<Id, T: Into<DynamicModInt64<Id>>>\
    \ $tr<T> for &DynamicModInt64<Id> {\n            type Output = DynamicModInt64<Id>;\n\
    \n            fn $f(self, rhs: T) -> Self::Output {\n                (*self).$f(rhs.into())\n\
    \            }\n        }\n\n        impl<Id, T: Into<DynamicModInt64<Id>>> $tr_a<T>\
    \ for DynamicModInt64<Id> {\n            fn $f_a(&mut self, rhs: T) {\n      \
    \          *self = (*self).$f(rhs.into());\n            }\n        }\n    )*};\n\
    }\n\nimpl_ops! {\n    Add, AddAssign, add, add_assign,\n    Sub, SubAssign, sub,\
    \ sub_assign,\n    Mul, MulAssign, mul, mul_assign,\n    Div, DivAssign, div,\
    \ div_assign,\n}\n\nconst fn inv_mod(x: u64, m: u64) -> u64 {\n    let (mut a,\
    \ mut b, mut x, mut y) = (1, 0, x, m);\n    if m == 1 {\n        return 0;\n \
    \   }\n\n    loop {\n        match x {\n            0 => panic!(\"gcd(x, m) is\
    \ not 1.\"),\n            1 => return a,\n            _ => {}\n        }\n   \
    \     b += a * (y / x);\n        y %= x;\n\n        match y {\n            0 =>\
    \ panic!(\"gcd(x, m) is not 1.\"),\n            1 => return m - b,\n         \
    \   _ => {}\n        }\n        a += b * (x / y);\n        x %= y;\n    }\n}\n"
  dependsOn:
  - crates/number_theory/modint/dynamic_modint_64/src/barrett_reduction.rs
  - crates/number_theory/modint/dynamic_modint_64/src/lib.rs
  - crates/number_theory/modint/dynamic_modint_64/src/numeric.rs
  isVerificationFile: false
  path: crates/number_theory/modint/dynamic_modint_64/src/ops.rs
  requiredBy:
  - crates/number_theory/prime_factorization/src/lib.rs
  - crates/number_theory/modint/dynamic_modint_64/src/lib.rs
  - crates/number_theory/modint/dynamic_modint_64/src/barrett_reduction.rs
  - crates/number_theory/modint/dynamic_modint_64/src/numeric.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/number_theory/modint/dynamic_modint_64/src/ops.rs
layout: document
redirect_from:
- /library/crates/number_theory/modint/dynamic_modint_64/src/ops.rs
- /library/crates/number_theory/modint/dynamic_modint_64/src/ops.rs.html
title: crates/number_theory/modint/dynamic_modint_64/src/ops.rs
---
