---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/graph/project-selection/src/lib.rs
    title: crates/graph/project-selection/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.3/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.3/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use project_selection::ProjectSelection;\n\nconst INF: i64 = 1 << 61;\n\n\
    pub struct KProjectSelection<const K: usize> {\n    n: usize,\n    ps: ProjectSelection,\n\
    }\n\nfn id(i: usize, d: usize, k: usize) -> usize {\n    if d == 0 {\n       \
    \ !0\n    } else {\n        i * (k - 1) + (d - 1)\n    }\n}\n\nimpl<const K: usize>\
    \ KProjectSelection<K> {\n    pub fn new(n: usize) -> Self {\n        let mut\
    \ ps = ProjectSelection::new(n * (K - 1));\n        for i in 0..n {\n        \
    \    for d in 1..K - 1 {\n                ps.add_cost_double_10(id(i, d, K), id(i,\
    \ d + 1, K), INF);\n            }\n        }\n        Self { n, ps }\n    }\n\n\
    \    pub fn add_cost(&mut self, cost: i64) {\n        self.ps.add_cost(cost);\n\
    \    }\n\n    pub fn add_profit(&mut self, profit: i64) {\n        self.ps.add_profit(profit);\n\
    \    }\n\n    pub fn add_cost_single(&mut self, i: usize, cost: [i64; K]) {\n\
    \        self.ps.add_cost(cost[K - 1]);\n        for d in 1..K {\n           \
    \ self.ps\n                .add_cost_single_1(id(i, d, K), cost[d - 1] - cost[d]);\n\
    \        }\n    }\n\n    pub fn add_profit_single(&mut self, i: usize, mut profit:\
    \ [i64; K]) {\n        for i in 0..K {\n            profit[i] = -profit[i];\n\
    \        }\n        self.add_cost_single(i, profit);\n    }\n\n    pub fn add_cost_double(&mut\
    \ self, i: usize, j: usize, mut cost: [[i64; K]; K]) {\n        let mut cost_i\
    \ = [0; K];\n        let mut cost_j = [0; K];\n        for di in 0..K {\n    \
    \        cost_i[di] = cost[di][0];\n            for dj in 0..K {\n           \
    \     cost[di][dj] -= cost_i[di];\n            }\n        }\n        for dj in\
    \ 0..K {\n            cost_j[dj] = cost[0][dj];\n            for di in 0..K {\n\
    \                cost[di][dj] -= cost_j[dj];\n            }\n        }\n     \
    \   self.add_cost_single(i, cost_i);\n        self.add_cost_single(j, cost_j);\n\
    \n        assert!((0..K).all(|di| cost[di][0] == 0));\n        assert!((0..K).all(|dj|\
    \ cost[0][dj] == 0));\n\n        for di in 1..K {\n            for dj in 1..K\
    \ {\n                let cost_00 =\n                    cost[di][dj] - cost[di][dj\
    \ - 1] - cost[di - 1][dj] + cost[di - 1][dj - 1];\n                assert!(cost_00\
    \ <= 0);\n                self.ps\n                    .add_profit_double_00(id(i,\
    \ di, K), id(j, dj, K), -cost_00);\n            }\n        }\n    }\n\n    pub\
    \ fn min_cost(&mut self) -> (i64, Vec<usize>) {\n        let (res, f) = self.ps.min_cost();\n\
    \        let mut g = vec![0; self.n];\n        for i in 0..self.n {\n        \
    \    for di in 1..K {\n                if !f[id(i, di, K)] {\n               \
    \     g[i] += 1;\n                }\n            }\n        }\n        (res, g)\n\
    \    }\n\n    pub fn max_profit(&mut self) -> (i64, Vec<usize>) {\n        let\
    \ (mut res, f) = self.min_cost();\n        res = -res;\n        (res, f)\n   \
    \ }\n}\n"
  dependsOn:
  - crates/graph/project-selection/src/lib.rs
  isVerificationFile: false
  path: crates/graph/k-project-selection/src/lib.rs
  requiredBy: []
  timestamp: '2024-04-05 14:38:09+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/graph/k-project-selection/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/k-project-selection/src/lib.rs
- /library/crates/graph/k-project-selection/src/lib.rs.html
title: crates/graph/k-project-selection/src/lib.rs
---
