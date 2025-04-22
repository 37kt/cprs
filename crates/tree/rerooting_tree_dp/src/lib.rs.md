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
    path: verify/library_checker/tree/tree_path_composite_sum/src/main.rs
    title: verify/library_checker/tree/tree_path_composite_sum/src/main.rs
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
  code: "use csr_array::CsrArray;\nuse graph::Edge;\n\npub trait TreeDpOperator {\n\
    \    type Value;\n    type Vertex;\n    type Edge;\n\n    fn unit() -> Self::Value;\n\
    \    fn add_vertex(x: &Self::Value, v: &Self::Vertex) -> Self::Value;\n    fn\
    \ add_edge(x: &Self::Value, e: &Self::Edge) -> Self::Value;\n    fn rake(x: &Self::Value,\
    \ y: &Self::Value) -> Self::Value;\n}\n\npub struct RerootingTreeDp<Op: TreeDpOperator>\
    \ {\n    par: Vec<usize>,\n    dp: Vec<Op::Value>,\n    dpc: Vec<Op::Value>,\n\
    \    dpp: Vec<Op::Value>,\n}\n\nimpl<Op: TreeDpOperator<Vertex = ()>> RerootingTreeDp<Op>\
    \ {\n    pub fn new(g: &CsrArray<impl Edge<Op::Edge>>) -> Self {\n        Self::with_vertices(g,\
    \ &vec![(); g.len()])\n    }\n}\n\nimpl<Op: TreeDpOperator> RerootingTreeDp<Op>\
    \ {\n    pub fn with_vertices(g: &CsrArray<impl Edge<Op::Edge>>, vs: &[Op::Vertex])\
    \ -> Self {\n        let n = g.len();\n        if n == 0 {\n            return\
    \ Self {\n                par: vec![],\n                dp: vec![],\n        \
    \        dpc: vec![],\n                dpp: vec![],\n            };\n        }\n\
    \        assert_eq!(n, vs.len());\n        assert_eq!((n - 1) * 2, g.flat_len(),\
    \ \"g must be a undirected tree\");\n\n        let mut par = vec![!0; n];\n  \
    \      let mut ord = Vec::with_capacity(n); // BFS \u9806\n        ord.push(0);\n\
    \        for i in 0..n {\n            let v = ord[i];\n            for e in &g[v]\
    \ {\n                let u = e.to();\n                if u == par[v] {\n     \
    \               continue;\n                }\n                par[u] = v;\n  \
    \              ord.push(u);\n            }\n        }\n\n        let mut dpc =\
    \ (0..n).map(|_| Op::unit()).collect::<Vec<_>>();\n        let mut dpp = (0..n).map(|_|\
    \ Op::unit()).collect::<Vec<_>>();\n        let mut dp = (0..n).map(|_| Op::unit()).collect::<Vec<_>>();\n\
    \        for &v in ord.iter().rev() {\n            let mut s = Op::unit();\n \
    \           for e in &g[v] {\n                let u = e.to();\n              \
    \  let w = e.weight();\n                if u == par[v] {\n                   \
    \ continue;\n                }\n                let x = Op::add_vertex(&dpc[u],\
    \ &vs[u]);\n                dp[u] = Op::add_edge(&x, w);\n                dpp[u]\
    \ = s;\n                s = Op::rake(&dpp[u], &dp[u]);\n            }\n      \
    \      std::mem::swap(&mut dpc[v], &mut s);\n            for e in g[v].iter().rev()\
    \ {\n                let u = e.to();\n                if u == par[v] {\n     \
    \               continue;\n                }\n                dpp[u] = Op::rake(&dpp[u],\
    \ &s);\n                s = Op::rake(&dp[u], &s);\n            }\n        }\n\n\
    \        // \u3053\u306E\u6642\u70B9\u3067\u306F\n        // dpc[v] = \u90E8\u5206\
    \u6728 v \u306E v \u629C\u304D\n        // dpp[v] = \u90E8\u5206\u6728 p \u306E\
    \ p, v \u629C\u304D\n\n        for &v in &ord {\n            for e in &g[v] {\n\
    \                let u = e.to();\n                let w = e.weight();\n      \
    \          if u == par[v] {\n                    continue;\n                }\n\
    \                dpp[u] = Op::add_vertex(&Op::rake(&dpp[u], &dp[v]), &vs[v]);\n\
    \                dp[u] = Op::add_edge(&dpp[u], w);\n            }\n          \
    \  dp[v] = Op::add_vertex(&Op::rake(&dp[v], &dpc[v]), &vs[v]);\n            dpc[v]\
    \ = Op::add_vertex(&dpc[v], &vs[v]);\n        }\n\n        Self { par, dp, dpc,\
    \ dpp }\n    }\n\n    pub fn fold_all(&self, v: usize) -> &Op::Value {\n     \
    \   &self.dp[v]\n    }\n\n    pub fn fold_subtree(&self, v: usize, p: usize) ->\
    \ &Op::Value {\n        let n = self.par.len();\n        assert!(v < n && p <\
    \ n);\n        if self.par[v] == p {\n            &self.dpc[v]\n        } else\
    \ if self.par[p] == v {\n            &self.dpp[v]\n        } else {\n        \
    \    panic!(\"v and p must be directly connected by an edge.\")\n        }\n \
    \   }\n}\n\nimpl<'a, Op: TreeDpOperator> IntoIterator for &'a RerootingTreeDp<Op>\
    \ {\n    type Item = &'a Op::Value;\n    type IntoIter = std::slice::Iter<'a,\
    \ Op::Value>;\n\n    fn into_iter(self) -> Self::IntoIter {\n        self.dp.iter()\n\
    \    }\n}\n"
  dependsOn:
  - crates/data_structure/csr_array/src/builder.rs
  - crates/data_structure/csr_array/src/csr_array.rs
  - crates/data_structure/csr_array/src/lib.rs
  - crates/graph/graph/src/builder.rs
  - crates/graph/graph/src/edge.rs
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/tree/rerooting_tree_dp/src/lib.rs
  requiredBy: []
  timestamp: '2025-03-06 07:03:37+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/tree/tree_path_composite_sum/src/main.rs
documentation_of: crates/tree/rerooting_tree_dp/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/rerooting_tree_dp/src/lib.rs
- /library/crates/tree/rerooting_tree_dp/src/lib.rs.html
title: crates/tree/rerooting_tree_dp/src/lib.rs
---
