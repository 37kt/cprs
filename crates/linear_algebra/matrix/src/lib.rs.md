---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_traits/src/lib.rs
    title: crates/algebra/algebraic_traits/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_traits/src/macros.rs
    title: crates/algebra/algebraic_traits/src/macros.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_traits/src/traits.rs
    title: crates/algebra/algebraic_traits/src/traits.rs
  - icon: ':heavy_check_mark:'
    path: crates/linear_algebra/matrix/src/index.rs
    title: crates/linear_algebra/matrix/src/index.rs
  - icon: ':heavy_check_mark:'
    path: crates/linear_algebra/matrix/src/ops.rs
    title: crates/linear_algebra/matrix/src/ops.rs
  - icon: ':heavy_check_mark:'
    path: crates/linear_algebra/matrix/src/row_reduction.rs
    title: crates/linear_algebra/matrix/src/row_reduction.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/linear_algebra/matrix/src/index.rs
    title: crates/linear_algebra/matrix/src/index.rs
  - icon: ':heavy_check_mark:'
    path: crates/linear_algebra/matrix/src/ops.rs
    title: crates/linear_algebra/matrix/src/ops.rs
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
  code: "use std::fmt::Debug;\n\nuse algebraic_traits::Semiring;\n\npub mod index;\n\
    pub mod ops;\npub mod row_reduction;\n\npub struct Matrix<T: Semiring> {\n   \
    \ h: usize,\n    w: usize,\n    val: Vec<T::Value>,\n}\n\nimpl<T: Semiring> Matrix<T>\
    \ {\n    pub fn from_fn(h: usize, w: usize, mut f: impl FnMut(usize, usize) ->\
    \ T::Value) -> Self {\n        Self {\n            h,\n            w,\n      \
    \      val: (0..h * w).map(|i| f(i / w, i % w)).collect(),\n        }\n    }\n\
    \n    pub fn zeros(h: usize, w: usize) -> Self {\n        Self::from_fn(h, w,\
    \ |_, _| T::zero())\n    }\n\n    pub fn ones(h: usize, w: usize) -> Self {\n\
    \        Self::from_fn(h, w, |_, _| T::one())\n    }\n\n    pub fn identity(n:\
    \ usize) -> Self {\n        Self::from_fn(n, n, |i, j| if i == j { T::one() }\
    \ else { T::zero() })\n    }\n\n    pub fn h(&self) -> usize {\n        self.h\n\
    \    }\n\n    pub fn w(&self) -> usize {\n        self.w\n    }\n\n    pub fn\
    \ is_square(&self) -> bool {\n        self.h == self.w\n    }\n}\n\nimpl<T: Semiring>\
    \ Matrix<T>\nwhere\n    T::Value: Clone,\n{\n    pub fn transpose(&self) -> Matrix<T>\
    \ {\n        Matrix::from_fn(self.w, self.h, |i, j| self.val[j * self.w + i].clone())\n\
    \    }\n}\n\nimpl<T: Semiring> Clone for Matrix<T>\nwhere\n    T::Value: Clone,\n\
    {\n    fn clone(&self) -> Self {\n        Self {\n            h: self.h(),\n \
    \           w: self.w(),\n            val: self.val.clone(),\n        }\n    }\n\
    }\n\nimpl<T: Semiring> Debug for Matrix<T>\nwhere\n    T::Value: Debug,\n{\n \
    \   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n   \
    \     write!(f, \"[\")?;\n        for (i, row) in self.iter().enumerate() {\n\
    \            write!(f, \"{:?}\", row)?;\n            if i < self.h() - 1 {\n \
    \               write!(f, \", \")?;\n            }\n        }\n        write!(f,\
    \ \"]\")\n    }\n}\n"
  dependsOn:
  - crates/algebra/algebraic_traits/src/lib.rs
  - crates/algebra/algebraic_traits/src/macros.rs
  - crates/algebra/algebraic_traits/src/traits.rs
  - crates/linear_algebra/matrix/src/index.rs
  - crates/linear_algebra/matrix/src/ops.rs
  - crates/linear_algebra/matrix/src/row_reduction.rs
  isVerificationFile: false
  path: crates/linear_algebra/matrix/src/lib.rs
  requiredBy:
  - crates/linear_algebra/matrix/src/row_reduction.rs
  - crates/linear_algebra/matrix/src/index.rs
  - crates/linear_algebra/matrix/src/ops.rs
  timestamp: '2025-03-08 00:53:12+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/linear_algebra/pow_of_matrix/src/main.rs
  - verify/library_checker/linear_algebra/inverse_matrix/src/main.rs
  - verify/library_checker/linear_algebra/matrix_det/src/main.rs
  - verify/library_checker/linear_algebra/matrix_product/src/main.rs
  - verify/library_checker/linear_algebra/matrix_rank/src/main.rs
documentation_of: crates/linear_algebra/matrix/src/lib.rs
layout: document
redirect_from:
- /library/crates/linear_algebra/matrix/src/lib.rs
- /library/crates/linear_algebra/matrix/src/lib.rs.html
title: crates/linear_algebra/matrix/src/lib.rs
---
