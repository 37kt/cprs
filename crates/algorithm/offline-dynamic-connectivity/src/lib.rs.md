---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/data-structure/rollback-union-find/src/lib.rs
    title: crates/data-structure/rollback-union-find/src/lib.rs
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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::collections::BTreeMap;\n\npub trait RollbackUnionFindTrait {\n \
    \   type Query: Clone;\n\n    fn add_edge(&mut self, u: usize, v: usize);\n  \
    \  fn undo(&mut self);\n    fn get(&mut self, query: Self::Query);\n}\n\npub struct\
    \ OfflineDynamicConnectivity<Q, UF>\nwhere\n    Q: Clone,\n    UF: RollbackUnionFindTrait<Query\
    \ = Q>,\n{\n    query_cnt: usize,\n    edge_cnt: Vec<BTreeMap<usize, usize>>,\n\
    \    appear: Vec<BTreeMap<usize, usize>>,\n    edges: Vec<((usize, usize), (usize,\
    \ usize))>,\n    reads: Vec<(usize, Q)>,\n    ri: usize,\n    segsz: usize,\n\
    \    uf: UF,\n}\n\nimpl<Q, UF> OfflineDynamicConnectivity<Q, UF>\nwhere\n    Q:\
    \ Clone,\n    UF: RollbackUnionFindTrait<Query = Q>,\n{\n    pub fn new(uf: UF)\
    \ -> Self {\n        Self {\n            query_cnt: 0,\n            edge_cnt:\
    \ vec![],\n            appear: vec![],\n            edges: vec![],\n         \
    \   reads: vec![],\n            ri: 0,\n            segsz: 0,\n            uf,\n\
    \        }\n    }\n\n    pub fn add_edge(&mut self, x: usize, y: usize) {\n  \
    \      let e = (x.min(y), x.max(y));\n        while self.edge_cnt.len() <= e.0\
    \ {\n            self.edge_cnt.push(BTreeMap::new());\n            self.appear.push(BTreeMap::new());\n\
    \        }\n        if *self.edge_cnt[e.0]\n            .entry(e.1)\n        \
    \    .and_modify(|e| *e += 1)\n            .or_insert(1)\n            == 1\n \
    \       {\n            self.appear[e.0].insert(e.1, self.query_cnt);\n       \
    \ }\n    }\n\n    pub fn remove_edge(&mut self, x: usize, y: usize) {\n      \
    \  let e = (x.min(y), x.max(y));\n        let c = self.edge_cnt[e.0].get_mut(&e.1).unwrap();\n\
    \        *c -= 1;\n        if *c == 0 {\n            let a = self.appear[e.0][&e.1];\n\
    \            if a < self.query_cnt {\n                self.edges.push((e, (a,\
    \ self.query_cnt)));\n            }\n            self.edge_cnt[e.0].remove(&e.1);\n\
    \            self.appear[e.0].remove(&e.1);\n        }\n    }\n\n    pub fn get(&mut\
    \ self, query: Q) {\n        self.reads.push((self.query_cnt, query));\n     \
    \   self.query_cnt += 1;\n    }\n\n    pub fn run(&mut self) {\n        for x\
    \ in 0..self.edge_cnt.len() {\n            for (&y, &c) in &self.edge_cnt[x] {\n\
    \                if c > 0 {\n                    self.edges\n                \
    \        .push(((x, y), (self.appear[x][&y], self.query_cnt)));\n            \
    \    }\n            }\n        }\n        self.segsz = 1;\n        while self.segsz\
    \ < self.query_cnt {\n            self.segsz *= 2;\n        }\n        self.dfs(1,\
    \ 0, self.segsz, (0..self.edges.len()).collect());\n    }\n\n    fn dfs(&mut self,\
    \ k: usize, l: usize, r: usize, ids: Vec<usize>) {\n        let mut st = vec![(k,\
    \ l, r, ids)];\n        while let Some((k, l, r, ids)) = st.pop() {\n        \
    \    if k == !0 {\n                for _ in 0..l {\n                    self.uf.undo();\n\
    \                }\n                continue;\n            }\n            let\
    \ m = (l + r) / 2;\n            let mut ls = vec![];\n            let mut rs =\
    \ vec![];\n            let mut cnt = 0;\n            for &i in &ids {\n      \
    \          let ((x, y), (a, b)) = self.edges[i];\n                if b <= l ||\
    \ r <= a {\n                    continue;\n                }\n               \
    \ if a <= l && r <= b {\n                    self.uf.add_edge(x, y);\n       \
    \             cnt += 1;\n                    continue;\n                }\n  \
    \              if a < m {\n                    ls.push(i);\n                }\n\
    \                if m < b {\n                    rs.push(i);\n               \
    \ }\n            }\n            st.push((!0, cnt, !0, vec![]));\n            if\
    \ k < self.segsz {\n                st.push((k * 2 + 1, m, r, rs));\n        \
    \        st.push((k * 2, l, m, ls));\n            } else {\n                while\
    \ self.ri < self.reads.len() && self.reads[self.ri].0 == k - self.segsz {\n  \
    \                  self.uf.get(self.reads[self.ri].1.clone());\n             \
    \       self.ri += 1;\n                }\n            }\n        }\n    }\n}\n"
  dependsOn:
  - crates/data-structure/rollback-union-find/src/lib.rs
  isVerificationFile: false
  path: crates/algorithm/offline-dynamic-connectivity/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-26 06:54:01+00:00'
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
