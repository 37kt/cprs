---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  - icon: ':warning:'
    path: crates/data-structure/splay-tree-internal/src/lib.rs
    title: crates/data-structure/splay-tree-internal/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/dynamic_sequence_range_affine_range_sum/src/main.rs
    title: verify/dynamic_sequence_range_affine_range_sum/src/main.rs
  - icon: ':x:'
    path: verify/range_reverse_range_sum/src/main.rs
    title: verify/range_reverse_range_sum/src/main.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{\n    fmt::Debug,\n    ops::{Bound, RangeBounds},\n};\n\nuse algebraic::{Act,\
    \ Monoid};\nuse splay_tree_internal::SplayTreeNode;\n\npub struct SplayTree<M,\
    \ F>(*mut SplayTreeNode<M, F>)\nwhere\n    M: Monoid,\n    M::S: Clone,\n    F:\
    \ Monoid + Act<X = M::S>,\n    F::S: Clone;\n\nimpl<M, F> From<&[M::S]> for SplayTree<M,\
    \ F>\nwhere\n    M: Monoid,\n    M::S: Clone,\n    F: Monoid + Act<X = M::S>,\n\
    \    F::S: Clone,\n{\n    fn from(a: &[M::S]) -> Self {\n        Self(SplayTreeNode::build(a))\n\
    \    }\n}\n\nimpl<M, F> SplayTree<M, F>\nwhere\n    M: Monoid,\n    M::S: Clone\
    \ + Debug,\n    F: Monoid + Act<X = M::S>,\n    F::S: Clone,\n{\n    pub fn dump(&self)\
    \ {\n        eprint!(\"[\");\n        Self::dump_dfs(unsafe { self.0.as_ref()\
    \ }.unwrap());\n        eprintln!(\"]\");\n    }\n\n    fn dump_dfs(t: &SplayTreeNode<M,\
    \ F>) {\n        if let Some(l) = unsafe { t.lch.as_ref() } {\n            Self::dump_dfs(l);\n\
    \        }\n        eprint!(\"{:?}, \", &t.val);\n        if let Some(r) = unsafe\
    \ { t.rch.as_ref() } {\n            Self::dump_dfs(r);\n        }\n    }\n\n \
    \   pub fn set(&mut self, k: usize, val: M::S) {\n        self.0 = SplayTreeNode::access(unsafe\
    \ { self.0.as_mut() }.unwrap(), k);\n        unsafe { self.0.as_mut() }.unwrap().val\
    \ = val;\n        SplayTreeNode::update(self.0);\n    }\n\n    pub fn get(&mut\
    \ self, k: usize) -> &M::S {\n        self.0 = SplayTreeNode::access(unsafe {\
    \ self.0.as_mut() }.unwrap(), k);\n        &unsafe { self.0.as_ref() }.unwrap().val\n\
    \    }\n\n    pub fn insert(&mut self, k: usize, val: M::S) {\n        SplayTreeNode::insert(&mut\
    \ self.0, k, val);\n    }\n\n    pub fn remove(&mut self, k: usize) -> M::S {\n\
    \        SplayTreeNode::remove(&mut self.0, k)\n    }\n\n    pub fn merge(&mut\
    \ self, r: Self) {\n        self.0 = SplayTreeNode::merge(self.0, r.0);\n    }\n\
    \n    pub fn split(self, k: usize) -> (Self, Self) {\n        let (x, y) = SplayTreeNode::split(self.0,\
    \ k);\n        (Self(x), Self(y))\n    }\n\n    pub fn apply(&mut self, range:\
    \ impl RangeBounds<usize>, f: F::S) {\n        let (l, r) = self.range_to_pair(range);\n\
    \        if l == r {\n            return;\n        }\n        let (x, y) = SplayTreeNode::split(self.0,\
    \ l);\n        let (y, z) = SplayTreeNode::split(y, r - l);\n        SplayTreeNode::propagate(unsafe\
    \ { y.as_mut() }.unwrap(), &f);\n        self.0 = SplayTreeNode::merge(x, SplayTreeNode::merge(y,\
    \ z));\n    }\n\n    pub fn reverse(&mut self, range: impl RangeBounds<usize>)\
    \ {\n        let (l, r) = self.range_to_pair(range);\n        if l == r {\n  \
    \          return;\n        }\n        let (x, y) = SplayTreeNode::split(self.0,\
    \ l);\n        let (y, z) = SplayTreeNode::split(y, r - l);\n        unsafe {\
    \ y.as_mut() }.unwrap().toggle();\n        self.0 = SplayTreeNode::merge(x, SplayTreeNode::merge(y,\
    \ z));\n    }\n\n    pub fn prod(&mut self, range: impl RangeBounds<usize>) ->\
    \ M::S {\n        let (l, r) = self.range_to_pair(range);\n        let (x, y)\
    \ = SplayTreeNode::split(self.0, l);\n        let (y, z) = SplayTreeNode::split(y,\
    \ r - l);\n        let res = unsafe { y.as_mut() }.map_or(M::e(), |v| v.prod.clone());\n\
    \        self.0 = SplayTreeNode::merge(x, SplayTreeNode::merge(y, z));\n     \
    \   res\n    }\n\n    fn range_to_pair(&self, range: impl RangeBounds<usize>)\
    \ -> (usize, usize) {\n        let l = match range.start_bound() {\n         \
    \   Bound::Included(&l) => l,\n            Bound::Excluded(&l) => l + 1,\n   \
    \         Bound::Unbounded => 0,\n        };\n        let r = match range.end_bound()\
    \ {\n            Bound::Included(&r) => r + 1,\n            Bound::Excluded(&r)\
    \ => r,\n            Bound::Unbounded => SplayTreeNode::len(self.0),\n       \
    \ };\n        (l, r)\n    }\n}\n\nfn deep_free<M, F>(t: *mut SplayTreeNode<M,\
    \ F>)\nwhere\n    M: Monoid,\n    M::S: Clone,\n    F: Monoid + Act<X = M::S>,\n\
    \    F::S: Clone,\n{\n    if let Some(t) = unsafe { t.as_mut() } {\n        deep_free(t.lch);\n\
    \        deep_free(t.rch);\n        unsafe {\n            std::mem::drop(Box::from_raw(t));\n\
    \        }\n    }\n}\n\nimpl<M, F> Drop for SplayTree<M, F>\nwhere\n    M: Monoid,\n\
    \    M::S: Clone,\n    F: Monoid + Act<X = M::S>,\n    F::S: Clone,\n{\n    fn\
    \ drop(&mut self) {\n        deep_free(self.0)\n    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/data-structure/splay-tree-internal/src/lib.rs
  isVerificationFile: false
  path: crates/data-structure/splay-tree/src/lib.rs
  requiredBy: []
  timestamp: '2024-03-18 01:19:47+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - verify/range_reverse_range_sum/src/main.rs
  - verify/dynamic_sequence_range_affine_range_sum/src/main.rs
documentation_of: crates/data-structure/splay-tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/splay-tree/src/lib.rs
- /library/crates/data-structure/splay-tree/src/lib.rs.html
title: crates/data-structure/splay-tree/src/lib.rs
---
