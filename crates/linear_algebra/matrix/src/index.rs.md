---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/linear_algebra/matrix/src/lib.rs
    title: crates/linear_algebra/matrix/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/linear_algebra/matrix/src/ops.rs
    title: crates/linear_algebra/matrix/src/ops.rs
  - icon: ':heavy_check_mark:'
    path: crates/linear_algebra/matrix/src/row_reduction.rs
    title: crates/linear_algebra/matrix/src/row_reduction.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/linear_algebra/matrix/src/lib.rs
    title: crates/linear_algebra/matrix/src/lib.rs
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
  code: "use std::{\n    iter::FusedIterator,\n    ops::{Index, IndexMut},\n};\n\n\
    use algebraic_traits::Semiring;\n\nuse crate::Matrix;\n\npub struct Iter<'a, T:\
    \ Semiring> {\n    w: usize,\n    val: &'a [T::Value],\n}\n\nimpl<T: Semiring>\
    \ Matrix<T> {\n    pub fn iter(&self) -> Iter<'_, T> {\n        Iter {\n     \
    \       w: self.w(),\n            val: &self.val,\n        }\n    }\n}\n\nimpl<T:\
    \ Semiring> Index<usize> for Matrix<T> {\n    type Output = [T::Value];\n\n  \
    \  fn index(&self, index: usize) -> &Self::Output {\n        &self.val[index *\
    \ self.w()..(index + 1) * self.w()]\n    }\n}\n\nimpl<T: Semiring> IndexMut<usize>\
    \ for Matrix<T> {\n    fn index_mut(&mut self, index: usize) -> &mut Self::Output\
    \ {\n        let w = self.w();\n        &mut self.val[index * w..(index + 1) *\
    \ w]\n    }\n}\n\nimpl<'a, T: Semiring> Iterator for Iter<'a, T> {\n    type Item\
    \ = &'a [T::Value];\n\n    fn next(&mut self) -> Option<Self::Item> {\n      \
    \  if self.val.is_empty() {\n            return None;\n        }\n        let\
    \ (head, tail) = self.val.split_at(self.w);\n        self.val = tail;\n      \
    \  Some(head)\n    }\n}\n\nimpl<'a, T: Semiring> DoubleEndedIterator for Iter<'a,\
    \ T> {\n    fn next_back(&mut self) -> Option<Self::Item> {\n        if self.val.is_empty()\
    \ {\n            return None;\n        }\n        let (head, tail) = self.val.split_at(self.val.len()\
    \ - self.w);\n        self.val = head;\n        Some(tail)\n    }\n}\n\nimpl<'a,\
    \ T: Semiring> ExactSizeIterator for Iter<'a, T> {\n    fn len(&self) -> usize\
    \ {\n        self.val.len() / self.w\n    }\n}\n\nimpl<'a, T: Semiring> FusedIterator\
    \ for Iter<'a, T> {}\n\nimpl<'a, T: Semiring> IntoIterator for &'a Matrix<T> {\n\
    \    type Item = &'a [T::Value];\n    type IntoIter = Iter<'a, T>;\n\n    fn into_iter(self)\
    \ -> Self::IntoIter {\n        self.iter()\n    }\n}\n"
  dependsOn:
  - crates/linear_algebra/matrix/src/lib.rs
  - crates/linear_algebra/matrix/src/ops.rs
  - crates/linear_algebra/matrix/src/row_reduction.rs
  isVerificationFile: false
  path: crates/linear_algebra/matrix/src/index.rs
  requiredBy:
  - crates/linear_algebra/matrix/src/lib.rs
  - crates/linear_algebra/matrix/src/row_reduction.rs
  - crates/linear_algebra/matrix/src/ops.rs
  timestamp: '2025-03-08 00:53:12+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/linear_algebra/matrix_rank/src/main.rs
  - verify/library_checker/linear_algebra/matrix_product/src/main.rs
  - verify/library_checker/linear_algebra/inverse_matrix/src/main.rs
  - verify/library_checker/linear_algebra/matrix_det/src/main.rs
  - verify/library_checker/linear_algebra/pow_of_matrix/src/main.rs
documentation_of: crates/linear_algebra/matrix/src/index.rs
layout: document
redirect_from:
- /library/crates/linear_algebra/matrix/src/index.rs
- /library/crates/linear_algebra/matrix/src/index.rs.html
title: crates/linear_algebra/matrix/src/index.rs
---
