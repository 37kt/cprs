---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/data-structure/rollback-union-find/src/lib.rs
    title: crates/data-structure/rollback-union-find/src/lib.rs
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
  code: "use std::collections::BTreeMap;\n\nuse rollback_union_find::RollbackUnionFind;\n\
    \nenum ReadQuery {\n    Leader(usize),\n    Same(usize, usize),\n    Size(usize),\n\
    \    Count,\n}\n\n#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub enum QueryResult\
    \ {\n    Leader(usize),\n    Same(bool),\n    Size(usize),\n    Count(usize),\n\
    }\n\npub struct OfflineDynamicConnectivity {\n    built: bool,\n    query_cnt:\
    \ usize,\n    component_cnt: usize,\n    edge_cnt: BTreeMap<(usize, usize), usize>,\n\
    \    appear: BTreeMap<(usize, usize), usize>,\n    edges: Vec<((usize, usize),\
    \ (usize, usize))>,\n    reads: Vec<(usize, ReadQuery)>,\n    ri: usize,\n   \
    \ res: Vec<QueryResult>,\n\n    segsz: usize,\n    seg: Vec<Vec<(usize, usize)>>,\n\
    \n    uf: RollbackUnionFind,\n}\n\nimpl OfflineDynamicConnectivity {\n    pub\
    \ fn new(n: usize) -> Self {\n        Self {\n            built: false,\n    \
    \        query_cnt: 0,\n            component_cnt: n,\n            edge_cnt: BTreeMap::new(),\n\
    \            appear: BTreeMap::new(),\n            edges: vec![],\n          \
    \  reads: vec![],\n            ri: 0,\n            res: vec![],\n\n          \
    \  segsz: 0,\n            seg: vec![],\n\n            uf: RollbackUnionFind::new(n),\n\
    \        }\n    }\n\n    pub fn add_edge(&mut self, x: usize, y: usize) {\n  \
    \      assert!(!self.built);\n        let e = (x.min(y), x.max(y));\n        self.edge_cnt.entry(e).and_modify(|e|\
    \ *e += 1).or_insert(1);\n        if self.edge_cnt[&e] == 1 {\n            self.appear.insert(e,\
    \ self.query_cnt);\n        }\n        self.query_cnt += 1;\n    }\n\n    pub\
    \ fn remove_edge(&mut self, x: usize, y: usize) {\n        assert!(!self.built);\n\
    \        let e = (x.min(y), x.max(y));\n        self.edge_cnt.entry(e).and_modify(|e|\
    \ *e -= 1);\n        if self.edge_cnt[&e] == 0 {\n            self.edges.push((e,\
    \ (self.appear[&e], self.query_cnt)));\n        }\n        self.query_cnt += 1;\n\
    \    }\n\n    pub fn leader(&mut self, x: usize) -> usize {\n        assert!(!self.built);\n\
    \        self.reads.push((self.query_cnt, ReadQuery::Leader(x)));\n        self.query_cnt\
    \ += 1;\n        self.reads.len() - 1\n    }\n\n    pub fn same(&mut self, x:\
    \ usize, y: usize) -> usize {\n        assert!(!self.built);\n        self.reads.push((self.query_cnt,\
    \ ReadQuery::Same(x, y)));\n        self.query_cnt += 1;\n        self.reads.len()\
    \ - 1\n    }\n\n    pub fn size(&mut self, x: usize) -> usize {\n        assert!(!self.built);\n\
    \        self.reads.push((self.query_cnt, ReadQuery::Size(x)));\n        self.query_cnt\
    \ += 1;\n        self.reads.len() - 1\n    }\n\n    pub fn count(&mut self) ->\
    \ usize {\n        assert!(!self.built);\n        self.reads.push((self.query_cnt,\
    \ ReadQuery::Count));\n        self.query_cnt += 1;\n        self.reads.len()\
    \ - 1\n    }\n\n    pub fn build(&mut self) -> Vec<QueryResult> {\n        assert!(!self.built);\n\
    \        self.built = true;\n\n        for (&e, &c) in &self.edge_cnt {\n    \
    \        if c > 0 {\n                self.edges.push((e, (self.appear[&e], self.query_cnt)));\n\
    \            }\n        }\n        self.segsz = 1;\n        while self.segsz <\
    \ self.query_cnt {\n            self.segsz *= 2;\n        }\n        self.seg\
    \ = vec![vec![]; self.segsz * 2];\n        for &((x, y), (l, r)) in &self.edges.clone()\
    \ {\n            self.add(l, r, (x, y), 1, 0, self.segsz);\n        }\n\n    \
    \    self.dfs(1);\n        self.res.clone()\n    }\n\n    fn add(&mut self, a:\
    \ usize, b: usize, e: (usize, usize), k: usize, l: usize, r: usize) {\n      \
    \  if b <= l || r <= a {\n            return;\n        }\n        if a <= l &&\
    \ r <= b {\n            self.seg[k].push(e);\n            return;\n        }\n\
    \        let m = (l + r) / 2;\n        self.add(a, b, e, k * 2, l, m);\n     \
    \   self.add(a, b, e, k * 2 + 1, m, r);\n    }\n\n    fn dfs(&mut self, k: usize)\
    \ {\n        let mut merge_cnt = 0;\n        for &(x, y) in &self.seg[k] {\n \
    \           if self.uf.merge(x, y) {\n                merge_cnt += 1;\n      \
    \      }\n        }\n        self.component_cnt -= merge_cnt;\n        if k <\
    \ self.segsz {\n            self.dfs(k * 2);\n            self.dfs(k * 2 + 1);\n\
    \        } else if self.ri < self.reads.len() && self.reads[self.ri].0 == k -\
    \ self.segsz {\n            match self.reads[self.ri].1 {\n                ReadQuery::Leader(x)\
    \ => {\n                    self.res.push(QueryResult::Leader(self.uf.leader(x)));\n\
    \                }\n                ReadQuery::Same(x, y) => {\n             \
    \       self.res.push(QueryResult::Same(self.uf.same(x, y)));\n              \
    \  }\n                ReadQuery::Size(x) => {\n                    self.res.push(QueryResult::Size(self.uf.size(x)));\n\
    \                }\n                ReadQuery::Count => {\n                  \
    \  self.res.push(QueryResult::Count(self.component_cnt));\n                }\n\
    \            }\n            self.ri += 1;\n        }\n        for _ in 0..self.seg[k].len()\
    \ {\n            self.uf.undo();\n        }\n        self.component_cnt += merge_cnt;\n\
    \    }\n}\n"
  dependsOn:
  - crates/data-structure/rollback-union-find/src/lib.rs
  isVerificationFile: false
  path: crates/algorithm/offline-dynamic-connectivity/src/lib.rs
  requiredBy: []
  timestamp: '2024-02-13 13:29:05+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/algorithm/offline-dynamic-connectivity/src/lib.rs
layout: document
redirect_from:
- /library/crates/algorithm/offline-dynamic-connectivity/src/lib.rs
- /library/crates/algorithm/offline-dynamic-connectivity/src/lib.rs.html
title: crates/algorithm/offline-dynamic-connectivity/src/lib.rs
---
