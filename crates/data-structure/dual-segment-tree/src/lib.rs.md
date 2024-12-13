---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/dual-range-tree/src/lib.rs
    title: crates/data-structure/dual-range-tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/vertex-get-range-contour-add/src/lib.rs
    title: crates/data-structure/vertex-get-range-contour-add/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/range_affine_point_get/src/main.rs
    title: verify/range_affine_point_get/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ops::{Bound, RangeBounds};\n\nuse algebraic::Monoid;\n\n#[derive(Clone)]\n\
    pub struct DualSegmentTree<M>\nwhere\n    M: Monoid,\n    M::S: Clone,\n{\n  \
    \  n: usize,\n    v: Vec<M::S>,\n}\n\nimpl<M> DualSegmentTree<M>\nwhere\n    M:\
    \ Monoid,\n    M::S: Clone,\n{\n    pub fn new(n: usize) -> Self {\n        Self\
    \ {\n            n,\n            v: vec![M::e(); n * 2],\n        }\n    }\n\n\
    \    pub fn apply_range(&mut self, range: impl RangeBounds<usize>, f: M::S) {\n\
    \        let mut l = match range.start_bound() {\n            Bound::Excluded(&l)\
    \ => l + 1,\n            Bound::Included(&l) => l,\n            Bound::Unbounded\
    \ => 0,\n        };\n        let mut r = match range.end_bound() {\n         \
    \   Bound::Excluded(&r) => r,\n            Bound::Included(&r) => r + 1,\n   \
    \         Bound::Unbounded => self.n,\n        };\n        assert!(l <= r);\n\
    \        assert!(r <= self.n);\n        l += self.n;\n        r += self.n;\n \
    \       self.propagate(l);\n        self.propagate(r);\n        while l < r {\n\
    \            if l & 1 != 0 {\n                self.v[l] = M::op(&f, &self.v[l]);\n\
    \                l += 1;\n            }\n            if r & 1 != 0 {\n       \
    \         r -= 1;\n                self.v[r] = M::op(&f, &self.v[r]);\n      \
    \      }\n            l >>= 1;\n            r >>= 1;\n        }\n    }\n\n   \
    \ pub fn apply(&mut self, k: usize, f: M::S) {\n        assert!(k < self.n);\n\
    \        self.apply_range(k..=k, f);\n    }\n\n    pub fn get(&self, mut k: usize)\
    \ -> M::S {\n        assert!(k < self.n);\n        k += self.n;\n        let mut\
    \ res = self.v[k].clone();\n        while k > 1 {\n            k >>= 1;\n    \
    \        res = M::op(&self.v[k], &res);\n        }\n        res\n    }\n\n   \
    \ fn push(&mut self, i: usize) {\n        self.v[i * 2] = M::op(&self.v[i], &self.v[i\
    \ * 2]);\n        self.v[i * 2 + 1] = M::op(&self.v[i], &self.v[i * 2 + 1]);\n\
    \        self.v[i] = M::e();\n    }\n\n    fn propagate(&mut self, i: usize) {\n\
    \        if i == 0 {\n            return;\n        }\n        let crz = i.trailing_zeros()\
    \ as usize;\n        for h in (crz + 1..64 - i.leading_zeros() as usize).rev()\
    \ {\n            self.push(i >> h);\n        }\n    }\n}\n\nimpl<M> From<Vec<M::S>>\
    \ for DualSegmentTree<M>\nwhere\n    M: Monoid,\n    M::S: Clone,\n{\n    fn from(mut\
    \ a: Vec<M::S>) -> Self {\n        let n = a.len();\n        let mut v = vec![M::e();\
    \ n];\n        v.append(&mut a);\n        for i in (1..n).rev() {\n          \
    \  v[i] = M::op(&v[i * 2], &v[i * 2 + 1]);\n        }\n        Self { n, v }\n\
    \    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  isVerificationFile: false
  path: crates/data-structure/dual-segment-tree/src/lib.rs
  requiredBy:
  - crates/data-structure/vertex-get-range-contour-add/src/lib.rs
  - crates/data-structure/dual-range-tree/src/lib.rs
  timestamp: '2024-06-28 10:31:31+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/range_affine_point_get/src/main.rs
documentation_of: crates/data-structure/dual-segment-tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/dual-segment-tree/src/lib.rs
- /library/crates/data-structure/dual-segment-tree/src/lib.rs.html
title: crates/data-structure/dual-segment-tree/src/lib.rs
---
