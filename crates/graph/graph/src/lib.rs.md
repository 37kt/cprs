---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/data-structure/heavy-light-decomposition/src/lib.rs
    title: crates/data-structure/heavy-light-decomposition/src/lib.rs
  - icon: ':warning:'
    path: crates/data-structure/tree-query/src/lib.rs
    title: crates/data-structure/tree-query/src/lib.rs
  - icon: ':warning:'
    path: crates/tree/centroid-decomposition/src/lib.rs
    title: crates/tree/centroid-decomposition/src/lib.rs
  - icon: ':warning:'
    path: crates/tree/re-rooting-dp/src/lib.rs
    title: crates/tree/re-rooting-dp/src/lib.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "#[derive(Clone)]\npub struct Graph<V, E>\nwhere\n    V: Copy,\n    E: Copy,\n\
    {\n    vs: Vec<V>,\n    es: Vec<Vec<(usize, E)>>,\n}\n\nimpl<V, E> Graph<V, E>\n\
    where\n    V: Copy + Default,\n    E: Copy,\n{\n    pub fn new(n: usize) -> Self\
    \ {\n        Self {\n            vs: vec![Default::default(); n],\n          \
    \  es: vec![vec![]; n],\n        }\n    }\n}\n\nimpl<V, E> From<Vec<V>> for Graph<V,\
    \ E>\nwhere\n    V: Copy,\n    E: Copy,\n{\n    fn from(vs: Vec<V>) -> Self {\n\
    \        Self {\n            es: vec![vec![]; vs.len()],\n            vs,\n  \
    \      }\n    }\n}\n\nimpl<V, E> Graph<V, E>\nwhere\n    V: Copy,\n    E: Copy,\n\
    {\n    pub fn size(&self) -> usize {\n        self.vs.len()\n    }\n\n    pub\
    \ fn set_vertex(&mut self, v: usize, w: V) {\n        self.vs[v] = w;\n    }\n\
    \n    pub fn add_edge(&mut self, u: usize, v: usize, w: E) {\n        self.es[u].push((v,\
    \ w));\n    }\n\n    pub fn add_undirected_edge(&mut self, u: usize, v: usize,\
    \ w: E) {\n        self.add_edge(u, v, w);\n        self.add_edge(v, u, w);\n\
    \    }\n\n    pub fn vertex(&self, v: usize) -> V {\n        self.vs[v]\n    }\n\
    \n    pub fn out_edges(&self, v: usize) -> &Vec<(usize, E)> {\n        &self.es[v]\n\
    \    }\n}\n\n#[cfg(test)]\nmod tests {\n    use super::*;\n    #[test]\n    fn\
    \ test() {\n        let mut g = Graph::<i64, i64>::new(100);\n        g.add_undirected_edge(0,\
    \ 1, 100);\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/graph/graph/src/lib.rs
  requiredBy:
  - crates/tree/centroid-decomposition/src/lib.rs
  - crates/tree/re-rooting-dp/src/lib.rs
  - crates/data-structure/heavy-light-decomposition/src/lib.rs
  - crates/data-structure/tree-query/src/lib.rs
  timestamp: '2023-04-16 02:58:04+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/graph/graph/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/graph/src/lib.rs
- /library/crates/graph/graph/src/lib.rs.html
title: crates/graph/graph/src/lib.rs
---
