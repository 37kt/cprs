---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/strongly-connected-components/src/lib.rs
    title: crates/graph/strongly-connected-components/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/two_sat/src/main.rs
    title: verify/two_sat/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.1/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.1/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use graph::Graph;\nuse strongly_connected_components::strongly_connected_components;\n\
    \npub struct TwoSatisfiability {\n    n: usize,\n    g: Graph<(), ()>,\n}\n\n\
    impl TwoSatisfiability {\n    pub fn new(n: usize) -> Self {\n        Self {\n\
    \            n,\n            g: Graph::new(n * 2),\n        }\n    }\n\n    pub\
    \ fn set(&mut self, x: usize) {\n        if x < self.n {\n            self.g.add_edge(self.id(!x),\
    \ self.id(x), ());\n        } else {\n            self.g.add_edge(self.id(x),\
    \ self.id(!x), ());\n        }\n    }\n\n    pub fn add(&mut self, x: usize, y:\
    \ usize) {\n        self.g.add_edge(self.id(!x), self.id(y), ());\n        self.g.add_edge(self.id(!y),\
    \ self.id(x), ());\n    }\n\n    pub fn if_then(&mut self, x: usize, y: usize)\
    \ {\n        self.add(!x, y);\n    }\n\n    pub fn solve(&self) -> Option<Vec<bool>>\
    \ {\n        let scc = strongly_connected_components(&self.g);\n        let mut\
    \ comp = vec![0; self.n * 2];\n        for i in 0..scc.size() {\n            for\
    \ &x in &scc.vertices()[i] {\n                comp[x] = i;\n            }\n  \
    \      }\n        let mut res = vec![false; self.n];\n        for i in 0..self.n\
    \ {\n            if comp[i] == comp[i + self.n] {\n                return None;\n\
    \            }\n            res[i] = comp[i] > comp[i + self.n];\n        }\n\
    \        Some(res)\n    }\n\n    fn id(&self, x: usize) -> usize {\n        assert!(x\
    \ < self.n || !x < self.n);\n        if x < self.n {\n            x\n        }\
    \ else {\n            !x + self.n\n        }\n    }\n}\n"
  dependsOn:
  - crates/graph/graph/src/lib.rs
  - crates/graph/strongly-connected-components/src/lib.rs
  isVerificationFile: false
  path: crates/math/two-satisfiability/src/lib.rs
  requiredBy: []
  timestamp: '2023-05-17 16:30:46+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/two_sat/src/main.rs
documentation_of: crates/math/two-satisfiability/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/two-satisfiability/src/lib.rs
- /library/crates/math/two-satisfiability/src/lib.rs.html
title: crates/math/two-satisfiability/src/lib.rs
---
