---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/csr_array/src/csr_array.rs
    title: crates/data_structure/csr_array/src/csr_array.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/csr_array/src/lib.rs
    title: crates/data_structure/csr_array/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/csr_array/src/csr_array.rs
    title: crates/data_structure/csr_array/src/csr_array.rs
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
  code: "use crate::CsrArray;\n\n#[derive(Clone)]\npub struct CsrArrayBuilder<T> {\n\
    \    pub(crate) n: usize,\n    pub(crate) idx: Vec<usize>,\n    pub(crate) val:\
    \ Vec<T>,\n}\n\nimpl<T> CsrArrayBuilder<T> {\n    pub fn new(n: usize) -> Self\
    \ {\n        Self {\n            n,\n            idx: vec![],\n            val:\
    \ vec![],\n        }\n    }\n\n    pub fn with_capacity(n: usize, capacity: usize)\
    \ -> Self {\n        Self {\n            n,\n            idx: Vec::with_capacity(capacity),\n\
    \            val: Vec::with_capacity(capacity),\n        }\n    }\n\n    pub fn\
    \ len(&self) -> usize {\n        self.n\n    }\n\n    pub fn is_empty(&self) ->\
    \ bool {\n        self.n == 0\n    }\n\n    pub fn push(&mut self, i: usize, x:\
    \ T) {\n        self.idx.push(i);\n        self.val.push(x);\n    }\n\n    pub\
    \ fn build(mut self) -> CsrArray<T> {\n        let mut sep = vec![0; self.n +\
    \ 1];\n        for &i in &self.idx {\n            sep[i] += 1;\n        }\n  \
    \      for i in 0..self.n {\n            sep[i + 1] += sep[i];\n        }\n  \
    \      let mut ord = vec![0; self.idx.len()];\n        for (j, &i) in self.idx.iter().enumerate().rev()\
    \ {\n            sep[i] -= 1;\n            ord[j] = sep[i];\n        }\n     \
    \   for i in 0..self.idx.len() {\n            while ord[i] != i {\n          \
    \      let j = ord[i];\n                ord.swap(i, j);\n                self.val.swap(i,\
    \ j);\n            }\n        }\n        CsrArray { sep, val: self.val }\n   \
    \ }\n}\n"
  dependsOn:
  - crates/data_structure/csr_array/src/csr_array.rs
  - crates/data_structure/csr_array/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/csr_array/src/builder.rs
  requiredBy:
  - crates/tree/dynamic_rerooting_tree_dp/src/lib.rs
  - crates/tree/dynamic_tree_dp/src/lib.rs
  - crates/tree/centroid_decomposition/src/lib.rs
  - crates/tree/heavy_light_decomposition/src/lib.rs
  - crates/tree/tree_contour_range/src/lib.rs
  - crates/tree/static_top_tree/src/lib.rs
  - crates/tree/rerooting_tree_dp/src/lib.rs
  - crates/graph/two_edge_connected_components/src/lib.rs
  - crates/graph/graph/src/lib.rs
  - crates/graph/extended_block_cut_tree/src/lib.rs
  - crates/graph/strongly_connected_components/src/lib.rs
  - crates/graph/dijkstra/src/lib.rs
  - crates/data_structure/csr_array/src/csr_array.rs
  - crates/data_structure/csr_array/src/lib.rs
  timestamp: '2025-03-02 23:45:27+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/graph/shortest_path/src/main.rs
documentation_of: crates/data_structure/csr_array/src/builder.rs
layout: document
redirect_from:
- /library/crates/data_structure/csr_array/src/builder.rs
- /library/crates/data_structure/csr_array/src/builder.rs.html
title: crates/data_structure/csr_array/src/builder.rs
---
