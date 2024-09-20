---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use algebraic::Monoid;\nuse dual_segment_tree::DualSegmentTree;\nuse graph::Graph;\n\
    use range_contour_query::RangeContourQuery;\n\npub struct VertexGetRangeContourAdd<M>\n\
    where\n    M: Monoid,\n    M::S: Clone,\n{\n    rcq: RangeContourQuery,\n    seg:\
    \ Vec<DualSegmentTree<M>>,\n}\n\nimpl<M> VertexGetRangeContourAdd<M>\nwhere\n\
    \    M: Monoid,\n    M::S: Clone,\n{\n    pub fn new(a: &[M::S], es: &[(usize,\
    \ usize)]) -> Self {\n        let g = Graph::from_unweighted_undirected_edges(a.len(),\
    \ es);\n        let rcq = RangeContourQuery::new(&g);\n        let mut seg = rcq\n\
    \            .seq\n            .iter()\n            .map(|v| DualSegmentTree::<M>::new(v.len()))\n\
    \            .collect::<Vec<_>>();\n        for i in 0..a.len() {\n          \
    \  seg[i].apply(0, a[i].clone());\n        }\n        Self { rcq, seg }\n    }\n\
    \n    pub fn get(&self, v: usize) -> M::S {\n        let mut res = M::e();\n \
    \       for (i, j) in self.rcq.point(v) {\n            let t = self.seg[i].get(j);\n\
    \            res = M::op(&res, &t);\n        }\n        res\n    }\n\n    pub\
    \ fn apply(&mut self, v: usize, x: M::S) {\n        self.apply_range(v, 0, 1,\
    \ x);\n    }\n\n    pub fn apply_range(&mut self, v: usize, l: usize, r: usize,\
    \ x: M::S) {\n        for (i, l, r) in self.rcq.range(v, l, r) {\n           \
    \ self.seg[i].apply_range(l..r, x.clone());\n        }\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/data-structure/vertex-get-range-contour-add/src/lib.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data-structure/vertex-get-range-contour-add/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/vertex-get-range-contour-add/src/lib.rs
- /library/crates/data-structure/vertex-get-range-contour-add/src/lib.rs.html
title: crates/data-structure/vertex-get-range-contour-add/src/lib.rs
---
