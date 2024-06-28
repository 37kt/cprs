---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/matrix_det_mod_2/src/main.rs
    title: verify/matrix_det_mod_2/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/matrix_product_mod_2/src/main.rs
    title: verify/matrix_product_mod_2/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.4/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.4/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{\n    fmt::{Debug, Display},\n    ops::{BitAnd, BitAndAssign, BitOr,\
    \ BitOrAssign, BitXor, BitXorAssign, Mul, MulAssign},\n};\n\n#[derive(Clone)]\n\
    struct BitSet {\n    n: usize,\n    v: Vec<u64>,\n}\n\n#[derive(Clone)]\npub struct\
    \ MatrixMod2 {\n    n: usize,\n    m: usize,\n    v: Vec<BitSet>,\n}\n\nimpl BitSet\
    \ {\n    fn new(n: usize) -> Self {\n        let m = (n + 63) / 64;\n        BitSet\
    \ { n, v: vec![0; m] }\n    }\n\n    fn set(&mut self, i: usize, f: bool) {\n\
    \        assert!(i < self.n);\n        let (a, b) = (i / 64, i % 64);\n      \
    \  if f {\n            self.v[a] |= 1 << b;\n        } else {\n            self.v[a]\
    \ &= !(1 << b);\n        }\n    }\n\n    fn get(&self, i: usize) -> bool {\n \
    \       assert!(i < self.n);\n        let (a, b) = (i / 64, i % 64);\n       \
    \ (self.v[a] >> b) & 1 == 1\n    }\n}\n\nimpl PartialEq for BitSet {\n    fn eq(&self,\
    \ other: &Self) -> bool {\n        assert_eq!(self.n, other.n);\n        self.v\
    \ == other.v\n    }\n}\n\nimpl Eq for BitSet {}\n\nimpl BitOrAssign<&BitSet> for\
    \ BitSet {\n    fn bitor_assign(&mut self, other: &Self) {\n        assert_eq!(self.n,\
    \ other.n);\n        for (a, &b) in self.v.iter_mut().zip(&other.v) {\n      \
    \      *a |= b;\n        }\n    }\n}\n\nimpl BitAndAssign<&BitSet> for BitSet\
    \ {\n    fn bitand_assign(&mut self, other: &Self) {\n        assert_eq!(self.n,\
    \ other.n);\n        for (a, &b) in self.v.iter_mut().zip(&other.v) {\n      \
    \      *a &= b;\n        }\n    }\n}\n\nimpl BitXorAssign<&BitSet> for BitSet\
    \ {\n    fn bitxor_assign(&mut self, other: &Self) {\n        assert_eq!(self.n,\
    \ other.n);\n        for (a, &b) in self.v.iter_mut().zip(&other.v) {\n      \
    \      *a ^= b;\n        }\n    }\n}\n\nimpl BitOr<Self> for &BitSet {\n    type\
    \ Output = BitSet;\n    fn bitor(self, other: Self) -> BitSet {\n        let mut\
    \ res = self.clone();\n        res |= other;\n        res\n    }\n}\n\nimpl BitAnd<Self>\
    \ for &BitSet {\n    type Output = BitSet;\n    fn bitand(self, other: Self) ->\
    \ BitSet {\n        let mut res = self.clone();\n        res &= other;\n     \
    \   res\n    }\n}\n\nimpl BitXor<Self> for &BitSet {\n    type Output = BitSet;\n\
    \    fn bitxor(self, other: Self) -> BitSet {\n        let mut res = self.clone();\n\
    \        res ^= other;\n        res\n    }\n}\n\nimpl Display for BitSet {\n \
    \   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n   \
    \     for i in 0..self.n {\n            write!(f, \"{}\", if self.get(i) { 1 }\
    \ else { 0 })?;\n        }\n        Ok(())\n    }\n}\n\nimpl Debug for BitSet\
    \ {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n\
    \        for i in 0..self.n {\n            write!(f, \"{}\", if self.get(i) {\
    \ 1 } else { 0 })?;\n        }\n        Ok(())\n    }\n}\n\nimpl MatrixMod2 {\n\
    \    pub fn new(n: usize, m: usize) -> Self {\n        let v = vec![BitSet::new(m);\
    \ n];\n        MatrixMod2 { n, m, v }\n    }\n\n    pub fn set(&mut self, i: usize,\
    \ j: usize, f: bool) {\n        assert!(i < self.n && j < self.m);\n        self.v[i].set(j,\
    \ f);\n    }\n\n    pub fn get(&self, i: usize, j: usize) -> bool {\n        assert!(i\
    \ < self.n && j < self.m);\n        self.v[i].get(j)\n    }\n\n    // (\u6383\u51FA\
    \u3057\u5F8C\u306E\u884C\u5217, rank)\n    pub fn gauss_elimination(&self) ->\
    \ (Self, usize) {\n        let mut a = self.clone();\n        let mut rank = 0;\n\
    \        for j in 0..self.m {\n            let mut pivot = None;\n           \
    \ for i in rank..self.n {\n                if a.get(i, j) {\n                \
    \    pivot = Some(i);\n                    break;\n                }\n       \
    \     }\n            if let Some(pivot) = pivot {\n                a.v.swap(rank,\
    \ pivot);\n                for i in 0..self.n {\n                    if i != rank\
    \ && a.get(i, j) {\n                        let t = a.v[rank].clone();\n     \
    \                   a.v[i] ^= &t;\n                    }\n                }\n\
    \                rank += 1;\n            }\n        }\n        (a, rank)\n   \
    \ }\n\n    pub fn det(&self) -> bool {\n        assert_eq!(self.n, self.m);\n\
    \        let (_, rank) = self.gauss_elimination();\n        rank == self.n\n \
    \   }\n}\n\nimpl PartialEq for MatrixMod2 {\n    fn eq(&self, other: &Self) ->\
    \ bool {\n        assert_eq!(self.n, other.n);\n        assert_eq!(self.m, other.m);\n\
    \        self.v == other.v\n    }\n}\n\nimpl Eq for MatrixMod2 {}\n\nimpl BitOrAssign<&MatrixMod2>\
    \ for MatrixMod2 {\n    fn bitor_assign(&mut self, other: &Self) {\n        assert_eq!(self.n,\
    \ other.n);\n        assert_eq!(self.m, other.m);\n        for (a, b) in self.v.iter_mut().zip(&other.v)\
    \ {\n            *a |= b;\n        }\n    }\n}\n\nimpl BitAndAssign<&MatrixMod2>\
    \ for MatrixMod2 {\n    fn bitand_assign(&mut self, other: &Self) {\n        assert_eq!(self.n,\
    \ other.n);\n        assert_eq!(self.m, other.m);\n        for (a, b) in self.v.iter_mut().zip(&other.v)\
    \ {\n            *a &= b;\n        }\n    }\n}\n\nimpl BitXorAssign<&MatrixMod2>\
    \ for MatrixMod2 {\n    fn bitxor_assign(&mut self, other: &Self) {\n        assert_eq!(self.n,\
    \ other.n);\n        assert_eq!(self.m, other.m);\n        for (a, b) in self.v.iter_mut().zip(&other.v)\
    \ {\n            *a ^= b;\n        }\n    }\n}\n\nimpl BitOr<&MatrixMod2> for\
    \ &MatrixMod2 {\n    type Output = MatrixMod2;\n    fn bitor(self, other: &MatrixMod2)\
    \ -> MatrixMod2 {\n        assert_eq!(self.n, other.n);\n        assert_eq!(self.m,\
    \ other.m);\n        let mut c = MatrixMod2::new(self.n, self.m);\n        for\
    \ i in 0..self.n {\n            c.v[i] = &self.v[i] | &other.v[i];\n        }\n\
    \        c\n    }\n}\n\nimpl BitAnd<&MatrixMod2> for &MatrixMod2 {\n    type Output\
    \ = MatrixMod2;\n    fn bitand(self, other: &MatrixMod2) -> MatrixMod2 {\n   \
    \     assert_eq!(self.n, other.n);\n        assert_eq!(self.m, other.m);\n   \
    \     let mut c = MatrixMod2::new(self.n, self.m);\n        for i in 0..self.n\
    \ {\n            c.v[i] = &self.v[i] & &other.v[i];\n        }\n        c\n  \
    \  }\n}\n\nimpl BitXor<&MatrixMod2> for &MatrixMod2 {\n    type Output = MatrixMod2;\n\
    \    fn bitxor(self, other: &MatrixMod2) -> MatrixMod2 {\n        assert_eq!(self.n,\
    \ other.n);\n        assert_eq!(self.m, other.m);\n        let mut c = MatrixMod2::new(self.n,\
    \ self.m);\n        for i in 0..self.n {\n            c.v[i] = &self.v[i] ^ &other.v[i];\n\
    \        }\n        c\n    }\n}\n\nimpl Mul<&MatrixMod2> for &MatrixMod2 {\n \
    \   type Output = MatrixMod2;\n    fn mul(self, rhs: &MatrixMod2) -> MatrixMod2\
    \ {\n        assert_eq!(self.m, rhs.n);\n        let mut c = MatrixMod2::new(self.n,\
    \ rhs.m);\n        for i in 0..self.n {\n            for j in 0..self.m {\n  \
    \              if self.get(i, j) {\n                    c.v[i] ^= &rhs.v[j];\n\
    \                }\n            }\n        }\n        c\n    }\n}\n\nimpl MulAssign<&MatrixMod2>\
    \ for MatrixMod2 {\n    fn mul_assign(&mut self, rhs: &MatrixMod2) {\n       \
    \ *self = &*self * rhs;\n    }\n}\n\nimpl Display for MatrixMod2 {\n    fn fmt(&self,\
    \ f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n        for i in 0..self.n\
    \ {\n            for j in 0..self.m {\n                write!(f, \"{}\", if self.get(i,\
    \ j) { 1 } else { 0 })?;\n            }\n            writeln!(f)?;\n        }\n\
    \        Ok(())\n    }\n}\n\nimpl Debug for MatrixMod2 {\n    fn fmt(&self, f:\
    \ &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n        for i in 0..self.n\
    \ {\n            for j in 0..self.m {\n                write!(f, \"{}\", if self.get(i,\
    \ j) { 1 } else { 0 })?;\n            }\n            writeln!(f)?;\n        }\n\
    \        Ok(())\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/math/matrix-mod2/src/lib.rs
  requiredBy: []
  timestamp: '2024-03-21 13:40:37+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/matrix_product_mod_2/src/main.rs
  - verify/matrix_det_mod_2/src/main.rs
documentation_of: crates/math/matrix-mod2/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/matrix-mod2/src/lib.rs
- /library/crates/math/matrix-mod2/src/lib.rs.html
title: crates/math/matrix-mod2/src/lib.rs
---
