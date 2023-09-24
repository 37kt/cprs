---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/heavy-light-decomposition/src/lib.rs
    title: crates/data-structure/heavy-light-decomposition/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/segment-tree/src/lib.rs
    title: crates/data-structure/segment-tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/vertex_add_path_sum/src/main.rs
    title: verify/vertex_add_path_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/vertex_add_subtree_sum/src/main.rs
    title: verify/vertex_add_subtree_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/vertex_set_path_composite/src/main.rs
    title: verify/vertex_set_path_composite/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.5/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.5/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::marker::PhantomData;\n\nuse algebraic::Monoid;\nuse graph::Graph;\n\
    use heavy_light_decomposition::HeavyLightDecomposition;\nuse segment_tree::SegmentTree;\n\
    \npub type TreeQueryVertex<M> = TreeQuery<M, Vertex>;\npub type TreeQueryEdge<M>\
    \ = TreeQuery<M, Edge>;\n\npub trait QueryType {\n    fn vertex() -> bool;\n \
    \   fn edge() -> bool;\n}\n\npub enum Vertex {}\npub enum Edge {}\n\nimpl QueryType\
    \ for Vertex {\n    fn vertex() -> bool {\n        true\n    }\n    fn edge()\
    \ -> bool {\n        false\n    }\n}\n\nimpl QueryType for Edge {\n    fn vertex()\
    \ -> bool {\n        false\n    }\n    fn edge() -> bool {\n        true\n   \
    \ }\n}\n\npub struct TreeQuery<M, Q>\nwhere\n    M: Monoid,\n    M::S: Clone,\n\
    \    Q: QueryType,\n{\n    n: usize,\n    hld: HeavyLightDecomposition,\n    seg_up:\
    \ SegmentTree<M>,\n    seg_down: SegmentTree<M>,\n    _marker: PhantomData<fn()\
    \ -> Q>,\n}\n\nimpl<M, Q> TreeQuery<M, Q>\nwhere\n    M: Monoid,\n    M::S: Clone,\n\
    \    Q: QueryType,\n{\n    pub fn prod_path(&self, u: usize, v: usize) -> M::S\
    \ {\n        let (up, down) = self.hld.path(u, v, Q::edge());\n        let mut\
    \ res = M::e();\n        for &(l, r) in &up {\n            let t = self.seg_up.prod(self.n\
    \ - r..self.n - l);\n            res = M::op(&res, &t);\n        }\n        for\
    \ &(l, r) in &down {\n            let t = self.seg_down.prod(l..r);\n        \
    \    res = M::op(&res, &t);\n        }\n        res\n    }\n\n    pub fn prod_subtree(&self,\
    \ v: usize) -> M::S {\n        let (l, r) = self.hld.subtree(v, Q::edge());\n\
    \        self.seg_down.prod(l..r)\n    }\n}\n\nimpl<V, M> TreeQuery<M, Vertex>\n\
    where\n    V: Clone,\n    M: Monoid<S = V>,\n{\n    pub fn build<E>(g: &Graph<V,\
    \ E>) -> Self\n    where\n        E: Clone,\n    {\n        let n = g.size();\n\
    \        let hld = HeavyLightDecomposition::new(g);\n        let mut a = vec![M::e();\
    \ n];\n        for v in 0..n {\n            let k = hld.vertex(v);\n         \
    \   a[k] = g.vertices()[v].clone();\n        }\n        let seg_down = SegmentTree::from(a.clone());\n\
    \        a.reverse();\n        let seg_up = SegmentTree::from(a);\n        Self\
    \ {\n            n,\n            hld,\n            seg_up,\n            seg_down,\n\
    \            _marker: PhantomData::default(),\n        }\n    }\n\n    pub fn\
    \ set(&mut self, v: usize, x: M::S) {\n        let k = self.hld.vertex(v);\n \
    \       self.seg_up.set(self.n - 1 - k, x.clone());\n        self.seg_down.set(k,\
    \ x);\n    }\n\n    pub fn get(&self, v: usize) -> M::S {\n        let k = self.hld.vertex(v);\n\
    \        self.seg_down.get(k)\n    }\n}\n\nimpl<E, M> TreeQuery<M, Edge>\nwhere\n\
    \    E: Clone,\n    M: Monoid<S = E>,\n{\n    pub fn build<V>(g: &Graph<V, E>)\
    \ -> Self\n    where\n        V: Clone,\n    {\n        let n = g.size();\n  \
    \      let hld = HeavyLightDecomposition::new(g);\n        let mut a = vec![M::e();\
    \ n];\n        for v in 0..n {\n            for (u, w) in g.out_edges(v) {\n \
    \               let k = hld.edge(*u, v);\n                a[k] = w.clone();\n\
    \            }\n        }\n        let seg_down = SegmentTree::from(a.clone());\n\
    \        a.reverse();\n        let seg_up = SegmentTree::from(a);\n        Self\
    \ {\n            n,\n            hld,\n            seg_up,\n            seg_down,\n\
    \            _marker: PhantomData::default(),\n        }\n    }\n\n    pub fn\
    \ set(&mut self, u: usize, v: usize, x: M::S) {\n        let k = self.hld.edge(u,\
    \ v);\n        self.seg_up.set(self.n - 1 - k, x.clone());\n        self.seg_down.set(k,\
    \ x);\n    }\n\n    pub fn get(&self, u: usize, v: usize) -> M::S {\n        let\
    \ k = self.hld.edge(u, v);\n        self.seg_down.get(k)\n    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/data-structure/heavy-light-decomposition/src/lib.rs
  - crates/data-structure/segment-tree/src/lib.rs
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/data-structure/tree-query/src/lib.rs
  requiredBy: []
  timestamp: '2023-05-17 16:30:46+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/vertex_set_path_composite/src/main.rs
  - verify/vertex_add_path_sum/src/main.rs
  - verify/vertex_add_subtree_sum/src/main.rs
documentation_of: crates/data-structure/tree-query/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/tree-query/src/lib.rs
- /library/crates/data-structure/tree-query/src/lib.rs.html
title: crates/data-structure/tree-query/src/lib.rs
---
