---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/range-tree/src/lib.rs
    title: crates/data-structure/range-tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/segment-tree-on-wavelet-matrix/src/lib.rs
    title: crates/data-structure/segment-tree-on-wavelet-matrix/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/tree-query/src/lib.rs
    title: crates/data-structure/tree-query/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/vertex-add-range-contour-sum/src/lib.rs
    title: crates/data-structure/vertex-add-range-contour-sum/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/point_add_range_sum/src/main.rs
    title: verify/point_add_range_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/vertex_add_path_sum/src/main.rs
    title: verify/vertex_add_path_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/vertex_add_subtree_sum/src/main.rs
    title: verify/vertex_add_subtree_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/vertex_set_path_composite/src/main.rs
    title: verify/vertex_set_path_composite/src/main.rs
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
    pub struct SegmentTree<M>\nwhere\n    M: Monoid,\n    M::S: Clone,\n{\n    n:\
    \ usize,\n    v: Vec<M::S>,\n}\n\nimpl<M> SegmentTree<M>\nwhere\n    M: Monoid,\n\
    \    M::S: Clone,\n{\n    pub fn new(n: usize) -> Self {\n        Self {\n   \
    \         n,\n            v: vec![M::e(); n * 2],\n        }\n    }\n\n    pub\
    \ fn set(&mut self, mut k: usize, x: M::S) {\n        k += self.n;\n        self.v[k]\
    \ = x;\n        while k > 1 {\n            k >>= 1;\n            self.v[k] = M::op(&self.v[k\
    \ * 2], &self.v[k * 2 + 1]);\n        }\n    }\n\n    pub fn get(&self, k: usize)\
    \ -> M::S {\n        assert!(k < self.n);\n        self.v[k + self.n].clone()\n\
    \    }\n\n    pub fn prod<R>(&self, range: R) -> M::S\n    where\n        R: RangeBounds<usize>,\n\
    \    {\n        let mut l = match range.start_bound() {\n            Bound::Excluded(&l)\
    \ => l + 1,\n            Bound::Included(&l) => l,\n            Bound::Unbounded\
    \ => 0,\n        };\n        let mut r = match range.end_bound() {\n         \
    \   Bound::Excluded(&r) => r,\n            Bound::Included(&r) => r + 1,\n   \
    \         Bound::Unbounded => self.n,\n        };\n        assert!(l <= r);\n\
    \        assert!(r <= self.n);\n        l += self.n;\n        r += self.n;\n \
    \       let mut sl = M::e();\n        let mut sr = M::e();\n        while l <\
    \ r {\n            if l & 1 != 0 {\n                sl = M::op(&sl, &self.v[l]);\n\
    \                l += 1;\n            }\n            if r & 1 != 0 {\n       \
    \         r -= 1;\n                sr = M::op(&self.v[r], &sr);\n            }\n\
    \            l >>= 1;\n            r >>= 1;\n        }\n        M::op(&sl, &sr)\n\
    \    }\n\n    pub fn max_right<P>(&self, mut l: usize, pred: P) -> usize\n   \
    \ where\n        P: Fn(&M::S) -> bool,\n    {\n        assert!(l <= self.n);\n\
    \        assert!(pred(&M::e()));\n        if pred(&self.prod(l..)) {\n       \
    \     return self.n;\n        }\n        l += self.n;\n        let mut s = M::e();\n\
    \        loop {\n            while l & 1 == 0 && self.is_good_node(l >> 1) {\n\
    \                l >>= 1;\n            }\n            if !pred(&M::op(&s, &self.v[l]))\
    \ {\n                while l < self.n {\n                    l <<= 1;\n      \
    \              let t = M::op(&s, &self.v[l]);\n                    if pred(&t)\
    \ {\n                        s = t;\n                        l += 1;\n       \
    \             }\n                }\n                return l - self.n;\n     \
    \       }\n            s = M::op(&s, &self.v[l]);\n            l += 1;\n     \
    \   }\n    }\n\n    pub fn min_left<P>(&self, mut r: usize, pred: P) -> usize\n\
    \    where\n        P: Fn(&M::S) -> bool,\n    {\n        assert!(r <= self.n);\n\
    \        assert!(pred(&M::e()));\n        if pred(&self.prod(..r)) {\n       \
    \     return 0;\n        }\n        r += self.n;\n        let mut s = M::e();\n\
    \        loop {\n            r -= 1;\n            while !self.is_good_node(r)\
    \ {\n                r = r * 2 + 1;\n            }\n            while r & 1 !=\
    \ 0 && self.is_good_node(r >> 1) {\n                r >>= 1;\n            }\n\
    \            if !pred(&M::op(&self.v[r], &s)) {\n                while r < self.n\
    \ {\n                    r = r * 2 + 1;\n                    let t = M::op(&self.v[r],\
    \ &s);\n                    if pred(&t) {\n                        s = t;\n  \
    \                      r -= 1;\n                    }\n                }\n   \
    \             return r + 1 - self.n;\n            }\n            s = M::op(&self.v[r],\
    \ &s);\n        }\n    }\n\n    #[inline]\n    fn is_good_node(&self, k: usize)\
    \ -> bool {\n        if k >= self.n {\n            true\n        } else {\n  \
    \          let d = k.leading_zeros() - self.n.leading_zeros();\n            self.n\
    \ >> d != k || self.n >> d << d == self.n\n        }\n    }\n}\n\nimpl<M> From<Vec<M::S>>\
    \ for SegmentTree<M>\nwhere\n    M: Monoid,\n    M::S: Clone,\n{\n    fn from(mut\
    \ a: Vec<M::S>) -> Self {\n        let n = a.len();\n        let mut v = vec![M::e();\
    \ n];\n        v.append(&mut a);\n        for i in (1..n).rev() {\n          \
    \  v[i] = M::op(&v[i * 2], &v[i * 2 + 1]);\n        }\n        Self { n, v }\n\
    \    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  isVerificationFile: false
  path: crates/data-structure/segment-tree/src/lib.rs
  requiredBy:
  - crates/data-structure/segment-tree-on-wavelet-matrix/src/lib.rs
  - crates/data-structure/vertex-add-range-contour-sum/src/lib.rs
  - crates/data-structure/range-tree/src/lib.rs
  - crates/data-structure/tree-query/src/lib.rs
  timestamp: '2024-03-18 01:19:47+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/vertex_add_subtree_sum/src/main.rs
  - verify/vertex_add_path_sum/src/main.rs
  - verify/vertex_set_path_composite/src/main.rs
  - verify/point_add_range_sum/src/main.rs
documentation_of: crates/data-structure/segment-tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/segment-tree/src/lib.rs
- /library/crates/data-structure/segment-tree/src/lib.rs.html
title: crates/data-structure/segment-tree/src/lib.rs
---
