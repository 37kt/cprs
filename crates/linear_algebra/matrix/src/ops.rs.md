---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/linear_algebra/matrix/src/index.rs
    title: crates/linear_algebra/matrix/src/index.rs
  - icon: ':heavy_check_mark:'
    path: crates/linear_algebra/matrix/src/lib.rs
    title: crates/linear_algebra/matrix/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/linear_algebra/matrix/src/row_reduction.rs
    title: crates/linear_algebra/matrix/src/row_reduction.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/linear_algebra/matrix/src/index.rs
    title: crates/linear_algebra/matrix/src/index.rs
  - icon: ':heavy_check_mark:'
    path: crates/linear_algebra/matrix/src/lib.rs
    title: crates/linear_algebra/matrix/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/linear_algebra/matrix/src/row_reduction.rs
    title: crates/linear_algebra/matrix/src/row_reduction.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/linear_algebra/inverse_matrix/src/main.rs
    title: verify/library_checker/linear_algebra/inverse_matrix/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/linear_algebra/matrix_det/src/main.rs
    title: verify/library_checker/linear_algebra/matrix_det/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/linear_algebra/matrix_product/src/main.rs
    title: verify/library_checker/linear_algebra/matrix_product/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/linear_algebra/matrix_rank/src/main.rs
    title: verify/library_checker/linear_algebra/matrix_rank/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/linear_algebra/pow_of_matrix/src/main.rs
    title: verify/library_checker/linear_algebra/pow_of_matrix/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};\n\n\
    use algebraic_traits::{Invertive, Ring, Semiring};\n\nuse crate::Matrix;\n\nimpl<T:\
    \ Semiring> Matrix<T>\nwhere\n    T::Value: Clone,\n{\n    pub fn pow(&self, mut\
    \ exp: usize) -> Self {\n        assert!(self.is_square());\n        let mut res\
    \ = Matrix::identity(self.h());\n        let mut base = self.clone();\n      \
    \  while exp != 0 {\n            if exp & 1 == 1 {\n                res *= &base;\n\
    \            }\n            base = &base * &base;\n            exp >>= 1;\n  \
    \      }\n        res\n    }\n}\n\nimpl<T: Semiring> PartialEq for Matrix<T>\n\
    where\n    T::Value: PartialEq,\n{\n    fn eq(&self, other: &Self) -> bool {\n\
    \        self.h() == other.h() && self.w() == other.w() && self.val == other.val\n\
    \    }\n}\n\nimpl<T: Semiring> Eq for Matrix<T> where T::Value: Eq {}\n\nimpl<T:\
    \ Ring> Neg for Matrix<T>\nwhere\n    T::Additive: Invertive,\n{\n    type Output\
    \ = Matrix<T>;\n\n    fn neg(mut self) -> Self::Output {\n        for v in &mut\
    \ self.val {\n            *v = T::neg(v);\n        }\n        self\n    }\n}\n\
    \nimpl<T: Ring> Neg for &Matrix<T>\nwhere\n    T::Additive: Invertive,\n{\n  \
    \  type Output = Matrix<T>;\n\n    fn neg(self) -> Self::Output {\n        Matrix::from_fn(self.h,\
    \ self.w, |i, j| T::neg(&self[i][j]))\n    }\n}\n\nimpl<T: Semiring> Add for Matrix<T>\
    \ {\n    type Output = Matrix<T>;\n\n    fn add(mut self, rhs: Self) -> Self::Output\
    \ {\n        self += &rhs;\n        self\n    }\n}\n\nimpl<T: Semiring> Add<&Matrix<T>>\
    \ for Matrix<T> {\n    type Output = Matrix<T>;\n\n    fn add(mut self, rhs: &Matrix<T>)\
    \ -> Self::Output {\n        self += rhs;\n        self\n    }\n}\n\nimpl<T: Semiring>\
    \ Add<Matrix<T>> for &Matrix<T> {\n    type Output = Matrix<T>;\n\n    fn add(self,\
    \ mut rhs: Matrix<T>) -> Self::Output {\n        rhs += self;\n        rhs\n \
    \   }\n}\n\nimpl<T: Semiring> Add for &Matrix<T>\nwhere\n    T::Value: Clone,\n\
    {\n    type Output = Matrix<T>;\n\n    fn add(self, rhs: &Matrix<T>) -> Self::Output\
    \ {\n        assert_eq!(self.h(), rhs.h());\n        assert_eq!(self.w(), rhs.w());\n\
    \        Matrix {\n            h: self.h(),\n            w: self.w(),\n      \
    \      val: self\n                .val\n                .iter()\n            \
    \    .zip(&rhs.val)\n                .map(|(x, y)| T::add(x, y))\n           \
    \     .collect(),\n        }\n    }\n}\n\nimpl<T: Semiring> AddAssign<&Matrix<T>>\
    \ for Matrix<T> {\n    fn add_assign(&mut self, rhs: &Matrix<T>) {\n        assert_eq!(self.h(),\
    \ rhs.h());\n        assert_eq!(self.w(), rhs.w());\n        self.val.iter_mut().zip(&rhs.val).for_each(|(x,\
    \ y)| {\n            *x = T::add(x, y);\n        });\n    }\n}\n\nimpl<T: Semiring>\
    \ AddAssign for Matrix<T> {\n    fn add_assign(&mut self, rhs: Matrix<T>) {\n\
    \        *self += &rhs;\n    }\n}\n\nimpl<T: Ring> Sub for Matrix<T>\nwhere\n\
    \    T::Additive: Invertive,\n{\n    type Output = Matrix<T>;\n\n    fn sub(mut\
    \ self, rhs: Self) -> Self::Output {\n        self -= &rhs;\n        self\n  \
    \  }\n}\n\nimpl<T: Ring> Sub<&Matrix<T>> for Matrix<T>\nwhere\n    T::Additive:\
    \ Invertive,\n{\n    type Output = Matrix<T>;\n\n    fn sub(mut self, rhs: &Matrix<T>)\
    \ -> Self::Output {\n        self -= rhs;\n        self\n    }\n}\n\nimpl<T: Ring>\
    \ Sub<Matrix<T>> for &Matrix<T>\nwhere\n    T::Additive: Invertive,\n{\n    type\
    \ Output = Matrix<T>;\n\n    fn sub(self, mut rhs: Matrix<T>) -> Self::Output\
    \ {\n        for (x, y) in self.val.iter().zip(rhs.val.iter_mut()) {\n       \
    \     *y = T::sub(x, y);\n        }\n        rhs\n    }\n}\n\nimpl<T: Ring> Sub\
    \ for &Matrix<T>\nwhere\n    T::Additive: Invertive,\n{\n    type Output = Matrix<T>;\n\
    \n    fn sub(self, rhs: &Matrix<T>) -> Self::Output {\n        assert_eq!(self.h(),\
    \ rhs.h());\n        assert_eq!(self.w(), rhs.w());\n        Matrix {\n      \
    \      h: self.h(),\n            w: self.w(),\n            val: self\n       \
    \         .val\n                .iter()\n                .zip(&rhs.val)\n    \
    \            .map(|(x, y)| T::sub(x, y))\n                .collect(),\n      \
    \  }\n    }\n}\n\nimpl<T: Ring> SubAssign<&Matrix<T>> for Matrix<T>\nwhere\n \
    \   T::Additive: Invertive,\n{\n    fn sub_assign(&mut self, rhs: &Matrix<T>)\
    \ {\n        assert_eq!(self.h(), rhs.h());\n        assert_eq!(self.w(), rhs.w());\n\
    \        self.val.iter_mut().zip(&rhs.val).for_each(|(x, y)| {\n            *x\
    \ = T::sub(x, y);\n        });\n    }\n}\n\nimpl<T: Ring> SubAssign for Matrix<T>\n\
    where\n    T::Additive: Invertive,\n{\n    fn sub_assign(&mut self, rhs: Matrix<T>)\
    \ {\n        *self -= &rhs;\n    }\n}\n\nimpl<T: Semiring> Mul<Matrix<T>> for\
    \ Matrix<T> {\n    type Output = Matrix<T>;\n\n    fn mul(self, rhs: Matrix<T>)\
    \ -> Self::Output {\n        &self * &rhs\n    }\n}\n\nimpl<T: Semiring> Mul<&Matrix<T>>\
    \ for Matrix<T> {\n    type Output = Matrix<T>;\n\n    fn mul(self, rhs: &Matrix<T>)\
    \ -> Self::Output {\n        &self * rhs\n    }\n}\n\nimpl<T: Semiring> Mul<Matrix<T>>\
    \ for &Matrix<T> {\n    type Output = Matrix<T>;\n\n    fn mul(self, rhs: Matrix<T>)\
    \ -> Self::Output {\n        self * &rhs\n    }\n}\n\nimpl<T: Semiring> Mul<&Matrix<T>>\
    \ for &Matrix<T> {\n    type Output = Matrix<T>;\n\n    fn mul(self, rhs: &Matrix<T>)\
    \ -> Self::Output {\n        assert_eq!(self.w(), rhs.h());\n        let mut res\
    \ = Matrix::<T>::zeros(self.h(), rhs.w());\n        for i in 0..self.h() {\n \
    \           for k in 0..self.w() {\n                for j in 0..rhs.w() {\n  \
    \                  res[i][j] = T::add(&res[i][j], &T::mul(&self[i][k], &rhs[k][j]));\n\
    \                }\n            }\n        }\n        res\n    }\n}\n\nimpl<T:\
    \ Semiring> MulAssign for Matrix<T> {\n    fn mul_assign(&mut self, rhs: Matrix<T>)\
    \ {\n        *self *= &rhs;\n    }\n}\n\nimpl<T: Semiring> MulAssign<&Matrix<T>>\
    \ for Matrix<T> {\n    fn mul_assign(&mut self, rhs: &Matrix<T>) {\n        *self\
    \ = &*self * rhs;\n    }\n}\n"
  dependsOn:
  - crates/linear_algebra/matrix/src/index.rs
  - crates/linear_algebra/matrix/src/lib.rs
  - crates/linear_algebra/matrix/src/row_reduction.rs
  isVerificationFile: false
  path: crates/linear_algebra/matrix/src/ops.rs
  requiredBy:
  - crates/linear_algebra/matrix/src/lib.rs
  - crates/linear_algebra/matrix/src/row_reduction.rs
  - crates/linear_algebra/matrix/src/index.rs
  timestamp: '2025-03-08 00:53:12+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/linear_algebra/inverse_matrix/src/main.rs
  - verify/library_checker/linear_algebra/matrix_det/src/main.rs
  - verify/library_checker/linear_algebra/pow_of_matrix/src/main.rs
  - verify/library_checker/linear_algebra/matrix_product/src/main.rs
  - verify/library_checker/linear_algebra/matrix_rank/src/main.rs
documentation_of: crates/linear_algebra/matrix/src/ops.rs
layout: document
redirect_from:
- /library/crates/linear_algebra/matrix/src/ops.rs
- /library/crates/linear_algebra/matrix/src/ops.rs.html
title: crates/linear_algebra/matrix/src/ops.rs
---
