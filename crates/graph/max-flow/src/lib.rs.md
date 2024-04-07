---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/graph/max-flow-lower-bound/src/lib.rs
    title: crates/graph/max-flow-lower-bound/src/lib.rs
  - icon: ':warning:'
    path: crates/graph/project-selection/src/lib.rs
    title: crates/graph/project-selection/src/lib.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://atcoder.jp/contests/abc225/tasks/abc225_g
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verify: https://atcoder.jp/contests/abc225/tasks/abc225_g\n\nuse std::collections::VecDeque;\n\
    \npub type FlowType = i64;\nconst INF: FlowType = std::i64::MAX / 2;\n\n/// dinic\u6CD5\
    +scaling\u3067\u6700\u5927\u6D41\u3092\u6C42\u3081\u308B\npub struct MaxFlow {\n\
    \    n: usize,\n    edges: Vec<Edge_>,\n    head: Vec<usize>,\n    next: Vec<usize>,\n\
    \    min_cost: Vec<i32>,\n    iter: Vec<usize>,\n}\n\n#[derive(Debug, Clone, Copy)]\n\
    pub struct Edge {\n    pub from: usize,\n    pub to: usize,\n    pub cap: FlowType,\n\
    \    pub flow: FlowType,\n}\n\nimpl MaxFlow {\n    pub fn new(n: usize) -> Self\
    \ {\n        Self {\n            n,\n            edges: vec![],\n            head:\
    \ vec![!0; n],\n            next: vec![],\n            min_cost: vec![],\n   \
    \         iter: vec![],\n        }\n    }\n\n    pub fn add_edge(&mut self, from:\
    \ usize, to: usize, cap: FlowType) -> usize {\n        assert!(from != to);\n\
    \        assert!(from < self.n);\n        assert!(to < self.n);\n        assert!(cap\
    \ >= 0);\n        let m = self.edges.len();\n        self.next.push(self.head[from]);\n\
    \        self.head[from] = m;\n        self.next.push(self.head[to]);\n      \
    \  self.head[to] = m + 1;\n        self.edges.push(Edge_ { to, cap });\n     \
    \   self.edges.push(Edge_ { to: from, cap: 0 });\n        m / 2\n    }\n\n   \
    \ pub fn max_flow(&mut self, s: usize, t: usize) -> FlowType {\n        assert!(s\
    \ != t);\n        let max_cap = self.edges.iter().map(|e| e.cap).max().unwrap_or(0);\n\
    \        if max_cap == 0 {\n            return 0;\n        }\n        let mut\
    \ flow = 0;\n        for i in (0..64 - max_cap.leading_zeros()).rev() {\n    \
    \        let now = 1 << i;\n            while self.build_augment_path(s, t, now)\
    \ {\n                self.iter = self.head.clone();\n                flow += self.find_augment_path(s,\
    \ t, now, INF);\n            }\n        }\n        flow\n    }\n\n    pub fn min_cut(&self,\
    \ s: usize) -> Vec<bool> {\n        let mut vis = vec![false; self.n];\n     \
    \   let mut q = VecDeque::new();\n        q.push_back(s);\n        while let Some(v)\
    \ = q.pop_front() {\n            vis[v] = true;\n            let mut e = self.head[v];\n\
    \            while e != !0 {\n                let Edge_ { to, cap } = self.edges[e];\n\
    \                if cap != 0 && !vis[to] {\n                    vis[to] = true;\n\
    \                    q.push_back(to);\n                }\n                e =\
    \ self.next[e];\n            }\n        }\n        vis\n    }\n\n    pub fn edges(&self)\
    \ -> Vec<Edge> {\n        (0..self.edges.len())\n            .step_by(2)\n   \
    \         .map(|i| {\n                let e = &self.edges[i];\n              \
    \  let f = &self.edges[i + 1];\n                Edge {\n                    from:\
    \ f.to,\n                    to: e.to,\n                    cap: e.cap + f.cap,\n\
    \                    flow: f.cap,\n                }\n            })\n       \
    \     .collect()\n    }\n\n    fn build_augment_path(&mut self, s: usize, t: usize,\
    \ base: FlowType) -> bool {\n        self.min_cost = vec![-1; self.n];\n     \
    \   let mut q = VecDeque::new();\n        self.min_cost[s] = 0;\n        q.push_back(s);\n\
    \        while q.len() > 0 && self.min_cost[t] == -1 {\n            let v = q.pop_front().unwrap();\n\
    \            let mut e = self.head[v];\n            while e != !0 {\n        \
    \        let Edge_ { to, cap } = self.edges[e];\n                if cap >= base\
    \ && self.min_cost[to] == -1 {\n                    self.min_cost[to] = self.min_cost[v]\
    \ + 1;\n                    q.push_back(to);\n                }\n            \
    \    e = self.next[e];\n            }\n        }\n        self.min_cost[t] !=\
    \ -1\n    }\n\n    fn find_augment_path(\n        &mut self,\n        v: usize,\n\
    \        t: usize,\n        base: FlowType,\n        flow: FlowType,\n    ) ->\
    \ FlowType {\n        if v == t {\n            return flow;\n        }\n     \
    \   let mut sum = 0;\n        while self.iter[v] != !0 {\n            let Edge_\
    \ { to, cap } = self.edges[self.iter[v]];\n            if cap >= base && self.min_cost[v]\
    \ < self.min_cost[to] {\n                let d = self.find_augment_path(to, t,\
    \ base, cap.min(flow - sum));\n                if d > 0 {\n                  \
    \  self.edges[self.iter[v]].cap -= d;\n                    self.edges[self.iter[v]\
    \ ^ 1].cap += d;\n                    sum += d;\n                    if flow -\
    \ sum < base {\n                        break;\n                    }\n      \
    \          }\n            }\n            self.iter[v] = self.next[self.iter[v]];\n\
    \        }\n        sum\n    }\n}\n\nstruct Edge_ {\n    to: usize,\n    cap:\
    \ FlowType,\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/graph/max-flow/src/lib.rs
  requiredBy:
  - crates/graph/project-selection/src/lib.rs
  - crates/graph/max-flow-lower-bound/src/lib.rs
  timestamp: '2023-04-27 21:13:38+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/graph/max-flow/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/max-flow/src/lib.rs
- /library/crates/graph/max-flow/src/lib.rs.html
title: crates/graph/max-flow/src/lib.rs
---
