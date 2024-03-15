---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use graph::Graph;\n\npub struct LowLink {\n    vis: Vec<bool>,\n    pub ord:\
    \ Vec<usize>,\n    pub low: Vec<usize>,\n    pub par: Vec<usize>,\n    pub articulation:\
    \ Vec<usize>,\n    pub bridge: Vec<(usize, usize)>,\n    cnt: usize,\n    pub\
    \ component_count: usize,\n}\n\nimpl LowLink {\n    pub fn new(g: &Graph<(), ()>)\
    \ -> Self {\n        let mut this = Self {\n            vis: vec![false; g.size()],\n\
    \            ord: vec![0; g.size()],\n            low: vec![0; g.size()],\n  \
    \          par: vec![!0; g.size()],\n            articulation: vec![],\n     \
    \       bridge: vec![],\n            cnt: 0,\n            component_count: 0,\n\
    \        };\n        for i in 0..g.size() {\n            if !this.vis[i] {\n \
    \               this.dfs(i, !0, g);\n                this.component_count += 1;\n\
    \            }\n        }\n        this\n    }\n\n    fn dfs(&mut self, v: usize,\
    \ p: usize, g: &Graph<(), ()>) {\n        self.vis[v] = true;\n        self.ord[v]\
    \ = self.cnt;\n        self.low[v] = self.cnt;\n        self.par[v] = p;\n   \
    \     self.cnt += 1;\n        let mut is_articulation = false;\n        let mut\
    \ cnt = 0;\n        for &(u, _) in g.out_edges(v) {\n            if !self.vis[u]\
    \ {\n                cnt += 1;\n                self.dfs(u, v, g);\n         \
    \       if u != p {\n                    self.low[v] = std::cmp::min(self.low[v],\
    \ self.low[u]);\n                }\n                if p != !0 && self.ord[v]\
    \ <= self.low[u] {\n                    is_articulation = true;\n            \
    \    }\n                if self.ord[v] < self.low[u] {\n                    self.bridge.push((std::cmp::min(u,\
    \ v), std::cmp::max(u, v)));\n                }\n            } else {\n      \
    \          if u != p {\n                    self.low[v] = std::cmp::min(self.low[v],\
    \ self.ord[u]);\n                }\n            }\n        }\n        if p ==\
    \ !0 && cnt > 1 {\n            is_articulation = true;\n        }\n        if\
    \ is_articulation {\n            self.articulation.push(v);\n        }\n    }\n\
    }\n"
  dependsOn:
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/graph/low-link/src/lib.rs
  requiredBy: []
  timestamp: '2023-12-24 10:25:28+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/graph/low-link/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/low-link/src/lib.rs
- /library/crates/graph/low-link/src/lib.rs.html
title: crates/graph/low-link/src/lib.rs
---