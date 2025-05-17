---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/union_find/src/lib.rs
    title: crates/data_structure/union_find/union_find/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/union_find/src/potentialized_union_find.rs
    title: crates/data_structure/union_find/union_find/src/potentialized_union_find.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/union_find/src/union_find_component_sum.rs
    title: crates/data_structure/union_find/union_find/src/union_find_component_sum.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/union_find/src/union_find_impl.rs
    title: crates/data_structure/union_find/union_find/src/union_find_impl.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/union_find/src/lib.rs
    title: crates/data_structure/union_find/union_find/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/union_find/src/potentialized_union_find.rs
    title: crates/data_structure/union_find/union_find/src/potentialized_union_find.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/union_find/src/union_find_component_sum.rs
    title: crates/data_structure/union_find/union_find/src/union_find_component_sum.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/union_find/src/union_find_impl.rs
    title: crates/data_structure/union_find/union_find/src/union_find_impl.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/unionfind/src/main.rs
    title: verify/library_checker/data_structure/unionfind/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/unionfind_with_potential/src/main.rs
    title: verify/library_checker/data_structure/unionfind_with_potential/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/unionfind_with_potential_non_commutative_group/src/main.rs
    title: verify/library_checker/data_structure/unionfind_with_potential_non_commutative_group/src/main.rs
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
  code: "use algebraic_structure::magma::TrivialGroup;\n\nuse crate::union_find_impl::UnionFindImpl;\n\
    \npub type UnionFind = UnionFindBase<false>;\npub type UndoableUnionFind = UnionFindBase<true>;\n\
    \n#[derive(Clone)]\npub struct UnionFindBase<const UNDOABLE: bool> {\n    inner:\
    \ UnionFindImpl<TrivialGroup, TrivialGroup, UNDOABLE>,\n}\n\nimpl<const UNDOABLE:\
    \ bool> UnionFindBase<UNDOABLE> {\n    pub fn new(n: usize) -> Self {\n      \
    \  Self {\n            inner: UnionFindImpl::new(n),\n        }\n    }\n\n   \
    \ pub fn len(&self) -> usize {\n        self.inner.len()\n    }\n\n    pub fn\
    \ is_empty(&self) -> bool {\n        self.inner.is_empty()\n    }\n\n    pub fn\
    \ merge_with(&mut self, x: usize, y: usize, f: impl FnMut(usize, usize)) -> bool\
    \ {\n        self.inner.merge_with(x, y, (), f)\n    }\n\n    pub fn merge(&mut\
    \ self, x: usize, y: usize) -> bool {\n        self.merge_with(x, y, |_, _| {})\n\
    \    }\n\n    pub fn root(&mut self, x: usize) -> usize {\n        self.inner.root(x)\n\
    \    }\n\n    pub fn same(&mut self, x: usize, y: usize) -> bool {\n        self.inner.same(x,\
    \ y)\n    }\n\n    pub fn size(&mut self, x: usize) -> usize {\n        self.inner.size(x)\n\
    \    }\n}\n\nimpl UnionFindBase<true> {\n    pub fn undo(&mut self) {\n      \
    \  self.inner.undo();\n    }\n}\n"
  dependsOn:
  - crates/data_structure/union_find/union_find/src/lib.rs
  - crates/data_structure/union_find/union_find/src/potentialized_union_find.rs
  - crates/data_structure/union_find/union_find/src/union_find_component_sum.rs
  - crates/data_structure/union_find/union_find/src/union_find_impl.rs
  isVerificationFile: false
  path: crates/data_structure/union_find/union_find/src/union_find.rs
  requiredBy:
  - crates/data_structure/union_find/union_find/src/lib.rs
  - crates/data_structure/union_find/union_find/src/union_find_component_sum.rs
  - crates/data_structure/union_find/union_find/src/potentialized_union_find.rs
  - crates/data_structure/union_find/union_find/src/union_find_impl.rs
  timestamp: '2025-03-07 01:17:39+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/unionfind/src/main.rs
  - verify/library_checker/data_structure/unionfind_with_potential/src/main.rs
  - verify/library_checker/data_structure/unionfind_with_potential_non_commutative_group/src/main.rs
documentation_of: crates/data_structure/union_find/union_find/src/union_find.rs
layout: document
redirect_from:
- /library/crates/data_structure/union_find/union_find/src/union_find.rs
- /library/crates/data_structure/union_find/union_find/src/union_find.rs.html
title: crates/data_structure/union_find/union_find/src/union_find.rs
---
