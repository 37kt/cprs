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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::marker::PhantomData;\n\nuse algebraic::Monoid;\nuse graph::Graph;\n\
    use heavy_light_decomposition::HeavyLightDecomposition;\nuse segment_tree::SegmentTree;\n\
    \n/// \u9802\u70B9\u30AF\u30A8\u30EA\u3092\u51E6\u7406\u3059\u308B\u305F\u3081\
    \u306E\u30C7\u30FC\u30BF\u69CB\u9020\u3002\npub type TreeQueryVertex<M> = TreeQuery<M,\
    \ Vertex>;\n\n/// \u8FBA\u30AF\u30A8\u30EA\u3092\u51E6\u7406\u3059\u308B\u305F\
    \u3081\u306E\u30C7\u30FC\u30BF\u69CB\u9020\u3002\npub type TreeQueryEdge<M> =\
    \ TreeQuery<M, Edge>;\n\npub trait QueryType {\n    fn vertex() -> bool;\n   \
    \ fn edge() -> bool;\n}\n\npub enum Vertex {}\npub enum Edge {}\n\nimpl QueryType\
    \ for Vertex {\n    fn vertex() -> bool {\n        true\n    }\n    fn edge()\
    \ -> bool {\n        false\n    }\n}\n\nimpl QueryType for Edge {\n    fn vertex()\
    \ -> bool {\n        false\n    }\n    fn edge() -> bool {\n        true\n   \
    \ }\n}\n\npub struct TreeQuery<M, Q>\nwhere\n    M: Monoid,\n    M::S: Clone,\n\
    \    Q: QueryType,\n{\n    n: usize,\n    hld: HeavyLightDecomposition,\n    seg_up:\
    \ SegmentTree<M>,\n    seg_down: SegmentTree<M>,\n    _marker: PhantomData<fn()\
    \ -> Q>,\n}\n\nimpl<M, Q> TreeQuery<M, Q>\nwhere\n    M: Monoid,\n    M::S: Clone,\n\
    \    Q: QueryType,\n{\n    /// \u9802\u70B9 u \u304B\u3089\u9802\u70B9 v \u306E\
    \u30D1\u30B9\u306B\u5BFE\u3059\u308B\u30AF\u30A8\u30EA\u3092\u51E6\u7406\u3059\
    \u308B\u3002\n    ///\n    /// # \u5F15\u6570\n    ///\n    /// - `u`: \u9802\u70B9\
    \ u\n    /// - `v`: \u9802\u70B9 v\n    ///\n    /// # \u623B\u308A\u5024\n  \
    \  ///\n    /// - \u30D1\u30B9 (u, v) \u4E0A\u306E\u9802\u70B9 (\u3082\u3057\u304F\
    \u306F\u8FBA) \u306E\u7DCF\u7A4D\n    ///\n    /// # \u8A08\u7B97\u91CF\n    ///\n\
    \    /// O(log^2 N)\n    pub fn prod_path(&self, u: usize, v: usize) -> M::S {\n\
    \        let (up, down) = self.hld.path(u, v, Q::edge());\n        let mut res\
    \ = M::e();\n        for &(l, r) in &up {\n            let t = self.seg_up.prod(self.n\
    \ - r..self.n - l);\n            res = M::op(&res, &t);\n        }\n        for\
    \ &(l, r) in &down {\n            let t = self.seg_down.prod(l..r);\n        \
    \    res = M::op(&res, &t);\n        }\n        res\n    }\n\n    /// \u9802\u70B9\
    \ v \u3092\u6839\u3068\u3059\u308B\u90E8\u5206\u6728\u306B\u5BFE\u3059\u308B\u30AF\
    \u30A8\u30EA\u3092\u51E6\u7406\u3059\u308B\u3002\n    ///\n    /// # \u5F15\u6570\
    \n    ///\n    /// - `v`: \u9802\u70B9 v\n    ///\n    /// # \u623B\u308A\u5024\
    \n    ///\n    /// - \u90E8\u5206\u6728 v \u306E\u9802\u70B9 (\u3082\u3057\u304F\
    \u306F\u8FBA) \u306E\u7DCF\u7A4D\n    ///\n    /// # \u8A08\u7B97\u91CF\n    ///\n\
    \    /// O(log^2 N)\n    pub fn prod_subtree(&self, v: usize) -> M::S {\n    \
    \    let (l, r) = self.hld.subtree(v, Q::edge());\n        self.seg_down.prod(l..r)\n\
    \    }\n}\n\nimpl<V, M> TreeQuery<M, Vertex>\nwhere\n    V: Clone,\n    M: Monoid<S\
    \ = V>,\n{\n    /// \u30B0\u30E9\u30D5\u304B\u3089 TreeQuery \u3092\u69CB\u7BC9\
    \u3059\u308B\u3002\n    ///\n    /// # \u5F15\u6570\n    ///\n    /// - `g`: \u30B0\
    \u30E9\u30D5\n    ///\n    /// # \u8A08\u7B97\u91CF\n    ///\n    /// O(N)\n \
    \   pub fn build<E>(g: &Graph<V, E>) -> Self\n    where\n        E: Clone,\n \
    \   {\n        let n = g.len();\n        let hld = HeavyLightDecomposition::new(g);\n\
    \        let mut a = vec![M::e(); n];\n        for v in 0..n {\n            let\
    \ k = hld.vertex(v);\n            a[k] = g.vertex(v).clone();\n        }\n   \
    \     let seg_down = SegmentTree::from(a.clone());\n        a.reverse();\n   \
    \     let seg_up = SegmentTree::from(a);\n        Self {\n            n,\n   \
    \         hld,\n            seg_up,\n            seg_down,\n            _marker:\
    \ PhantomData::default(),\n        }\n    }\n\n    /// \u9802\u70B9 v \u306E\u5024\
    \u3092 x \u306B\u5909\u66F4\u3059\u308B\u3002\n    ///\n    /// # \u5F15\u6570\
    \n    ///\n    /// - `v`: \u9802\u70B9 v\n    /// - `x`: \u65B0\u3057\u3044\u5024\
    \n    ///\n    /// # \u8A08\u7B97\u91CF\n    ///\n    /// O(log N)\n    pub fn\
    \ set(&mut self, v: usize, x: M::S) {\n        let k = self.hld.vertex(v);\n \
    \       self.seg_up.set(self.n - 1 - k, x.clone());\n        self.seg_down.set(k,\
    \ x);\n    }\n\n    /// \u9802\u70B9 v \u306E\u5024\u3092\u53D6\u5F97\u3059\u308B\
    \u3002\n    ///\n    /// # \u5F15\u6570\n    ///\n    /// - `v`: \u9802\u70B9\
    \ v\n    ///\n    /// # \u623B\u308A\u5024\n    ///\n    /// - \u9802\u70B9 v\
    \ \u306E\u5024\n    ///\n    /// # \u8A08\u7B97\u91CF\n    ///\n    /// O(log\
    \ N)\n    pub fn get(&self, v: usize) -> M::S {\n        let k = self.hld.vertex(v);\n\
    \        self.seg_down.get(k)\n    }\n}\n\nimpl<E, M> TreeQuery<M, Edge>\nwhere\n\
    \    E: Clone,\n    M: Monoid<S = E>,\n{\n    /// \u30B0\u30E9\u30D5\u304B\u3089\
    \ TreeQuery \u3092\u69CB\u7BC9\u3059\u308B\u3002\n    ///\n    /// # \u5F15\u6570\
    \n    ///\n    /// - `g`: \u30B0\u30E9\u30D5\n    ///\n    /// # \u8A08\u7B97\u91CF\
    \n    ///\n    /// O(N)\n    pub fn build<V>(g: &Graph<V, E>) -> Self\n    where\n\
    \        V: Clone,\n    {\n        let n = g.len();\n        let hld = HeavyLightDecomposition::new(g);\n\
    \        let mut a = vec![M::e(); n];\n        for v in 0..n {\n            for\
    \ (u, w) in &g[v] {\n                let k = hld.edge(*u, v);\n              \
    \  a[k] = w.clone();\n            }\n        }\n        let seg_down = SegmentTree::from(a.clone());\n\
    \        a.reverse();\n        let seg_up = SegmentTree::from(a);\n        Self\
    \ {\n            n,\n            hld,\n            seg_up,\n            seg_down,\n\
    \            _marker: PhantomData::default(),\n        }\n    }\n\n    /// \u8FBA\
    \ (u, v) \u306E\u5024\u3092 x \u306B\u5909\u66F4\u3059\u308B\u3002\n    ///\n\
    \    /// # \u5F15\u6570\n    ///\n    /// - `u`: \u9802\u70B9 u\n    /// - `v`:\
    \ \u9802\u70B9 v\n    /// - `x`: \u65B0\u3057\u3044\u5024\n    ///\n    /// #\
    \ \u8A08\u7B97\u91CF\n    ///\n    /// O(log N)\n    pub fn set(&mut self, u:\
    \ usize, v: usize, x: M::S) {\n        let k = self.hld.edge(u, v);\n        self.seg_up.set(self.n\
    \ - 1 - k, x.clone());\n        self.seg_down.set(k, x);\n    }\n\n    /// \u8FBA\
    \ (u, v) \u306E\u5024\u3092\u53D6\u5F97\u3059\u308B\u3002\n    ///\n    /// #\
    \ \u5F15\u6570\n    ///\n    /// - `u`: \u9802\u70B9 u\n    /// - `v`: \u9802\u70B9\
    \ v\n    ///\n    /// # \u623B\u308A\u5024\n    ///\n    /// - \u8FBA (u, v) \u306E\
    \u5024\n    ///\n    /// # \u8A08\u7B97\u91CF\n    ///\n    /// O(log N)\n   \
    \ pub fn get(&self, u: usize, v: usize) -> M::S {\n        let k = self.hld.edge(u,\
    \ v);\n        self.seg_down.get(k)\n    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/data-structure/heavy-light-decomposition/src/lib.rs
  - crates/data-structure/segment-tree/src/lib.rs
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/data-structure/tree-query/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-27 03:53:35+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/vertex_add_subtree_sum/src/main.rs
  - verify/vertex_add_path_sum/src/main.rs
  - verify/vertex_set_path_composite/src/main.rs
documentation_of: crates/data-structure/tree-query/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/tree-query/src/lib.rs
- /library/crates/data-structure/tree-query/src/lib.rs.html
title: crates/data-structure/tree-query/src/lib.rs
---
