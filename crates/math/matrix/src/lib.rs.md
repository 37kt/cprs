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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{\n    fmt::{Debug, Write},\n    ops::{Add, Index, IndexMut, Mul,\
    \ Neg, Sub},\n};\n\npub trait Element: Clone {\n    type S: Copy;\n\n    /// 0\n\
    \    fn zero() -> Self::S;\n\n    /// 1\n    fn one() -> Self::S;\n\n    /// a+b\n\
    \    fn add(a: Self::S, b: Self::S) -> Self::S;\n\n    /// a*b\n    fn mul(a:\
    \ Self::S, b: Self::S) -> Self::S;\n\n    #[allow(unused_variables)]\n    ///\
    \ -a\n    fn neg(a: Self::S) -> Self::S {\n        unreachable!()\n    }\n\n \
    \   #[allow(unused_variables)]\n    /// 1/a\n    fn recip(a: Self::S) -> Self::S\
    \ {\n        unreachable!()\n    }\n}\n\n#[derive(Clone)]\npub struct Matrix<E:\
    \ Element> {\n    h: usize,\n    w: usize,\n    v: Vec<E::S>,\n}\n\nimpl<E: Element>\
    \ Matrix<E> {\n    pub fn new(h: usize, w: usize) -> Self {\n        Self {\n\
    \            h,\n            w,\n            v: vec![E::zero(); h * w],\n    \
    \    }\n    }\n\n    pub fn e(n: usize) -> Self {\n        let mut a = Self::new(n,\
    \ n);\n        for i in 0..n {\n            a[i][i] = E::one();\n        }\n \
    \       a\n    }\n\n    pub fn h(&self) -> usize {\n        self.h\n    }\n\n\
    \    pub fn w(&self) -> usize {\n        self.w\n    }\n\n    pub fn pow(&self,\
    \ mut k: usize) -> Self {\n        assert_eq!(self.h, self.w);\n        let mut\
    \ res = Self::e(self.h);\n        let mut x = self.clone();\n        while k >\
    \ 0 {\n            if k & 1 == 1 {\n                res = &res * &x;\n       \
    \     }\n            x = &x * &x;\n            k >>= 1;\n        }\n        res\n\
    \    }\n}\n\nimpl<E> From<Vec<Vec<E::S>>> for Matrix<E>\nwhere\n    E: Element,\n\
    {\n    fn from(v: Vec<Vec<E::S>>) -> Self {\n        assert!(v.iter().all(|x|\
    \ x.len() == v[0].len()));\n        Self {\n            h: v.len(),\n        \
    \    w: v[0].len(),\n            v: v.into_iter().flatten().collect(),\n     \
    \   }\n    }\n}\n\nimpl<E> Debug for Matrix<E>\nwhere\n    E: Element,\n    E::S:\
    \ Debug,\n{\n    #[allow(unused_must_use)]\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)\
    \ -> std::fmt::Result {\n        f.write_char('[');\n        for i in 0..self.h\
    \ {\n            if i > 0 {\n                f.write_str(\", \");\n          \
    \  }\n            f.write_fmt(format_args!(\"{:?}\", &self[i]));\n        }\n\
    \        f.write_char(']')\n    }\n}\n\nimpl<E: Element> Index<usize> for Matrix<E>\
    \ {\n    type Output = [E::S];\n    fn index(&self, index: usize) -> &Self::Output\
    \ {\n        &self.v[index * self.w..(index + 1) * self.w]\n    }\n}\n\nimpl<E:\
    \ Element> IndexMut<usize> for Matrix<E> {\n    fn index_mut(&mut self, index:\
    \ usize) -> &mut Self::Output {\n        &mut self.v[index * self.w..(index +\
    \ 1) * self.w]\n    }\n}\n\nimpl<E: Element> Neg for &Matrix<E> {\n    type Output\
    \ = Matrix<E>;\n    fn neg(self) -> Self::Output {\n        Self::Output {\n \
    \           h: self.h,\n            w: self.w,\n            v: self.v.iter().map(|&x|\
    \ E::neg(x)).collect(),\n        }\n    }\n}\n\nimpl<E: Element> Add<Self> for\
    \ &Matrix<E> {\n    type Output = Matrix<E>;\n    fn add(self, rhs: Self) -> Self::Output\
    \ {\n        assert_eq!(self.h, rhs.h);\n        assert_eq!(self.w, rhs.w);\n\
    \        let mut res = Matrix::new(self.h, self.w);\n        for i in 0..self.h\
    \ * self.w {\n            res.v[i] = E::add(self.v[i], rhs.v[i]);\n        }\n\
    \        res\n    }\n}\n\nimpl<E: Element> Sub<Self> for &Matrix<E> {\n    type\
    \ Output = Matrix<E>;\n    fn sub(self, rhs: Self) -> Self::Output {\n       \
    \ assert_eq!(self.h, rhs.h);\n        assert_eq!(self.w, rhs.w);\n        let\
    \ mut res = Matrix::new(self.h, self.w);\n        for i in 0..self.h * self.w\
    \ {\n            res.v[i] = E::add(self.v[i], E::neg(rhs.v[i]));\n        }\n\
    \        res\n    }\n}\n\nimpl<E: Element> Mul<Self> for &Matrix<E> {\n    type\
    \ Output = Matrix<E>;\n    fn mul(self, rhs: Self) -> Self::Output {\n       \
    \ assert_eq!(self.w, rhs.h);\n        let mut res = Matrix::new(self.h, rhs.w);\n\
    \        for i in 0..self.h {\n            for k in 0..self.w {\n            \
    \    for j in 0..rhs.w {\n                    res[i][j] = E::add(res[i][j], E::mul(self[i][k],\
    \ rhs[k][j]));\n                }\n            }\n        }\n        res\n   \
    \ }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/math/matrix/src/lib.rs
  requiredBy: []
  timestamp: '2023-04-21 11:20:46+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/math/matrix/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/matrix/src/lib.rs
- /library/crates/math/matrix/src/lib.rs.html
title: crates/math/matrix/src/lib.rs
---
