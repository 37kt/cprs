---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/rational/src/lib.rs
    title: crates/number_theory/rational/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/rational/src/numeric.rs
    title: crates/number_theory/rational/src/numeric.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/rational/src/lib.rs
    title: crates/number_theory/rational/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/rational/src/numeric.rs
    title: crates/number_theory/rational/src/numeric.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/stress_test/number_theory/comparing_rational/src/main.rs
    title: verify/stress_test/number_theory/comparing_rational/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://misawa.github.io/others/avoid_errors/compare_fractions.html
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{\n    cmp::Ordering,\n    ops::{Add, AddAssign, Div, DivAssign,\
    \ Mul, MulAssign, Neg, Sub, SubAssign},\n};\n\nuse numeric_traits::{Integer, Recip};\n\
    \nuse crate::Rational;\n\nimpl<const AUTO_REDUCE: bool> Neg for Rational<AUTO_REDUCE>\
    \ {\n    type Output = Self;\n\n    fn neg(self) -> Self::Output {\n        Self::from_raw(-self.num,\
    \ self.den)\n    }\n}\n\nimpl<const AUTO_REDUCE: bool> Neg for &Rational<AUTO_REDUCE>\
    \ {\n    type Output = Rational<AUTO_REDUCE>;\n\n    fn neg(self) -> Self::Output\
    \ {\n        -*self\n    }\n}\n\nimpl<const AUTO_REDUCE: bool> Add for Rational<AUTO_REDUCE>\
    \ {\n    type Output = Self;\n\n    fn add(self, rhs: Self) -> Self::Output {\n\
    \        if AUTO_REDUCE {\n            let g = self.den.gcd(rhs.den);\n      \
    \      if g == 0 {\n                if self.is_nan() || rhs.is_nan() {\n     \
    \               return Self::nan();\n                } else {\n              \
    \      return Self::from_raw((self.num.signum() + rhs.num.signum()).signum(),\
    \ 0);\n                }\n            }\n            let num = self.num * (rhs.den\
    \ / g) + rhs.num * (self.den / g);\n            let den = self.den / g * rhs.den;\n\
    \            Self::new(num, den)\n        } else {\n            let num = self.num\
    \ * rhs.den + rhs.num * self.den;\n            let den = self.den * rhs.den;\n\
    \            Self::from_raw(num, den)\n        }\n    }\n}\n\nimpl<const AUTO_REDUCE:\
    \ bool> Sub for Rational<AUTO_REDUCE> {\n    type Output = Self;\n\n    fn sub(self,\
    \ rhs: Self) -> Self::Output {\n        self + -rhs\n    }\n}\n\nimpl<const AUTO_REDUCE:\
    \ bool> Mul for Rational<AUTO_REDUCE> {\n    type Output = Self;\n\n    fn mul(self,\
    \ rhs: Self) -> Self::Output {\n        let (g1, g2) = if AUTO_REDUCE {\n    \
    \        (self.num.gcd(rhs.den), self.den.gcd(rhs.num))\n        } else {\n  \
    \          (1, 1)\n        };\n        if g1 == 0 || g2 == 0 {\n            return\
    \ Self::nan();\n        }\n        let num = (self.num / g1) * (rhs.num / g2);\n\
    \        let den = (self.den / g2) * (rhs.den / g1);\n        Self::from_raw(num,\
    \ den)\n    }\n}\n\nimpl<const AUTO_REDUCE: bool> Div for Rational<AUTO_REDUCE>\
    \ {\n    type Output = Self;\n\n    #[allow(clippy::suspicious_arithmetic_impl)]\n\
    \    fn div(self, rhs: Self) -> Self::Output {\n        self * rhs.recip()\n \
    \   }\n}\n\nimpl<const AUTO_REDUCE: bool> PartialEq for Rational<AUTO_REDUCE>\
    \ {\n    fn eq(&self, other: &Self) -> bool {\n        self.cmp(other).is_eq()\n\
    \    }\n}\n\nimpl<const AUTO_REDUCE: bool> Eq for Rational<AUTO_REDUCE> {}\n\n\
    // https://misawa.github.io/others/avoid_errors/compare_fractions.html\nfn compare<const\
    \ AUTO_REDUCE: bool>(\n    mut a: Rational<AUTO_REDUCE>,\n    mut b: Rational<AUTO_REDUCE>,\n\
    ) -> Ordering {\n    if a.num <= 0 || b.num <= 0 {\n        if a.num == 0 || b.num\
    \ == 0 || ((a.num < 0) ^ (b.num < 0)) {\n            return a.num.cmp(&b.num);\n\
    \        } else {\n            return compare(-b, -a);\n        }\n    }\n   \
    \ let ord = (a.num / a.den).cmp(&(b.num / b.den));\n    if !ord.is_eq() {\n  \
    \      return ord;\n    }\n    a.num %= a.den;\n    b.num %= b.den;\n    if a.num\
    \ == 0 || b.num == 0 {\n        (a.num * b.den).cmp(&(a.den * b.num))\n    } else\
    \ {\n        compare(b.recip(), a.recip())\n    }\n}\n\nimpl<const AUTO_REDUCE:\
    \ bool> PartialOrd for Rational<AUTO_REDUCE> {\n    fn partial_cmp(&self, other:\
    \ &Self) -> Option<Ordering> {\n        if self.is_nan() || other.is_nan() {\n\
    \            return None;\n        }\n        let sgn = |x: Rational<AUTO_REDUCE>|\
    \ {\n            if x.is_inf() {\n                1\n            } else if x.is_neg_inf()\
    \ {\n                -1\n            } else {\n                0\n           \
    \ }\n        };\n        let sgn_l = sgn(*self);\n        let sgn_r = sgn(*other);\n\
    \        if sgn_l == 0 && sgn_r == 0 {\n            return Some(compare(*self,\
    \ *other));\n        }\n        match sgn_l.cmp(&sgn_r) {\n            Ordering::Equal\
    \ => None,\n            res => Some(res),\n        }\n    }\n}\n\nimpl<const AUTO_REDUCE:\
    \ bool> Ord for Rational<AUTO_REDUCE> {\n    fn cmp(&self, other: &Self) -> Ordering\
    \ {\n        self.partial_cmp(other).unwrap()\n    }\n}\n\nmacro_rules! impl_ops\
    \ {\n    ($(\n        $tr:ident,\n        $tr_a:ident,\n        $f:ident,\n  \
    \      $f_a:ident,\n    )*) => {$(\n        impl<const AUTO_REDUCE: bool> $tr<&Rational<AUTO_REDUCE>>\
    \ for Rational<AUTO_REDUCE> {\n            type Output = Rational<AUTO_REDUCE>;\n\
    \n            fn $f(self, rhs: &Rational<AUTO_REDUCE>) -> Self::Output {\n   \
    \             self.$f(*rhs)\n            }\n        }\n\n        impl<const AUTO_REDUCE:\
    \ bool> $tr<Rational<AUTO_REDUCE>> for &Rational<AUTO_REDUCE> {\n            type\
    \ Output = Rational<AUTO_REDUCE>;\n\n            fn $f(self, rhs: Rational<AUTO_REDUCE>)\
    \ -> Self::Output {\n                (*self).$f(rhs)\n            }\n        }\n\
    \n        impl<const AUTO_REDUCE: bool> $tr<&Rational<AUTO_REDUCE>> for &Rational<AUTO_REDUCE>\
    \ {\n            type Output = Rational<AUTO_REDUCE>;\n\n            fn $f(self,\
    \ rhs: &Rational<AUTO_REDUCE>) -> Self::Output {\n                (*self).$f(*rhs)\n\
    \            }\n        }\n\n        impl<const AUTO_REDUCE: bool> $tr_a<Rational<AUTO_REDUCE>>\
    \ for Rational<AUTO_REDUCE> {\n            fn $f_a(&mut self, rhs: Rational<AUTO_REDUCE>)\
    \ {\n                *self = (*self).$f(rhs);\n            }\n        }\n\n  \
    \      impl<const AUTO_REDUCE: bool> $tr_a<&Rational<AUTO_REDUCE>> for Rational<AUTO_REDUCE>\
    \ {\n            fn $f_a(&mut self, rhs: &Rational<AUTO_REDUCE>) {\n         \
    \       *self = (*self).$f(*rhs);\n            }\n        }\n    )*};\n}\n\nimpl_ops!\
    \ {\n    Add, AddAssign, add, add_assign,\n    Sub, SubAssign, sub, sub_assign,\n\
    \    Mul, MulAssign, mul, mul_assign,\n    Div, DivAssign, div, div_assign,\n\
    }\n"
  dependsOn:
  - crates/number_theory/rational/src/lib.rs
  - crates/number_theory/rational/src/numeric.rs
  isVerificationFile: false
  path: crates/number_theory/rational/src/ops.rs
  requiredBy:
  - crates/number_theory/rational/src/lib.rs
  - crates/number_theory/rational/src/numeric.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/stress_test/number_theory/comparing_rational/src/main.rs
documentation_of: crates/number_theory/rational/src/ops.rs
layout: document
redirect_from:
- /library/crates/number_theory/rational/src/ops.rs
- /library/crates/number_theory/rational/src/ops.rs.html
title: crates/number_theory/rational/src/ops.rs
---
