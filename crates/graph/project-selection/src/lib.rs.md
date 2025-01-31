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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use max_flow::MaxFlow;\n\ntype Cost1 = [i64; 2];\ntype Cost2 = [Cost1; 2];\n\
    type Cost3 = [Cost2; 2];\n\nconst S: usize = !0;\nconst T: usize = !1;\n\n///\
    \ Project Selection Problem  \n///\n/// # \u6982\u8981\n/// n \u500B\u306E\u30A2\
    \u30A4\u30C6\u30E0\u306B\u3064\u3044\u3066 0 \u304B 1 \u3092\u9078\u3076\u3002\
    \  \n///\n/// - \u30A2\u30A4\u30C6\u30E0 i \u306B\u3064\u3044\u3066\u9078\u629E\
    \u80A2 x \u3092\u9078\u3076\u3068\u30B3\u30B9\u30C8\u304C\u304B\u304B\u308B\n\
    /// - \u30A2\u30A4\u30C6\u30E0 i, j \u306B\u3064\u3044\u3066\u305D\u308C\u305E\
    \u308C\u9078\u629E\u80A2 x, y \u3092\u9078\u3076\u3068\u30B3\u30B9\u30C8\u304C\
    \u304B\u304B\u308B\n/// - \u30A2\u30A4\u30C6\u30E0 i, j, k \u306B\u3064\u3044\u3066\
    \u305D\u308C\u305E\u308C\u9078\u629E\u80A2 x, y, z \u3092\u9078\u3076\u3068\u30B3\
    \u30B9\u30C8\u304C\u304B\u304B\u308B\n///\n/// \u3068\u3044\u3063\u305F\u6761\u4EF6\
    \u304C\u3042\u308B\u3002\u3053\u306E\u3068\u304D\u306E\u6700\u5C0F\u30B3\u30B9\
    \u30C8\u3068\u305D\u308C\u3092\u9054\u6210\u3059\u308B\u9078\u629E\u80A2\u306E\
    \u7D44\u3092\u6C42\u3081\u308B\u3002\n#[derive(Clone)]\npub struct ProjectSelection\
    \ {\n    n_item: usize,\n    n_aux: usize,\n    cost0: i64,\n    cost1: Vec<Cost1>,\n\
    \    edges: Vec<(usize, usize, i64)>,\n}\n\nimpl ProjectSelection {\n    pub fn\
    \ new(n: usize) -> Self {\n        Self {\n            n_item: n,\n          \
    \  n_aux: 0,\n            cost0: 0,\n            cost1: vec![Default::default();\
    \ n],\n            edges: vec![],\n        }\n    }\n\n    /// \u9078\u629E\u80A2\
    \u306E\u9078\u3073\u65B9\u306B\u4F9D\u5B58\u3057\u306A\u3044\u30B3\u30B9\u30C8\
    \u3092\u8FFD\u52A0\u3059\u308B\n    pub fn add_cost(&mut self, cost: i64) {\n\
    \        self.cost0 += cost;\n    }\n\n    /// \u9078\u629E\u80A2\u306E\u9078\u3073\
    \u65B9\u306B\u4F9D\u5B58\u3057\u306A\u3044\u5229\u76CA\u3092\u8FFD\u52A0\u3059\
    \u308B\n    pub fn add_profit(&mut self, profit: i64) {\n        self.add_cost(-profit);\n\
    \    }\n\n    /// \u30A2\u30A4\u30C6\u30E0 i \u306B\u3064\u3044\u3066\u9078\u629E\
    \u80A2 j \u3092\u9078\u3076\u3068\u30B3\u30B9\u30C8 cost\\[j\\] \u304B\u304B\u308B\
    \u3068\u3044\u3046\u6761\u4EF6\u3092\u8FFD\u52A0\u3059\u308B\n    pub fn add_cost_1item(&mut\
    \ self, i: usize, cost: Cost1) {\n        self.cost1[i][0] += cost[0];\n     \
    \   self.cost1[i][1] += cost[1];\n    }\n\n    /// \u30A2\u30A4\u30C6\u30E0 i\
    \ \u306B\u3064\u3044\u3066\u9078\u629E\u80A2 j \u3092\u9078\u3076\u3068\u5229\u76CA\
    \ profit\\[j\\] \u304C\u5F97\u3089\u308C\u308B\u3068\u3044\u3046\u6761\u4EF6\u3092\
    \u8FFD\u52A0\u3059\u308B\n    pub fn add_profit_1item(&mut self, i: usize, profit:\
    \ Cost1) {\n        self.add_cost_1item(i, [-profit[0], -profit[1]]);\n    }\n\
    \n    /// \u30A2\u30A4\u30C6\u30E0 i \u306B\u3064\u3044\u3066\u9078\u629E\u80A2\
    \ 0 \u3092\u9078\u3076\u3068\u30B3\u30B9\u30C8 cost \u304B\u304B\u308B\u3068\u3044\
    \u3046\u6761\u4EF6\u3092\u8FFD\u52A0\u3059\u308B\n    pub fn add_cost_1item_0(&mut\
    \ self, i: usize, cost: i64) {\n        self.add_cost_1item(i, [cost, 0]);\n \
    \   }\n\n    /// \u30A2\u30A4\u30C6\u30E0 i \u306B\u3064\u3044\u3066\u9078\u629E\
    \u80A2 0 \u3092\u9078\u3076\u3068\u5229\u76CA profit \u304C\u5F97\u3089\u308C\u308B\
    \u3068\u3044\u3046\u6761\u4EF6\u3092\u8FFD\u52A0\u3059\u308B\n    pub fn add_profit_1item_0(&mut\
    \ self, i: usize, profit: i64) {\n        self.add_profit_1item(i, [profit, 0]);\n\
    \    }\n\n    /// \u30A2\u30A4\u30C6\u30E0 i \u306B\u3064\u3044\u3066\u9078\u629E\
    \u80A2 1 \u3092\u9078\u3076\u3068\u30B3\u30B9\u30C8 cost \u304B\u304B\u308B\u3068\
    \u3044\u3046\u6761\u4EF6\u3092\u8FFD\u52A0\u3059\u308B\n    pub fn add_cost_1item_1(&mut\
    \ self, i: usize, cost: i64) {\n        self.add_cost_1item(i, [0, cost]);\n \
    \   }\n\n    /// \u30A2\u30A4\u30C6\u30E0 i \u306B\u3064\u3044\u3066\u9078\u629E\
    \u80A2 1 \u3092\u9078\u3076\u3068\u5229\u76CA profit \u304C\u5F97\u3089\u308C\u308B\
    \u3068\u3044\u3046\u6761\u4EF6\u3092\u8FFD\u52A0\u3059\u308B\n    pub fn add_profit_1item_1(&mut\
    \ self, i: usize, profit: i64) {\n        self.add_profit_1item(i, [0, profit]);\n\
    \    }\n\n    /// \u30A2\u30A4\u30C6\u30E0 i, j \u306B\u3064\u3044\u3066\u305D\
    \u308C\u305E\u308C\u9078\u629E\u80A2 x, y \u3092\u9078\u3076\u3068\u30B3\u30B9\
    \u30C8 cost\\[x\\]\\[y\\] \u304B\u304B\u308B\u3068\u3044\u3046\u6761\u4EF6\u3092\
    \u8FFD\u52A0\u3059\u308B\n    pub fn add_cost_2items(&mut self, i: usize, j: usize,\
    \ cost: Cost2) {\n        assert!(i != j);\n        self.add_cost(cost[0][0]);\n\
    \        self.add_cost_1item_1(i, cost[1][0] - cost[0][0]);\n        self.add_cost_1item_1(j,\
    \ cost[1][1] - cost[1][0]);\n        self.add_cost_2items_01(i, j, (cost[0][1]\
    \ + cost[1][0]) - (cost[0][0] + cost[1][1]));\n    }\n\n    /// \u30A2\u30A4\u30C6\
    \u30E0 i, j \u306B\u3064\u3044\u3066\u305D\u308C\u305E\u308C\u9078\u629E\u80A2\
    \ x, y \u3092\u9078\u3076\u3068\u5229\u76CA profit\\[x\\]\\[y\\] \u304C\u5F97\u3089\
    \u308C\u308B\u3068\u3044\u3046\u6761\u4EF6\u3092\u8FFD\u52A0\u3059\u308B\n   \
    \ pub fn add_profit_2items(&mut self, i: usize, j: usize, profit: Cost2) {\n \
    \       self.add_cost_2items(\n            i,\n            j,\n            [\n\
    \                [-profit[0][0], -profit[0][1]],\n                [-profit[1][0],\
    \ -profit[1][1]],\n            ],\n        );\n    }\n\n    /// \u30A2\u30A4\u30C6\
    \u30E0 i, j \u306B\u3064\u3044\u3066\u305D\u308C\u305E\u308C\u9078\u629E\u80A2\
    \ 0, 1 \u3092\u9078\u3076\u3068\u30B3\u30B9\u30C8 cost \u304B\u304B\u308B\u3068\
    \u3044\u3046\u6761\u4EF6\u3092\u8FFD\u52A0\u3059\u308B\n    pub fn add_cost_2items_01(&mut\
    \ self, i: usize, j: usize, cost: i64) {\n        assert!(i != j);\n        self.add_edge(i,\
    \ j, cost);\n    }\n\n    /// \u30A2\u30A4\u30C6\u30E0 i, j \u306B\u3064\u3044\
    \u3066\u305D\u308C\u305E\u308C\u9078\u629E\u80A2 1, 0 \u3092\u9078\u3076\u3068\
    \u30B3\u30B9\u30C8 cost \u304B\u304B\u308B\u3068\u3044\u3046\u6761\u4EF6\u3092\
    \u8FFD\u52A0\u3059\u308B\n    pub fn add_cost_2items_10(&mut self, i: usize, j:\
    \ usize, cost: i64) {\n        self.add_cost_2items_01(j, i, cost);\n    }\n\n\
    \    /// \u30A2\u30A4\u30C6\u30E0 i, j \u306B\u3064\u3044\u3066\u7570\u306A\u308B\
    \u9078\u629E\u80A2\u3092\u9078\u3076\u3068\u30B3\u30B9\u30C8 cost \u304B\u304B\
    \u308B\u3068\u3044\u3046\u6761\u4EF6\u3092\u8FFD\u52A0\u3059\u308B\n    pub fn\
    \ add_cost_2items_not_same(&mut self, i: usize, j: usize, cost: i64) {\n     \
    \   self.add_cost_2items(i, j, [[0, cost], [cost, 0]]);\n    }\n\n    /// \u30A2\
    \u30A4\u30C6\u30E0 i, j \u306B\u3064\u3044\u3066\u3069\u3061\u3089\u3082\u9078\
    \u629E\u80A2 0 \u3092\u9078\u3076\u3068\u5229\u76CA profit \u304C\u5F97\u3089\u308C\
    \u308B\u3068\u3044\u3046\u6761\u4EF6\u3092\u8FFD\u52A0\u3059\u308B\n    pub fn\
    \ add_profit_2items_00(&mut self, i: usize, j: usize, profit: i64) {\n       \
    \ self.add_profit_2items(i, j, [[profit, 0], [0, 0]]);\n    }\n\n    /// \u30A2\
    \u30A4\u30C6\u30E0 i, j \u306B\u3064\u3044\u3066\u3069\u3061\u3089\u3082\u9078\
    \u629E\u80A2 1 \u3092\u9078\u3076\u3068\u5229\u76CA profit \u304C\u5F97\u3089\u308C\
    \u308B\u3068\u3044\u3046\u6761\u4EF6\u3092\u8FFD\u52A0\u3059\u308B\n    pub fn\
    \ add_profit_2items_11(&mut self, i: usize, j: usize, profit: i64) {\n       \
    \ self.add_profit_2items(i, j, [[0, 0], [0, profit]]);\n    }\n\n    /// \u30A2\
    \u30A4\u30C6\u30E0 i, j \u306B\u3064\u3044\u3066\u540C\u3058\u9078\u629E\u80A2\
    \u3092\u9078\u3076\u3068\u5229\u76CA profit \u304C\u5F97\u3089\u308C\u308B\u3068\
    \u3044\u3046\u6761\u4EF6\u3092\u8FFD\u52A0\u3059\u308B\n    pub fn add_profit_2items_same(&mut\
    \ self, i: usize, j: usize, profit: i64) {\n        self.add_profit_2items(i,\
    \ j, [[profit, 0], [0, profit]]);\n    }\n\n    /// \u30A2\u30A4\u30C6\u30E0 i,\
    \ j, k \u306B\u3064\u3044\u3066\u305D\u308C\u305E\u308C\u9078\u629E\u80A2 x, y,\
    \ z \u3092\u9078\u3076\u3068\u30B3\u30B9\u30C8 cost\\[x\\]\\[y\\]\\[z\\] \u304B\
    \u304B\u308B\u3068\u3044\u3046\u6761\u4EF6\u3092\u8FFD\u52A0\u3059\u308B\n   \
    \ pub fn add_cost_3items(&mut self, i: usize, j: usize, k: usize, cost: Cost3)\
    \ {\n        assert!(i != j && j != k && k != i);\n        let a = cost[0][0][0];\n\
    \        let b = cost[0][0][1];\n        let c = cost[0][1][0];\n        let d\
    \ = cost[0][1][1];\n        let e = cost[1][0][0];\n        let f = cost[1][0][1];\n\
    \        let g = cost[1][1][0];\n        let h = cost[1][1][1];\n        let p\
    \ = (a + d + f + g) - (b + c + e + h);\n        if p >= 0 {\n            let p1\
    \ = f - b;\n            let p2 = g - e;\n            let p3 = d - c;\n       \
    \     let p12 = (c + e) - (a + g);\n            let p23 = (b + c) - (a + d);\n\
    \            let p31 = (b + e) - (a + f);\n            self.add_cost(a);\n   \
    \         self.add_cost_1item_1(i, p1);\n            self.add_cost_1item_1(j,\
    \ p2);\n            self.add_cost_1item_1(k, p3);\n            self.add_cost_2items_01(i,\
    \ j, p12);\n            self.add_cost_2items_01(j, k, p23);\n            self.add_cost_2items_01(k,\
    \ i, p31);\n            self.add_profit_all_1(&[i, j, k], p);\n        } else\
    \ {\n            let p1 = c - g;\n            let p2 = b - d;\n            let\
    \ p3 = e - f;\n            let p21 = (d + f) - (b + h);\n            let p32 =\
    \ (f + g) - (e + h);\n            let p13 = (d + g) - (c + h);\n            self.add_cost(h);\n\
    \            self.add_cost_1item_0(i, p1);\n            self.add_cost_1item_0(j,\
    \ p2);\n            self.add_cost_1item_0(k, p3);\n            self.add_cost_2items_10(i,\
    \ j, p21);\n            self.add_cost_2items_10(j, k, p32);\n            self.add_cost_2items_10(k,\
    \ i, p13);\n            self.add_profit_all_0(&[i, j, k], -p);\n        }\n  \
    \  }\n\n    /// \u30A2\u30A4\u30C6\u30E0 i, j, k \u306B\u3064\u3044\u3066\u305D\
    \u308C\u305E\u308C\u9078\u629E\u80A2 x, y, z \u3092\u9078\u3076\u3068\u5229\u76CA\
    \ profit\\[x\\]\\[y\\]\\[z\\] \u304C\u5F97\u3089\u308C\u308B\u3068\u3044\u3046\
    \u6761\u4EF6\u3092\u8FFD\u52A0\u3059\u308B\n    pub fn add_profit_3items(&mut\
    \ self, i: usize, j: usize, k: usize, profit: Cost3) {\n        self.add_cost_3items(\n\
    \            i,\n            j,\n            k,\n            [\n             \
    \   [\n                    [-profit[0][0][0], -profit[0][0][1]],\n           \
    \         [-profit[0][1][0], -profit[0][1][1]],\n                ],\n        \
    \        [\n                    [-profit[1][0][0], -profit[1][0][1]],\n      \
    \              [-profit[1][1][0], -profit[1][1][1]],\n                ],\n   \
    \         ],\n        );\n    }\n\n    /// \u30A2\u30A4\u30C6\u30E0 is \u306B\u3064\
    \u3044\u3066\u3059\u3079\u3066\u9078\u629E\u80A2 0 \u3092\u9078\u3076\u3068\u5229\
    \u76CA profit \u304C\u5F97\u3089\u308C\u308B\u3068\u3044\u3046\u6761\u4EF6\u3092\
    \u8FFD\u52A0\u3059\u308B\n    pub fn add_profit_all_0(&mut self, is: &[usize],\
    \ profit: i64) {\n        let n = is.len();\n        let mut is = is.to_vec();\n\
    \        is.sort();\n        is.dedup();\n        assert!(is.len() == n);\n\n\
    \        if is.len() == 0 {\n            self.add_profit(profit);\n        } else\
    \ if is.len() == 1 {\n            self.add_profit_1item_0(is[0], profit);\n  \
    \      } else if is.len() == 2 {\n            self.add_profit_2items_00(is[0],\
    \ is[1], profit);\n        } else {\n            self.add_profit(profit);\n  \
    \          let aux = self.n_item + self.n_aux;\n            self.n_aux += 1;\n\
    \            self.add_edge(S, aux, profit);\n            for &i in &is {\n   \
    \             self.add_edge(aux, i, profit);\n            }\n        }\n    }\n\
    \n    /// \u30A2\u30A4\u30C6\u30E0 is \u306B\u3064\u3044\u3066\u3059\u3079\u3066\
    \u9078\u629E\u80A2 1 \u3092\u9078\u3076\u3068\u5229\u76CA profit \u304C\u5F97\u3089\
    \u308C\u308B\u3068\u3044\u3046\u6761\u4EF6\u3092\u8FFD\u52A0\u3059\u308B\n   \
    \ pub fn add_profit_all_1(&mut self, is: &[usize], profit: i64) {\n        let\
    \ n = is.len();\n        let mut is = is.to_vec();\n        is.sort();\n     \
    \   is.dedup();\n        assert!(is.len() == n);\n\n        if is.len() == 0 {\n\
    \            self.add_profit(profit);\n        } else if is.len() == 1 {\n   \
    \         self.add_profit_1item_1(is[0], profit);\n        } else if is.len()\
    \ == 2 {\n            self.add_profit_2items_11(is[0], is[1], profit);\n     \
    \   } else {\n            self.add_profit(profit);\n            let aux = self.n_item\
    \ + self.n_aux;\n            self.n_aux += 1;\n            for &i in &is {\n \
    \               self.add_edge(i, aux, profit);\n            }\n            self.add_edge(aux,\
    \ T, profit);\n        }\n    }\n\n    /// \u30A2\u30A4\u30C6\u30E0 is \u306B\u3064\
    \u3044\u3066\u3069\u308C\u304B 1 \u3064\u3067\u3082\u9078\u629E\u80A2 0 \u3092\
    \u9078\u3076\u3068\u30B3\u30B9\u30C8 cost \u304B\u304B\u308B\u3068\u3044\u3046\
    \u6761\u4EF6\u3092\u8FFD\u52A0\u3059\u308B\n    pub fn add_cost_any_0(&mut self,\
    \ is: &[usize], cost: i64) {\n        self.add_cost(cost);\n        self.add_profit_all_1(is,\
    \ cost);\n    }\n\n    /// \u30A2\u30A4\u30C6\u30E0 is \u306B\u3064\u3044\u3066\
    \u3069\u308C\u304B 1 \u3064\u3067\u3082\u9078\u629E\u80A2 1 \u3092\u9078\u3076\
    \u3068\u30B3\u30B9\u30C8 cost \u304B\u304B\u308B\u3068\u3044\u3046\u6761\u4EF6\
    \u3092\u8FFD\u52A0\u3059\u308B\n    pub fn add_cost_any_1(&mut self, is: &[usize],\
    \ cost: i64) {\n        self.add_cost(cost);\n        self.add_profit_all_0(is,\
    \ cost);\n    }\n\n    /// \u6700\u5C0F\u30B3\u30B9\u30C8\u3068\u305D\u308C\u3092\
    \u9054\u6210\u3059\u308B\u9078\u629E\u80A2\u306E\u7D44\u3092\u6C42\u3081\u308B\
    \n    pub fn min_cost(&mut self) -> (i64, Vec<usize>) {\n        let mut g = MaxFlow::new(self.n_item\
    \ + self.n_aux + 2);\n        let s = self.n_item + self.n_aux;\n        let t\
    \ = s + 1;\n\n        for i in 0..self.n_item {\n            let cost = self.cost1[i];\n\
    \            if cost[0] <= cost[1] {\n                self.add_cost(cost[0]);\n\
    \                self.add_edge(S, i, cost[1] - cost[0]);\n            } else {\n\
    \                self.add_cost(cost[1]);\n                self.add_edge(i, T,\
    \ cost[0] - cost[1]);\n            }\n            self.cost1[i] = [0, 0];\n  \
    \      }\n\n        for &(i, j, cost) in &self.edges {\n            let u = match\
    \ i {\n                S => s,\n                T => t,\n                _ =>\
    \ i,\n            };\n            let v = match j {\n                S => s,\n\
    \                T => t,\n                _ => j,\n            };\n          \
    \  g.add_edge(u, v, cost);\n        }\n        let res = self.cost0 + g.max_flow(s,\
    \ t);\n        let mut cut = g.min_cut(s);\n        cut.truncate(self.n_item);\n\
    \        let choice = cut.iter().map(|&b| !b as usize).collect();\n        (res,\
    \ choice)\n    }\n\n    /// \u6700\u5927\u5229\u76CA\u3068\u305D\u308C\u3092\u9054\
    \u6210\u3059\u308B\u9078\u629E\u80A2\u306E\u7D44\u3092\u6C42\u3081\u308B\n   \
    \ pub fn max_profit(&mut self) -> (i64, Vec<usize>) {\n        let (mut res, choice)\
    \ = self.min_cost();\n        res = -res;\n        (res, choice)\n    }\n\n  \
    \  fn add_edge(&mut self, i: usize, j: usize, cost: i64) {\n        assert!(cost\
    \ >= 0);\n        assert!(i != j);\n        assert!(i == S || i == T || i < self.n_item\
    \ + self.n_aux);\n        assert!(j == S || j == T || j < self.n_item + self.n_aux);\n\
    \        if cost == 0 {\n            return;\n        }\n        self.edges.push((i,\
    \ j, cost));\n    }\n}\n"
  dependsOn:
  - crates/graph/max-flow/src/lib.rs
  isVerificationFile: false
  path: crates/graph/project-selection/src/lib.rs
  requiredBy:
  - crates/graph/k-project-selection/src/lib.rs
  timestamp: '2024-12-27 03:53:35+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/graph/project-selection/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/project-selection/src/lib.rs
- /library/crates/graph/project-selection/src/lib.rs.html
title: crates/graph/project-selection/src/lib.rs
---
