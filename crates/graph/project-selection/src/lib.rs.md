---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/graph/max-flow/src/lib.rs
    title: crates/graph/max-flow/src/lib.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/graph/k-project-selection/src/lib.rs
    title: crates/graph/k-project-selection/src/lib.rs
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
  code: "use max_flow::MaxFlow;\n\ntype Cost1 = [i64; 2];\ntype Cost2 = [Cost1; 2];\n\
    type Cost3 = [Cost2; 2];\n\nconst S: usize = !0;\nconst T: usize = !1;\n\n#[derive(Clone)]\n\
    pub struct ProjectSelection {\n    n_item: usize,\n    n_aux: usize,\n    cost0:\
    \ i64,\n    cost1: Vec<Cost1>,\n    edges: Vec<(usize, usize, i64)>,\n}\n\nimpl\
    \ ProjectSelection {\n    pub fn new(n: usize) -> Self {\n        Self {\n   \
    \         n_item: n,\n            n_aux: 0,\n            cost0: 0,\n         \
    \   cost1: vec![Default::default(); n],\n            edges: vec![],\n        }\n\
    \    }\n\n    pub fn add_cost(&mut self, cost: i64) {\n        self.cost0 += cost;\n\
    \    }\n\n    pub fn add_profit(&mut self, profit: i64) {\n        self.add_cost(-profit);\n\
    \    }\n\n    pub fn add_cost_single(&mut self, i: usize, cost: Cost1) {\n   \
    \     self.cost1[i][0] += cost[0];\n        self.cost1[i][1] += cost[1];\n   \
    \ }\n\n    pub fn add_profit_single(&mut self, i: usize, profit: Cost1) {\n  \
    \      self.add_cost_single(i, [-profit[0], -profit[1]]);\n    }\n\n    pub fn\
    \ add_cost_single_0(&mut self, i: usize, cost: i64) {\n        self.add_cost_single(i,\
    \ [cost, 0]);\n    }\n\n    pub fn add_profit_single_0(&mut self, i: usize, profit:\
    \ i64) {\n        self.add_profit_single(i, [profit, 0]);\n    }\n\n    pub fn\
    \ add_cost_single_1(&mut self, i: usize, cost: i64) {\n        self.add_cost_single(i,\
    \ [0, cost]);\n    }\n\n    pub fn add_profit_single_1(&mut self, i: usize, profit:\
    \ i64) {\n        self.add_profit_single(i, [0, profit]);\n    }\n\n    pub fn\
    \ add_cost_double(&mut self, i: usize, j: usize, cost: Cost2) {\n        assert!(i\
    \ != j);\n        self.add_cost(cost[0][0]);\n        self.add_cost_single_1(i,\
    \ cost[1][0] - cost[0][0]);\n        self.add_cost_single_1(j, cost[1][1] - cost[1][0]);\n\
    \        self.add_cost_double_01(i, j, (cost[0][1] + cost[1][0]) - (cost[0][0]\
    \ + cost[1][1]));\n    }\n\n    pub fn add_profit_double(&mut self, i: usize,\
    \ j: usize, profit: Cost2) {\n        self.add_cost_double(\n            i,\n\
    \            j,\n            [\n                [-profit[0][0], -profit[0][1]],\n\
    \                [-profit[1][0], -profit[1][1]],\n            ],\n        );\n\
    \    }\n\n    pub fn add_cost_double_01(&mut self, i: usize, j: usize, cost: i64)\
    \ {\n        assert!(i != j);\n        self.add_edge(i, j, cost);\n    }\n\n \
    \   pub fn add_cost_double_10(&mut self, i: usize, j: usize, cost: i64) {\n  \
    \      self.add_cost_double_01(j, i, cost);\n    }\n\n    pub fn add_cost_double_not_same(&mut\
    \ self, i: usize, j: usize, cost: i64) {\n        self.add_cost_double(i, j, [[0,\
    \ cost], [cost, 0]]);\n    }\n\n    pub fn add_profit_double_00(&mut self, i:\
    \ usize, j: usize, profit: i64) {\n        self.add_profit_double(i, j, [[profit,\
    \ 0], [0, 0]]);\n    }\n\n    pub fn add_profit_double_11(&mut self, i: usize,\
    \ j: usize, profit: i64) {\n        self.add_profit_double(i, j, [[0, 0], [0,\
    \ profit]]);\n    }\n\n    pub fn add_profit_double_same(&mut self, i: usize,\
    \ j: usize, profit: i64) {\n        self.add_profit_double(i, j, [[profit, 0],\
    \ [0, profit]]);\n    }\n\n    pub fn add_cost_triple(&mut self, i: usize, j:\
    \ usize, k: usize, cost: Cost3) {\n        assert!(i != j && j != k && k != i);\n\
    \        let a = cost[0][0][0];\n        let b = cost[0][0][1];\n        let c\
    \ = cost[0][1][0];\n        let d = cost[0][1][1];\n        let e = cost[1][0][0];\n\
    \        let f = cost[1][0][1];\n        let g = cost[1][1][0];\n        let h\
    \ = cost[1][1][1];\n        let p = (a + d + f + g) - (b + c + e + h);\n     \
    \   if p >= 0 {\n            let p1 = f - b;\n            let p2 = g - e;\n  \
    \          let p3 = d - c;\n            let p12 = (c + e) - (a + g);\n       \
    \     let p23 = (b + c) - (a + d);\n            let p31 = (b + e) - (a + f);\n\
    \            self.add_cost(a);\n            self.add_cost_single_1(i, p1);\n \
    \           self.add_cost_single_1(j, p2);\n            self.add_cost_single_1(k,\
    \ p3);\n            self.add_cost_double_01(i, j, p12);\n            self.add_cost_double_01(j,\
    \ k, p23);\n            self.add_cost_double_01(k, i, p31);\n            self.add_profit_all_1(&[i,\
    \ j, k], p);\n        } else {\n            let p1 = c - g;\n            let p2\
    \ = b - d;\n            let p3 = e - f;\n            let p21 = (d + f) - (b +\
    \ h);\n            let p32 = (f + g) - (e + h);\n            let p13 = (d + g)\
    \ - (c + h);\n            self.add_cost(h);\n            self.add_cost_single_0(i,\
    \ p1);\n            self.add_cost_single_0(j, p2);\n            self.add_cost_single_0(k,\
    \ p3);\n            self.add_cost_double_10(i, j, p21);\n            self.add_cost_double_10(j,\
    \ k, p32);\n            self.add_cost_double_10(k, i, p13);\n            self.add_profit_all_0(&[i,\
    \ j, k], -p);\n        }\n    }\n\n    pub fn add_profit_triple(&mut self, i:\
    \ usize, j: usize, k: usize, profit: Cost3) {\n        self.add_cost_triple(\n\
    \            i,\n            j,\n            k,\n            [\n             \
    \   [\n                    [-profit[0][0][0], -profit[0][0][1]],\n           \
    \         [-profit[0][1][0], -profit[0][1][1]],\n                ],\n        \
    \        [\n                    [-profit[1][0][0], -profit[1][0][1]],\n      \
    \              [-profit[1][1][0], -profit[1][1][1]],\n                ],\n   \
    \         ],\n        );\n    }\n\n    pub fn add_profit_all_0(&mut self, is:\
    \ &[usize], profit: i64) {\n        let n = is.len();\n        let mut is = is.to_vec();\n\
    \        is.sort();\n        is.dedup();\n        assert!(is.len() == n);\n\n\
    \        if is.len() == 0 {\n            self.add_profit(profit);\n        } else\
    \ if is.len() == 1 {\n            self.add_profit_single_0(is[0], profit);\n \
    \       } else if is.len() == 2 {\n            self.add_profit_double_00(is[0],\
    \ is[1], profit);\n        } else {\n            self.add_profit(profit);\n  \
    \          let aux = self.n_item + self.n_aux;\n            self.n_aux += 1;\n\
    \            self.add_edge(S, aux, profit);\n            for &i in &is {\n   \
    \             self.add_edge(aux, i, profit);\n            }\n        }\n    }\n\
    \n    pub fn add_profit_all_1(&mut self, is: &[usize], profit: i64) {\n      \
    \  let n = is.len();\n        let mut is = is.to_vec();\n        is.sort();\n\
    \        is.dedup();\n        assert!(is.len() == n);\n\n        if is.len() ==\
    \ 0 {\n            self.add_profit(profit);\n        } else if is.len() == 1 {\n\
    \            self.add_profit_single_1(is[0], profit);\n        } else if is.len()\
    \ == 2 {\n            self.add_profit_double_11(is[0], is[1], profit);\n     \
    \   } else {\n            self.add_profit(profit);\n            let aux = self.n_item\
    \ + self.n_aux;\n            self.n_aux += 1;\n            for &i in &is {\n \
    \               self.add_edge(i, aux, profit);\n            }\n            self.add_edge(aux,\
    \ T, profit);\n        }\n    }\n\n    pub fn add_cost_any_0(&mut self, is: &[usize],\
    \ cost: i64) {\n        self.add_cost(cost);\n        self.add_profit_all_1(is,\
    \ cost);\n    }\n\n    pub fn add_cost_any_1(&mut self, is: &[usize], cost: i64)\
    \ {\n        self.add_cost(cost);\n        self.add_profit_all_0(is, cost);\n\
    \    }\n\n    pub fn min_cost(&mut self) -> (i64, Vec<bool>) {\n        let mut\
    \ g = MaxFlow::new(self.n_item + self.n_aux + 2);\n        let s = self.n_item\
    \ + self.n_aux;\n        let t = s + 1;\n\n        for i in 0..self.n_item {\n\
    \            let cost = self.cost1[i];\n            if cost[0] <= cost[1] {\n\
    \                self.add_cost(cost[0]);\n                self.add_edge(S, i,\
    \ cost[1] - cost[0]);\n            } else {\n                self.add_cost(cost[1]);\n\
    \                self.add_edge(i, T, cost[0] - cost[1]);\n            }\n    \
    \        self.cost1[i] = [0, 0];\n        }\n\n        for &(i, j, cost) in &self.edges\
    \ {\n            let u = match i {\n                S => s,\n                T\
    \ => t,\n                _ => i,\n            };\n            let v = match j\
    \ {\n                S => s,\n                T => t,\n                _ => j,\n\
    \            };\n            g.add_edge(u, v, cost);\n        }\n        let res\
    \ = self.cost0 + g.max_flow(s, t);\n        let mut cut = g.min_cut(s);\n    \
    \    cut.truncate(self.n_item);\n        for i in 0..self.n_item {\n         \
    \   cut[i] = !cut[i];\n        }\n        (res, cut)\n    }\n\n    pub fn max_profit(&mut\
    \ self) -> (i64, Vec<bool>) {\n        let (mut res, cut) = self.min_cost();\n\
    \        res = -res;\n        (res, cut)\n    }\n\n    fn add_edge(&mut self,\
    \ i: usize, j: usize, cost: i64) {\n        assert!(cost >= 0);\n        assert!(i\
    \ != j);\n        assert!(i == S || i == T || i < self.n_item + self.n_aux);\n\
    \        assert!(j == S || j == T || j < self.n_item + self.n_aux);\n        if\
    \ cost == 0 {\n            return;\n        }\n        self.edges.push((i, j,\
    \ cost));\n    }\n}\n"
  dependsOn:
  - crates/graph/max-flow/src/lib.rs
  isVerificationFile: false
  path: crates/graph/project-selection/src/lib.rs
  requiredBy:
  - crates/graph/k-project-selection/src/lib.rs
  timestamp: '2024-04-05 14:38:09+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/graph/project-selection/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/project-selection/src/lib.rs
- /library/crates/graph/project-selection/src/lib.rs.html
title: crates/graph/project-selection/src/lib.rs
---
