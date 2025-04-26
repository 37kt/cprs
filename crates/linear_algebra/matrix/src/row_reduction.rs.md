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
    path: crates/linear_algebra/matrix/src/ops.rs
    title: crates/linear_algebra/matrix/src/ops.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/linear_algebra/matrix/src/index.rs
    title: crates/linear_algebra/matrix/src/index.rs
  - icon: ':heavy_check_mark:'
    path: crates/linear_algebra/matrix/src/lib.rs
    title: crates/linear_algebra/matrix/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/linear_algebra/matrix/src/ops.rs
    title: crates/linear_algebra/matrix/src/ops.rs
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
  code: "use algebraic_traits::{Field, Invertive};\n\nuse crate::Matrix;\n\nimpl<T:\
    \ Field> Matrix<T>\nwhere\n    T::Value: PartialEq + Clone,\n    T::Additive:\
    \ Invertive,\n    T::Multiplicative: Invertive,\n{\n    pub fn swap_row(&mut self,\
    \ i: usize, j: usize) {\n        if i == j {\n            return;\n        }\n\
    \        let w = self.w();\n        for k in 0..w {\n            self.val.swap(i\
    \ * w + k, j * w + k);\n        }\n    }\n\n    /// (\u6383\u51FA\u3057\u5F8C\u306E\
    \u884C\u5217, rank, det)\n    pub fn row_reduction(&self) -> (Self, usize, Option<T::Value>)\
    \ {\n        let mut a = self.clone();\n        let (rank, det) = a.row_reduction_inplace();\n\
    \        (a, rank, det)\n    }\n\n    /// (rank, det)\n    pub fn row_reduction_inplace(&mut\
    \ self) -> (usize, Option<T::Value>) {\n        if self.val.is_empty() {\n   \
    \         return (0, self.is_square().then_some(T::one()));\n        }\n\n   \
    \     let h = self.h();\n        let w = self.w();\n\n        let mut rank = 0;\n\
    \        let mut det = T::one();\n        for pivot in 0..w {\n            let\
    \ Some(i) = (rank..h).find(|&i| self[i][pivot] != T::zero()) else {\n        \
    \        det = T::zero();\n                continue;\n            };\n\n     \
    \       if i != rank {\n                self.swap_row(i, rank);\n            \
    \    det = T::neg(&det);\n            }\n            det = T::mul(&det, &self[rank][pivot]);\n\
    \n            let recip = T::recip(&self[rank][pivot]);\n            for j in\
    \ pivot..w {\n                self[rank][j] = T::mul(&self[rank][j], &recip);\n\
    \            }\n\n            for i in 0..h {\n                if i == rank ||\
    \ self[i][pivot] == T::zero() {\n                    continue;\n             \
    \   }\n                let c = T::div(&self[i][pivot], &self[rank][pivot]);\n\
    \                for j in pivot..w {\n                    self[i][j] = T::sub(&self[i][j],\
    \ &T::mul(&c, &self[rank][j]));\n                }\n            }\n\n        \
    \    rank += 1;\n        }\n\n        (rank, self.is_square().then_some(det))\n\
    \    }\n\n    pub fn rank(&self) -> usize {\n        let (_, rank, _) = self.row_reduction();\n\
    \        rank\n    }\n\n    pub fn det(&self) -> Option<T::Value> {\n        let\
    \ (_, _, det) = self.row_reduction();\n        det\n    }\n\n    pub fn inv(&self)\
    \ -> Option<Self> {\n        if !self.is_square() {\n            return None;\n\
    \        }\n        let n = self.h();\n        let mut a = Self::from_fn(n, n\
    \ * 2, |i, j| {\n            if j < n {\n                self[i][j].clone()\n\
    \            } else if j == i + n {\n                T::one()\n            } else\
    \ {\n                T::zero()\n            }\n        });\n        a.row_reduction_inplace();\n\
    \        if a[n - 1][n - 1] != T::one() {\n            return None;\n        }\n\
    \        let inv = Self::from_fn(n, n, |i, j| a[i][j + n].clone());\n        Some(inv)\n\
    \    }\n}\n"
  dependsOn:
  - crates/linear_algebra/matrix/src/index.rs
  - crates/linear_algebra/matrix/src/lib.rs
  - crates/linear_algebra/matrix/src/ops.rs
  isVerificationFile: false
  path: crates/linear_algebra/matrix/src/row_reduction.rs
  requiredBy:
  - crates/linear_algebra/matrix/src/lib.rs
  - crates/linear_algebra/matrix/src/ops.rs
  - crates/linear_algebra/matrix/src/index.rs
  timestamp: '2025-03-08 00:53:12+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/linear_algebra/inverse_matrix/src/main.rs
  - verify/library_checker/linear_algebra/matrix_product/src/main.rs
  - verify/library_checker/linear_algebra/matrix_rank/src/main.rs
  - verify/library_checker/linear_algebra/pow_of_matrix/src/main.rs
  - verify/library_checker/linear_algebra/matrix_det/src/main.rs
documentation_of: crates/linear_algebra/matrix/src/row_reduction.rs
layout: document
redirect_from:
- /library/crates/linear_algebra/matrix/src/row_reduction.rs
- /library/crates/linear_algebra/matrix/src/row_reduction.rs.html
title: crates/linear_algebra/matrix/src/row_reduction.rs
---
