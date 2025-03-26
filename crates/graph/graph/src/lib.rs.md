---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/csr_array/src/builder.rs
    title: crates/data_structure/csr_array/src/builder.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/csr_array/src/csr_array.rs
    title: crates/data_structure/csr_array/src/csr_array.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/csr_array/src/lib.rs
    title: crates/data_structure/csr_array/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/builder.rs
    title: crates/graph/graph/src/builder.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/edge.rs
    title: crates/graph/graph/src/edge.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/graph/extended_block_cut_tree/src/lib.rs
    title: crates/graph/extended_block_cut_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/builder.rs
    title: crates/graph/graph/src/builder.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/edge.rs
    title: crates/graph/graph/src/edge.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/strongly_connected_components/src/lib.rs
    title: crates/graph/strongly_connected_components/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/two_edge_connected_components/src/lib.rs
    title: crates/graph/two_edge_connected_components/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/rerooting_tree_dp/src/lib.rs
    title: crates/tree/rerooting_tree_dp/src/lib.rs
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
    path: verify/library_checker/tree/tree_path_composite_sum/src/main.rs
    title: verify/library_checker/tree/tree_path_composite_sum/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: 'pub mod builder;

    pub use builder::*;


    pub mod edge;

    pub use edge::*;

    '
  dependsOn:
  - crates/data_structure/csr_array/src/builder.rs
  - crates/data_structure/csr_array/src/csr_array.rs
  - crates/data_structure/csr_array/src/lib.rs
  - crates/graph/graph/src/builder.rs
  - crates/graph/graph/src/edge.rs
  isVerificationFile: false
  path: crates/graph/graph/src/lib.rs
  requiredBy:
  - crates/tree/rerooting_tree_dp/src/lib.rs
  - crates/graph/two_edge_connected_components/src/lib.rs
  - crates/graph/graph/src/builder.rs
  - crates/graph/graph/src/edge.rs
  - crates/graph/extended_block_cut_tree/src/lib.rs
  - crates/graph/strongly_connected_components/src/lib.rs
  timestamp: '2025-03-06 00:54:38+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/tree/tree_path_composite_sum/src/main.rs
  - verify/library_checker/graph/two_edge_connected_components/src/main.rs
  - verify/library_checker/graph/scc/src/main.rs
  - verify/library_checker/graph/shortest_path/src/main.rs
  - verify/library_checker/graph/biconnected_components/src/main.rs
documentation_of: crates/graph/graph/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/graph/src/lib.rs
- /library/crates/graph/graph/src/lib.rs.html
title: crates/graph/graph/src/lib.rs
---
