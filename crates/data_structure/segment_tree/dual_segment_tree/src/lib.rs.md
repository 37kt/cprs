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
    path: verify/library_checker/data_structure/range_affine_point_get/src/main.rs
    title: verify/library_checker/data_structure/range_affine_point_get/src/main.rs
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
  code: "use std::ops::RangeBounds;\n\nuse algebraic_traits::Monoid;\nuse as_half_open_range::AsHalfOpenRange;\n\
    use numeric_traits::Integer;\n\npub struct DualSegmentTree<M>\nwhere\n    M: Monoid,\n\
    \    M::Value: Clone + Eq,\n{\n    n: usize,\n    sz: usize,\n    lg: usize,\n\
    \    lz: Vec<M::Value>,\n}\n\nimpl<M> FromIterator<M::Value> for DualSegmentTree<M>\n\
    where\n    M: Monoid,\n    M::Value: Clone + Eq,\n{\n    fn from_iter<T: IntoIterator<Item\
    \ = M::Value>>(iter: T) -> Self {\n        let a = iter.into_iter().collect::<Vec<_>>();\n\
    \        let n = a.len();\n        let lg = n.checked_ceil_log2().unwrap_or(0);\n\
    \        let sz = 1 << lg;\n        let lz = (0..sz)\n            .map(|_| M::unit())\n\
    \            .chain(a)\n            .chain((0..sz - n).map(|_| M::unit()))\n \
    \           .collect();\n        Self { n, sz, lg, lz }\n    }\n}\n\nimpl<M> DualSegmentTree<M>\n\
    where\n    M: Monoid,\n    M::Value: Clone + Eq,\n{\n    pub fn new(n: usize)\
    \ -> Self {\n        let lg = n.checked_ceil_log2().unwrap_or(0);\n        let\
    \ sz = 1 << lg;\n        let lz = vec![M::unit(); sz * 2];\n        Self { n,\
    \ sz, lg, lz }\n    }\n\n    pub fn from_fn(n: usize, f: impl FnMut(usize) ->\
    \ M::Value) -> Self {\n        Self::from_iter((0..n).map(f))\n    }\n\n    pub\
    \ fn len(&self) -> usize {\n        self.n\n    }\n\n    pub fn is_empty(&self)\
    \ -> bool {\n        self.n == 0\n    }\n\n    pub fn set(&mut self, mut i: usize,\
    \ x: M::Value) {\n        assert!(i < self.n);\n        i += self.sz;\n      \
    \  for h in (1..=self.lg).rev() {\n            self.push(i >> h);\n        }\n\
    \        self.lz[i] = x;\n    }\n\n    pub fn get(&mut self, mut i: usize) ->\
    \ M::Value {\n        assert!(i < self.n);\n        i += self.sz;\n        for\
    \ h in (1..=self.lg).rev() {\n            self.push(i >> h);\n        }\n    \
    \    self.lz[i].clone()\n    }\n\n    pub fn apply_range(&mut self, range: impl\
    \ RangeBounds<usize>, f: M::Value) {\n        let (mut l, mut r) = range.as_half_open_range(0,\
    \ self.n);\n        if l == r {\n            return;\n        }\n        l +=\
    \ self.sz;\n        r += self.sz;\n\n        // if !COMMUTATIVE {\n        for\
    \ h in (1..=self.lg).rev() {\n            if l >> h << h != l {\n            \
    \    self.push(l >> h);\n            }\n            if r >> h << h != r {\n  \
    \              self.push((r - 1) >> h);\n            }\n        }\n        //\
    \ }\n\n        while l < r {\n            if l & 1 != 0 {\n                self.apply_at(l,\
    \ &f);\n                l += 1;\n            }\n            if r & 1 != 0 {\n\
    \                r -= 1;\n                self.apply_at(r, &f);\n            }\n\
    \            l >>= 1;\n            r >>= 1;\n        }\n    }\n\n    fn push(&mut\
    \ self, i: usize) {\n        if self.lz[i] == M::unit() {\n            return;\n\
    \        }\n        let f = std::mem::replace(&mut self.lz[i], M::unit());\n \
    \       self.lz[i << 1] = M::op(&self.lz[i << 1], &f);\n        self.lz[i << 1\
    \ | 1] = M::op(&self.lz[i << 1 | 1], &f);\n    }\n\n    fn apply_at(&mut self,\
    \ i: usize, f: &M::Value) {\n        self.lz[i] = M::op(&self.lz[i], f);\n   \
    \ }\n}\n"
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
  path: crates/data_structure/segment_tree/dual_segment_tree/src/lib.rs
  requiredBy: []
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/range_affine_point_get/src/main.rs
documentation_of: crates/data_structure/segment_tree/dual_segment_tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/segment_tree/dual_segment_tree/src/lib.rs
- /library/crates/data_structure/segment_tree/dual_segment_tree/src/lib.rs.html
title: crates/data_structure/segment_tree/dual_segment_tree/src/lib.rs
---
