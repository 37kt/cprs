---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/allocator.rs
    title: crates/data_structure/splay_tree/src/allocator.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/lib.rs
    title: crates/data_structure/splay_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/node.rs
    title: crates/data_structure/splay_tree/src/node.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/operator.rs
    title: crates/data_structure/splay_tree/src/operator.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/tree.rs
    title: crates/data_structure/splay_tree/src/tree.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/allocator.rs
    title: crates/data_structure/splay_tree/src/allocator.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/lib.rs
    title: crates/data_structure/splay_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/node.rs
    title: crates/data_structure/splay_tree/src/node.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/operator.rs
    title: crates/data_structure/splay_tree/src/operator.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/tree.rs
    title: crates/data_structure/splay_tree/src/tree.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/dynamic_sequence_range_affine_range_sum/src/main.rs
    title: verify/library_checker/data_structure/dynamic_sequence_range_affine_range_sum/src/main.rs
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
  code: "use crate::{operator::Operator, tree::Tree, Actable, Foldable};\n\npub struct\
    \ SplaySequence<O>\nwhere\n    O: Operator,\n{\n    tree: Tree<O>,\n}\n\nimpl<O>\
    \ SplaySequence<O>\nwhere\n    O: Operator,\n    O::X: Default,\n{\n    pub fn\
    \ new(n: usize) -> Self {\n        Self { tree: Tree::new(n) }\n    }\n}\n\nimpl<O>\
    \ Default for SplaySequence<O>\nwhere\n    O: Operator,\n    O::X: Default,\n\
    {\n    fn default() -> Self {\n        Self::new(0)\n    }\n}\n\nimpl<O> SplaySequence<O>\n\
    where\n    O: Operator,\n{\n    pub fn from_fn(n: usize, f: impl FnMut(usize)\
    \ -> O::X) -> Self {\n        Self {\n            tree: Tree::from_fn(n, f),\n\
    \        }\n    }\n\n    pub fn len(&self) -> usize {\n        self.tree.len()\n\
    \    }\n\n    pub fn is_empty(&self) -> bool {\n        self.tree.is_empty()\n\
    \    }\n\n    pub fn get(&mut self, i: usize) -> Option<&O::X> {\n        self.tree.get(i)\n\
    \    }\n\n    pub fn set(&mut self, i: usize, x: O::X) {\n        self.tree.set(i,\
    \ x);\n    }\n\n    pub fn insert(&mut self, i: usize, x: O::X) {\n        self.tree.insert(i,\
    \ x);\n    }\n\n    pub fn remove(&mut self, i: usize) -> O::X {\n        self.tree.remove(i)\n\
    \    }\n\n    pub fn reverse(&mut self, range: impl std::ops::RangeBounds<usize>)\
    \ {\n        self.tree.reverse(range);\n    }\n}\n\nimpl<O> SplaySequence<O>\n\
    where\n    O: Operator + Foldable,\n{\n    pub fn fold(&mut self, range: impl\
    \ std::ops::RangeBounds<usize>) -> O::P {\n        self.tree.fold(range)\n   \
    \ }\n}\n\nimpl<O> SplaySequence<O>\nwhere\n    O: Operator + Actable,\n{\n   \
    \ pub fn apply(&mut self, range: impl std::ops::RangeBounds<usize>, f: O::F) {\n\
    \        self.tree.apply(range, f);\n    }\n}\n"
  dependsOn:
  - crates/data_structure/splay_tree/src/allocator.rs
  - crates/data_structure/splay_tree/src/lib.rs
  - crates/data_structure/splay_tree/src/node.rs
  - crates/data_structure/splay_tree/src/operator.rs
  - crates/data_structure/splay_tree/src/tree.rs
  isVerificationFile: false
  path: crates/data_structure/splay_tree/src/sequence.rs
  requiredBy:
  - crates/data_structure/splay_tree/src/lib.rs
  - crates/data_structure/splay_tree/src/operator.rs
  - crates/data_structure/splay_tree/src/allocator.rs
  - crates/data_structure/splay_tree/src/tree.rs
  - crates/data_structure/splay_tree/src/node.rs
  timestamp: '2025-04-09 07:52:13+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/dynamic_sequence_range_affine_range_sum/src/main.rs
documentation_of: crates/data_structure/splay_tree/src/sequence.rs
layout: document
redirect_from:
- /library/crates/data_structure/splay_tree/src/sequence.rs
- /library/crates/data_structure/splay_tree/src/sequence.rs.html
title: crates/data_structure/splay_tree/src/sequence.rs
---
