---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/math/stern-brocot-tree/src/lib.rs
    title: crates/math/stern-brocot-tree/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/rational_approximation/src/main.rs
    title: verify/rational_approximation/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/stern_brocot_tree/src/main.rs
    title: verify/stern_brocot_tree/src/main.rs
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
  code: "use std::{\n    fmt::{Debug, Display},\n    ops::{Add, AddAssign, Div, DivAssign,\
    \ Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},\n    sync::atomic::{AtomicBool,\
    \ Ordering::SeqCst},\n};\n\nuse algebraic::{One, Zero};\n\nstatic AUTO_REDUCE:\
    \ AtomicBool = AtomicBool::new(true);\n\npub trait ZTrait:\n    Copy\n    + PartialEq\n\
    \    + PartialOrd\n    + Eq\n    + Ord\n    + Zero\n    + One\n    + Add<Output\
    \ = Self>\n    + Sub<Output = Self>\n    + Mul<Output = Self>\n    + Div<Output\
    \ = Self>\n    + Neg<Output = Self>\n    + Rem<Output = Self>\n    + AddAssign\n\
    \    + SubAssign\n    + MulAssign\n    + DivAssign\n    + RemAssign\n    + Debug\n\
    \    + Display\n    + std::iter::Sum\n    + From<i32>\n{\n    fn abs(&self) ->\
    \ Self {\n        if *self < Self::zero() {\n            -*self\n        } else\
    \ {\n            *self\n        }\n    }\n}\n\nimpl ZTrait for i32 {}\nimpl ZTrait\
    \ for i64 {}\nimpl ZTrait for i128 {}\n\n#[derive(Clone, Copy)]\npub struct Rational<T>\n\
    where\n    T: ZTrait,\n{\n    pub num: T,\n    pub den: T,\n}\n\nfn gcd<T: ZTrait>(mut\
    \ a: T, mut b: T) -> T {\n    a = a.abs();\n    b = b.abs();\n    while b != T::zero()\
    \ {\n        a %= b;\n        std::mem::swap(&mut a, &mut b);\n    }\n    a\n\
    }\n\nimpl<T> Default for Rational<T>\nwhere\n    T: ZTrait,\n{\n    fn default()\
    \ -> Self {\n        Self {\n            num: T::zero(),\n            den: T::one(),\n\
    \        }\n    }\n}\n\nimpl<T> Display for Rational<T>\nwhere\n    T: ZTrait,\n\
    {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n\
    \        write!(f, \"{}/{}\", self.num, self.den)\n    }\n}\n\nimpl<T> Debug for\
    \ Rational<T>\nwhere\n    T: ZTrait,\n{\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)\
    \ -> std::fmt::Result {\n        write!(f, \"{}/{}\", self.num, self.den)\n  \
    \  }\n}\n\nimpl<T> From<T> for Rational<T>\nwhere\n    T: ZTrait,\n{\n    fn from(x:\
    \ T) -> Self {\n        Self {\n            num: x,\n            den: T::one(),\n\
    \        }\n    }\n}\n\nimpl<T> PartialEq for Rational<T>\nwhere\n    T: ZTrait,\n\
    {\n    fn eq(&self, other: &Self) -> bool {\n        self.num * other.den == other.num\
    \ * self.den\n    }\n\n    fn ne(&self, other: &Self) -> bool {\n        !(self\
    \ == other)\n    }\n}\n\nimpl<T> Eq for Rational<T> where T: ZTrait {}\n\nimpl<T>\
    \ PartialOrd for Rational<T>\nwhere\n    T: ZTrait,\n{\n    fn partial_cmp(&self,\
    \ other: &Self) -> Option<std::cmp::Ordering> {\n        Some(self.cmp(other))\n\
    \    }\n}\n\nimpl<T> Ord for Rational<T>\nwhere\n    T: ZTrait,\n{\n    fn cmp(&self,\
    \ other: &Self) -> std::cmp::Ordering {\n        (self.num * other.den).cmp(&(other.num\
    \ * self.den))\n    }\n}\n\nimpl<T> Rational<T>\nwhere\n    T: ZTrait,\n{\n  \
    \  pub fn set_auto_reduce(auto_reduce: bool) {\n        AUTO_REDUCE.store(auto_reduce,\
    \ SeqCst);\n    }\n\n    pub fn new(num: T, den: T) -> Self {\n        let mut\
    \ res = Self { num, den };\n        res.normalize();\n        res\n    }\n\n \
    \   pub fn raw(num: T, den: T) -> Self {\n        Self { num, den }\n    }\n\n\
    \    pub fn num(&self) -> T {\n        self.num\n    }\n\n    pub fn den(&self)\
    \ -> T {\n        self.den\n    }\n\n    pub fn abs(&self) -> Self {\n       \
    \ Self {\n            num: self.num.abs(),\n            den: self.den,\n     \
    \   }\n    }\n\n    pub fn normalize(&mut self) {\n        assert!(self.num !=\
    \ T::zero() || self.den != T::zero());\n        if self.den == T::zero() {\n \
    \           self.num = if self.num > T::zero() {\n                T::one()\n \
    \           } else {\n                -T::one()\n            };\n            self.den\
    \ = T::zero();\n            return;\n        }\n        if self.den < T::zero()\
    \ {\n            self.num = -self.num;\n            self.den = -self.den;\n  \
    \      }\n        if self.num == T::zero() {\n            self.den = T::one();\n\
    \        }\n        if AUTO_REDUCE.load(SeqCst) {\n            self.reduce();\n\
    \        }\n    }\n\n    pub fn reduce(&mut self) {\n        let g = gcd(self.num,\
    \ self.den);\n        self.num /= g;\n        self.den /= g;\n    }\n}\n\nimpl<T>\
    \ Neg for Rational<T>\nwhere\n    T: ZTrait,\n{\n    type Output = Self;\n   \
    \ fn neg(self) -> Self::Output {\n        Self {\n            num: -self.num,\n\
    \            den: self.den,\n        }\n    }\n}\n\nimpl<T> Neg for &Rational<T>\n\
    where\n    T: ZTrait,\n{\n    type Output = Rational<T>;\n    fn neg(self) ->\
    \ Self::Output {\n        Rational {\n            num: -self.num,\n          \
    \  den: self.den,\n        }\n    }\n}\n\nimpl<T> AddAssign<Self> for Rational<T>\n\
    where\n    T: ZTrait,\n{\n    fn add_assign(&mut self, rhs: Self) {\n        self.num\
    \ = self.num * rhs.den + rhs.num * self.den;\n        self.den *= rhs.den;\n \
    \       self.normalize();\n    }\n}\n\nimpl<T> SubAssign<Self> for Rational<T>\n\
    where\n    T: ZTrait,\n{\n    fn sub_assign(&mut self, rhs: Self) {\n        *self\
    \ += -rhs;\n    }\n}\n\nimpl<T> MulAssign<Self> for Rational<T>\nwhere\n    T:\
    \ ZTrait,\n{\n    fn mul_assign(&mut self, rhs: Self) {\n        self.num *= rhs.num;\n\
    \        self.den *= rhs.den;\n        self.normalize();\n    }\n}\n\nimpl<T>\
    \ DivAssign<Self> for Rational<T>\nwhere\n    T: ZTrait,\n{\n    fn div_assign(&mut\
    \ self, rhs: Self) {\n        self.num *= rhs.den;\n        self.den *= rhs.num;\n\
    \        self.normalize();\n    }\n}\n\nmacro_rules! impl_ops {\n    ($(\n   \
    \     $trait:ident,\n        $trait_assign:ident,\n        $fn:ident,\n      \
    \  $fn_assign:ident,\n    )*) => {$(\n        impl<T> $trait_assign<&Rational<T>>\
    \ for Rational<T>\n        where\n            T: ZTrait,\n        {\n        \
    \    fn $fn_assign(&mut self, rhs: &Rational<T>) {\n                self.$fn_assign(*rhs);\n\
    \            }\n        }\n        impl<T, U: Into<Rational<T>>> $trait<U> for\
    \ Rational<T>\n        where\n            T: ZTrait,\n        {\n            type\
    \ Output = Rational<T>;\n            fn $fn(mut self, rhs: U) -> Self::Output\
    \ {\n                self.$fn_assign(rhs.into());\n                self\n    \
    \        }\n        }\n        impl<T> $trait<&Rational<T>> for Rational<T>\n\
    \        where\n            T: ZTrait,\n        {\n            type Output = Rational<T>;\n\
    \            fn $fn(self, rhs: &Rational<T>) -> Self::Output {\n             \
    \   self.$fn(*rhs)\n            }\n        }\n        impl<T, U: Into<Rational<T>>>\
    \ $trait<U> for &Rational<T>\n        where\n            T: ZTrait,\n        {\n\
    \            type Output = Rational<T>;\n            fn $fn(self, rhs: U) -> Self::Output\
    \ {\n                (*self).$fn(rhs.into())\n            }\n        }\n     \
    \   impl<T> $trait<&Rational<T>> for &Rational<T>\n        where\n           \
    \ T: ZTrait,\n        {\n            type Output = Rational<T>;\n            fn\
    \ $fn(self, rhs: &Rational<T>) -> Self::Output {\n                (*self).$fn(*rhs)\n\
    \            }\n        }\n    )*};\n}\n\nimpl_ops! {\n    Add, AddAssign, add,\
    \ add_assign,\n    Sub, SubAssign, sub, sub_assign,\n    Mul, MulAssign, mul,\
    \ mul_assign,\n    Div, DivAssign, div, div_assign,\n}\n\nimpl<T> Zero for Rational<T>\n\
    where\n    T: ZTrait,\n{\n    fn zero() -> Self {\n        Self::from(T::zero())\n\
    \    }\n\n    fn is_zero(&self) -> bool {\n        self.num == T::zero()\n   \
    \ }\n}\n\nimpl<T> One for Rational<T>\nwhere\n    T: ZTrait,\n{\n    fn one()\
    \ -> Self {\n        Self::from(T::one())\n    }\n\n    fn is_one(&self) -> bool\
    \ {\n        self.num == T::one() && self.den == T::one()\n    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  isVerificationFile: false
  path: crates/algebraic/rational/src/lib.rs
  requiredBy:
  - crates/math/stern-brocot-tree/src/lib.rs
  timestamp: '2024-03-21 11:10:47+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/rational_approximation/src/main.rs
  - verify/stern_brocot_tree/src/main.rs
documentation_of: crates/algebraic/rational/src/lib.rs
layout: document
redirect_from:
- /library/crates/algebraic/rational/src/lib.rs
- /library/crates/algebraic/rational/src/lib.rs.html
title: crates/algebraic/rational/src/lib.rs
---
