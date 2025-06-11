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
    path: crates/algebra/numeric_traits/src/cast.rs
    title: crates/algebra/numeric_traits/src/cast.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/inf.rs
    title: crates/algebra/numeric_traits/src/inf.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/integer.rs
    title: crates/algebra/numeric_traits/src/integer.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/lib.rs
    title: crates/algebra/numeric_traits/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/numeric.rs
    title: crates/algebra/numeric_traits/src/numeric.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/signed.rs
    title: crates/algebra/numeric_traits/src/signed.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/zero_one.rs
    title: crates/algebra/numeric_traits/src/zero_one.rs
  - icon: ':warning:'
    path: crates/misc/as_half_open_range/src/lib.rs
    title: crates/misc/as_half_open_range/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/majority_voting/src/main.rs
    title: verify/library_checker/data_structure/majority_voting/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/majority_voting_bt/src/main.rs
    title: verify/library_checker/data_structure/majority_voting_bt/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/point_set_range_composite/src/main.rs
    title: verify/library_checker/data_structure/point_set_range_composite/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/vertex_set_path_composite/src/main.rs
    title: verify/library_checker/tree/vertex_set_path_composite/src/main.rs
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
  code: "use std::ops::{Deref, DerefMut, RangeBounds};\n\nuse algebraic_traits::Monoid;\n\
    use as_half_open_range::AsHalfOpenRange;\nuse numeric_traits::Integer;\n\npub\
    \ struct SegmentTree<M>\nwhere\n    M: Monoid,\n    M::Value: Clone,\n{\n    n:\
    \ usize,\n    sz: usize,\n    v: Vec<M::Value>,\n}\n\nimpl<M> FromIterator<M::Value>\
    \ for SegmentTree<M>\nwhere\n    M: Monoid,\n    M::Value: Clone,\n{\n    fn from_iter<T:\
    \ IntoIterator<Item = M::Value>>(iter: T) -> Self {\n        let a = iter.into_iter().collect::<Vec<_>>();\n\
    \        let n = a.len();\n        let sz = n.checked_ceil_pow2().unwrap_or(1);\n\
    \        let v: Vec<_> = (0..sz)\n            .map(|_| M::unit())\n          \
    \  .chain(a)\n            .chain((0..sz - n).map(|_| M::unit()))\n           \
    \ .collect();\n        let mut seg = Self { n, sz, v };\n        for i in (1..sz).rev()\
    \ {\n            seg.update(i);\n        }\n        seg\n    }\n}\n\nimpl<M> SegmentTree<M>\n\
    where\n    M: Monoid,\n    M::Value: Clone,\n{\n    pub fn new(n: usize) -> Self\
    \ {\n        let sz = n.checked_ceil_pow2().unwrap_or(1);\n        let v = vec![M::unit();\
    \ sz * 2];\n        Self { n, sz, v }\n    }\n\n    pub fn from_fn(n: usize, f:\
    \ impl FnMut(usize) -> M::Value) -> Self {\n        Self::from_iter((0..n).map(f))\n\
    \    }\n\n    pub fn len(&self) -> usize {\n        self.n\n    }\n\n    pub fn\
    \ is_empty(&self) -> bool {\n        self.n == 0\n    }\n\n    pub fn set(&mut\
    \ self, i: usize, x: M::Value) {\n        assert!(i < self.n);\n        let mut\
    \ i = i + self.sz;\n        self.v[i] = x;\n        while i > 1 {\n          \
    \  i >>= 1;\n            self.update(i);\n        }\n    }\n\n    pub fn add(&mut\
    \ self, i: usize, x: M::Value) {\n        assert!(i < self.n);\n        let mut\
    \ i = i + self.sz;\n        self.v[i] = M::op(&self.v[i], &x);\n        while\
    \ i > 1 {\n            i >>= 1;\n            self.update(i);\n        }\n    }\n\
    \n    pub fn get(&self, i: usize) -> M::Value {\n        assert!(i < self.n);\n\
    \        self.v[i + self.sz].clone()\n    }\n\n    pub fn get_mut(&mut self, i:\
    \ usize) -> GetMut<M> {\n        assert!(i < self.n);\n        GetMut {\n    \
    \        i: i + self.sz,\n            seg: self,\n        }\n    }\n\n    pub\
    \ fn as_slice(&self) -> &[M::Value] {\n        &self.v[self.sz..self.sz + self.n]\n\
    \    }\n\n    pub fn fold(&self, range: impl RangeBounds<usize>) -> M::Value {\n\
    \        let (mut l, mut r) = range.as_half_open_range(0, self.n);\n        l\
    \ += self.sz;\n        r += self.sz;\n        let mut sl = M::unit();\n      \
    \  let mut sr = M::unit();\n        while l < r {\n            if l & 1 != 0 {\n\
    \                sl = M::op(&sl, &self.v[l]);\n                l += 1;\n     \
    \       }\n            if r & 1 != 0 {\n                r -= 1;\n            \
    \    sr = M::op(&self.v[r], &sr);\n            }\n            l >>= 1;\n     \
    \       r >>= 1;\n        }\n        M::op(&sl, &sr)\n    }\n\n    pub fn fold_all(&self)\
    \ -> M::Value {\n        self.v[1].clone()\n    }\n\n    pub fn max_right(&self,\
    \ l: usize, mut pred: impl FnMut(&M::Value) -> bool) -> usize {\n        assert!(l\
    \ <= self.n);\n        assert!(pred(&M::unit()));\n        if l == self.n {\n\
    \            return self.n;\n        }\n        let mut r = l + self.sz;\n   \
    \     let mut s = M::unit();\n        loop {\n            r >>= r.lsb_index();\n\
    \            let t = M::op(&s, &self.v[r]);\n            if !pred(&t) {\n    \
    \            while r < self.sz {\n                    r <<= 1;\n             \
    \       let t = M::op(&s, &self.v[r]);\n                    if pred(&t) {\n  \
    \                      s = t;\n                        r += 1;\n             \
    \       }\n                }\n                return r - self.sz;\n          \
    \  }\n            s = t;\n            r += 1;\n            if r == r.lsb() {\n\
    \                break;\n            }\n        }\n        self.n\n    }\n\n \
    \   pub fn min_left(&self, r: usize, mut pred: impl FnMut(&M::Value) -> bool)\
    \ -> usize {\n        assert!(r <= self.n);\n        assert!(pred(&M::unit()));\n\
    \        if r == 0 {\n            return 0;\n        }\n        let mut l = r\
    \ + self.sz;\n        let mut s = M::unit();\n        loop {\n            l -=\
    \ 1;\n            l >>= (!l).lsb_index();\n            l = l.max(1);\n       \
    \     let t = M::op(&self.v[l], &s);\n            if !pred(&t) {\n           \
    \     while l < self.sz {\n                    l = l << 1 | 1;\n             \
    \       let t = M::op(&self.v[l], &s);\n                    if pred(&t) {\n  \
    \                      s = t;\n                        l -= 1;\n             \
    \       }\n                }\n                return l + 1 - self.sz;\n      \
    \      }\n            s = t;\n            if l == l.lsb() {\n                break;\n\
    \            }\n        }\n        0\n    }\n\n    fn update(&mut self, i: usize)\
    \ {\n        self.v[i] = M::op(&self.v[i << 1], &self.v[i << 1 | 1]);\n    }\n\
    }\n\npub struct GetMut<'a, M>\nwhere\n    M: Monoid,\n    M::Value: Clone,\n{\n\
    \    seg: &'a mut SegmentTree<M>,\n    i: usize,\n}\n\nimpl<'a, M> Deref for GetMut<'a,\
    \ M>\nwhere\n    M: Monoid,\n    M::Value: Clone,\n{\n    type Target = M::Value;\n\
    \n    fn deref(&self) -> &Self::Target {\n        &self.seg.v[self.i]\n    }\n\
    }\n\nimpl<'a, M> DerefMut for GetMut<'a, M>\nwhere\n    M: Monoid,\n    M::Value:\
    \ Clone,\n{\n    fn deref_mut(&mut self) -> &mut Self::Target {\n        &mut\
    \ self.seg.v[self.i]\n    }\n}\n\nimpl<'a, M> Drop for GetMut<'a, M>\nwhere\n\
    \    M: Monoid,\n    M::Value: Clone,\n{\n    fn drop(&mut self) {\n        let\
    \ mut i = self.i;\n        while i > 1 {\n            i >>= 1;\n            self.seg.update(i);\n\
    \        }\n    }\n}\n\nimpl<M> Clone for SegmentTree<M>\nwhere\n    M: Monoid,\n\
    \    M::Value: Clone,\n{\n    fn clone(&self) -> Self {\n        Self {\n    \
    \        n: self.n,\n            sz: self.sz,\n            v: self.v.clone(),\n\
    \        }\n    }\n}\n"
  dependsOn:
  - crates/algebra/algebraic_traits/src/lib.rs
  - crates/algebra/algebraic_traits/src/macros.rs
  - crates/algebra/algebraic_traits/src/traits.rs
  - crates/algebra/numeric_traits/src/cast.rs
  - crates/algebra/numeric_traits/src/inf.rs
  - crates/algebra/numeric_traits/src/integer.rs
  - crates/algebra/numeric_traits/src/lib.rs
  - crates/algebra/numeric_traits/src/numeric.rs
  - crates/algebra/numeric_traits/src/signed.rs
  - crates/algebra/numeric_traits/src/zero_one.rs
  - crates/misc/as_half_open_range/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/segment_tree/segment_tree/src/lib.rs
  requiredBy: []
  timestamp: '2025-04-22 06:09:15+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/point_set_range_composite/src/main.rs
  - verify/library_checker/data_structure/majority_voting_bt/src/main.rs
  - verify/library_checker/data_structure/majority_voting/src/main.rs
  - verify/library_checker/tree/vertex_set_path_composite/src/main.rs
documentation_of: crates/data_structure/segment_tree/segment_tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/segment_tree/segment_tree/src/lib.rs
- /library/crates/data_structure/segment_tree/segment_tree/src/lib.rs.html
title: crates/data_structure/segment_tree/segment_tree/src/lib.rs
---
