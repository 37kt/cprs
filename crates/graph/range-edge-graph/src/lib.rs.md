---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yuki1014/src/main.rs
    title: verify/yuki1014/src/main.rs
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
  code: "use std::ops::RangeBounds;\n\nuse graph::DirectedGraph;\n\n/// \u533A\u9593\
    \u304B\u3089\u533A\u9593\u306B\u8FBA\u3092\u5F35\u308B\u30C6\u30AF\u30CB\u30C3\
    \u30AF  \npub struct RangeEdgeGraph<E>\nwhere\n    E: Clone + Default,\n{\n  \
    \  n: usize,\n    m: usize,\n    es: Vec<(usize, usize, E)>,\n}\n\nimpl<E> RangeEdgeGraph<E>\n\
    where\n    E: Clone + Default,\n{\n    /// \u9802\u70B9\u6570 n \u3067\u521D\u671F\
    \u5316\u3059\u308B\n    pub fn new(n: usize) -> Self {\n        let mut s = Self\
    \ {\n            n,\n            m: n * 3,\n            es: vec![],\n        };\n\
    \        for i in 1..n {\n            let l = i << 1 | 0;\n            let r =\
    \ i << 1 | 1;\n            s.es.push((s.id(n * 0 + i), s.id(n * 0 + l), E::default()));\n\
    \            s.es.push((s.id(n * 0 + i), s.id(n * 0 + r), E::default()));\n  \
    \          s.es.push((s.id(n * 2 + l), s.id(n * 2 + i), E::default()));\n    \
    \        s.es.push((s.id(n * 2 + r), s.id(n * 2 + i), E::default()));\n      \
    \  }\n        s\n    }\n\n    /// \u9802\u70B9\u96C6\u5408 u \u304B\u3089\u9802\
    \u70B9\u96C6\u5408 v \u306B\u91CD\u307F w \u306E\u8FBA\u3092\u5F35\u308B\n   \
    \ pub fn add_edge(&mut self, u: impl RangeBounds<usize>, v: impl RangeBounds<usize>,\
    \ w: E) {\n        let (mut l1, mut r1) = range_to_pair(u, self.n);\n        let\
    \ (mut l2, mut r2) = range_to_pair(v, self.n);\n        l1 += self.n;\n      \
    \  r1 += self.n;\n        if l1 == r1 || l2 == r2 {\n            return;\n   \
    \     }\n        let k = self.m;\n        self.m += 1;\n        while l1 < r1\
    \ {\n            if l1 & 1 == 1 {\n                self.es.push((self.id(self.n\
    \ * 2 + l1), k, E::default()));\n                l1 += 1;\n            }\n   \
    \         if r1 & 1 == 1 {\n                r1 -= 1;\n                self.es.push((self.id(self.n\
    \ * 2 + r1), k, E::default()));\n            }\n            l1 >>= 1;\n      \
    \      r1 >>= 1;\n        }\n        l2 += self.n;\n        r2 += self.n;\n  \
    \      while l2 < r2 {\n            if l2 & 1 == 1 {\n                self.es.push((k,\
    \ self.id(l2), w.clone()));\n                l2 += 1;\n            }\n       \
    \     if r2 & 1 == 1 {\n                r2 -= 1;\n                self.es.push((k,\
    \ self.id(r2), w.clone()));\n            }\n            l2 >>= 1;\n          \
    \  r2 >>= 1;\n        }\n    }\n\n    /// \u8D85\u9802\u70B9\u3092\u7528\u3044\
    \u3066\u30B0\u30E9\u30D5\u3092\u69CB\u7BC9\u3059\u308B  \n    /// \u5143\u306E\
    \u9802\u70B9\u756A\u53F7\u3068\u3053\u306E\u30B0\u30E9\u30D5\u306E\u9802\u70B9\
    \u756A\u53F7\u306F\u5BFE\u5FDC\u3057\u3066\u3044\u308B\n    pub fn build(&self)\
    \ -> DirectedGraph<(), E> {\n        DirectedGraph::from_edges(self.m, &self.es)\n\
    \    }\n\n    fn id(&self, mut v: usize) -> usize {\n        if self.n * 3 <=\
    \ v {\n            v -= self.n * 2;\n        }\n        if v < self.n {\n    \
    \        v += self.n;\n        } else if v < self.n * 2 {\n            v -= self.n;\n\
    \        }\n        v\n    }\n}\n\nfn range_to_pair(i: impl RangeBounds<usize>,\
    \ n: usize) -> (usize, usize) {\n    let start = match i.start_bound() {\n   \
    \     std::ops::Bound::Included(&i) => i,\n        std::ops::Bound::Excluded(&i)\
    \ => i + 1,\n        std::ops::Bound::Unbounded => 0,\n    };\n    let end = match\
    \ i.end_bound() {\n        std::ops::Bound::Included(&i) => i + 1,\n        std::ops::Bound::Excluded(&i)\
    \ => i,\n        std::ops::Bound::Unbounded => n,\n    };\n    (start, end)\n\
    }\n"
  dependsOn:
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/graph/range-edge-graph/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-14 05:25:42+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yuki1014/src/main.rs
documentation_of: crates/graph/range-edge-graph/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/range-edge-graph/src/lib.rs
- /library/crates/graph/range-edge-graph/src/lib.rs.html
title: crates/graph/range-edge-graph/src/lib.rs
---
