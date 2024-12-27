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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use project_selection::ProjectSelection;\n\nconst INF: i64 = 1 << 61;\n\n\
    /// Project Selection Problem  \n///\n/// # \u6982\u8981\n/// n \u500B\u306E\u30A2\
    \u30A4\u30C6\u30E0\u306B\u3064\u3044\u3066\u9078\u629E\u80A2 [0, k_i) \u3092\u9078\
    \u3076\u3002  \n///\n/// - \u30A2\u30A4\u30C6\u30E0 i \u306B\u3064\u3044\u3066\
    \u9078\u629E\u80A2 x \u3092\u9078\u3076\u3068\u30B3\u30B9\u30C8\u304C\u304B\u304B\
    \u308B\n/// - \u30A2\u30A4\u30C6\u30E0 i, j \u306B\u3064\u3044\u3066\u305D\u308C\
    \u305E\u308C\u9078\u629E\u80A2 x, y \u3092\u9078\u3076\u3068\u30B3\u30B9\u30C8\
    \u304C\u304B\u304B\u308B\n///\n/// \u3068\u3044\u3063\u305F\u6761\u4EF6\u304C\u3042\
    \u308B\u3002\u3053\u306E\u3068\u304D\u306E\u6700\u5C0F\u30B3\u30B9\u30C8\u3068\
    \u305D\u308C\u3092\u9054\u6210\u3059\u308B\u9078\u629E\u80A2\u306E\u7D44\u3092\
    \u6C42\u3081\u308B\u3002\npub struct KProjectSelection {\n    n: usize,\n    ks:\
    \ Vec<usize>,\n    id: Vec<Vec<usize>>,\n    ps: ProjectSelection,\n}\n\nimpl\
    \ KProjectSelection {\n    /// \u30A2\u30A4\u30C6\u30E0 i \u306E\u9078\u629E\u80A2\
    \u304C ks[i] \u500B\u3042\u308B\u3068\u3057\u3066\u521D\u671F\u5316\u3059\u308B\
    \n    pub fn new(ks: &[usize]) -> Self {\n        let n = ks.len();\n        let\
    \ mut id = vec![vec![]; n];\n        let mut cnt = 0;\n        for i in 0..n {\n\
    \            assert!(ks[i] > 0);\n            id[i].resize(ks[i], !0);\n     \
    \       for d in 1..ks[i] {\n                id[i][d] = cnt;\n               \
    \ cnt += 1;\n            }\n        }\n        let mut ps = ProjectSelection::new(cnt);\n\
    \        for i in 0..n {\n            for d in 1..ks[i] - 1 {\n              \
    \  ps.add_cost_2items_10(id[i][d], id[i][d + 1], INF);\n            }\n      \
    \  }\n        Self {\n            n,\n            ks: ks.to_vec(),\n         \
    \   id,\n            ps,\n        }\n    }\n\n    /// \u9078\u629E\u80A2\u306E\
    \u9078\u3073\u65B9\u306B\u4F9D\u5B58\u3057\u306A\u3044\u30B3\u30B9\u30C8\u3092\
    \u8FFD\u52A0\u3059\u308B\n    pub fn add_cost(&mut self, cost: i64) {\n      \
    \  self.ps.add_cost(cost);\n    }\n\n    /// \u9078\u629E\u80A2\u306E\u9078\u3073\
    \u65B9\u306B\u4F9D\u5B58\u3057\u306A\u3044\u5229\u76CA\u3092\u8FFD\u52A0\u3059\
    \u308B\n    pub fn add_profit(&mut self, profit: i64) {\n        self.ps.add_profit(profit);\n\
    \    }\n\n    /// \u30A2\u30A4\u30C6\u30E0 i \u306B\u3064\u3044\u3066\u9078\u629E\
    \u80A2 x \u3092\u9078\u3076\u3068\u30B3\u30B9\u30C8 cost[x] \u304B\u304B\u308B\
    \u3068\u3044\u3046\u6761\u4EF6\u3092\u8FFD\u52A0\u3059\u308B\n    pub fn add_cost_1item(&mut\
    \ self, i: usize, cost: &[i64]) {\n        assert!(cost.len() == self.ks[i]);\n\
    \        self.ps.add_cost(cost[self.ks[i] - 1]);\n        for d in 1..self.ks[i]\
    \ {\n            self.ps\n                .add_cost_1item_1(self.id[i][d], cost[d\
    \ - 1] - cost[d]);\n        }\n    }\n\n    /// \u30A2\u30A4\u30C6\u30E0 i \u306B\
    \u3064\u3044\u3066\u9078\u629E\u80A2 x \u3092\u9078\u3076\u3068\u5229\u76CA profit[x]\
    \ \u304C\u5F97\u3089\u308C\u308B\u3068\u3044\u3046\u6761\u4EF6\u3092\u8FFD\u52A0\
    \u3059\u308B\n    pub fn add_profit_1item(&mut self, i: usize, profit: &[i64])\
    \ {\n        let mut profit = profit.to_vec();\n        for p in &mut profit {\n\
    \            *p = -*p;\n        }\n        self.add_cost_1item(i, &profit);\n\
    \    }\n\n    /// \u30A2\u30A4\u30C6\u30E0 i, j \u306B\u3064\u3044\u3066\u305D\
    \u308C\u305E\u308C\u9078\u629E\u80A2 x, y \u3092\u9078\u3076\u3068\u30B3\u30B9\
    \u30C8 cost[x][y] \u304B\u304B\u308B\u3068\u3044\u3046\u6761\u4EF6\u3092\u8FFD\
    \u52A0\u3059\u308B\n    pub fn add_cost_2items(&mut self, i: usize, j: usize,\
    \ cost: &Vec<Vec<i64>>) {\n        let mut cost = cost.clone();\n        assert!(i\
    \ != j);\n        assert!(cost.len() == self.ks[i]);\n        for di in 0..self.ks[i]\
    \ {\n            assert!(cost[di].len() == self.ks[j]);\n        }\n\n       \
    \ let mut cost_i = vec![0; self.ks[i]];\n        let mut cost_j = vec![0; self.ks[j]];\n\
    \        for di in 0..self.ks[i] {\n            cost_i[di] = cost[di][0];\n  \
    \          for dj in 0..self.ks[j] {\n                cost[di][dj] -= cost_i[di];\n\
    \            }\n        }\n        for dj in 0..self.ks[j] {\n            cost_j[dj]\
    \ = cost[0][dj];\n            for di in 0..self.ks[i] {\n                cost[di][dj]\
    \ -= cost_j[dj];\n            }\n        }\n        self.add_cost_1item(i, &cost_i);\n\
    \        self.add_cost_1item(j, &cost_j);\n\n        assert!((0..self.ks[i]).all(|di|\
    \ cost[di][0] == 0));\n        assert!((0..self.ks[j]).all(|dj| cost[0][dj] ==\
    \ 0));\n\n        for di in 1..self.ks[i] {\n            for dj in 1..self.ks[j]\
    \ {\n                let cost_00 =\n                    cost[di][dj] - cost[di][dj\
    \ - 1] - cost[di - 1][dj] + cost[di - 1][dj - 1];\n                assert!(cost_00\
    \ <= 0);\n                self.ps\n                    .add_profit_2items_00(self.id[i][di],\
    \ self.id[j][dj], -cost_00);\n            }\n        }\n    }\n\n    /// \u6700\
    \u5C0F\u30B3\u30B9\u30C8\u3068\u305D\u308C\u3092\u9054\u6210\u3059\u308B\u9078\
    \u629E\u80A2\u306E\u7D44\u3092\u6C42\u3081\u308B\n    pub fn min_cost(&mut self)\
    \ -> (i64, Vec<usize>) {\n        let (res, f) = self.ps.min_cost();\n       \
    \ let mut g = vec![0; self.n];\n        for i in 0..self.n {\n            for\
    \ di in 1..self.ks[i] {\n                if f[self.id[i][di]] == 0 {\n       \
    \             g[i] += 1;\n                }\n            }\n        }\n      \
    \  (res, g)\n    }\n\n    /// \u6700\u5927\u5229\u76CA\u3068\u305D\u308C\u3092\
    \u9054\u6210\u3059\u308B\u9078\u629E\u80A2\u306E\u7D44\u3092\u6C42\u3081\u308B\
    \n    pub fn max_profit(&mut self) -> (i64, Vec<usize>) {\n        let (mut res,\
    \ f) = self.min_cost();\n        res = -res;\n        (res, f)\n    }\n}\n"
  dependsOn:
  - crates/graph/project-selection/src/lib.rs
  isVerificationFile: false
  path: crates/graph/k-project-selection/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-27 03:53:35+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/graph/k-project-selection/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/k-project-selection/src/lib.rs
- /library/crates/graph/k-project-selection/src/lib.rs.html
title: crates/graph/k-project-selection/src/lib.rs
---
