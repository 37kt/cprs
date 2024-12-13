---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  - icon: ':warning:'
    path: crates/data-structure/range-contour-query/src/lib.rs
    title: crates/data-structure/range-contour-query/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/segment-tree/src/lib.rs
    title: crates/data-structure/segment-tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/vertex_add_range_contour_sum_on_tree/src/main.rs
    title: verify/vertex_add_range_contour_sum_on_tree/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use algebraic::Monoid;\nuse graph::Graph;\nuse range_contour_query::RangeContourQuery;\n\
    use segment_tree::SegmentTree;\n\npub struct VertexAddRangeContourSum<M>\nwhere\n\
    \    M: Monoid,\n    M::S: Clone,\n{\n    rcq: RangeContourQuery,\n    seg: Vec<SegmentTree<M>>,\n\
    }\n\nimpl<M> VertexAddRangeContourSum<M>\nwhere\n    M: Monoid,\n    M::S: Clone,\n\
    {\n    pub fn new(a: &[M::S], es: &[(usize, usize)]) -> Self {\n        let g\
    \ = Graph::from_unweighted_undirected_edges(a.len(), es);\n        let rcq = RangeContourQuery::new(&g);\n\
    \        let seg = rcq\n            .seq\n            .iter()\n            .map(|v|\
    \ SegmentTree::<M>::from(v.iter().map(|&i| a[i].clone()).collect::<Vec<_>>()))\n\
    \            .collect::<Vec<_>>();\n        Self { rcq, seg }\n    }\n\n    pub\
    \ fn set(&mut self, v: usize, x: M::S) {\n        for (i, j) in self.rcq.point(v)\
    \ {\n            self.seg[i].set(j, x.clone());\n        }\n    }\n\n    pub fn\
    \ add(&mut self, v: usize, x: M::S) {\n        for (i, j) in self.rcq.point(v)\
    \ {\n            let t = self.seg[i].get(j);\n            self.seg[i].set(j, M::op(&t,\
    \ &x));\n        }\n    }\n\n    pub fn get(&self, v: usize) -> M::S {\n     \
    \   self.seg[v].get(0)\n    }\n\n    pub fn prod(&self, v: usize, l: usize, r:\
    \ usize) -> M::S {\n        let mut res = M::e();\n        for (i, l, r) in self.rcq.range(v,\
    \ l, r) {\n            let t = self.seg[i].prod(l..r);\n            res = M::op(&res,\
    \ &t);\n        }\n        res\n    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/data-structure/range-contour-query/src/lib.rs
  - crates/data-structure/segment-tree/src/lib.rs
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/data-structure/vertex-add-range-contour-sum/src/lib.rs
  requiredBy: []
  timestamp: '2024-06-28 10:31:31+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/vertex_add_range_contour_sum_on_tree/src/main.rs
documentation_of: crates/data-structure/vertex-add-range-contour-sum/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/vertex-add-range-contour-sum/src/lib.rs
- /library/crates/data-structure/vertex-add-range-contour-sum/src/lib.rs.html
title: crates/data-structure/vertex-add-range-contour-sum/src/lib.rs
---
