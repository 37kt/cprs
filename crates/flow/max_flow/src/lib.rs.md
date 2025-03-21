---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/cast.rs
    title: crates/algebra/numeric_traits/src/cast.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/inf.rs
    title: crates/algebra/numeric_traits/src/inf.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/integer.rs
    title: crates/algebra/numeric_traits/src/integer.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/lib.rs
    title: crates/algebra/numeric_traits/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/numeric.rs
    title: crates/algebra/numeric_traits/src/numeric.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/signed.rs
    title: crates/algebra/numeric_traits/src/signed.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/zero_one.rs
    title: crates/algebra/numeric_traits/src/zero_one.rs
  - icon: ':heavy_check_mark:'
    path: crates/flow/max_flow/src/graph.rs
    title: crates/flow/max_flow/src/graph.rs
  - icon: ':heavy_check_mark:'
    path: crates/flow/max_flow/src/queue.rs
    title: crates/flow/max_flow/src/queue.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/flow/binary_optimization/src/lib.rs
    title: crates/flow/binary_optimization/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/flow/max_flow/src/graph.rs
    title: crates/flow/max_flow/src/graph.rs
  - icon: ':heavy_check_mark:'
    path: crates/flow/max_flow/src/queue.rs
    title: crates/flow/max_flow/src/queue.rs
  - icon: ':heavy_check_mark:'
    path: crates/flow/multivalued_optimization/src/lib.rs
    title: crates/flow/multivalued_optimization/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yukicoder/flow/yuki1479_maxflow/src/main.rs
    title: verify/yukicoder/flow/yuki1479_maxflow/src/main.rs
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
  code: "/// TODO: \u8FBA\u306E\u5909\u66F4\nuse std::iter::FusedIterator;\n\nuse\
    \ numeric_traits::Integer;\n\nmod graph;\nmod queue;\n\n/// dinic + scaling  \n\
    #[derive(Clone)]\npub struct MaxFlow {\n    edges: Vec<Vec<graph::Edge>>,\n  \
    \  pos: Vec<(usize, usize)>,\n\n    iter: Vec<usize>,\n    dist: Vec<u64>,\n \
    \   queue: queue::Queue,\n\n    zero: u64,\n\n    src: Option<usize>,\n}\n\n#[derive(Debug,\
    \ Clone, Copy)]\npub struct Edge {\n    pub src: usize,\n    pub dst: usize,\n\
    \    pub cap: i64,\n    pub flow: i64,\n}\n\nimpl MaxFlow {\n    pub fn new()\
    \ -> Self {\n        Self {\n            edges: vec![],\n            pos: vec![],\n\
    \n            iter: vec![],\n            dist: vec![],\n            queue: queue::Queue::new(),\n\
    \n            zero: 1 << 60,\n\n            src: None,\n        }\n    }\n\n \
    \   pub fn num_vertices(&self) -> usize {\n        self.edges.len()\n    }\n\n\
    \    pub fn num_edges(&self) -> usize {\n        self.pos.len()\n    }\n\n   \
    \ pub fn add_vertex(&mut self) -> usize {\n        self.src = None;\n\n      \
    \  let v = self.edges.len();\n        self.edges.push(vec![]);\n        self.iter.push(0);\n\
    \        self.dist.push(self.zero);\n        v\n    }\n\n    pub fn add_vertices(&mut\
    \ self, n: usize) -> Vec<usize> {\n        self.src = None;\n\n        let v =\
    \ self.edges.len();\n        self.edges.resize(v + n, vec![]);\n        self.iter.resize(v\
    \ + n, 0);\n        self.dist.resize(v + n, self.zero);\n        (v..v + n).collect()\n\
    \    }\n\n    pub fn add_edge(&mut self, src: usize, dst: usize, cap: i64) ->\
    \ usize {\n        assert!(src < self.num_vertices());\n        assert!(dst <\
    \ self.num_vertices());\n        assert!(cap >= 0);\n\n        self.src = None;\n\
    \n        let e = self.pos.len();\n        let i = self.edges[src].len();\n  \
    \      let j = self.edges[dst].len() + if src == dst { 1 } else { 0 };\n     \
    \   self.edges[src].push(graph::Edge { dst, cap, rev: j });\n        self.edges[dst].push(graph::Edge\
    \ {\n            dst: src,\n            cap: 0,\n            rev: i,\n       \
    \ });\n        self.pos.push((src, i));\n        e\n    }\n\n    pub fn edge(&self,\
    \ e: usize) -> Edge {\n        let (src, i) = self.pos[e];\n        let e1 = self.edges[src][i];\n\
    \        let e2 = self.edges[e1.dst][e1.rev];\n        Edge {\n            src,\n\
    \            dst: e1.dst,\n            cap: e1.cap + e2.cap,\n            flow:\
    \ e2.cap,\n        }\n    }\n\n    pub fn edges(\n        &self,\n    ) -> impl\
    \ Iterator<Item = Edge> + DoubleEndedIterator + ExactSizeIterator + FusedIterator\
    \ + '_\n    {\n        (0..self.num_edges()).map(|e| self.edge(e))\n    }\n\n\
    \    pub fn max_flow(&mut self, src: usize, dst: usize, limit: Option<i64>) ->\
    \ i64 {\n        assert!(src != dst);\n        assert!(src < self.num_vertices());\n\
    \        assert!(dst < self.num_vertices());\n\n        self.src = Some(src);\n\
    \        let limit = limit.unwrap_or(i64::MAX);\n\n        let max_cap = self\n\
    \            .edges\n            .iter()\n            .flatten()\n           \
    \ .map(|e| e.cap)\n            .max()\n            .unwrap_or(0);\n        if\
    \ max_cap == 0 {\n            return 0;\n        }\n\n        let mut flow = 0;\n\
    \        for f in (0..=limit.min(max_cap).floor_log2()).map(|i| 1 << i).rev()\
    \ {\n            while flow <= limit - f && self.build_augmenting_path(src, dst,\
    \ f) {\n                self.iter.fill(0);\n                flow += self.find_augmenting_path(src,\
    \ dst, f, limit - flow);\n            }\n        }\n\n        flow\n    }\n\n\
    \    /// src \u5074\u304C 0, dst \u5074\u304C 1  \n    /// max_flow \u306E\u5F8C\
    \u306B\u547C\u3073\u51FA\u3059\n    pub fn min_cut(&mut self) -> Vec<usize> {\n\
    \        let src = self.src.expect(\"max_flow is not called\");\n\n        let\
    \ mut res = vec![1; self.num_vertices()];\n\n        self.queue.clear();\n   \
    \     self.queue.set_capacity(self.num_vertices());\n        res[src] = 0;\n \
    \       self.queue.push(src);\n\n        while let Some(v) = self.queue.pop()\
    \ {\n            for e in &self.edges[v] {\n                if e.cap > 0 && res[e.dst]\
    \ == 1 {\n                    res[e.dst] = 0;\n                    self.queue.push(e.dst);\n\
    \                }\n            }\n        }\n        res\n    }\n\n    fn build_augmenting_path(&mut\
    \ self, src: usize, dst: usize, base: i64) -> bool {\n        self.zero -= 1 <<\
    \ 30;\n\n        self.queue.clear();\n        self.queue.set_capacity(self.num_vertices());\n\
    \n        self.dist[src] = self.zero;\n        self.queue.push(src);\n       \
    \ while let Some(v) = self.queue.pop() {\n            for e in &self.edges[v]\
    \ {\n                if e.cap >= base && self.dist[e.dst] > self.dist[v] + 1 {\n\
    \                    self.dist[e.dst] = self.dist[v] + 1;\n                  \
    \  self.queue.push(e.dst);\n                    if e.dst == dst {\n          \
    \              return true;\n                    }\n                }\n      \
    \      }\n        }\n        false\n    }\n\n    fn find_augmenting_path(&mut\
    \ self, v: usize, dst: usize, base: i64, flow: i64) -> i64 {\n        if v ==\
    \ dst {\n            return flow;\n        }\n\n        let mut sum = 0;\n   \
    \     while self.iter[v] < self.edges[v].len() {\n            let e = self.edges[v][self.iter[v]];\n\
    \            if e.cap >= base && self.dist[v] < self.dist[e.dst] {\n         \
    \       let diff = self.find_augmenting_path(e.dst, dst, base, e.cap.min(flow\
    \ - sum));\n                if diff > 0 {\n                    self.edges[v][self.iter[v]].cap\
    \ -= diff;\n                    self.edges[e.dst][e.rev].cap += diff;\n      \
    \              sum += diff;\n                    if flow - sum < base {\n    \
    \                    break;\n                    }\n                }\n      \
    \      }\n            self.iter[v] += 1;\n        }\n        sum\n    }\n}\n"
  dependsOn:
  - crates/algebra/numeric_traits/src/cast.rs
  - crates/algebra/numeric_traits/src/inf.rs
  - crates/algebra/numeric_traits/src/integer.rs
  - crates/algebra/numeric_traits/src/lib.rs
  - crates/algebra/numeric_traits/src/numeric.rs
  - crates/algebra/numeric_traits/src/signed.rs
  - crates/algebra/numeric_traits/src/zero_one.rs
  - crates/flow/max_flow/src/graph.rs
  - crates/flow/max_flow/src/queue.rs
  isVerificationFile: false
  path: crates/flow/max_flow/src/lib.rs
  requiredBy:
  - crates/flow/multivalued_optimization/src/lib.rs
  - crates/flow/binary_optimization/src/lib.rs
  - crates/flow/max_flow/src/graph.rs
  - crates/flow/max_flow/src/queue.rs
  timestamp: '2025-03-20 09:27:03+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yukicoder/flow/yuki1479_maxflow/src/main.rs
documentation_of: crates/flow/max_flow/src/lib.rs
layout: document
redirect_from:
- /library/crates/flow/max_flow/src/lib.rs
- /library/crates/flow/max_flow/src/lib.rs.html
title: crates/flow/max_flow/src/lib.rs
---
