---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
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
  code: "use std::{\n    fmt::{Debug, Display},\n    mem::swap,\n    ops::{Add, AddAssign,\
    \ Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},\n    sync::atomic::{AtomicBool,\
    \ Ordering::SeqCst},\n};\n\ntype Z = i64;\n\nstatic AUTO_REDUCE: AtomicBool =\
    \ AtomicBool::new(true);\n\n#[derive(Clone, Copy)]\npub struct Rational {\n  \
    \  num: Z,\n    den: Z,\n}\n\nfn gcd(mut a: Z, mut b: Z) -> Z {\n    a = a.abs();\n\
    \    b = b.abs();\n    while b != 0 {\n        a %= b;\n        swap(&mut a, &mut\
    \ b);\n    }\n    a\n}\n\nimpl Default for Rational {\n    fn default() -> Self\
    \ {\n        Self { num: 0, den: 1 }\n    }\n}\n\nimpl Display for Rational {\n\
    \    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n  \
    \      write!(f, \"{}/{}\", self.num, self.den)\n    }\n}\n\nimpl Debug for Rational\
    \ {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n\
    \        write!(f, \"{}/{}\", self.num, self.den)\n    }\n}\n\nimpl From<Z> for\
    \ Rational {\n    fn from(x: Z) -> Self {\n        Self { num: x, den: 1 }\n \
    \   }\n}\n\nimpl PartialEq for Rational {\n    fn eq(&self, other: &Self) -> bool\
    \ {\n        self.num * other.den == other.num * self.den\n    }\n\n    fn ne(&self,\
    \ other: &Self) -> bool {\n        !(self == other)\n    }\n}\n\nimpl Eq for Rational\
    \ {}\n\nimpl PartialOrd for Rational {\n    fn partial_cmp(&self, other: &Self)\
    \ -> Option<std::cmp::Ordering> {\n        Some(self.cmp(other))\n    }\n}\n\n\
    impl Ord for Rational {\n    fn cmp(&self, other: &Self) -> std::cmp::Ordering\
    \ {\n        (self.num * other.den).cmp(&(other.num * self.den))\n    }\n}\n\n\
    impl Rational {\n    pub fn set_auto_reduce(auto_reduce: bool) {\n        AUTO_REDUCE.store(auto_reduce,\
    \ SeqCst);\n    }\n\n    pub fn new(num: Z, den: Z) -> Self {\n        let mut\
    \ res = Self { num, den };\n        res.normalize();\n        res\n    }\n\n \
    \   pub fn num(&self) -> Z {\n        self.num\n    }\n\n    pub fn den(&self)\
    \ -> Z {\n        self.den\n    }\n\n    pub fn abs(&self) -> Self {\n       \
    \ Self {\n            num: self.num.abs(),\n            den: self.den,\n     \
    \   }\n    }\n\n    pub fn normalize(&mut self) {\n        assert!(self.den !=\
    \ 0);\n        if self.den < 0 {\n            self.num = -self.num;\n        \
    \    self.den = -self.den;\n        }\n        if self.num == 0 {\n          \
    \  self.den = 1;\n        }\n        if AUTO_REDUCE.load(SeqCst) {\n         \
    \   self.reduce();\n        }\n    }\n\n    pub fn reduce(&mut self) {\n     \
    \   let g = gcd(self.num, self.den);\n        self.num /= g;\n        self.den\
    \ /= g;\n    }\n}\n\nimpl Neg for Rational {\n    type Output = Self;\n    fn\
    \ neg(self) -> Self::Output {\n        Self {\n            num: -self.num,\n \
    \           den: self.den,\n        }\n    }\n}\n\nimpl Neg for &Rational {\n\
    \    type Output = Rational;\n    fn neg(self) -> Self::Output {\n        Rational\
    \ {\n            num: -self.num,\n            den: self.den,\n        }\n    }\n\
    }\n\nimpl AddAssign<Self> for Rational {\n    fn add_assign(&mut self, rhs: Self)\
    \ {\n        self.num = self.num * rhs.den + rhs.num * self.den;\n        self.den\
    \ *= rhs.den;\n        self.normalize();\n    }\n}\n\nimpl SubAssign<Self> for\
    \ Rational {\n    fn sub_assign(&mut self, rhs: Self) {\n        *self += -rhs;\n\
    \    }\n}\n\nimpl MulAssign<Self> for Rational {\n    fn mul_assign(&mut self,\
    \ rhs: Self) {\n        self.num *= rhs.num;\n        self.den *= rhs.den;\n \
    \       self.normalize();\n    }\n}\n\nimpl DivAssign<Self> for Rational {\n \
    \   fn div_assign(&mut self, rhs: Self) {\n        self.num *= rhs.den;\n    \
    \    self.den *= rhs.num;\n        self.normalize();\n    }\n}\n\nmacro_rules!\
    \ impl_ops {\n    ($(\n        $trait:ident,\n        $trait_assign:ident,\n \
    \       $fn:ident,\n        $fn_assign:ident,\n    )*) => {$(\n        impl $trait_assign<&Rational>\
    \ for Rational {\n            fn $fn_assign(&mut self, rhs: &Rational) {\n   \
    \             self.$fn_assign(*rhs);\n            }\n        }\n        impl<T:\
    \ Into<Rational>> $trait<T> for Rational {\n            type Output = Rational;\n\
    \            fn $fn(mut self, rhs: T) -> Self::Output {\n                self.$fn_assign(rhs.into());\n\
    \                self\n            }\n        }\n        impl $trait<&Rational>\
    \ for Rational {\n            type Output = Rational;\n            fn $fn(self,\
    \ rhs: &Rational) -> Self::Output {\n                self.$fn(*rhs)\n        \
    \    }\n        }\n        impl<T: Into<Rational>> $trait<T> for &Rational {\n\
    \            type Output = Rational;\n            fn $fn(self, rhs: T) -> Self::Output\
    \ {\n                (*self).$fn(rhs.into())\n            }\n        }\n     \
    \   impl $trait<&Rational> for &Rational {\n            type Output = Rational;\n\
    \            fn $fn(self, rhs: &Rational) -> Self::Output {\n                (*self).$fn(*rhs)\n\
    \            }\n        }\n    )*};\n}\n\nimpl_ops! {\n    Add, AddAssign, add,\
    \ add_assign,\n    Sub, SubAssign, sub, sub_assign,\n    Mul, MulAssign, mul,\
    \ mul_assign,\n    Div, DivAssign, div, div_assign,\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/algebraic/rational/src/lib.rs
  requiredBy: []
  timestamp: '2023-05-16 16:25:17+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/algebraic/rational/src/lib.rs
layout: document
redirect_from:
- /library/crates/algebraic/rational/src/lib.rs
- /library/crates/algebraic/rational/src/lib.rs.html
title: crates/algebraic/rational/src/lib.rs
---
