---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/assignment/src/main.rs
    title: verify/assignment/src/main.rs
  - icon: ':x:'
    path: verify/min_cost_b_flow/src/main.rs
    title: verify/min_cost_b_flow/src/main.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
  attributes:
    links:
    - https://misawa.github.io/others/flow/lets_use_capacity_scaling.html
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.5/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.5/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// reference: https://misawa.github.io/others/flow/lets_use_capacity_scaling.html\n\
    \nuse std::{cmp::Reverse, collections::BinaryHeap};\n\nconst UNREACHABLE: i64\
    \ = std::i64::MAX;\nconst SCALING_FACTOR: i64 = 2;\n\n#[derive(Debug, Clone, Copy)]\n\
    pub struct Edge {\n    pub from: usize,\n    pub to: usize,\n    pub lower: i64,\n\
    \    pub upper: i64,\n    pub cost: i64,\n    pub gain: i64,\n    pub flow: i64,\n\
    }\n\npub struct MinCostBFlow {\n    n: usize,\n    edges: Vec<Edge_>,\n    head:\
    \ Vec<usize>,\n    next: Vec<usize>,\n    b: Vec<i64>,\n    farthest: i64,\n \
    \   potential: Vec<i64>,\n    dist: Vec<i64>,\n    parent: Vec<usize>,\n    pq:\
    \ BinaryHeap<Reverse<(i64, usize)>>,\n    excess_vs: Vec<usize>,\n    deficit_vs:\
    \ Vec<usize>,\n}\n\nimpl MinCostBFlow {\n    pub fn new() -> Self {\n        Self\
    \ {\n            n: 0,\n            edges: vec![],\n            head: vec![],\n\
    \            next: vec![],\n            b: vec![],\n            farthest: 0,\n\
    \            potential: vec![],\n            dist: vec![],\n            parent:\
    \ vec![],\n            pq: Default::default(),\n            excess_vs: vec![],\n\
    \            deficit_vs: vec![],\n        }\n    }\n\n    pub fn add_vertex(&mut\
    \ self) -> usize {\n        self.n += 1;\n        self.head.push(!0);\n      \
    \  self.b.push(0);\n        self.n - 1\n    }\n\n    pub fn add_vertices(&mut\
    \ self, size: usize) -> Vec<usize> {\n        self.n += size;\n        self.head.append(&mut\
    \ vec![!0; size]);\n        self.b.append(&mut vec![0; size]);\n        (self.n\
    \ - size..self.n).collect()\n    }\n\n    pub fn add_edge(&mut self, from: usize,\
    \ to: usize, lower: i64, upper: i64, cost: i64) -> usize {\n        assert!(lower\
    \ <= upper);\n        assert!(from < self.n);\n        assert!(to < self.n);\n\
    \        let m = self.edges.len();\n        self.next.push(self.head[from]);\n\
    \        self.head[from] = m;\n        self.next.push(self.head[to]);\n      \
    \  self.head[to] = m + 1;\n        self.edges.push(Edge_ {\n            to,\n\
    \            cap: upper,\n            cost,\n            flow: 0,\n        });\n\
    \        self.edges.push(Edge_ {\n            to: from,\n            cap: -lower,\n\
    \            cost: -cost,\n            flow: 0,\n        });\n        m / 2\n\
    \    }\n\n    pub fn add_supply(&mut self, v: usize, amount: i64) {\n        self.b[v]\
    \ += amount;\n    }\n\n    pub fn add_demand(&mut self, v: usize, amount: i64)\
    \ {\n        self.b[v] -= amount;\n    }\n\n    pub fn get_edge(&self, e: usize)\
    \ -> Edge {\n        assert!(e <= self.edges.len() / 2);\n        let e = e *\
    \ 2;\n        Edge {\n            from: self.from(e),\n            to: self.to(e),\n\
    \            lower: self.lower(e),\n            upper: self.upper(e),\n      \
    \      cost: self.cost(e),\n            gain: self.gain(e),\n            flow:\
    \ self.flow(e),\n        }\n    }\n\n    pub fn get_edges(&self) -> Vec<Edge>\
    \ {\n        (0..self.edges.len() / 2)\n            .map(|e| self.get_edge(e))\n\
    \            .collect()\n    }\n\n    pub fn min_cost_b_flow(&mut self) -> Result<i64,\
    \ i64> {\n        self.potential.resize(self.n, 0);\n        for u in 0..self.n\
    \ {\n            let mut e = self.head[u];\n            while e != !0 {\n    \
    \            let rcap = self.residual_cap(e);\n                if rcap < 0 {\n\
    \                    self.push(e, rcap);\n                    self.b[u] -= rcap;\n\
    \                    let v = self.to(e);\n                    self.b[v] += rcap;\n\
    \                }\n                e = self.next[e];\n            }\n       \
    \ }\n\n        let mut inf_flow = 1;\n        for e in 0..self.edges.len() {\n\
    \            inf_flow = inf_flow.max(self.residual_cap(e));\n        }\n     \
    \   let mut delta = 1;\n        while delta * SCALING_FACTOR <= inf_flow {\n \
    \           delta *= SCALING_FACTOR;\n        }\n\n        while delta > 0 {\n\
    \            self.saturate_negative(delta);\n            while self.dual(delta)\
    \ {\n                self.primal(delta);\n            }\n            delta /=\
    \ SCALING_FACTOR;\n        }\n\n        let mut value = 0;\n        for Edge_\
    \ { flow, cost, .. } in &self.edges {\n            value += flow * cost;\n   \
    \     }\n        value /= 2;\n\n        if self.excess_vs.is_empty() && self.deficit_vs.is_empty()\
    \ {\n            Ok(value)\n        } else {\n            Err(value)\n       \
    \ }\n    }\n\n    /// (\u30B3\u30B9\u30C8, \u6D41\u91CF)\n    pub fn min_cost_flow(&mut\
    \ self, s: usize, t: usize) -> Result<(i64, i64), (i64, i64)> {\n        assert!(s\
    \ != t);\n        let mut inf_flow = self.b[s].abs();\n        let mut e = self.head[s];\n\
    \        while e != !0 {\n            inf_flow += 0.max(self.edges[e].cap);\n\
    \            e = self.next[e];\n        }\n\n        self.add_edge(t, s, 0, inf_flow,\
    \ 0);\n        if let Err(circulation_value) = self.min_cost_b_flow() {\n    \
    \        self.head[s] = self.next[s];\n            self.head[t] = self.next[t];\n\
    \            self.edges.pop();\n            self.edges.pop();\n            return\
    \ Err((circulation_value, 0));\n        }\n\n        let mut inf_flow = self.b[s].abs();\n\
    \        let mut e = self.head[s];\n        while e != !0 {\n            inf_flow\
    \ += self.residual_cap(e);\n            e = self.next[e];\n        }\n       \
    \ self.b[s] += inf_flow;\n        self.b[t] -= inf_flow;\n        let mf_value\
    \ = match self.min_cost_b_flow() {\n            Ok(v) => v,\n            Err(v)\
    \ => v,\n        };\n        self.b[s] -= inf_flow;\n        self.b[t] += inf_flow;\n\
    \n        self.head[s] = self.next[s];\n        self.head[t] = self.next[t];\n\
    \        self.edges.pop();\n        self.edges.pop();\n        Ok((mf_value, self.b[t]))\n\
    \    }\n\n    pub fn get_result_value_i128(&mut self) -> i128 {\n        let mut\
    \ value = 0;\n        for e in &self.edges {\n            value += e.flow as i128\
    \ * e.cost as i128;\n        }\n        value / 2\n    }\n\n    pub fn get_potential(&mut\
    \ self) -> Vec<i64> {\n        self.potential = vec![0; self.n];\n        for\
    \ _ in 0..self.n {\n            for e in 0..self.edges.len() {\n             \
    \   if self.residual_cap(e) > 0 {\n                    let to = self.to(e);\n\
    \                    self.potential[to] =\n                        self.potential[to].min(self.potential[self.from(e)]\
    \ + self.cost(e));\n                }\n            }\n        }\n        self.potential.clone()\n\
    \    }\n\n    fn from(&self, e: usize) -> usize {\n        self.edges[e ^ 1].to\n\
    \    }\n\n    fn to(&self, e: usize) -> usize {\n        self.edges[e].to\n  \
    \  }\n\n    fn flow(&self, e: usize) -> i64 {\n        self.edges[e].flow\n  \
    \  }\n\n    fn lower(&self, e: usize) -> i64 {\n        -self.edges[e ^ 1].cap\n\
    \    }\n\n    fn upper(&self, e: usize) -> i64 {\n        self.edges[e].cap\n\
    \    }\n\n    fn cost(&self, e: usize) -> i64 {\n        self.edges[e].cost\n\
    \    }\n\n    fn gain(&self, e: usize) -> i64 {\n        -self.edges[e].cost\n\
    \    }\n\n    fn push(&mut self, e: usize, amount: i64) {\n        self.edges[e].flow\
    \ += amount;\n        self.edges[e ^ 1].flow -= amount;\n    }\n\n    fn residual_cost(&self,\
    \ e: usize) -> i64 {\n        self.cost(e) + self.potential[self.from(e)] - self.potential[self.to(e)]\n\
    \    }\n\n    fn residual_cap(&self, e: usize) -> i64 {\n        self.edges[e].cap\
    \ - self.edges[e].flow\n    }\n\n    fn dual(&mut self, delta: i64) -> bool {\n\
    \        self.dist = vec![UNREACHABLE; self.n];\n        self.parent = vec![!0;\
    \ self.n];\n        // self.excess_vs.retain(|v| self.b[*v] >= delta);\n     \
    \   for i in (0..self.excess_vs.len()).rev() {\n            if self.b[self.excess_vs[i]]\
    \ < delta {\n                self.excess_vs.swap_remove(i);\n            }\n \
    \       }\n        // self.deficit_vs.retain(|v| self.b[*v] <= -delta);\n    \
    \    for i in (0..self.deficit_vs.len()).rev() {\n            if self.b[self.deficit_vs[i]]\
    \ > -delta {\n                self.deficit_vs.swap_remove(i);\n            }\n\
    \        }\n        for &v in &self.excess_vs {\n            self.dist[v] = 0;\n\
    \            self.pq.push(Reverse((0, v)));\n        }\n        self.farthest\
    \ = 0;\n        let mut deficit_count = 0;\n        while let Some(Reverse((d,\
    \ u))) = self.pq.pop() {\n            if self.dist[u] < d {\n                continue;\n\
    \            }\n            self.farthest = d;\n            if self.b[u] <= -delta\
    \ {\n                deficit_count += 1;\n            }\n            if deficit_count\
    \ >= self.deficit_vs.len() {\n                break;\n            }\n        \
    \    let mut e = self.head[u];\n            while e != !0 {\n                if\
    \ self.residual_cap(e) >= delta {\n                    let v = self.to(e);\n \
    \                   let new_dist = d + self.residual_cost(e);\n              \
    \      if self.dist[v] > new_dist {\n                        self.dist[v] = new_dist;\n\
    \                        self.pq.push(Reverse((new_dist, v)));\n             \
    \           self.parent[v] = e;\n                    }\n                }\n  \
    \              e = self.next[e];\n            }\n        }\n        self.pq.clear();\n\
    \        for v in 0..self.n {\n            self.potential[v] += self.dist[v].min(self.farthest);\n\
    \        }\n        deficit_count > 0\n    }\n\n    fn primal(&mut self, delta:\
    \ i64) {\n        for i in 0..self.deficit_vs.len() {\n            let t = self.deficit_vs[i];\n\
    \            if self.dist[t] > self.farthest {\n                continue;\n  \
    \          }\n            let mut f = -self.b[t];\n            let mut v = t;\n\
    \            while self.parent[v] != !0 && f >= delta {\n                f = f.min(self.residual_cap(self.parent[v]));\n\
    \                v = self.from(self.parent[v]);\n            }\n            f\
    \ = f.min(self.b[v]);\n            if f < delta {\n                continue;\n\
    \            }\n            v = t;\n            while self.parent[v] != !0 {\n\
    \                self.push(self.parent[v], f);\n                let u = self.from(self.parent[v]);\n\
    \                self.parent[v] = !0;\n                v = u;\n            }\n\
    \            self.b[t] += f;\n            self.b[v] -= f;\n        }\n    }\n\n\
    \    fn saturate_negative(&mut self, delta: i64) {\n        self.excess_vs.clear();\n\
    \        self.deficit_vs.clear();\n        for u in 0..self.n {\n            let\
    \ mut e = self.head[u];\n            while e != !0 {\n                let rcap\
    \ = self.residual_cap(e);\n                let rcost = self.residual_cost(e);\n\
    \                if rcost < 0 && rcap >= delta {\n                    self.push(e,\
    \ rcap);\n                    self.b[u] -= rcap;\n                    let v =\
    \ self.to(e);\n                    self.b[v] += rcap;\n                }\n   \
    \             e = self.next[e];\n            }\n        }\n        for v in 0..self.n\
    \ {\n            if self.b[v] > 0 {\n                self.excess_vs.push(v);\n\
    \            } else if self.b[v] < 0 {\n                self.deficit_vs.push(v);\n\
    \            }\n        }\n    }\n}\n\nstruct Edge_ {\n    to: usize,\n    cap:\
    \ i64,\n    cost: i64,\n    flow: i64,\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/graph/min-cost-b-flow/src/lib.rs
  requiredBy: []
  timestamp: '2023-05-23 15:04:49+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - verify/assignment/src/main.rs
  - verify/min_cost_b_flow/src/main.rs
documentation_of: crates/graph/min-cost-b-flow/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/min-cost-b-flow/src/lib.rs
- /library/crates/graph/min-cost-b-flow/src/lib.rs.html
title: crates/graph/min-cost-b-flow/src/lib.rs
---
