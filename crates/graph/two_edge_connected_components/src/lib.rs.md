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
    path: verify/library_checker/graph/two_edge_connected_components/src/main.rs
    title: verify/library_checker/graph/two_edge_connected_components/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use csr_array::CsrArray;\nuse graph::Edge;\n\n/// \u6A4B\u3092\u9664\u3044\
    \u305F\u3068\u304D\u306E\u9023\u7D50\u6210\u5206\n#[derive(Clone)]\npub struct\
    \ TwoEdgeConnectedComponents {\n    pub comp: Vec<usize>,\n    pub groups: CsrArray<usize>,\n\
    }\n\nimpl TwoEdgeConnectedComponents {\n    pub fn new<W>(g: &CsrArray<impl Edge<W>>)\
    \ -> Self {\n        let mut tecc = TeccImpl {\n            comp: vec![!0; g.len()],\n\
    \            comp_cnt: 0,\n            ord: vec![!0; g.len()],\n            low:\
    \ vec![!0; g.len()],\n            path: vec![],\n            t: 0,\n        };\n\
    \n        for v in 0..g.len() {\n            if tecc.comp[v] == !0 {\n       \
    \         tecc.dfs(g, v, !0);\n            }\n        }\n\n        let groups\
    \ = CsrArray::new(\n            tecc.comp_cnt,\n            tecc.comp.iter().enumerate().map(|(v,\
    \ &c)| (c, v)),\n        );\n\n        Self {\n            comp: tecc.comp,\n\
    \            groups,\n        }\n    }\n}\n\nstruct TeccImpl {\n    comp: Vec<usize>,\n\
    \    comp_cnt: usize,\n    ord: Vec<usize>,\n    low: Vec<usize>,\n    path: Vec<usize>,\n\
    \    t: usize,\n}\n\nimpl TeccImpl {\n    fn dfs<W>(&mut self, g: &CsrArray<impl\
    \ Edge<W>>, v: usize, p: usize) {\n        self.ord[v] = self.t;\n        self.low[v]\
    \ = self.t;\n        self.t += 1;\n        self.path.push(v);\n        let mut\
    \ f = false;\n        for e in &g[v] {\n            let u = e.to();\n        \
    \    if self.ord[u] == !0 {\n                self.dfs(g, u, v);\n            \
    \    self.low[v] = self.low[v].min(self.low[u]);\n            } else if u == p\
    \ && !f {\n                f = true;\n            } else {\n                self.low[v]\
    \ = self.low[v].min(self.ord[u]);\n            }\n        }\n        if self.ord[v]\
    \ == self.low[v] {\n            while self.comp[v] == !0 {\n                self.comp[self.path.pop().unwrap()]\
    \ = self.comp_cnt;\n            }\n            self.comp_cnt += 1;\n        }\n\
    \    }\n}\n"
  dependsOn:
  - crates/data_structure/csr_array/src/builder.rs
  - crates/data_structure/csr_array/src/csr_array.rs
  - crates/data_structure/csr_array/src/lib.rs
  - crates/graph/graph/src/builder.rs
  - crates/graph/graph/src/edge.rs
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/graph/two_edge_connected_components/src/lib.rs
  requiredBy: []
  timestamp: '2025-03-26 02:28:07+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/graph/two_edge_connected_components/src/main.rs
documentation_of: crates/graph/two_edge_connected_components/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/two_edge_connected_components/src/lib.rs
- /library/crates/graph/two_edge_connected_components/src/lib.rs.html
title: crates/graph/two_edge_connected_components/src/lib.rs
---
