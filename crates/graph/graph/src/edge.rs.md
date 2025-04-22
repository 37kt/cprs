---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/builder.rs
    title: crates/graph/graph/src/builder.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/graph/extended_block_cut_tree/src/lib.rs
    title: crates/graph/extended_block_cut_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/builder.rs
    title: crates/graph/graph/src/builder.rs
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
    path: crates/tree/rerooting_tree_dp/src/lib.rs
    title: crates/tree/rerooting_tree_dp/src/lib.rs
  - icon: ':warning:'
    path: crates/tree/tree_contour_range/src/lib.rs
    title: crates/tree/tree_contour_range/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/graph/biconnected_components/src/main.rs
    title: verify/library_checker/graph/biconnected_components/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/graph/scc/src/main.rs
    title: verify/library_checker/graph/scc/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/graph/shortest_path/src/main.rs
    title: verify/library_checker/graph/shortest_path/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/graph/two_edge_connected_components/src/main.rs
    title: verify/library_checker/graph/two_edge_connected_components/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/frequency_table_of_tree_distance/src/main.rs
    title: verify/library_checker/tree/frequency_table_of_tree_distance/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/tree_path_composite_sum/src/main.rs
    title: verify/library_checker/tree/tree_path_composite_sum/src/main.rs
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
  code: "pub trait Edge<Weight> {\n    fn to(&self) -> usize;\n    fn weight(&self)\
    \ -> &Weight;\n}\n\nimpl Edge<()> for usize {\n    fn to(&self) -> usize {\n \
    \       *self\n    }\n\n    fn weight(&self) -> &() {\n        &()\n    }\n}\n\
    \nimpl<Weight> Edge<Weight> for (usize, Weight) {\n    fn to(&self) -> usize {\n\
    \        self.0\n    }\n\n    fn weight(&self) -> &Weight {\n        &self.1\n\
    \    }\n}\n"
  dependsOn:
  - crates/graph/graph/src/builder.rs
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/graph/graph/src/edge.rs
  requiredBy:
  - crates/tree/centroid_decomposition/src/lib.rs
  - crates/tree/tree_contour_range/src/lib.rs
  - crates/tree/rerooting_tree_dp/src/lib.rs
  - crates/graph/two_edge_connected_components/src/lib.rs
  - crates/graph/graph/src/lib.rs
  - crates/graph/graph/src/builder.rs
  - crates/graph/extended_block_cut_tree/src/lib.rs
  - crates/graph/strongly_connected_components/src/lib.rs
  timestamp: '2025-03-06 00:54:38+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/tree/tree_path_composite_sum/src/main.rs
  - verify/library_checker/tree/frequency_table_of_tree_distance/src/main.rs
  - verify/library_checker/graph/two_edge_connected_components/src/main.rs
  - verify/library_checker/graph/scc/src/main.rs
  - verify/library_checker/graph/shortest_path/src/main.rs
  - verify/library_checker/graph/biconnected_components/src/main.rs
documentation_of: crates/graph/graph/src/edge.rs
layout: document
redirect_from:
- /library/crates/graph/graph/src/edge.rs
- /library/crates/graph/graph/src/edge.rs.html
title: crates/graph/graph/src/edge.rs
---
