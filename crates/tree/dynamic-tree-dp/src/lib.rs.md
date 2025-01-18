---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy-light-decomposition/src/lib.rs
    title: crates/tree/heavy-light-decomposition/src/lib.rs
  - icon: ':warning:'
    path: crates/tree/static-top-tree/src/lib.rs
    title: crates/tree/static-top-tree/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/point_set_tree_path_composite_sum_fixed_root/src/main.rs
    title: verify/point_set_tree_path_composite_sum_fixed_root/src/main.rs
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
  code: "use graph::UndirectedGraph;\nuse static_top_tree::StaticTopTree;\n\npub trait\
    \ DynamicTreeDPOperator {\n    type V: Clone;\n    type E: Clone;\n    type X:\
    \ Clone;\n\n    fn e() -> Self::X;\n\n    /// \u9802\u70B9\u3068\u305D\u306E\u89AA\
    \u3078\u306E\u8FBA\u304B\u3089\u306A\u308B\u30AF\u30E9\u30B9\u30BF\u30FC (o\u2190\
    a\\]  \n    /// o \u306F virtual\n    fn single(v: &Self::V, e: Option<&Self::E>)\
    \ -> Self::X;\n\n    /// (a\u2190b\\], (a\u2190c\\] \u2192 (a\u2190b\\]\n    fn\
    \ rake(l: &Self::X, r: &Self::X) -> Self::X;\n\n    /// (a\u2190b\\], (b\u2190\
    c\\] \u2192 (a\u2190c\\]\n    fn compress(p: &Self::X, c: &Self::X) -> Self::X;\n\
    }\n\npub struct DynamicTreeDP<Op>\nwhere\n    Op: DynamicTreeDPOperator,\n   \
    \ Op::X: Clone,\n{\n    stt: StaticTopTree,\n    vertices: Vec<Op::V>,\n    edges:\
    \ Vec<Op::E>,\n    dp: Vec<Op::X>,\n}\n\nimpl<Op> DynamicTreeDP<Op>\nwhere\n \
    \   Op: DynamicTreeDPOperator,\n    Op::X: Clone,\n{\n    pub fn new(g: &UndirectedGraph<Op::V,\
    \ Op::E>, root: usize) -> Self\n    where\n        Op::V: Clone,\n        Op::E:\
    \ Clone,\n    {\n        let n = g.len();\n        let stt = StaticTopTree::new(g,\
    \ root);\n        let vertices = (0..n).map(|i| g.vertex(i).clone()).collect::<Vec<_>>();\n\
    \        let mut ord = vec![(0, 0); n - 1];\n        for u in 0..n {\n       \
    \     for (i, (v, _)) in g[u].iter().enumerate() {\n                if u < *v\
    \ {\n                    let j = stt.hld.edge_index(u, *v);\n                \
    \    ord[j] = (u, i);\n                }\n            }\n        }\n        let\
    \ edges = ord\n            .into_iter()\n            .map(|(v, i)| g[v][i].1.clone())\n\
    \            .collect::<Vec<_>>();\n\n        let mut dp = (0..n * 2 - 1).map(|_|\
    \ Op::e()).collect::<Vec<_>>();\n        for v in 0..n {\n            // vertex_index(v)\
    \ - 1 \u304C\u89AA\u3078\u306E\u8FBA\u306E index\n            let i = stt.hld.vertex_index(v);\n\
    \            if i == 0 {\n                dp[v] = Op::single(&vertices[v], None);\n\
    \            } else {\n                dp[v] = Op::single(&vertices[v], Some(&edges[i\
    \ - 1]));\n            }\n        }\n\n        let mut treedp = Self {\n     \
    \       stt,\n            vertices,\n            edges,\n            dp,\n   \
    \     };\n\n        for v in n..n * 2 - 1 {\n            treedp.update(v);\n \
    \       }\n\n        treedp\n    }\n\n    pub fn set_vertex(&mut self, v: usize,\
    \ x: Op::V) {\n        self.vertices[v] = x;\n        let i = self.stt.hld.vertex_index(v);\n\
    \        if i == 0 {\n            self.dp[v] = Op::single(&self.vertices[v], None);\n\
    \        } else {\n            self.dp[v] = Op::single(&self.vertices[v], Some(&self.edges[i\
    \ - 1]));\n        }\n        let mut v = self.stt.par[v];\n        while v !=\
    \ !0 {\n            self.update(v);\n            v = self.stt.par[v];\n      \
    \  }\n    }\n\n    pub fn set_edge(&mut self, u: usize, v: usize, x: Op::E) {\n\
    \        let i = self.stt.hld.edge_index(u, v);\n        self.edges[i] = x;\n\
    \        let v = if self.stt.hld.parent(u) == v { u } else { v };\n        self.dp[v]\
    \ = Op::single(&self.vertices[v], Some(&self.edges[i]));\n        let mut v =\
    \ self.stt.par[v];\n        while v != !0 {\n            self.update(v);\n   \
    \         v = self.stt.par[v];\n        }\n    }\n\n    pub fn prod(&self) ->\
    \ Op::X {\n        self.dp.last().unwrap().clone()\n    }\n\n    fn update(&mut\
    \ self, v: usize) {\n        let l = &self.dp[self.stt.lch[v]];\n        let r\
    \ = &self.dp[self.stt.rch[v]];\n        if self.stt.is_compress[v] {\n       \
    \     self.dp[v] = Op::compress(l, r);\n        } else {\n            self.dp[v]\
    \ = Op::rake(l, r);\n        }\n    }\n}\n"
  dependsOn:
  - crates/graph/graph/src/lib.rs
  - crates/tree/heavy-light-decomposition/src/lib.rs
  - crates/tree/static-top-tree/src/lib.rs
  isVerificationFile: false
  path: crates/tree/dynamic-tree-dp/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-18 04:01:37+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/point_set_tree_path_composite_sum_fixed_root/src/main.rs
documentation_of: crates/tree/dynamic-tree-dp/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/dynamic-tree-dp/src/lib.rs
- /library/crates/tree/dynamic-tree-dp/src/lib.rs.html
title: crates/tree/dynamic-tree-dp/src/lib.rs
---
