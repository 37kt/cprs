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
  code: '// use centroid_decomposition::CentroidDecomposition;

    // use csr_array::CsrArray;

    // use graph::{Edge, GraphBuilder, UndirectedGraph};


    // #[derive(Clone)]

    // pub struct TreeContourRange {

    //     comp: Vec<usize>,

    //     depth: Vec<usize>,

    // }


    // impl TreeContourRange {

    //     pub fn new<W>(g: &CsrArray<impl Edge<W>>) -> Self {

    //         todo!()

    //     }


    //     pub fn len(&self) -> usize {

    //         0

    //     }


    //     pub fn is_empty(&self) -> bool {

    //         self.len() == 0

    //     }

    // }


    // #[test]

    // fn test() {

    //     use rand::Rng;

    //     let mut rng = rand::thread_rng();


    //     let mut edges = vec![];

    //     let n = 200000;

    //     for i in 1..n {

    //         let p = rng.gen_range(0..i);

    //         edges.push((p, i));

    //     }

    //     // for i in 0..n / 2 {

    //     //     edges.push((i, i + n / 2));

    //     //     edges.push((i + n / 2, n));

    //     // }

    //     let n = n + 1;

    //     let g = UndirectedGraph::from_edges(n, edges);

    //     let mut cnt = vec![0; n];

    //     let mut weight = vec![1; n];

    //     CentroidDecomposition::solve(&g, |cd| {

    //         cnt[cd.root] += weight[cd.root];

    //         for &v in cd.vs {

    //             cnt[v] += weight[v];

    //         }

    //         weight[cd.root] = 0;

    //     });


    //     cnt.sort_unstable();

    //     eprintln!("cnt: {:?}", &cnt[..100]);

    //     cnt.reverse();

    //     eprintln!("cnt: {:?}", &cnt[..100]);

    // }

    '
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
  timestamp: '2025-04-07 08:03:10+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/tree/tree_contour_range/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/tree_contour_range/src/lib.rs
- /library/crates/tree/tree_contour_range/src/lib.rs.html
title: crates/tree/tree_contour_range/src/lib.rs
---
