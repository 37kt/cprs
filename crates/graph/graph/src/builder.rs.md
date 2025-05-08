---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/edge.rs
    title: crates/graph/graph/src/edge.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/graph/extended_block_cut_tree/src/lib.rs
    title: crates/graph/extended_block_cut_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/edge.rs
    title: crates/graph/graph/src/edge.rs
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
  - icon: ':heavy_check_mark:'
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
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/vertex_add_range_contour_sum_on_tree/src/main.rs
    title: verify/library_checker/tree/vertex_add_range_contour_sum_on_tree/src/main.rs
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
  code: "use csr_array::CsrArray;\n\npub trait GraphBuilder<E, G> {\n    fn from_edges(n:\
    \ usize, edges: impl IntoIterator<Item = E>) -> G;\n}\n\npub enum DirectedGraph\
    \ {}\n\npub enum UndirectedGraph {}\n\nimpl<T> GraphBuilder<(usize, usize, T),\
    \ CsrArray<(usize, T)>> for DirectedGraph {\n    fn from_edges(\n        n: usize,\n\
    \        edges: impl IntoIterator<Item = (usize, usize, T)>,\n    ) -> CsrArray<(usize,\
    \ T)> {\n        CsrArray::new(n, edges.into_iter().map(|(a, b, c)| (a, (b, c))))\n\
    \    }\n}\n\nimpl<'a, T> GraphBuilder<&'a (usize, usize, T), CsrArray<(usize,\
    \ T)>> for DirectedGraph\nwhere\n    T: 'a + Clone,\n{\n    fn from_edges(\n \
    \       n: usize,\n        edges: impl IntoIterator<Item = &'a (usize, usize,\
    \ T)>,\n    ) -> CsrArray<(usize, T)> {\n        CsrArray::new(n, edges.into_iter().cloned().map(|(a,\
    \ b, c)| (a, (b, c))))\n    }\n}\n\nimpl GraphBuilder<(usize, usize), CsrArray<usize>>\
    \ for DirectedGraph {\n    fn from_edges(n: usize, edges: impl IntoIterator<Item\
    \ = (usize, usize)>) -> CsrArray<usize> {\n        CsrArray::new(n, edges.into_iter())\n\
    \    }\n}\n\nimpl<'a> GraphBuilder<&'a (usize, usize), CsrArray<usize>> for DirectedGraph\
    \ {\n    fn from_edges(\n        n: usize,\n        edges: impl IntoIterator<Item\
    \ = &'a (usize, usize)>,\n    ) -> CsrArray<usize> {\n        CsrArray::new(n,\
    \ edges.into_iter().cloned())\n    }\n}\n\nimpl<T> GraphBuilder<(usize, usize,\
    \ T), CsrArray<(usize, T)>> for UndirectedGraph\nwhere\n    T: Clone,\n{\n   \
    \ fn from_edges(\n        n: usize,\n        edges: impl IntoIterator<Item = (usize,\
    \ usize, T)>,\n    ) -> CsrArray<(usize, T)> {\n        CsrArray::new(\n     \
    \       n,\n            edges\n                .into_iter()\n                .flat_map(|(a,\
    \ b, c)| [(a, (b, c.clone())), (b, (a, c))]),\n        )\n    }\n}\n\nimpl<'a,\
    \ T> GraphBuilder<&'a (usize, usize, T), CsrArray<(usize, T)>> for UndirectedGraph\n\
    where\n    T: 'a + Clone,\n{\n    fn from_edges(\n        n: usize,\n        edges:\
    \ impl IntoIterator<Item = &'a (usize, usize, T)>,\n    ) -> CsrArray<(usize,\
    \ T)> {\n        CsrArray::new(\n            n,\n            edges\n         \
    \       .into_iter()\n                .cloned()\n                .flat_map(|(a,\
    \ b, c)| [(a, (b, c.clone())), (b, (a, c))]),\n        )\n    }\n}\n\nimpl GraphBuilder<(usize,\
    \ usize), CsrArray<usize>> for UndirectedGraph {\n    fn from_edges(n: usize,\
    \ edges: impl IntoIterator<Item = (usize, usize)>) -> CsrArray<usize> {\n    \
    \    CsrArray::new(n, edges.into_iter().flat_map(|(a, b)| [(a, b), (b, a)]))\n\
    \    }\n}\n\nimpl<'a> GraphBuilder<&'a (usize, usize), CsrArray<usize>> for UndirectedGraph\
    \ {\n    fn from_edges(\n        n: usize,\n        edges: impl IntoIterator<Item\
    \ = &'a (usize, usize)>,\n    ) -> CsrArray<usize> {\n        CsrArray::new(n,\
    \ edges.into_iter().flat_map(|&(a, b)| [(a, b), (b, a)]))\n    }\n}\n"
  dependsOn:
  - crates/graph/graph/src/edge.rs
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/graph/graph/src/builder.rs
  requiredBy:
  - crates/tree/tree_contour_range/src/lib.rs
  - crates/tree/rerooting_tree_dp/src/lib.rs
  - crates/tree/centroid_decomposition/src/lib.rs
  - crates/graph/strongly_connected_components/src/lib.rs
  - crates/graph/two_edge_connected_components/src/lib.rs
  - crates/graph/extended_block_cut_tree/src/lib.rs
  - crates/graph/graph/src/lib.rs
  - crates/graph/graph/src/edge.rs
  timestamp: '2025-03-06 00:54:38+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/tree/frequency_table_of_tree_distance/src/main.rs
  - verify/library_checker/tree/tree_path_composite_sum/src/main.rs
  - verify/library_checker/tree/vertex_add_range_contour_sum_on_tree/src/main.rs
  - verify/library_checker/graph/two_edge_connected_components/src/main.rs
  - verify/library_checker/graph/biconnected_components/src/main.rs
  - verify/library_checker/graph/shortest_path/src/main.rs
  - verify/library_checker/graph/scc/src/main.rs
documentation_of: crates/graph/graph/src/builder.rs
layout: document
redirect_from:
- /library/crates/graph/graph/src/builder.rs
- /library/crates/graph/graph/src/builder.rs.html
title: crates/graph/graph/src/builder.rs
---
