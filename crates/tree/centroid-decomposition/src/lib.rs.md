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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use graph::Graph;\n\n/// \u91CD\u5FC3\u5206\u89E3\u3092\u3059\u308B\n///\
    \ \u5165\u529B: \u6728\n/// \u623B\u308A\u5024: \u91CD\u5FC3\u5206\u89E3\u5F8C\
    \u306E\u6728?\u306B\u3064\u3044\u3066\u306E (\u89AA, \u884C\u304D\u304C\u3051\u9806\
    )\npub fn build<V, E>(g: &Graph<V, E>) -> (Vec<usize>, Vec<usize>)\nwhere\n  \
    \  V: Clone,\n    E: Clone,\n{\n    let mut cd = CentroidDecomposition::new(g.size());\n\
    \    cd.build(0, g);\n    (cd.par, cd.ord)\n}\n\nstruct CentroidDecomposition\
    \ {\n    sz: Vec<usize>,\n    par: Vec<usize>,\n    used: Vec<bool>,\n    ord:\
    \ Vec<usize>,\n}\n\nimpl CentroidDecomposition {\n    fn new(n: usize) -> Self\
    \ {\n        Self {\n            sz: vec![1; n],\n            par: vec![!0; n],\n\
    \            used: vec![false; n],\n            ord: vec![],\n        }\n    }\n\
    \n    fn build<V, E>(&mut self, v: usize, g: &Graph<V, E>) -> usize\n    where\n\
    \        V: Clone,\n        E: Clone,\n    {\n        let sz = self.dfs_size(v,\
    \ !0, g);\n        let c = self.search_centroid(v, !0, sz / 2, g);\n        self.used[c]\
    \ = true;\n        self.ord.push(v);\n        for &(u, _) in g.out_edges(v) {\n\
    \            if !self.used[u] {\n                let d = self.build(u, g);\n \
    \               self.par[d] = c;\n            }\n        }\n        self.used[c]\
    \ = false;\n        c\n    }\n\n    fn dfs_size<V, E>(&mut self, v: usize, p:\
    \ usize, g: &Graph<V, E>) -> usize\n    where\n        V: Clone,\n        E: Clone,\n\
    \    {\n        self.sz[v] = 1;\n        for &(u, _) in g.out_edges(v) {\n   \
    \         if u == p || self.used[u] {\n                continue;\n           \
    \ }\n            self.sz[v] += self.dfs_size(u, v, g);\n        }\n        self.sz[v]\n\
    \    }\n\n    fn search_centroid<V, E>(&mut self, v: usize, p: usize, mid: usize,\
    \ g: &Graph<V, E>) -> usize\n    where\n        V: Clone,\n        E: Clone,\n\
    \    {\n        for &(u, _) in g.out_edges(v) {\n            if u == p || self.used[u]\
    \ {\n                continue;\n            }\n            if self.sz[u] > mid\
    \ {\n                return self.search_centroid(u, v, mid, g);\n            }\n\
    \        }\n        v\n    }\n}\n"
  dependsOn:
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/tree/centroid-decomposition/src/lib.rs
  requiredBy: []
  timestamp: '2023-05-17 16:30:46+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/tree/centroid-decomposition/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/centroid-decomposition/src/lib.rs
- /library/crates/tree/centroid-decomposition/src/lib.rs.html
title: crates/tree/centroid-decomposition/src/lib.rs
---
