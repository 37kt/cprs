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
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/graph/biconnected_components/src/main.rs
    title: verify/library_checker/graph/biconnected_components/src/main.rs
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
  code: "use csr_array::CsrArray;\nuse graph::{Edge, GraphBuilder, UndirectedGraph};\n\
    \npub fn extended_block_cut_tree<W>(g: &CsrArray<impl Edge<W>>) -> CsrArray<usize>\
    \ {\n    let n = g.len();\n    let mut ebct = EbctImpl {\n        next: vec![!0;\
    \ n],\n        depth: vec![!0; n],\n        add: vec![0; n],\n        used: vec![false;\
    \ n],\n        cnt: n,\n        edges: vec![],\n    };\n\n    for v in 0..n {\n\
    \        if ebct.depth[v] == !0 {\n            ebct.depth[v] = 0;\n          \
    \  ebct.dfs1(g, v);\n        }\n    }\n\n    for v in 0..n {\n        if ebct.depth[v]\
    \ == 0 {\n            ebct.dfs2(g, v, ebct.cnt);\n        }\n        if g[v].is_empty()\
    \ {\n            ebct.edges.push((v, ebct.cnt));\n            ebct.cnt += 1;\n\
    \        }\n    }\n\n    UndirectedGraph::from_edges(ebct.cnt, ebct.edges)\n}\n\
    \nstruct EbctImpl {\n    next: Vec<usize>,\n    depth: Vec<usize>,\n    add: Vec<i32>,\n\
    \    used: Vec<bool>,\n    cnt: usize,\n    edges: Vec<(usize, usize)>,\n}\n\n\
    impl EbctImpl {\n    fn dfs1<W>(&mut self, g: &CsrArray<impl Edge<W>>, v: usize)\
    \ {\n        for e in &g[v] {\n            let u = e.to();\n            if self.depth[u]\
    \ == !0 {\n                self.depth[u] = self.depth[v] + 1;\n              \
    \  self.next[v] = u;\n                self.dfs1(g, u);\n                self.add[v]\
    \ += self.add[u];\n            } else if self.depth[u] + 1 < self.depth[v] {\n\
    \                self.add[v] += 1;\n                self.add[self.next[u]] -=\
    \ 1;\n            }\n        }\n    }\n\n    fn dfs2<W>(&mut self, g: &CsrArray<impl\
    \ Edge<W>>, v: usize, b: usize) {\n        self.used[v] = true;\n        let mut\
    \ ok = false;\n        for e in &g[v] {\n            let u = e.to();\n       \
    \     if self.depth[u] == self.depth[v] + 1 && !self.used[u] {\n             \
    \   if self.add[u] > 0 {\n                    if !ok {\n                     \
    \   ok = true;\n                        self.edges.push((v, b));\n           \
    \         }\n                    self.dfs2(g, u, b);\n                } else {\n\
    \                    self.edges.push((v, self.cnt));\n                    self.cnt\
    \ += 1;\n                    self.dfs2(g, u, self.cnt - 1);\n                }\n\
    \            }\n        }\n        if !ok && self.depth[v] > 0 {\n           \
    \ self.edges.push((v, b));\n        }\n    }\n}\n"
  dependsOn:
  - crates/data_structure/csr_array/src/builder.rs
  - crates/data_structure/csr_array/src/csr_array.rs
  - crates/data_structure/csr_array/src/lib.rs
  - crates/graph/graph/src/builder.rs
  - crates/graph/graph/src/edge.rs
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/graph/extended_block_cut_tree/src/lib.rs
  requiredBy: []
  timestamp: '2025-03-26 03:07:13+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/graph/biconnected_components/src/main.rs
documentation_of: crates/graph/extended_block_cut_tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/extended_block_cut_tree/src/lib.rs
- /library/crates/graph/extended_block_cut_tree/src/lib.rs.html
title: crates/graph/extended_block_cut_tree/src/lib.rs
---
