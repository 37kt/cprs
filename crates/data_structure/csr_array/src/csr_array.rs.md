---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/csr_array/src/builder.rs
    title: crates/data_structure/csr_array/src/builder.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/csr_array/src/lib.rs
    title: crates/data_structure/csr_array/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/csr_array/src/builder.rs
    title: crates/data_structure/csr_array/src/builder.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/csr_array/src/lib.rs
    title: crates/data_structure/csr_array/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/dijkstra/src/lib.rs
    title: crates/graph/dijkstra/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/extended_block_cut_tree/src/lib.rs
    title: crates/graph/extended_block_cut_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/strongly_connected_components/src/lib.rs
    title: crates/graph/strongly_connected_components/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/two_edge_connected_components/src/lib.rs
    title: crates/graph/two_edge_connected_components/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/centroid_decomposition/src/lib.rs
    title: crates/tree/centroid_decomposition/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/dynamic_rerooting_tree_dp/src/lib.rs
    title: crates/tree/dynamic_rerooting_tree_dp/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/dynamic_tree_dp/src/lib.rs
    title: crates/tree/dynamic_tree_dp/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy_light_decomposition/src/lib.rs
    title: crates/tree/heavy_light_decomposition/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/rerooting_tree_dp/src/lib.rs
    title: crates/tree/rerooting_tree_dp/src/lib.rs
  - icon: ':warning:'
    path: crates/tree/static_top_tree/src/lib.rs
    title: crates/tree/static_top_tree/src/lib.rs
  - icon: ':warning:'
    path: crates/tree/tree_contour_range/src/lib.rs
    title: crates/tree/tree_contour_range/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/graph/shortest_path/src/main.rs
    title: verify/library_checker/graph/shortest_path/src/main.rs
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
  code: "use std::{iter::FusedIterator, ops::Index};\n\nuse crate::CsrArrayBuilder;\n\
    \nimpl<T> CsrArray<T> {\n    pub fn new(n: usize, idx_val: impl IntoIterator<Item\
    \ = (usize, T)>) -> Self {\n        let (idx, val) = idx_val.into_iter().unzip();\n\
    \        let builder = CsrArrayBuilder { n, idx, val };\n        builder.build()\n\
    \    }\n\n    pub fn len(&self) -> usize {\n        self.sep.len() - 1\n    }\n\
    \n    pub fn is_empty(&self) -> bool {\n        self.sep.len() == 1\n    }\n\n\
    \    pub fn flat_len(&self) -> usize {\n        self.val.len()\n    }\n\n    pub\
    \ fn iter(&self) -> Iter<'_, T> {\n        Iter {\n            sep: &self.sep,\n\
    \            val: &self.val,\n        }\n    }\n}\n\npub struct Iter<'a, T> {\n\
    \    sep: &'a [usize],\n    val: &'a [T],\n}\n\nimpl<'a, T> Iterator for Iter<'a,\
    \ T> {\n    type Item = &'a [T];\n\n    fn next(&mut self) -> Option<Self::Item>\
    \ {\n        let &[l, r, ..] = self.sep else {\n            return None;\n   \
    \     };\n        let len = r - l;\n        let (head, tail) = self.val.split_at(len);\n\
    \        self.sep = &self.sep[1..];\n        self.val = tail;\n        Some(head)\n\
    \    }\n}\n\nimpl<'a, T> DoubleEndedIterator for Iter<'a, T> {\n    fn next_back(&mut\
    \ self) -> Option<Self::Item> {\n        let &[.., l, r] = self.sep else {\n \
    \           return None;\n        };\n        let len = r - l;\n        let (head,\
    \ tail) = self.val.split_at(self.val.len() - len);\n        self.sep = &self.sep[..self.sep.len()\
    \ - 1];\n        self.val = head;\n        Some(tail)\n    }\n}\n\nimpl<'a, T>\
    \ ExactSizeIterator for Iter<'a, T> {\n    fn len(&self) -> usize {\n        self.sep.len()\
    \ - 1\n    }\n}\n\nimpl<'a, T> FusedIterator for Iter<'a, T> {}\n\nimpl<'a, T>\
    \ IntoIterator for &'a CsrArray<T> {\n    type Item = &'a [T];\n    type IntoIter\
    \ = Iter<'a, T>;\n\n    fn into_iter(self) -> Self::IntoIter {\n        self.iter()\n\
    \    }\n}\n\nimpl<T> Index<usize> for CsrArray<T> {\n    type Output = [T];\n\n\
    \    fn index(&self, index: usize) -> &Self::Output {\n        assert!(index <\
    \ self.len());\n        &self.val[self.sep[index]..self.sep[index + 1]]\n    }\n\
    }\n\nimpl<T: std::fmt::Debug> std::fmt::Debug for CsrArray<T> {\n    fn fmt(&self,\
    \ f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n        write!(f, \"\
    [\")?;\n        for (i, v) in self.iter().enumerate() {\n            write!(f,\
    \ \"{:?}\", v)?;\n            if i < self.len() - 1 {\n                write!(f,\
    \ \", \")?;\n            }\n        }\n        write!(f, \"]\")\n    }\n}\n\n\
    #[derive(Clone)]\npub struct CsrArray<T> {\n    pub(crate) sep: Vec<usize>,\n\
    \    pub(crate) val: Vec<T>,\n}\n"
  dependsOn:
  - crates/data_structure/csr_array/src/builder.rs
  - crates/data_structure/csr_array/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/csr_array/src/csr_array.rs
  requiredBy:
  - crates/tree/static_top_tree/src/lib.rs
  - crates/tree/rerooting_tree_dp/src/lib.rs
  - crates/tree/dynamic_rerooting_tree_dp/src/lib.rs
  - crates/tree/heavy_light_decomposition/src/lib.rs
  - crates/tree/tree_contour_range/src/lib.rs
  - crates/tree/centroid_decomposition/src/lib.rs
  - crates/tree/dynamic_tree_dp/src/lib.rs
  - crates/graph/extended_block_cut_tree/src/lib.rs
  - crates/graph/two_edge_connected_components/src/lib.rs
  - crates/graph/graph/src/lib.rs
  - crates/graph/strongly_connected_components/src/lib.rs
  - crates/graph/dijkstra/src/lib.rs
  - crates/data_structure/csr_array/src/lib.rs
  - crates/data_structure/csr_array/src/builder.rs
  timestamp: '2025-03-02 23:45:27+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/graph/shortest_path/src/main.rs
documentation_of: crates/data_structure/csr_array/src/csr_array.rs
layout: document
redirect_from:
- /library/crates/data_structure/csr_array/src/csr_array.rs
- /library/crates/data_structure/csr_array/src/csr_array.rs.html
title: crates/data_structure/csr_array/src/csr_array.rs
---
