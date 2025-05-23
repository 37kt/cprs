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
    path: verify/library_checker/data_structure/point_add_range_sum/src/main.rs
    title: verify/library_checker/data_structure/point_add_range_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/point_add_rectangle_sum/src/main.rs
    title: verify/library_checker/data_structure/point_add_rectangle_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/point_set_range_frequency/src/main.rs
    title: verify/library_checker/data_structure/point_set_range_frequency/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/rectangle_add_point_get/src/main.rs
    title: verify/library_checker/data_structure/rectangle_add_point_get/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/vertex_add_path_sum/src/main.rs
    title: verify/library_checker/tree/vertex_add_path_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/vertex_add_range_contour_sum_on_tree/src/main.rs
    title: verify/library_checker/tree/vertex_add_range_contour_sum_on_tree/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/vertex_add_subtree_sum/src/main.rs
    title: verify/library_checker/tree/vertex_add_subtree_sum/src/main.rs
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
  code: "// TODO: \u4E8C\u5206\u63A2\u7D22\n\nuse std::ops::{Deref, DerefMut, RangeBounds};\n\
    \nuse algebraic_traits::{AbelianGroup, CommutativeMonoid};\nuse as_half_open_range::AsHalfOpenRange;\n\
    use numeric_traits::Integer;\n\npub struct FenwickTree<M>\nwhere\n    M: CommutativeMonoid,\n\
    \    M::Value: Clone,\n{\n    n: usize,\n    v: Vec<M::Value>,\n}\n\nimpl<M> Clone\
    \ for FenwickTree<M>\nwhere\n    M: CommutativeMonoid,\n    M::Value: Clone,\n\
    {\n    fn clone(&self) -> Self {\n        Self {\n            n: self.n,\n   \
    \         v: self.v.clone(),\n        }\n    }\n}\n\nimpl<M> FromIterator<M::Value>\
    \ for FenwickTree<M>\nwhere\n    M: CommutativeMonoid,\n    M::Value: Clone,\n\
    {\n    fn from_iter<T: IntoIterator<Item = M::Value>>(iter: T) -> Self {\n   \
    \     let mut v = iter.into_iter().collect::<Vec<_>>();\n        let n = v.len();\n\
    \        for i in 1..=n {\n            let j = i + i.lsb();\n            if j\
    \ <= n {\n                v[j - 1] = M::op(&v[j - 1], &v[i - 1]);\n          \
    \  }\n        }\n        Self { n, v }\n    }\n}\n\nimpl<M> FenwickTree<M>\nwhere\n\
    \    M: CommutativeMonoid,\n    M::Value: Clone,\n{\n    pub fn from_fn(n: usize,\
    \ f: impl FnMut(usize) -> M::Value) -> Self {\n        Self::from_iter((0..n).map(f))\n\
    \    }\n\n    pub fn new(n: usize) -> Self {\n        Self {\n            n,\n\
    \            v: vec![M::unit(); n],\n        }\n    }\n\n    pub fn len(&self)\
    \ -> usize {\n        self.n\n    }\n\n    pub fn is_empty(&self) -> bool {\n\
    \        self.n == 0\n    }\n\n    pub fn add(&mut self, mut i: usize, x: M::Value)\
    \ {\n        assert!(i < self.n);\n        i += 1;\n        while i <= self.n\
    \ {\n            self.v[i - 1] = M::op(&self.v[i - 1], &x);\n            i +=\
    \ i.lsb();\n        }\n    }\n\n    pub fn fold_prefix(&self, mut i: usize) ->\
    \ M::Value {\n        let mut s = M::unit();\n        while i > 0 {\n        \
    \    s = M::op(&s, &self.v[i - 1]);\n            i -= i.lsb();\n        }\n  \
    \      s\n    }\n}\n\nimpl<G> FenwickTree<G>\nwhere\n    G: AbelianGroup,\n  \
    \  G::Value: Clone,\n{\n    pub fn fold(&self, range: impl RangeBounds<usize>)\
    \ -> G::Value {\n        let (mut l, mut r) = range.as_half_open_range(0, self.n);\n\
    \        let mut s = G::unit();\n        while r > l {\n            s = G::op(&s,\
    \ &self.v[r - 1]);\n            r -= r.lsb();\n        }\n        let mut t =\
    \ G::unit();\n        while l > r {\n            t = G::op(&t, &self.v[l - 1]);\n\
    \            l -= l.lsb();\n        }\n        G::op(&s, &G::inv(&t))\n    }\n\
    \n    pub fn get(&self, i: usize) -> G::Value {\n        self.fold(i..i + 1)\n\
    \    }\n\n    pub fn set(&mut self, i: usize, x: G::Value) {\n        self.add(i,\
    \ G::op(&x, &G::inv(&self.get(i))));\n    }\n\n    pub fn get_mut(&mut self, i:\
    \ usize) -> GetMut<G> {\n        GetMut {\n            x: self.get(i),\n     \
    \       ft: self,\n            i,\n        }\n    }\n}\n\npub struct GetMut<'a,\
    \ G>\nwhere\n    G: AbelianGroup,\n    G::Value: Clone,\n{\n    ft: &'a mut FenwickTree<G>,\n\
    \    i: usize,\n    x: G::Value,\n}\n\nimpl<'a, G> Deref for GetMut<'a, G>\nwhere\n\
    \    G: AbelianGroup,\n    G::Value: Clone,\n{\n    type Target = G::Value;\n\n\
    \    fn deref(&self) -> &Self::Target {\n        &self.x\n    }\n}\n\nimpl<'a,\
    \ G> DerefMut for GetMut<'a, G>\nwhere\n    G: AbelianGroup,\n    G::Value: Clone,\n\
    {\n    fn deref_mut(&mut self) -> &mut Self::Target {\n        &mut self.x\n \
    \   }\n}\n\nimpl<'a, G> Drop for GetMut<'a, G>\nwhere\n    G: AbelianGroup,\n\
    \    G::Value: Clone,\n{\n    fn drop(&mut self) {\n        let Self { ft, i,\
    \ x } = self;\n        ft.set(*i, x.clone());\n    }\n}\n"
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
  path: crates/data_structure/fenwick_tree/fenwick_tree/src/lib.rs
  requiredBy: []
  timestamp: '2025-04-22 06:09:15+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/point_set_range_frequency/src/main.rs
  - verify/library_checker/data_structure/point_add_range_sum/src/main.rs
  - verify/library_checker/data_structure/rectangle_add_point_get/src/main.rs
  - verify/library_checker/data_structure/point_add_rectangle_sum/src/main.rs
  - verify/library_checker/tree/vertex_add_range_contour_sum_on_tree/src/main.rs
  - verify/library_checker/tree/vertex_add_path_sum/src/main.rs
  - verify/library_checker/tree/vertex_add_subtree_sum/src/main.rs
documentation_of: crates/data_structure/fenwick_tree/fenwick_tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/fenwick_tree/fenwick_tree/src/lib.rs
- /library/crates/data_structure/fenwick_tree/fenwick_tree/src/lib.rs.html
title: crates/data_structure/fenwick_tree/fenwick_tree/src/lib.rs
---
