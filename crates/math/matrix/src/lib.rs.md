---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/inverse_matrix/src/main.rs
    title: verify/inverse_matrix/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/matrix_det/src/main.rs
    title: verify/matrix_det/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/matrix_product/src/main.rs
    title: verify/matrix_product/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.5/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.5/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{\n    fmt::{Debug, Display},\n    ops::{Add, AddAssign, Div, DivAssign,\
    \ Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},\n};\n\n#[derive(Clone)]\n\
    pub struct Matrix<T>\nwhere\n    T: From<i64> + Clone,\n{\n    n: usize,\n   \
    \ m: usize,\n    v: Box<[T]>,\n}\n\nimpl<T> From<Vec<Vec<T>>> for Matrix<T>\n\
    where\n    T: From<i64> + Clone,\n{\n    fn from(v: Vec<Vec<T>>) -> Self {\n \
    \       let n = v.len();\n        let m = v[0].len();\n        assert!(v.iter().all(|x|\
    \ x.len() == m));\n        Self {\n            n,\n            m,\n          \
    \  v: v.into_iter()\n                .flatten()\n                .collect::<Vec<_>>()\n\
    \                .into_boxed_slice(),\n        }\n    }\n}\n\nimpl<T> Debug for\
    \ Matrix<T>\nwhere\n    T: From<i64> + Clone + Debug,\n{\n    fn fmt(&self, f:\
    \ &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n        for i in 0..self.n\
    \ {\n            for j in 0..self.m {\n                write!(\n             \
    \       f,\n                    \"{:?}{}\",\n                    self[i][j],\n\
    \                    if j + 1 == self.m {\n                        if i + 1 ==\
    \ self.n {\n                            \"\"\n                        } else {\n\
    \                            \"\\n\"\n                        }\n            \
    \        } else {\n                        \" \"\n                    }\n    \
    \            )?\n            }\n        }\n        Ok(())\n    }\n}\n\nimpl<T>\
    \ Display for Matrix<T>\nwhere\n    T: From<i64> + Clone + Display,\n{\n    fn\
    \ fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n        for\
    \ i in 0..self.n {\n            for j in 0..self.m {\n                write!(\n\
    \                    f,\n                    \"{}{}\",\n                    self[i][j],\n\
    \                    if j + 1 == self.m {\n                        if i + 1 ==\
    \ self.n {\n                            \"\"\n                        } else {\n\
    \                            \"\\n\"\n                        }\n            \
    \        } else {\n                        \" \"\n                    }\n    \
    \            )?\n            }\n        }\n        Ok(())\n    }\n}\n\nimpl<T>\
    \ Index<usize> for Matrix<T>\nwhere\n    T: From<i64> + Clone,\n{\n    type Output\
    \ = [T];\n    fn index(&self, index: usize) -> &Self::Output {\n        &self.v[index\
    \ * self.m..(index + 1) * self.m]\n    }\n}\n\nimpl<T> IndexMut<usize> for Matrix<T>\n\
    where\n    T: From<i64> + Clone,\n{\n    fn index_mut(&mut self, index: usize)\
    \ -> &mut Self::Output {\n        &mut self.v[index * self.m..(index + 1) * self.m]\n\
    \    }\n}\n\nimpl<T> Matrix<T>\nwhere\n    T: From<i64> + Clone,\n{\n    pub fn\
    \ zeros(n: usize, m: usize) -> Self {\n        Self {\n            n,\n      \
    \      m,\n            v: vec![0.into(); n * m].into_boxed_slice(),\n        }\n\
    \    }\n\n    pub fn e(n: usize) -> Self {\n        let mut a = Self::zeros(n,\
    \ n);\n        for i in 0..n {\n            a[i][i] = 1.into();\n        }\n \
    \       a\n    }\n\n    pub fn n(&self) -> usize {\n        self.n\n    }\n\n\
    \    pub fn m(&self) -> usize {\n        self.m\n    }\n\n    pub fn transpose(&self)\
    \ -> Self {\n        let mut a = Self::zeros(self.m, self.n);\n        for i in\
    \ 0..self.n {\n            for j in 0..self.m {\n                a[j][i] = self[i][j].clone();\n\
    \            }\n        }\n        a\n    }\n}\n\nimpl<T> Matrix<T>\nwhere\n \
    \   T: From<i64> + Clone + Add<Output = T> + Mul<Output = T>,\n{\n    pub fn pow(&self,\
    \ mut k: usize) -> Self {\n        assert!(self.n == self.m);\n        let mut\
    \ a = self.clone();\n        let mut b = Self::e(self.n);\n        while k > 0\
    \ {\n            if k & 1 != 0 {\n                b *= &a;\n            }\n  \
    \          a *= &a.clone();\n            k >>= 1;\n        }\n        b\n    }\n\
    }\n\nimpl<T> Matrix<T>\nwhere\n    T: From<i64>\n        + Clone\n        + Eq\n\
    \        + Neg<Output = T>\n        + Add<Output = T>\n        + Sub<Output =\
    \ T>\n        + Mul<Output = T>\n        + Div<Output = T>,\n{\n    // (\u6383\
    \u51FA\u3057\u5F8C\u306E\u884C\u5217, rank, det(\u6B63\u65B9\u884C\u5217\u306E\
    \u5834\u5408))\n    pub fn gauss_elimination(&self) -> (Self, usize, Option<T>)\
    \ {\n        let mut a = self.clone();\n        let mut rank = 0;\n        let\
    \ mut det: T = 1.into();\n        let one = 1.into();\n        let zero = 0.into();\n\
    \        for j in 0..self.m {\n            if let Some(i) = (rank..self.n).position(|i|\
    \ a[i][j] != zero) {\n                let i = i + rank;\n                if rank\
    \ < i {\n                    det = -det;\n                    let (x, y) = a.v.split_at_mut(i\
    \ * self.m);\n                    x[rank * self.m..(rank + 1) * self.m].swap_with_slice(&mut\
    \ y[0..self.m]);\n                }\n                det = det * a[rank][j].clone();\n\
    \                if a[rank][j] != one {\n                    let coef = one.clone()\
    \ / a[rank][j].clone();\n                    for k in j..self.m {\n          \
    \              a[rank][k] = a[rank][k].clone() * coef.clone();\n             \
    \       }\n                }\n                for i in 0..self.n {\n         \
    \           if i == rank {\n                        continue;\n              \
    \      }\n                    if a[i][j] != zero {\n                        let\
    \ coef = a[i][j].clone() / a[rank][j].clone();\n                        for k\
    \ in j..self.m {\n                            a[i][k] = a[i][k].clone() - a[rank][k].clone()\
    \ * coef.clone();\n                        }\n                    }\n        \
    \        }\n                rank += 1;\n            } else {\n               \
    \ det = zero.clone();\n            }\n        }\n        (a, rank, Some(det))\n\
    \    }\n\n    pub fn inv(&self) -> Option<Self> {\n        assert!(self.n == self.m);\n\
    \        let one: T = 1.into();\n        let mut a = Self::zeros(self.n, self.n\
    \ * 2);\n        for i in 0..self.n {\n            for j in 0..self.n {\n    \
    \            a[i][j] = self[i][j].clone();\n            }\n            a[i][self.n\
    \ + i] = one.clone();\n        }\n        let (b, _, _) = a.gauss_elimination();\n\
    \        if b[self.n - 1][self.n - 1] != one {\n            return None;\n   \
    \     }\n        let mut c = Self::zeros(self.n, self.n);\n        for i in 0..self.n\
    \ {\n            for j in 0..self.n {\n                c[i][j] = b[i][self.n +\
    \ j].clone();\n            }\n        }\n        Some(c)\n    }\n}\n\nimpl<T>\
    \ Neg for Matrix<T>\nwhere\n    T: From<i64> + Clone + Neg<Output = T>,\n{\n \
    \   type Output = Self;\n    fn neg(mut self) -> Self::Output {\n        for x\
    \ in self.v.as_mut() {\n            *x = -x.clone();\n        }\n        self\n\
    \    }\n}\n\nimpl<T> Neg for &Matrix<T>\nwhere\n    T: From<i64> + Clone + Neg<Output\
    \ = T>,\n{\n    type Output = Matrix<T>;\n    fn neg(self) -> Self::Output {\n\
    \        -self.clone()\n    }\n}\n\nimpl<T> AddAssign<&Self> for Matrix<T>\nwhere\n\
    \    T: From<i64> + Clone + Add<Output = T>,\n{\n    fn add_assign(&mut self,\
    \ rhs: &Self) {\n        assert!((self.n, self.m) == (rhs.n, rhs.m));\n      \
    \  for (x, y) in self.v.as_mut().iter_mut().zip(rhs.v.as_ref()) {\n          \
    \  *x = x.clone() + y.clone();\n        }\n    }\n}\n\nimpl<T> SubAssign<&Self>\
    \ for Matrix<T>\nwhere\n    T: From<i64> + Clone + Sub<Output = T>,\n{\n    fn\
    \ sub_assign(&mut self, rhs: &Self) {\n        assert!((self.n, self.m) == (rhs.n,\
    \ rhs.m));\n        for (x, y) in self.v.as_mut().iter_mut().zip(rhs.v.as_ref())\
    \ {\n            *x = x.clone() - y.clone();\n        }\n    }\n}\n\nimpl<T> MulAssign<&Self>\
    \ for Matrix<T>\nwhere\n    T: From<i64> + Clone + Add<Output = T> + Mul<Output\
    \ = T>,\n{\n    fn mul_assign(&mut self, rhs: &Self) {\n        assert!(self.m\
    \ == rhs.n);\n        let mut a = Self::zeros(self.n, rhs.m);\n        for i in\
    \ 0..self.n {\n            for j in 0..rhs.m {\n                for k in 0..self.m\
    \ {\n                    a[i][j] = a[i][j].clone() + self[i][k].clone() * rhs[k][j].clone();\n\
    \                }\n            }\n        }\n        *self = a;\n    }\n}\n\n\
    impl<T> MulAssign<T> for Matrix<T>\nwhere\n    T: From<i64> + Clone + Mul<Output\
    \ = T>,\n{\n    fn mul_assign(&mut self, rhs: T) {\n        for x in self.v.as_mut()\
    \ {\n            *x = x.clone() * rhs.clone();\n        }\n    }\n}\n\nimpl<T>\
    \ DivAssign<T> for Matrix<T>\nwhere\n    T: From<i64> + Clone + Mul<Output = T>\
    \ + Div<Output = T>,\n{\n    fn div_assign(&mut self, rhs: T) {\n        *self\
    \ *= T::from(1) / rhs;\n    }\n}\n\nimpl<T> Add<Self> for &Matrix<T>\nwhere\n\
    \    T: From<i64> + Clone + Add<Output = T>,\n{\n    type Output = Matrix<T>;\n\
    \    fn add(self, rhs: Self) -> Self::Output {\n        let mut a = self.clone();\n\
    \        a += rhs;\n        a\n    }\n}\n\nimpl<T> Sub<Self> for &Matrix<T>\n\
    where\n    T: From<i64> + Clone + Sub<Output = T>,\n{\n    type Output = Matrix<T>;\n\
    \    fn sub(self, rhs: Self) -> Self::Output {\n        let mut a = self.clone();\n\
    \        a -= rhs;\n        a\n    }\n}\n\nimpl<T> Mul<Self> for &Matrix<T>\n\
    where\n    T: From<i64> + Clone + Add<Output = T> + Mul<Output = T>,\n{\n    type\
    \ Output = Matrix<T>;\n    fn mul(self, rhs: Self) -> Self::Output {\n       \
    \ let mut a = self.clone();\n        a *= rhs;\n        a\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/math/matrix/src/lib.rs
  requiredBy: []
  timestamp: '2023-06-13 17:39:04+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/inverse_matrix/src/main.rs
  - verify/matrix_product/src/main.rs
  - verify/matrix_det/src/main.rs
documentation_of: crates/math/matrix/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/matrix/src/lib.rs
- /library/crates/math/matrix/src/lib.rs.html
title: crates/math/matrix/src/lib.rs
---
