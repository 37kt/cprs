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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use project_selection::ProjectSelection;\n\nconst INF: i64 = 1 << 61;\n\n\
    pub struct KProjectSelection {\n    n: usize,\n    ks: Vec<usize>,\n    id: Vec<Vec<usize>>,\n\
    \    ps: ProjectSelection,\n}\n\nimpl KProjectSelection {\n    pub fn new(ks:\
    \ &[usize]) -> Self {\n        let n = ks.len();\n        let mut id = vec![vec![];\
    \ n];\n        let mut cnt = 0;\n        for i in 0..n {\n            assert!(ks[i]\
    \ > 0);\n            id[i].resize(ks[i], !0);\n            for d in 1..ks[i] {\n\
    \                id[i][d] = cnt;\n                cnt += 1;\n            }\n \
    \       }\n        let mut ps = ProjectSelection::new(cnt);\n        for i in\
    \ 0..n {\n            for d in 1..ks[i] - 1 {\n                ps.add_cost_double_10(id[i][d],\
    \ id[i][d + 1], INF);\n            }\n        }\n        Self {\n            n,\n\
    \            ks: ks.to_vec(),\n            id,\n            ps,\n        }\n \
    \   }\n\n    pub fn add_cost(&mut self, cost: i64) {\n        self.ps.add_cost(cost);\n\
    \    }\n\n    pub fn add_profit(&mut self, profit: i64) {\n        self.ps.add_profit(profit);\n\
    \    }\n\n    pub fn add_cost_single(&mut self, i: usize, cost: &[i64]) {\n  \
    \      assert!(cost.len() == self.ks[i]);\n        self.ps.add_cost(cost[self.ks[i]\
    \ - 1]);\n        for d in 1..self.ks[i] {\n            self.ps\n            \
    \    .add_cost_single_1(self.id[i][d], cost[d - 1] - cost[d]);\n        }\n  \
    \  }\n\n    pub fn add_profit_single(&mut self, i: usize, profit: &[i64]) {\n\
    \        let mut profit = profit.to_vec();\n        for p in &mut profit {\n \
    \           *p = -*p;\n        }\n        self.add_cost_single(i, &profit);\n\
    \    }\n\n    pub fn add_cost_double(&mut self, i: usize, j: usize, mut cost:\
    \ Vec<Vec<i64>>) {\n        assert!(i != j);\n        assert!(cost.len() == self.ks[i]);\n\
    \        for di in 0..self.ks[i] {\n            assert!(cost[di].len() == self.ks[j]);\n\
    \        }\n\n        let mut cost_i = vec![0; self.ks[i]];\n        let mut cost_j\
    \ = vec![0; self.ks[j]];\n        for di in 0..self.ks[i] {\n            cost_i[di]\
    \ = cost[di][0];\n            for dj in 0..self.ks[j] {\n                cost[di][dj]\
    \ -= cost_i[di];\n            }\n        }\n        for dj in 0..self.ks[j] {\n\
    \            cost_j[dj] = cost[0][dj];\n            for di in 0..self.ks[i] {\n\
    \                cost[di][dj] -= cost_j[dj];\n            }\n        }\n     \
    \   self.add_cost_single(i, &cost_i);\n        self.add_cost_single(j, &cost_j);\n\
    \n        assert!((0..self.ks[i]).all(|di| cost[di][0] == 0));\n        assert!((0..self.ks[j]).all(|dj|\
    \ cost[0][dj] == 0));\n\n        for di in 1..self.ks[i] {\n            for dj\
    \ in 1..self.ks[j] {\n                let cost_00 =\n                    cost[di][dj]\
    \ - cost[di][dj - 1] - cost[di - 1][dj] + cost[di - 1][dj - 1];\n            \
    \    assert!(cost_00 <= 0);\n                self.ps\n                    .add_profit_double_00(self.id[i][di],\
    \ self.id[j][dj], -cost_00);\n            }\n        }\n    }\n\n    pub fn min_cost(&mut\
    \ self) -> (i64, Vec<usize>) {\n        let (res, f) = self.ps.min_cost();\n \
    \       let mut g = vec![0; self.n];\n        for i in 0..self.n {\n         \
    \   for di in 1..self.ks[i] {\n                if !f[self.id[i][di]] {\n     \
    \               g[i] += 1;\n                }\n            }\n        }\n    \
    \    (res, g)\n    }\n\n    pub fn max_profit(&mut self) -> (i64, Vec<usize>)\
    \ {\n        let (mut res, f) = self.min_cost();\n        res = -res;\n      \
    \  (res, f)\n    }\n}\n"
  dependsOn:
  - crates/graph/project-selection/src/lib.rs
  isVerificationFile: false
  path: crates/graph/k-project-selection/src/lib.rs
  requiredBy: []
  timestamp: '2024-04-23 13:31:46+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/graph/k-project-selection/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/k-project-selection/src/lib.rs
- /library/crates/graph/k-project-selection/src/lib.rs.html
title: crates/graph/k-project-selection/src/lib.rs
---
