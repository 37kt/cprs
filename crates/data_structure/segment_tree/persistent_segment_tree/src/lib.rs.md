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
    path: crates/data_structure/segment_tree/persistent_segment_tree/src/node.rs
    title: crates/data_structure/segment_tree/persistent_segment_tree/src/node.rs
  - icon: ':warning:'
    path: crates/misc/as_half_open_range/src/lib.rs
    title: crates/misc/as_half_open_range/src/lib.rs
  - icon: ':warning:'
    path: crates/misc/simple_arena/src/lib.rs
    title: crates/misc/simple_arena/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segment_tree/persistent_segment_tree/src/node.rs
    title: crates/data_structure/segment_tree/persistent_segment_tree/src/node.rs
  - icon: ':warning:'
    path: verify/sandbox/test/src/main.rs
    title: verify/sandbox/test/src/main.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/rectangle_sum_pseg/src/main.rs
    title: verify/library_checker/data_structure/rectangle_sum_pseg/src/main.rs
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
  code: "use std::{ops::RangeBounds, ptr::NonNull};\n\nuse algebraic_traits::Monoid;\n\
    use as_half_open_range::AsHalfOpenRange;\nuse node::Node;\n\nmod node;\n\npub\
    \ struct PersistentSegmentTree<M>\nwhere\n    M: Monoid,\n{\n    n: usize,\n \
    \   root: NonNull<Node<M>>,\n}\n\nimpl<M> Clone for PersistentSegmentTree<M>\n\
    where\n    M: Monoid,\n{\n    fn clone(&self) -> Self {\n        Self {\n    \
    \        n: self.n,\n            root: Node::copy(Some(self.root)).unwrap(),\n\
    \        }\n    }\n}\n\nimpl<M> PersistentSegmentTree<M>\nwhere\n    M: Monoid,\n\
    {\n    pub fn new(n: usize) -> Self {\n        let root = Node::new_ptr(None,\
    \ None, M::unit());\n        Self { n, root }\n    }\n\n    pub fn len(&self)\
    \ -> usize {\n        self.n\n    }\n\n    pub fn is_empty(&self) -> bool {\n\
    \        self.n == 0\n    }\n\n    pub fn set(&self, i: usize, x: M::Value) ->\
    \ Self {\n        assert!(i < self.n);\n\n        let new_root = Self::set_rec(Some(self.root),\
    \ 0, self.n, i, x).unwrap();\n        Self {\n            n: self.n,\n       \
    \     root: new_root,\n        }\n    }\n\n    pub fn add(&self, i: usize, x:\
    \ M::Value) -> Self {\n        assert!(i < self.n);\n\n        let new_root =\
    \ Self::add_rec(Some(self.root), 0, self.n, i, x).unwrap();\n        Self {\n\
    \            n: self.n,\n            root: new_root,\n        }\n    }\n\n   \
    \ pub fn copy_range(&self, range: impl RangeBounds<usize>, source: &Self) -> Self\
    \ {\n        let (l, r) = range.as_half_open_range(0, self.n);\n        let new_root\
    \ =\n            Self::copy_range_rec(Some(self.root), Some(source.root), 0, self.n,\
    \ l, r).unwrap();\n        Self {\n            n: self.n,\n            root: new_root,\n\
    \        }\n    }\n\n    pub fn get(&self, i: usize) -> M::Value {\n        assert!(i\
    \ < self.n);\n        let mut v = Some(self.root);\n        let mut a = 0;\n \
    \       let mut b = self.n;\n        while a + 1 < b && v.is_some() {\n      \
    \      let c = (a + b) / 2;\n            let v_ref = unsafe { v.as_ref().unwrap().as_ref()\
    \ };\n            if i < c {\n                v = v_ref.l;\n                b\
    \ = c;\n            } else {\n                v = v_ref.r;\n                a\
    \ = c;\n            }\n        }\n\n        Node::fold(v)\n    }\n\n    pub fn\
    \ fold(&self, range: impl RangeBounds<usize>) -> M::Value {\n        let (l, r)\
    \ = range.as_half_open_range(0, self.n);\n        Self::fold_rec(Some(self.root),\
    \ 0, self.n, l, r)\n    }\n\n    fn set_rec(\n        v: Option<NonNull<Node<M>>>,\n\
    \        a: usize,\n        b: usize,\n        i: usize,\n        x: M::Value,\n\
    \    ) -> Option<NonNull<Node<M>>> {\n        if a == b {\n            return\
    \ None;\n        }\n\n        let mut v = Node::copy(v).unwrap_or(Node::new_ptr(None,\
    \ None, M::unit()));\n        let v_ref = unsafe { v.as_mut() };\n        if a\
    \ + 1 == b {\n            v_ref.x = x;\n            return Some(v);\n        }\n\
    \n        let c = (a + b) / 2;\n        if i < c {\n            v_ref.l = Self::set_rec(v_ref.l,\
    \ a, c, i, x);\n        } else {\n            v_ref.r = Self::set_rec(v_ref.r,\
    \ c, b, i, x);\n        }\n\n        Node::update(v);\n        Some(v)\n    }\n\
    \n    fn add_rec(\n        v: Option<NonNull<Node<M>>>,\n        a: usize,\n \
    \       b: usize,\n        i: usize,\n        x: M::Value,\n    ) -> Option<NonNull<Node<M>>>\
    \ {\n        if a == b {\n            return None;\n        }\n\n        let mut\
    \ v = Node::copy(v).unwrap_or(Node::new_ptr(None, None, M::unit()));\n       \
    \ let v_ref = unsafe { v.as_mut() };\n        if a + 1 == b {\n            v_ref.x\
    \ = M::op(&v_ref.x, &x);\n            return Some(v);\n        }\n\n        let\
    \ c = (a + b) / 2;\n        if i < c {\n            v_ref.l = Self::add_rec(v_ref.l,\
    \ a, c, i, x);\n        } else {\n            v_ref.r = Self::add_rec(v_ref.r,\
    \ c, b, i, x);\n        }\n\n        Node::update(v);\n        Some(v)\n    }\n\
    \n    fn fold_rec(v: Option<NonNull<Node<M>>>, a: usize, b: usize, l: usize, r:\
    \ usize) -> M::Value {\n        if v.is_none() || b <= l || r <= a {\n       \
    \     return M::unit();\n        }\n\n        let v = unsafe { v.as_ref().unwrap().as_ref()\
    \ };\n        if l <= a && b <= r {\n            v.x.clone()\n        } else {\n\
    \            let c = (a + b) / 2;\n            M::op(\n                &Self::fold_rec(v.l,\
    \ a, c, l, r),\n                &Self::fold_rec(v.r, c, b, l, r),\n          \
    \  )\n        }\n    }\n\n    fn copy_range_rec(\n        v: Option<NonNull<Node<M>>>,\n\
    \        source: Option<NonNull<Node<M>>>,\n        a: usize,\n        b: usize,\n\
    \        l: usize,\n        r: usize,\n    ) -> Option<NonNull<Node<M>>> {\n \
    \       if b <= l || r <= a {\n            v\n        } else if l <= a && b <=\
    \ r {\n            Node::copy(source)\n        } else {\n            let vl =\
    \ v.as_ref().and_then(|v| unsafe { v.as_ref() }.l);\n            let vr = v.as_ref().and_then(|v|\
    \ unsafe { v.as_ref() }.r);\n            let sl = source.as_ref().and_then(|v|\
    \ unsafe { v.as_ref() }.l);\n            let sr = source.as_ref().and_then(|v|\
    \ unsafe { v.as_ref() }.r);\n            let c = (a + b) / 2;\n            let\
    \ res_l = Self::copy_range_rec(vl, sl, a, c, l, r);\n            let res_r = Self::copy_range_rec(vr,\
    \ sr, c, b, l, r);\n            Node::merge(res_l, res_r)\n        }\n    }\n\
    }\n"
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
  - crates/data_structure/segment_tree/persistent_segment_tree/src/node.rs
  - crates/misc/as_half_open_range/src/lib.rs
  - crates/misc/simple_arena/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/segment_tree/persistent_segment_tree/src/lib.rs
  requiredBy:
  - crates/data_structure/segment_tree/persistent_segment_tree/src/node.rs
  - verify/sandbox/test/src/main.rs
  timestamp: '2025-04-30 05:56:56+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/rectangle_sum_pseg/src/main.rs
documentation_of: crates/data_structure/segment_tree/persistent_segment_tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/segment_tree/persistent_segment_tree/src/lib.rs
- /library/crates/data_structure/segment_tree/persistent_segment_tree/src/lib.rs.html
title: crates/data_structure/segment_tree/persistent_segment_tree/src/lib.rs
---
