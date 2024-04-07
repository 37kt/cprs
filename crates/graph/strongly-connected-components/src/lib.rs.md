---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/math/two-satisfiability/src/lib.rs
    title: crates/math/two-satisfiability/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/scc/src/main.rs
    title: verify/scc/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use graph::Graph;\n\n/// \u5F37\u9023\u7D50\u6210\u5206\u5206\u89E3\u3092\
    \u3057\u3001\u7E2E\u7D04\u5F8C\u306E\u30B0\u30E9\u30D5\u3092\u8FD4\u3059\u3002\
    \n/// \u7E2E\u7D04\u5F8C\u306E\u9802\u70B9\u306E\u5024\u306B\u306F\u3001\u5F37\
    \u9023\u7D50\u6210\u5206\u306B\u5C5E\u3059\u308B\u9802\u70B9\u756A\u53F7\u304C\
    \u683C\u7D0D\u3055\u308C\u308B\u3002\npub fn strongly_connected_components<V,\
    \ E>(g: &Graph<V, E>) -> Graph<Vec<usize>, E>\nwhere\n    V: Clone,\n    E: Clone,\n\
    {\n    let n = g.len();\n    let mut scc = Scc {\n        comp: vec![0; n],\n\
    \        low: vec![0; n],\n        ord: vec![!0; n],\n        vis: vec![],\n \
    \       t: 0,\n        m: 0,\n    };\n    for v in 0..n {\n        if scc.ord[v]\
    \ == !0 {\n            scc.dfs(v, g);\n        }\n    }\n    let mut groups =\
    \ vec![vec![]; scc.m];\n    for v in 0..n {\n        scc.comp[v] = scc.m - 1 -\
    \ scc.comp[v];\n        groups[scc.comp[v]].push(v);\n    }\n    let mut edges\
    \ = vec![];\n    for v in 0..n {\n        for (u, w) in &g[v] {\n            let\
    \ a = scc.comp[v];\n            let b = scc.comp[*u];\n            if a != b {\n\
    \                edges.push((a, b, w.clone()));\n            }\n        }\n  \
    \  }\n    Graph::from_vertices_and_directed_edges(&groups, &edges)\n}\n\nimpl\
    \ Scc {\n    fn dfs<V, E>(&mut self, v: usize, g: &Graph<V, E>)\n    where\n \
    \       V: Clone,\n        E: Clone,\n    {\n        self.low[v] = self.t;\n \
    \       self.ord[v] = self.t;\n        self.t += 1;\n        self.vis.push(v);\n\
    \        for &(u, _) in &g[v] {\n            if self.ord[u] == !0 {\n        \
    \        self.dfs(u, g);\n                self.low[v] = self.low[v].min(self.low[u]);\n\
    \            } else {\n                self.low[v] = self.low[v].min(self.ord[u]);\n\
    \            }\n        }\n        if self.low[v] == self.ord[v] {\n         \
    \   loop {\n                let u = self.vis.pop().unwrap();\n               \
    \ self.ord[u] = g.len();\n                self.comp[u] = self.m;\n           \
    \     if u == v {\n                    break;\n                }\n           \
    \ }\n            self.m += 1;\n        }\n    }\n}\n\nstruct Scc {\n    comp:\
    \ Vec<usize>,\n    low: Vec<usize>,\n    ord: Vec<usize>,\n    vis: Vec<usize>,\n\
    \    t: usize,\n    m: usize,\n}\n"
  dependsOn:
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/graph/strongly-connected-components/src/lib.rs
  requiredBy:
  - crates/math/two-satisfiability/src/lib.rs
  timestamp: '2024-04-07 08:56:09+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/scc/src/main.rs
documentation_of: crates/graph/strongly-connected-components/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/strongly-connected-components/src/lib.rs
- /library/crates/graph/strongly-connected-components/src/lib.rs.html
title: crates/graph/strongly-connected-components/src/lib.rs
---
