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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use graph::Graph;\nuse strongly_connected_components::strongly_connected_components;\n\
    \n/// 2-SAT\npub struct TwoSatisfiability {\n    n: usize,\n    es: Vec<(usize,\
    \ usize)>,\n}\n\nimpl TwoSatisfiability {\n    /// n \u500B\u306E\u5909\u6570\u3092\
    \u6301\u3064 2-SAT \u3092\u521D\u671F\u5316\u3059\u308B\n    pub fn new(n: usize)\
    \ -> Self {\n        Self { n, es: vec![] }\n    }\n\n    /// \u6761\u4EF6 x_i\
    \ = f \u3092\u8FFD\u52A0\u3059\u308B\n    pub fn set(&mut self, i: usize, f: bool)\
    \ {\n        self.es.push((self.id(i, !f), self.id(i, f)));\n    }\n\n    ///\
    \ \u6761\u4EF6 x_i = f -> x_j = g \u3092\u8FFD\u52A0\u3059\u308B\n    pub fn if_then(&mut\
    \ self, i: usize, f: bool, j: usize, g: bool) {\n        self.or(i, !f, j, g);\n\
    \    }\n\n    /// \u6761\u4EF6 x_i = f \u2228 x_j = g \u3092\u8FFD\u52A0\u3059\
    \u308B\n    pub fn or(&mut self, i: usize, f: bool, j: usize, g: bool) {\n   \
    \     self.es.push((self.id(i, !f), self.id(j, g)));\n        self.es.push((self.id(j,\
    \ !g), self.id(i, f)));\n    }\n\n    /// \u6761\u4EF6 \uFFE2(x_i = f \u2227 x_j\
    \ = g) \u3092\u8FFD\u52A0\u3059\u308B\n    pub fn nand(&mut self, i: usize, f:\
    \ bool, j: usize, g: bool) {\n        self.or(i, !f, j, !g);\n    }\n\n    pub\
    \ fn solve(&self) -> Option<Vec<bool>> {\n        let g = Graph::from_unweighted_directed_edges(self.n\
    \ * 2, &self.es);\n        let (_, comp) = strongly_connected_components(&g);\n\
    \        let mut res = vec![false; self.n];\n        for i in 0..self.n {\n  \
    \          if comp[i] == comp[i + self.n] {\n                return None;\n  \
    \          }\n            res[i] = comp[i] > comp[i + self.n];\n        }\n  \
    \      Some(res)\n    }\n\n    fn id(&self, i: usize, f: bool) -> usize {\n  \
    \      assert!(i < self.n);\n        if f {\n            i\n        } else {\n\
    \            i + self.n\n        }\n    }\n}\n"
  dependsOn:
  - crates/graph/graph/src/lib.rs
  - crates/graph/strongly-connected-components/src/lib.rs
  isVerificationFile: false
  path: crates/math/two-satisfiability/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-27 04:46:01+00:00'
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
