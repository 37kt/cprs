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
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/centroid_decomposition/src/lib.rs
    title: crates/tree/centroid_decomposition/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use centroid_decomposition::CentroidDecomposition;\nuse csr_array::CsrArray;\n\
    use graph::{Edge, GraphBuilder, UndirectedGraph};\n\n#[derive(Clone)]\npub struct\
    \ TreeContourRange {\n    comp: Vec<usize>,\n    depth: Vec<usize>,\n}\n\nimpl\
    \ TreeContourRange {\n    pub fn new<W>(g: &CsrArray<impl Edge<W>>) -> Self {\n\
    \        todo!()\n    }\n\n    pub fn len(&self) -> usize {\n        0\n    }\n\
    \n    pub fn is_empty(&self) -> bool {\n        self.len() == 0\n    }\n}\n\n\
    #[test]\nfn test() {\n    use rand::Rng;\n    let mut rng = rand::thread_rng();\n\
    \n    let mut edges = vec![];\n    let n = 200000;\n    for i in 1..n {\n    \
    \    let p = rng.gen_range(0..i);\n        edges.push((p, i));\n    }\n    //\
    \ for i in 0..n / 2 {\n    //     edges.push((i, i + n / 2));\n    //     edges.push((i\
    \ + n / 2, n));\n    // }\n    let n = n + 1;\n    let g = UndirectedGraph::from_edges(n,\
    \ edges);\n    let mut cnt = vec![0; n];\n    let mut weight = vec![1; n];\n \
    \   CentroidDecomposition::solve(&g, |cd| {\n        cnt[cd.root] += weight[cd.root];\n\
    \        for &v in cd.vs {\n            cnt[v] += weight[v];\n        }\n    \
    \    weight[cd.root] = 0;\n    });\n\n    cnt.sort_unstable();\n    eprintln!(\"\
    cnt: {:?}\", &cnt[..100]);\n    cnt.reverse();\n    eprintln!(\"cnt: {:?}\", &cnt[..100]);\n\
    }\n"
  dependsOn:
  - crates/data_structure/csr_array/src/builder.rs
  - crates/data_structure/csr_array/src/csr_array.rs
  - crates/data_structure/csr_array/src/lib.rs
  - crates/graph/graph/src/builder.rs
  - crates/graph/graph/src/edge.rs
  - crates/graph/graph/src/lib.rs
  - crates/tree/centroid_decomposition/src/lib.rs
  isVerificationFile: false
  path: crates/tree/tree_contour_range/src/lib.rs
  requiredBy: []
  timestamp: '2025-03-29 09:22:56+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/tree/tree_contour_range/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/tree_contour_range/src/lib.rs
- /library/crates/tree/tree_contour_range/src/lib.rs.html
title: crates/tree/tree_contour_range/src/lib.rs
---
