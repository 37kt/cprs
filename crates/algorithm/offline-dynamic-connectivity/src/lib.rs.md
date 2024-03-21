---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/dynamic_graph_vertex_add_component_sum/src/main.rs
    title: verify/dynamic_graph_vertex_add_component_sum/src/main.rs
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
  code: "use std::collections::BTreeMap;\n\npub trait RollbackUnionFindTrait {\n \
    \   type Query: Clone;\n\n    fn add_edge(&mut self, u: usize, v: usize);\n  \
    \  fn undo(&mut self);\n    fn get(&mut self, query: Self::Query);\n}\n\npub struct\
    \ OfflineDynamicConnectivity<Q, UF>\nwhere\n    Q: Clone,\n    UF: RollbackUnionFindTrait<Query\
    \ = Q>,\n{\n    query_cnt: usize,\n    edge_cnt: BTreeMap<(usize, usize), usize>,\n\
    \    appear: BTreeMap<(usize, usize), usize>,\n    edges: Vec<((usize, usize),\
    \ (usize, usize))>,\n    reads: Vec<(usize, Q)>,\n    ri: usize,\n    segsz: usize,\n\
    \    seg: Vec<Vec<(usize, usize)>>,\n    uf: UF,\n}\n\nimpl<Q, UF> OfflineDynamicConnectivity<Q,\
    \ UF>\nwhere\n    Q: Clone,\n    UF: RollbackUnionFindTrait<Query = Q>,\n{\n \
    \   pub fn new(uf: UF) -> Self {\n        Self {\n            query_cnt: 0,\n\
    \            edge_cnt: BTreeMap::new(),\n            appear: BTreeMap::new(),\n\
    \            edges: vec![],\n            reads: vec![],\n            ri: 0,\n\
    \            segsz: 0,\n            seg: vec![],\n            uf,\n        }\n\
    \    }\n\n    pub fn add_edge(&mut self, x: usize, y: usize) {\n        let e\
    \ = (x.min(y), x.max(y));\n        self.edge_cnt.entry(e).and_modify(|e| *e +=\
    \ 1).or_insert(1);\n        self.query_cnt += 1;\n        if self.edge_cnt[&e]\
    \ == 1 {\n            self.appear.insert(e, self.query_cnt);\n        }\n    }\n\
    \n    pub fn remove_edge(&mut self, x: usize, y: usize) {\n        let e = (x.min(y),\
    \ x.max(y));\n        self.edge_cnt.entry(e).and_modify(|e| *e -= 1);\n      \
    \  self.query_cnt += 1;\n        if self.edge_cnt[&e] == 0 {\n            self.edges.push((e,\
    \ (self.appear[&e], self.query_cnt)));\n        }\n    }\n\n    pub fn get(&mut\
    \ self, query: Q) {\n        self.reads.push((self.query_cnt, query));\n    }\n\
    \n    pub fn run(&mut self) {\n        for (&e, &c) in &self.edge_cnt {\n    \
    \        if c > 0 {\n                self.edges.push((e, (self.appear[&e], self.query_cnt\
    \ + 1)));\n            }\n        }\n        self.segsz = 1;\n        while self.segsz\
    \ < self.query_cnt + 1 {\n            self.segsz *= 2;\n        }\n        self.seg\
    \ = vec![vec![]; self.segsz * 2];\n        for &((x, y), (l, r)) in &self.edges.clone()\
    \ {\n            self.add(l, r, (x, y), 1, 0, self.segsz);\n        }\n\n    \
    \    self.dfs(1);\n    }\n\n    fn add(&mut self, a: usize, b: usize, e: (usize,\
    \ usize), k: usize, l: usize, r: usize) {\n        if b <= l || r <= a {\n   \
    \         return;\n        }\n        if a <= l && r <= b {\n            self.seg[k].push(e);\n\
    \            return;\n        }\n        let m = (l + r) / 2;\n        self.add(a,\
    \ b, e, k * 2, l, m);\n        self.add(a, b, e, k * 2 + 1, m, r);\n    }\n\n\
    \    fn dfs(&mut self, k: usize) {\n        for &(x, y) in &self.seg[k] {\n  \
    \          self.uf.add_edge(x, y);\n        }\n        if k < self.segsz {\n \
    \           self.dfs(k * 2);\n            self.dfs(k * 2 + 1);\n        } else\
    \ {\n            while self.ri < self.reads.len() && self.reads[self.ri].0 ==\
    \ k - self.segsz {\n                self.uf.get(self.reads[self.ri].1.clone());\n\
    \                self.ri += 1;\n            }\n        }\n        for _ in 0..self.seg[k].len()\
    \ {\n            self.uf.undo();\n        }\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/algorithm/offline-dynamic-connectivity/src/lib.rs
  requiredBy: []
  timestamp: '2024-03-21 16:19:45+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/dynamic_graph_vertex_add_component_sum/src/main.rs
documentation_of: crates/algorithm/offline-dynamic-connectivity/src/lib.rs
layout: document
redirect_from:
- /library/crates/algorithm/offline-dynamic-connectivity/src/lib.rs
- /library/crates/algorithm/offline-dynamic-connectivity/src/lib.rs.html
title: crates/algorithm/offline-dynamic-connectivity/src/lib.rs
---
