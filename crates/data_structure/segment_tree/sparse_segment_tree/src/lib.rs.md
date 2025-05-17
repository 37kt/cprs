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
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segment_tree/sparse_segment_tree/src/node.rs
    title: crates/data_structure/segment_tree/sparse_segment_tree/src/node.rs
  - icon: ':warning:'
    path: crates/misc/as_half_open_range/src/lib.rs
    title: crates/misc/as_half_open_range/src/lib.rs
  - icon: ':warning:'
    path: crates/misc/simple_arena/src/lib.rs
    title: crates/misc/simple_arena/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segment_tree/sparse_segment_tree/src/node.rs
    title: crates/data_structure/segment_tree/sparse_segment_tree/src/node.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/point_set_range_composite_large_array/src/main.rs
    title: verify/library_checker/data_structure/point_set_range_composite_large_array/src/main.rs
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
  code: "// ABC403-G\n\nuse std::{ops::RangeBounds, ptr::NonNull};\n\nuse algebraic_traits::Monoid;\n\
    use as_half_open_range::AsHalfOpenRange;\nuse node::Node;\n\nmod node;\n\npub\
    \ struct SparseSegmentTree<M>\nwhere\n    M: Monoid,\n{\n    n: usize,\n    root:\
    \ NonNull<Node<M>>,\n}\n\nimpl<M> SparseSegmentTree<M>\nwhere\n    M: Monoid,\n\
    {\n    pub fn new(n: usize) -> Self {\n        let root = Node::new_ptr(M::unit());\n\
    \        Self { n, root }\n    }\n\n    pub fn len(&self) -> usize {\n       \
    \ self.n\n    }\n\n    pub fn is_empty(&self) -> bool {\n        self.n == 0\n\
    \    }\n\n    pub fn set(&mut self, i: usize, x: M::Value) {\n        assert!(i\
    \ < self.n);\n        Self::set_rec(&mut Some(self.root), 0, self.n, i, x);\n\
    \    }\n\n    pub fn add(&mut self, i: usize, x: M::Value) {\n        assert!(i\
    \ < self.n);\n        Self::add_rec(&mut Some(self.root), 0, self.n, i, x);\n\
    \    }\n\n    pub fn clear_range(&mut self, range: impl RangeBounds<usize>) {\n\
    \        let (l, r) = range.as_half_open_range(0, self.n);\n        Self::clear_range_rec(&mut\
    \ Some(self.root), 0, self.n, l, r);\n    }\n\n    pub fn get(&self, i: usize)\
    \ -> M::Value {\n        assert!(i < self.n);\n        let mut v = Some(self.root);\n\
    \        let mut a = 0;\n        let mut b = self.n;\n        while a + 1 < b\
    \ && v.is_some() {\n            let c = (a + b) / 2;\n            let v_ref =\
    \ unsafe { v.as_ref().unwrap().as_ref() };\n            if i < c {\n         \
    \       v = v_ref.l;\n                b = c;\n            } else {\n         \
    \       v = v_ref.r;\n                a = c;\n            }\n        }\n\n   \
    \     v.as_ref()\n            .map_or(M::unit(), |v| unsafe { v.as_ref() }.x.clone())\n\
    \    }\n\n    pub fn fold(&self, range: impl RangeBounds<usize>) -> M::Value {\n\
    \        let (l, r) = range.as_half_open_range(0, self.n);\n        Self::fold_rec(Some(self.root),\
    \ 0, self.n, l, r)\n    }\n\n    fn fold_node(v: Option<NonNull<Node<M>>>) ->\
    \ M::Value {\n        v.as_ref()\n            .map_or(M::unit(), |v| unsafe {\
    \ v.as_ref() }.x.clone())\n    }\n\n    fn update_node(v: &mut Node<M>) {\n  \
    \      v.x = M::op(&Self::fold_node(v.l), &Self::fold_node(v.r));\n    }\n\n \
    \   fn set_rec(v: &mut Option<NonNull<Node<M>>>, a: usize, b: usize, i: usize,\
    \ x: M::Value) {\n        if a == b {\n            return;\n        }\n\n    \
    \    if v.is_none() {\n            *v = Some(Node::new_ptr(M::unit()));\n    \
    \    }\n        let v = unsafe { v.as_mut().unwrap().as_mut() };\n        if a\
    \ + 1 == b {\n            v.x = x;\n            return;\n        }\n\n       \
    \ let c = (a + b) / 2;\n        if i < c {\n            Self::set_rec(&mut v.l,\
    \ a, c, i, x);\n        } else {\n            Self::set_rec(&mut v.r, c, b, i,\
    \ x);\n        }\n\n        Self::update_node(v);\n    }\n\n    fn add_rec(v:\
    \ &mut Option<NonNull<Node<M>>>, a: usize, b: usize, i: usize, x: M::Value) {\n\
    \        if a == b {\n            return;\n        }\n\n        if v.is_none()\
    \ {\n            *v = Some(Node::new_ptr(M::unit()));\n        }\n        let\
    \ v = unsafe { v.as_mut().unwrap().as_mut() };\n        if a + 1 == b {\n    \
    \        v.x = M::op(&v.x, &x);\n            return;\n        }\n\n        let\
    \ c = (a + b) / 2;\n        if i < c {\n            Self::add_rec(&mut v.l, a,\
    \ c, i, x);\n        } else {\n            Self::add_rec(&mut v.r, c, b, i, x);\n\
    \        }\n\n        Self::update_node(v);\n    }\n\n    fn fold_rec(v: Option<NonNull<Node<M>>>,\
    \ a: usize, b: usize, l: usize, r: usize) -> M::Value {\n        if v.is_none()\
    \ || b <= l || r <= a {\n            return M::unit();\n        }\n\n        let\
    \ v = unsafe { v.as_ref().unwrap().as_ref() };\n        if l <= a && b <= r {\n\
    \            v.x.clone()\n        } else {\n            let c = (a + b) / 2;\n\
    \            M::op(\n                &Self::fold_rec(v.l, a, c, l, r),\n     \
    \           &Self::fold_rec(v.r, c, b, l, r),\n            )\n        }\n    }\n\
    \n    fn clear_range_rec(v: &mut Option<NonNull<Node<M>>>, a: usize, b: usize,\
    \ l: usize, r: usize) {\n        if v.is_none() || b <= l || r <= a {\n      \
    \      return;\n        }\n        if l <= a && b <= r {\n            *v = None;\n\
    \            return;\n        }\n        let v = unsafe { v.as_mut().unwrap().as_mut()\
    \ };\n\n        let c = (a + b) / 2;\n        Self::clear_range_rec(&mut v.l,\
    \ a, c, l, r);\n        Self::clear_range_rec(&mut v.r, c, b, l, r);\n\n     \
    \   Self::update_node(v);\n    }\n}\n"
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
  - crates/data_structure/segment_tree/sparse_segment_tree/src/node.rs
  - crates/misc/as_half_open_range/src/lib.rs
  - crates/misc/simple_arena/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/segment_tree/sparse_segment_tree/src/lib.rs
  requiredBy:
  - crates/data_structure/segment_tree/sparse_segment_tree/src/node.rs
  timestamp: '2025-04-30 05:56:56+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/point_set_range_composite_large_array/src/main.rs
documentation_of: crates/data_structure/segment_tree/sparse_segment_tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/segment_tree/sparse_segment_tree/src/lib.rs
- /library/crates/data_structure/segment_tree/sparse_segment_tree/src/lib.rs.html
title: crates/data_structure/segment_tree/sparse_segment_tree/src/lib.rs
---
