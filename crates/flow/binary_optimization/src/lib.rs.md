---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/flow/max_flow/src/graph.rs
    title: crates/flow/max_flow/src/graph.rs
  - icon: ':heavy_check_mark:'
    path: crates/flow/max_flow/src/lib.rs
    title: crates/flow/max_flow/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/flow/max_flow/src/queue.rs
    title: crates/flow/max_flow/src/queue.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yukicoder/flow/yuki2713_binopt/src/main.rs
    title: verify/yukicoder/flow/yuki2713_binopt/src/main.rs
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
  code: "use std::borrow::Borrow;\n\nuse max_flow::MaxFlow;\n\n/// 2 \u5024\u5909\u6570\
    \u306E\u6700\u9069\u5316 (\u6700\u5C0F\u5316)\n#[derive(Clone)]\npub struct BinaryOptimization\
    \ {\n    n_item: usize,\n    n_aux: usize,\n\n    src: usize,\n    dst: usize,\n\
    \n    cost_0: i64,           // 0 \u5909\u6570\u306B\u3064\u3044\u3066\u306E\u30B3\
    \u30B9\u30C8\n    cost_1: Vec<[i64; 2]>, // 1 \u5909\u6570\u306B\u3064\u3044\u3066\
    \u306E\u30B3\u30B9\u30C8\n\n    edges: Vec<(usize, usize, i64)>,\n}\n\nimpl BinaryOptimization\
    \ {\n    pub fn new(n: usize) -> Self {\n        Self {\n            n_item: n,\n\
    \            n_aux: 0,\n\n            src: n,\n            dst: n + 1,\n\n   \
    \         cost_0: 0,\n            cost_1: vec![[0; 2]; n],\n\n            edges:\
    \ vec![],\n        }\n    }\n\n    /// \u5B9A\u6570\u3092\u52A0\u7B97\u3059\u308B\
    \  \n    ///\n    /// # \u5F15\u6570\n    /// - `cost`: \u52A0\u7B97\u3059\u308B\
    \u5B9A\u6570\u9805\u306E\u30B3\u30B9\u30C8\u3002\n    pub fn add_nullary(&mut\
    \ self, cost: i64) {\n        self.cost_0 = self.cost_0.saturating_add(cost);\n\
    \    }\n\n    /// 1 \u5909\u6570\u95A2\u6570\u3092\u52A0\u7B97\u3059\u308B  \n\
    \    /// +\u221E \u306F None \u3067\u8868\u3059  \n    ///\n    /// # \u5F15\u6570\
    \n    /// - `i`: \u5BFE\u8C61\u306E\u5909\u6570\u306E\u30A4\u30F3\u30C7\u30C3\u30AF\
    \u30B9\u3002\n    /// - `cost`: \u5404\u9078\u629E\u80A2\u306B\u5BFE\u3059\u308B\
    \u30B3\u30B9\u30C8\u3092\u8FD4\u3059\u30AF\u30ED\u30FC\u30B8\u30E3\u3002None\u306E\
    \u5834\u5408\u306F+\u221E\u3068\u307F\u306A\u3059\u3002\n    pub fn add_unary(&mut\
    \ self, i: usize, mut cost: impl FnMut(usize) -> Option<i64>) {\n        for bi\
    \ in 0..2 {\n            let x = cost(bi).unwrap_or(i64::MAX);\n            self.cost_1[i][bi]\
    \ = self.cost_1[i][bi].saturating_add(x);\n        }\n    }\n\n    /// 2 \u5909\
    \u6570\u95A2\u6570\u3092\u52A0\u7B97\u3059\u308B  \n    /// +\u221E \u306F None\
    \ \u3067\u8868\u3059  \n    ///\n    /// # \u5236\u7D04\n    /// - Monge \u6027\
    \u3092\u6E80\u305F\u3059  \n    /// - `|cost| <= 10^9` or `cost = None`  \n  \
    \  /// - None \u306F\u975E\u5BFE\u89D2\u7DDA\u4E0A\u306B\u9AD8\u3005 1 \u3064\u307E\
    \u3067\u3001\u5BFE\u89D2\u7DDA\u4E0A\u306B\u5B58\u5728\u3057\u306A\u3044\n   \
    \ ///\n    /// # \u5F15\u6570\n    /// - `i`, `j`: \u5BFE\u8C61\u306E2\u5909\u6570\
    \u95A2\u6570\u306B\u9069\u7528\u3059\u308B\u5909\u6570\u306E\u30A4\u30F3\u30C7\
    \u30C3\u30AF\u30B9\u3002\n    pub fn add_binary(\n        &mut self,\n       \
    \ i: usize,\n        j: usize,\n        mut cost: impl FnMut(usize, usize) ->\
    \ Option<i64>,\n    ) {\n        const INF: i64 = 1 << 40; // TODO: \u3053\u308C\
    \u306F\u5927\u4E08\u592B\uFF1F\n\n        let cost: [[i64; 2]; 2] =\n        \
    \    std::array::from_fn(|bi| std::array::from_fn(|bj| cost(bi, bj).unwrap_or(INF)));\n\
    \n        let x = cost[0][1] + cost[1][0] - cost[0][0] - cost[1][1];\n       \
    \ assert!(x >= 0, \"Monge property is violated\");\n\n        self.add_nullary(cost[0][0]);\n\
    \        self.add_unary(i, |bi| Some([0, cost[1][0] - cost[0][0]][bi]));\n   \
    \     self.add_unary(j, |bj| Some([0, cost[1][1] - cost[1][0]][bj]));\n      \
    \  self.add_edge(i, j, x);\n    }\n\n    /// 3 \u5909\u6570\u95A2\u6570\u3092\u52A0\
    \u7B97\u3059\u308B  \n    ///\n    /// ??????????\n    pub fn add_ternary(\n \
    \       &mut self,\n        _i: usize,\n        _j: usize,\n        _k: usize,\n\
    \        mut _cost: impl FnMut(usize, usize, usize) -> i64,\n    ) {\n       \
    \ todo!()\n    }\n\n    /// \u5168\u3066\u306E\u5909\u6570\u304C 0 \u306E\u5834\
    \u5408\u306B\u306E\u307F\u30B3\u30B9\u30C8\u3092\u52A0\u7B97\u3059\u308B  \n \
    \   ///\n    /// # \u5236\u7D04\n    /// - `items.len() < 2` or `cost <= 0`\n\
    \    ///\n    /// # \u5F15\u6570\n    /// - `items`: \u5BFE\u8C61\u306E\u5909\u6570\
    \u306E\u30A4\u30F3\u30C7\u30C3\u30AF\u30B9\u3002\n    /// - `cost`: \u52A0\u7B97\
    \u3059\u308B\u30B3\u30B9\u30C8\u3002\n    pub fn add_if_all_0<I>(&mut self, items:\
    \ I, cost: i64)\n    where\n        I: IntoIterator,\n        I::Item: Borrow<usize>,\n\
    \    {\n        if cost == 0 {\n            return;\n        }\n\n        let\
    \ mut items = items.into_iter().map(|x| *x.borrow()).collect::<Vec<_>>();\n  \
    \      items.sort_unstable();\n        items.dedup();\n\n        assert!(\n  \
    \          items.len() < 2 || cost <= 0,\n            \"if items.len() >= 2, cost\
    \ must be non-positive\"\n        );\n\n        match &items[..] {\n         \
    \   [] => self.add_nullary(cost),\n            &[i] => self.add_unary(i, |bi|\
    \ Some([cost, 0][bi])),\n            &[i, j] => self.add_binary(i, j, |bi, bj|\
    \ Some([[cost, 0], [0, 0]][bi][bj])),\n            items => {\n              \
    \  self.add_nullary(cost);\n                let aux = self.n_item + 2 + self.n_aux;\n\
    \                self.n_aux += 1;\n                self.add_edge(self.src, aux,\
    \ -cost);\n                for &i in items {\n                    self.add_edge(aux,\
    \ i, -cost);\n                }\n            }\n        }\n    }\n\n    /// \u5168\
    \u3066\u306E\u5909\u6570\u304C 1 \u306E\u5834\u5408\u306B\u306E\u307F\u30B3\u30B9\
    \u30C8\u3092\u52A0\u7B97\u3059\u308B  \n    ///\n    /// # \u5236\u7D04\n    ///\
    \ - `items.len() < 2` or `cost <= 0`\n    ///\n    /// # \u5F15\u6570\n    ///\
    \ - `items`: \u5BFE\u8C61\u306E\u5909\u6570\u306E\u30A4\u30F3\u30C7\u30C3\u30AF\
    \u30B9\u3002\n    /// - `cost`: \u52A0\u7B97\u3059\u308B\u30B3\u30B9\u30C8\u3002\
    \n    pub fn add_if_all_1<I>(&mut self, items: I, cost: i64)\n    where\n    \
    \    I: IntoIterator,\n        I::Item: Borrow<usize>,\n    {\n        if cost\
    \ == 0 {\n            return;\n        }\n\n        let mut items = items.into_iter().map(|x|\
    \ *x.borrow()).collect::<Vec<_>>();\n        items.sort_unstable();\n        items.dedup();\n\
    \n        assert!(\n            items.len() < 2 || cost <= 0,\n            \"\
    if items.len() >= 2, cost must be non-positive\"\n        );\n\n        match\
    \ &items[..] {\n            [] => self.add_nullary(cost),\n            &[i] =>\
    \ self.add_unary(i, |bi| Some([0, cost][bi])),\n            &[i, j] => self.add_binary(i,\
    \ j, |bi, bj| Some([[0, 0], [0, cost]][bi][bj])),\n            items => {\n  \
    \              self.add_nullary(cost);\n                let aux = self.n_item\
    \ + 2 + self.n_aux;\n                self.n_aux += 1;\n                for &i\
    \ in items {\n                    self.add_edge(i, aux, -cost);\n            \
    \    }\n                self.add_edge(aux, self.dst, -cost);\n            }\n\
    \        }\n    }\n\n    /// \u3042\u308B\u5909\u6570\u304C 0 \u306E\u5834\u5408\
    \u306B\u306E\u307F\u30B3\u30B9\u30C8\u3092\u52A0\u7B97\u3059\u308B  \n    ///\n\
    \    /// # \u5236\u7D04\n    /// - `items.len() < 2` or `cost >= 0`\n    ///\n\
    \    /// # \u5F15\u6570\n    /// - `items`: \u5BFE\u8C61\u306E\u5909\u6570\u306E\
    \u30A4\u30F3\u30C7\u30C3\u30AF\u30B9\u3002\n    /// - `cost`: \u52A0\u7B97\u3059\
    \u308B\u30B3\u30B9\u30C8\u3002\n    pub fn add_if_any_0<I>(&mut self, items: I,\
    \ cost: i64)\n    where\n        I: IntoIterator,\n        I::Item: Borrow<usize>,\n\
    \    {\n        self.add_nullary(cost);\n        self.add_if_all_1(items, -cost);\n\
    \    }\n\n    /// \u3042\u308B\u5909\u6570\u304C 1 \u306E\u5834\u5408\u306B\u306E\
    \u307F\u30B3\u30B9\u30C8\u3092\u52A0\u7B97\u3059\u308B  \n    ///\n    /// # \u5236\
    \u7D04\n    /// - `items.len() < 2` or `cost >= 0`\n    ///\n    /// # \u5F15\u6570\
    \n    /// - `items`: \u5BFE\u8C61\u306E\u5909\u6570\u306E\u30A4\u30F3\u30C7\u30C3\
    \u30AF\u30B9\u3002\n    /// - `cost`: \u52A0\u7B97\u3059\u308B\u30B3\u30B9\u30C8\
    \u3002\n    pub fn add_if_any_1<I>(&mut self, items: I, cost: i64)\n    where\n\
    \        I: IntoIterator,\n        I::Item: Borrow<usize>,\n    {\n        self.add_nullary(cost);\n\
    \        self.add_if_all_0(items, -cost);\n    }\n\n    /// \u6700\u9069\u5316\
    \u554F\u984C\u3092\u89E3\u3044\u3066\u3001\u6700\u5C0F\u30B3\u30B9\u30C8\u3068\
    \u5404\u5909\u6570\u306E\u9078\u629E\u5024\u3092\u8FD4\u3059\u3002\n    ///\n\
    \    /// # \u623B\u308A\u5024\n    /// - \u30BF\u30D7\u30EB (\u6700\u5C0F\u30B3\
    \u30B9\u30C8, \u5404\u5909\u6570\u306B\u5BFE\u3059\u308B\u9078\u629E\u3055\u308C\
    \u305F\u30A4\u30F3\u30C7\u30C3\u30AF\u30B9)\u3002\n    pub fn solve(&mut self)\
    \ -> (i64, Vec<usize>) {\n        for (i, cost) in std::mem::take(&mut self.cost_1).into_iter().enumerate()\
    \ {\n            if cost[0] < cost[1] {\n                self.add_nullary(cost[0]);\n\
    \                self.add_edge(self.src, i, cost[1].saturating_sub(cost[0]));\n\
    \            } else {\n                self.add_nullary(cost[1]);\n          \
    \      self.add_edge(i, self.dst, cost[0].saturating_sub(cost[1]));\n        \
    \    }\n        }\n\n        let mut flow = MaxFlow::new();\n        flow.add_vertices(self.n_item\
    \ + 2 + self.n_aux);\n        for &(i, j, cost) in &self.edges {\n           \
    \ flow.add_edge(i, j, cost);\n        }\n\n        let cost = self\n         \
    \   .cost_0\n            .saturating_add(flow.max_flow(self.src, self.dst, None));\n\
    \        let mut cut = flow.min_cut();\n        cut.truncate(self.n_item);\n\n\
    \        (cost, cut)\n    }\n\n    fn add_edge(&mut self, i: usize, j: usize,\
    \ cost: i64) {\n        assert!(cost >= 0);\n        if cost == 0 {\n        \
    \    return;\n        }\n        self.edges.push((i, j, cost));\n    }\n}\n"
  dependsOn:
  - crates/flow/max_flow/src/graph.rs
  - crates/flow/max_flow/src/lib.rs
  - crates/flow/max_flow/src/queue.rs
  isVerificationFile: false
  path: crates/flow/binary_optimization/src/lib.rs
  requiredBy: []
  timestamp: '2025-03-25 09:58:57+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yukicoder/flow/yuki2713_binopt/src/main.rs
documentation_of: crates/flow/binary_optimization/src/lib.rs
layout: document
redirect_from:
- /library/crates/flow/binary_optimization/src/lib.rs
- /library/crates/flow/binary_optimization/src/lib.rs.html
title: crates/flow/binary_optimization/src/lib.rs
---
