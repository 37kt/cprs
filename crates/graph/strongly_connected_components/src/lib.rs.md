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
    path: verify/library_checker/graph/scc/src/main.rs
    title: verify/library_checker/graph/scc/src/main.rs
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
  code: "use csr_array::CsrArray;\nuse graph::Edge;\n\npub struct StronglyConnectedComponents\
    \ {\n    pub comp: Vec<usize>,\n    pub groups: CsrArray<usize>,\n}\n\nimpl StronglyConnectedComponents\
    \ {\n    pub fn new<W>(g: &CsrArray<impl Edge<W>>) -> Self {\n        let n =\
    \ g.len();\n        let mut scc = SccImpl {\n            comp: vec![0; n],\n \
    \           low: vec![0; n],\n            ord: vec![!0; n],\n            path:\
    \ vec![],\n            t: 0,\n            comp_cnt: 0,\n        };\n\n       \
    \ for v in 0..n {\n            if scc.ord[v] == !0 {\n                scc.dfs(g,\
    \ v);\n            }\n        }\n        for v in 0..n {\n            scc.comp[v]\
    \ = scc.comp_cnt - 1 - scc.comp[v];\n        }\n\n        let groups = CsrArray::new(\n\
    \            scc.comp_cnt,\n            scc.comp.iter().enumerate().map(|(v, &c)|\
    \ (c, v)),\n        );\n\n        Self {\n            comp: scc.comp,\n      \
    \      groups,\n        }\n    }\n}\n\nstruct SccImpl {\n    comp: Vec<usize>,\n\
    \    low: Vec<usize>,\n    ord: Vec<usize>,\n    path: Vec<usize>,\n    t: usize,\n\
    \    comp_cnt: usize,\n}\n\nimpl SccImpl {\n    fn dfs<W>(&mut self, g: &CsrArray<impl\
    \ Edge<W>>, v: usize) {\n        self.low[v] = self.t;\n        self.ord[v] =\
    \ self.t;\n        self.t += 1;\n        self.path.push(v);\n\n        for e in\
    \ &g[v] {\n            let u = e.to();\n            if self.ord[u] == !0 {\n \
    \               self.dfs(g, u);\n                self.low[v] = self.low[v].min(self.low[u]);\n\
    \            } else {\n                self.low[v] = self.low[v].min(self.ord[u]);\n\
    \            }\n        }\n\n        if self.low[v] == self.ord[v] {\n       \
    \     while let Some(u) = self.path.pop() {\n                self.ord[u] = !1;\n\
    \                self.comp[u] = self.comp_cnt;\n                if u == v {\n\
    \                    break;\n                }\n            }\n            self.comp_cnt\
    \ += 1;\n        }\n    }\n}\n"
  dependsOn:
  - crates/data_structure/csr_array/src/builder.rs
  - crates/data_structure/csr_array/src/csr_array.rs
  - crates/data_structure/csr_array/src/lib.rs
  - crates/graph/graph/src/builder.rs
  - crates/graph/graph/src/edge.rs
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/graph/strongly_connected_components/src/lib.rs
  requiredBy: []
  timestamp: '2025-03-06 00:54:38+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/graph/scc/src/main.rs
documentation_of: crates/graph/strongly_connected_components/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/strongly_connected_components/src/lib.rs
- /library/crates/graph/strongly_connected_components/src/lib.rs.html
title: crates/graph/strongly_connected_components/src/lib.rs
---
