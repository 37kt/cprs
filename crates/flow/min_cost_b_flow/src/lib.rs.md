---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/data_structure/radix_heap/src/lib.rs
    title: crates/data_structure/radix_heap/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/graph/assignment/src/main.rs
    title: verify/library_checker/graph/assignment/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/graph/min_cost_b_flow/src/main.rs
    title: verify/library_checker/graph/min_cost_b_flow/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://misawa.github.io/others/flow/lets_use_capacity_scaling.html
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// https://misawa.github.io/others/flow/lets_use_capacity_scaling.html\n\
    // TODO: dual \u304C\u3081\u3061\u3083\u304F\u3061\u3083\u9045\u3044\n// TODO:\
    \ \u6D41\u91CF\u5236\u9650, slope\n\nuse std::cmp::Ordering;\nuse std::iter::FusedIterator;\n\
    \nuse radix_heap::RadixHeap;\n\n#[derive(Clone)]\npub struct MinCostBFlow {\n\
    \    n: usize,\n    m: usize,\n\n    graph: Vec<Vec<internal::Edge>>,\n    pos:\
    \ Vec<(usize, usize)>,\n\n    b: Vec<i64>,\n    potential: Vec<i64>,\n\n    farthest:\
    \ u64,\n    dist: Vec<u64>,\n    parent: Vec<(usize, usize)>,\n    pq: RadixHeap,\n\
    \n    excess_vs: Vec<usize>,\n    deficit_vs: Vec<usize>,\n}\n\n#[derive(Debug,\
    \ Clone, Copy)]\npub struct Edge {\n    pub src: usize,\n    pub dst: usize,\n\
    \    pub lower: i64,\n    pub upper: i64,\n    pub cost: i64,\n    pub flow: i64,\n\
    }\n\nimpl Default for MinCostBFlow {\n    fn default() -> Self {\n        Self::new()\n\
    \    }\n}\n\nimpl MinCostBFlow {\n    pub fn new() -> Self {\n        Self {\n\
    \            n: 0,\n            m: 0,\n\n            graph: vec![],\n        \
    \    pos: vec![],\n\n            b: vec![],\n            potential: vec![],\n\n\
    \            farthest: 0,\n            dist: vec![],\n            parent: vec![],\n\
    \            pq: RadixHeap::new(0),\n\n            excess_vs: vec![],\n      \
    \      deficit_vs: vec![],\n        }\n    }\n\n    /// \u9802\u70B9\u3092\u8FFD\
    \u52A0\u3057\u3066\u305D\u306E id \u3092\u8FD4\u3059\n    pub fn add_vertex(&mut\
    \ self) -> usize {\n        self.n += 1;\n        self.graph.push(vec![]);\n \
    \       self.b.push(0);\n        self.n - 1\n    }\n\n    /// \u9802\u70B9\u3092\
    \ n \u500B\u8FFD\u52A0\u3057\u3066\u305D\u308C\u3089\u306E id \u3092\u8FD4\u3059\
    \n    pub fn add_vertices(\n        &mut self,\n        n: usize,\n    ) -> impl\
    \ Iterator<Item = usize> + DoubleEndedIterator + ExactSizeIterator + FusedIterator\
    \ {\n        self.graph.resize(self.n + n, vec![]);\n        self.b.resize(self.n\
    \ + n, 0);\n        self.n += n;\n        self.n - n..self.n\n    }\n\n    ///\
    \ src->dst \u306B\u6D41\u91CF\u5236\u7D04 [lower, upper], cost \u306E\u8FBA\u3092\
    \u5F35\u308A\u3001\u305D\u306E id \u3092\u8FD4\u3059\n    pub fn add_edge(&mut\
    \ self, src: usize, dst: usize, lower: i64, upper: i64, cost: i64) -> usize {\n\
    \        assert!(src < self.n);\n        assert!(dst < self.n);\n        assert!(lower\
    \ <= upper);\n        let i = self.graph[src].len();\n        let j = self.graph[dst].len()\
    \ + if src == dst { 1 } else { 0 };\n        self.graph[src].push(internal::Edge\
    \ {\n            dst,\n            cap: upper,\n            cost,\n          \
    \  flow: 0,\n            rev: j,\n        });\n        self.graph[dst].push(internal::Edge\
    \ {\n            dst: src,\n            cap: -lower,\n            cost: -cost,\n\
    \            flow: 0,\n            rev: i,\n        });\n        self.pos.push((src,\
    \ i));\n        self.m += 1;\n        self.m - 1\n    }\n\n    /// \u9802\u70B9\
    \ v \u306B\u6E67\u304D\u51FA\u3057\u3092\u8FFD\u52A0\u3059\u308B\n    pub fn add_supply(&mut\
    \ self, v: usize, amount: i64) {\n        self.b[v] += amount;\n    }\n\n    ///\
    \ \u9802\u70B9 v \u306B\u5438\u3044\u8FBC\u307F\u3092\u8FFD\u52A0\u3059\u308B\n\
    \    pub fn add_demand(&mut self, v: usize, amount: i64) {\n        self.b[v]\
    \ -= amount;\n    }\n\n    pub fn edge(&self, e: usize) -> Edge {\n        let\
    \ (src, i) = self.pos[e];\n        let e = &self.graph[src][i];\n        Edge\
    \ {\n            src: self.src(e),\n            dst: self.dst(e),\n          \
    \  lower: self.lower(e),\n            upper: self.upper(e),\n            cost:\
    \ self.cost(e),\n            flow: self.flow(e),\n        }\n    }\n\n    pub\
    \ fn edges(\n        &self,\n    ) -> impl Iterator<Item = Edge> + DoubleEndedIterator\
    \ + ExactSizeIterator + FusedIterator + '_\n    {\n        (0..self.m).map(|e|\
    \ self.edge(e))\n    }\n\n    /// min cost b-flow \u3092\u6C42\u3081\u308B  \n\
    \    /// - `Ok(cost)`: \u6700\u5C0F\u8CBB\u7528 (\u6700\u5927\u6D41\u3068\u306F\
    \u9650\u3089\u306A\u3044)  \n    /// - `Err(cost)`: \u6D41\u91CF\u306E\u5236\u7D04\
    \u3092\u6E80\u305F\u3059\u30D5\u30ED\u30FC\u304C\u5B58\u5728\u3057\u306A\u3044\
    \n    pub fn min_cost_b_flow(&mut self) -> Result<i128, i128> {\n        self.potential.resize(self.n,\
    \ 0);\n        self.potential.fill(0);\n\n        for v in 0..self.n {\n     \
    \       for i in 0..self.graph[v].len() {\n                let e = self.graph[v][i];\n\
    \                let rcap = self.residual_cap(&e);\n                if rcap <\
    \ 0 {\n                    self.push(v, i, rcap);\n                    self.b[v]\
    \ -= rcap;\n                    let u = self.dst(&e);\n                    self.b[u]\
    \ += rcap;\n                }\n            }\n        }\n\n        let inf_flow\
    \ = self\n            .graph\n            .iter()\n            .filter_map(|v|\
    \ v.iter().map(|e| self.residual_cap(e)).max())\n            .max()\n        \
    \    .unwrap_or(1);\n        let mut delta = 1;\n        while delta * 2 <= inf_flow\
    \ {\n            delta *= 2;\n        }\n\n        while delta > 0 {\n       \
    \     self.saturate_negative(delta);\n            while self.dual(delta) {\n \
    \               self.primal(delta);\n            }\n            delta /= 2;\n\
    \        }\n\n        let cost = self\n            .graph\n            .iter()\n\
    \            .flatten()\n            .map(|e| e.flow as i128 * e.cost as i128)\n\
    \            .sum::<i128>()\n            / 2;\n\n        if self.excess_vs.is_empty()\
    \ && self.deficit_vs.is_empty() {\n            Ok(cost)\n        } else {\n  \
    \          Err(cost)\n        }\n    }\n\n    /// src->dst \u9593\u306B\u6700\u5927\
    \u6D41\u3092\u6D41\u3057\u305F\u6642\u306E\u6700\u5C0F\u8CBB\u7528\u3092\u6C42\
    \u3081\u308B  \n    /// - `Ok((flow, cost))`: \u6700\u5927\u6D41, \u6700\u5C0F\
    \u8CBB\u7528  \n    /// - `Err(cost)`: \u6D41\u91CF\u306E\u5236\u7D04\u3092\u6E80\
    \u305F\u3059\u30D5\u30ED\u30FC\u304C\u5B58\u5728\u3057\u306A\u3044\n    pub fn\
    \ min_cost_flow(&mut self, src: usize, dst: usize) -> Result<(i64, i128), i128>\
    \ {\n        assert!(src != dst);\n\n        let mut inf_flow = self.b[src].abs();\n\
    \        for e in &self.graph[src] {\n            inf_flow += 0.max(self.residual_cap(e));\n\
    \        }\n\n        self.add_edge(dst, src, 0, inf_flow, 0);\n        if let\
    \ Err(circulation_cost) = self.min_cost_b_flow() {\n            self.graph[src].pop();\n\
    \            self.graph[dst].pop();\n            self.pos.pop();\n           \
    \ self.m -= 1;\n            return Err(circulation_cost);\n        }\n\n     \
    \   let mut inf_flow = self.b[src].abs();\n        for e in &self.graph[src] {\n\
    \            inf_flow += self.residual_cap(e);\n        }\n\n        self.b[src]\
    \ += inf_flow;\n        self.b[dst] -= inf_flow;\n        let cost = match self.min_cost_b_flow()\
    \ {\n            Ok(cost) => cost,\n            Err(cost) => cost,\n        };\n\
    \        self.b[src] -= inf_flow;\n        self.b[dst] += inf_flow;\n\n      \
    \  self.graph[src].pop();\n        self.graph[dst].pop();\n        self.pos.pop();\n\
    \        self.m -= 1;\n\n        Ok((self.b[dst], cost))\n    }\n\n    pub fn\
    \ potentials(&mut self) -> Vec<i64> {\n        self.potential.resize(self.n, 0);\n\
    \        self.potential.fill(0);\n\n        for _ in 0..self.n {\n           \
    \ for v in 0..self.n {\n                for e in &self.graph[v] {\n          \
    \          if self.residual_cap(e) > 0 {\n                        let u = self.dst(e);\n\
    \                        self.potential[u] =\n                            self.potential[u].min(self.potential[self.src(e)]\
    \ + self.cost(e))\n                    }\n                }\n            }\n \
    \       }\n\n        self.potential.clone()\n    }\n\n    fn src(&self, e: &internal::Edge)\
    \ -> usize {\n        self.graph[e.dst][e.rev].dst\n    }\n\n    fn dst(&self,\
    \ e: &internal::Edge) -> usize {\n        e.dst\n    }\n\n    fn flow(&self, e:\
    \ &internal::Edge) -> i64 {\n        e.flow\n    }\n\n    fn lower(&self, e: &internal::Edge)\
    \ -> i64 {\n        -self.graph[e.dst][e.rev].cap\n    }\n\n    fn upper(&self,\
    \ e: &internal::Edge) -> i64 {\n        e.cap\n    }\n\n    fn cost(&self, e:\
    \ &internal::Edge) -> i64 {\n        e.cost\n    }\n\n    fn residual_cost(&self,\
    \ e: &internal::Edge) -> i64 {\n        self.cost(e) + self.potential[self.src(e)]\
    \ - self.potential[self.dst(e)]\n    }\n\n    fn residual_cap(&self, e: &internal::Edge)\
    \ -> i64 {\n        e.cap - e.flow\n    }\n\n    fn push(&mut self, src: usize,\
    \ i: usize, amount: i64) {\n        let internal::Edge { dst, rev, .. } = self.graph[src][i];\n\
    \        self.graph[src][i].flow += amount;\n        self.graph[dst][rev].flow\
    \ -= amount;\n    }\n\n    fn primal(&mut self, delta: i64) {\n        for i in\
    \ 0..self.deficit_vs.len() {\n            let t = self.deficit_vs[i];\n      \
    \      if self.dist[t] > self.farthest {\n                continue;\n        \
    \    }\n            let mut f = -self.b[t];\n            let mut v = t;\n    \
    \        while self.parent[v].0 != !0 && f >= delta {\n                let (src,\
    \ i) = self.parent[v];\n                f = f.min(self.residual_cap(&self.graph[src][i]));\n\
    \                v = src;\n            }\n            f = f.min(self.b[v]);\n\
    \            if f < delta {\n                continue;\n            }\n      \
    \      v = t;\n            while self.parent[v].0 != !0 {\n                let\
    \ (src, i) = self.parent[v];\n                self.push(src, i, f);\n        \
    \        let u = src;\n                self.parent[v] = (!0, !0);\n          \
    \      v = u;\n            }\n            self.b[v] -= f;\n            self.b[t]\
    \ += f;\n        }\n    }\n\n    fn dual(&mut self, delta: i64) -> bool {\n  \
    \      self.dist.resize(self.n, u64::MAX);\n        self.dist.fill(u64::MAX);\n\
    \        self.parent.resize(self.n, (!0, !0));\n        self.parent.fill((!0,\
    \ !0));\n        self.excess_vs.retain(|&v| self.b[v] >= delta);\n        self.deficit_vs.retain(|&v|\
    \ self.b[v] <= -delta);\n\n        self.pq.clear(self.n);\n        for &v in &self.excess_vs\
    \ {\n            self.dist[v] = 0;\n            self.pq.push(0, v);\n        }\n\
    \        self.farthest = 0;\n        let mut deficit_cnt = 0;\n        while let\
    \ Some((d, v)) = self.pq.pop() {\n            if self.dist[v] < d {\n        \
    \        continue;\n            }\n            self.farthest = d;\n          \
    \  if self.b[v] <= -delta {\n                deficit_cnt += 1;\n            }\n\
    \            if deficit_cnt >= self.deficit_vs.len() {\n                break;\n\
    \            }\n            for (i, e) in self.graph[v].iter().enumerate() {\n\
    \                if self.residual_cap(e) >= delta {\n                    let u\
    \ = self.dst(e);\n                    let new_d = d + self.residual_cost(e) as\
    \ u64;\n                    if self.dist[u] > new_d {\n                      \
    \  self.dist[u] = new_d;\n                        self.pq.push(new_d, u);\n  \
    \                      self.parent[u] = (v, i);\n                    }\n     \
    \           }\n            }\n        }\n\n        for v in 0..self.n {\n    \
    \        self.potential[v] += self.dist[v].min(self.farthest) as i64;\n      \
    \  }\n\n        deficit_cnt > 0\n    }\n\n    fn saturate_negative(&mut self,\
    \ delta: i64) {\n        self.excess_vs.clear();\n        self.deficit_vs.clear();\n\
    \        for v in 0..self.n {\n            for i in 0..self.graph[v].len() {\n\
    \                let e = self.graph[v][i];\n                let rcap = self.residual_cap(&e);\n\
    \                let rcost = self.residual_cost(&e);\n                if rcost\
    \ < 0 && rcap >= delta {\n                    self.push(v, i, rcap);\n       \
    \             self.b[v] -= rcap;\n                    let u = self.dst(&e);\n\
    \                    self.b[u] += rcap;\n                }\n            }\n  \
    \      }\n        for v in 0..self.n {\n            match self.b[v].cmp(&0) {\n\
    \                Ordering::Greater => self.excess_vs.push(v),\n              \
    \  Ordering::Less => self.deficit_vs.push(v),\n                Ordering::Equal\
    \ => {}\n            }\n        }\n    }\n}\n\nmod internal {\n    #[derive(Clone,\
    \ Copy)]\n    pub(crate) struct Edge {\n        pub(crate) dst: usize,\n     \
    \   pub(crate) cap: i64,\n        pub(crate) cost: i64,\n        pub(crate) flow:\
    \ i64,\n        pub(crate) rev: usize,\n    }\n}\n"
  dependsOn:
  - crates/data_structure/radix_heap/src/lib.rs
  isVerificationFile: false
  path: crates/flow/min_cost_b_flow/src/lib.rs
  requiredBy: []
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/graph/min_cost_b_flow/src/main.rs
  - verify/library_checker/graph/assignment/src/main.rs
documentation_of: crates/flow/min_cost_b_flow/src/lib.rs
layout: document
redirect_from:
- /library/crates/flow/min_cost_b_flow/src/lib.rs
- /library/crates/flow/min_cost_b_flow/src/lib.rs.html
title: crates/flow/min_cost_b_flow/src/lib.rs
---
