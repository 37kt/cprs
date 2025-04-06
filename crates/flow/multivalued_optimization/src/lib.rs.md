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
  _extendedRequiredBy:
  - icon: ':warning:'
    path: verify/sandbox/abc347g/src/main.rs
    title: verify/sandbox/abc347g/src/main.rs
  - icon: ':warning:'
    path: verify/sandbox/abc397g/src/main.rs
    title: verify/sandbox/abc397g/src/main.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yukicoder/flow/yuki0119_mopt/src/main.rs
    title: verify/yukicoder/flow/yuki0119_mopt/src/main.rs
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
  code: "use std::borrow::Borrow;\n\nuse max_flow::MaxFlow;\n\n/// \u591A\u5024\u5909\
    \u6570\u306E\u6700\u9069\u5316 (\u6700\u5C0F\u5316)\n#[derive(Clone)]\npub struct\
    \ MultivaluedOptimization {\n    n_options: Vec<usize>,\n    n_vertices: usize,\n\
    \n    src: usize,\n    dst: usize,\n\n    id: Vec<Vec<usize>>,\n\n    cost_0:\
    \ i64,\n\n    edges: Vec<(usize, usize, i64)>,\n}\n\nimpl MultivaluedOptimization\
    \ {\n    /// \u65B0\u3057\u3044 MultivaluedOptimization \u306E\u30A4\u30F3\u30B9\
    \u30BF\u30F3\u30B9\u3092\u751F\u6210\u3059\u308B\u30B3\u30F3\u30B9\u30C8\u30E9\
    \u30AF\u30BF\u3002\n    ///\n    /// # \u5F15\u6570\n    /// - `n_options`: \u5404\
    \u5909\u6570\u306E\u9078\u629E\u80A2\u306E\u6570\u3092\u8868\u3059\u5024\u306E\
    \u30A4\u30C6\u30EC\u30FC\u30BF\u3002\n    ///\n    /// # \u623B\u308A\u5024\n\
    \    /// \u65B0\u3057\u304F\u521D\u671F\u5316\u3055\u308C\u305F MultivaluedOptimization\
    \ \u306E\u30A4\u30F3\u30B9\u30BF\u30F3\u30B9\u3002\n    pub fn new<I>(n_options:\
    \ I) -> Self\n    where\n        I: IntoIterator,\n        I::Item: Borrow<usize>,\n\
    \    {\n        let n_options: Vec<_> = n_options.into_iter().map(|c| *c.borrow()).collect();\n\
    \n        let src = 0;\n        let dst = 1;\n        let mut cur = 2;\n     \
    \   let id = n_options\n            .iter()\n            .map(|&n| {\n       \
    \         assert!(n > 0);\n                let mut id = vec![!0; n];\n       \
    \         for i in id.iter_mut().skip(1) {\n                    *i = cur;\n  \
    \                  cur += 1;\n                }\n                id\n        \
    \    })\n            .collect();\n\n        let mut opt = Self {\n           \
    \ n_options,\n            n_vertices: cur,\n            src,\n            dst,\n\
    \            id,\n            cost_0: 0,\n            edges: vec![],\n       \
    \ };\n\n        for i in 0..opt.n_options.len() {\n            for j in 1..opt.n_options[i]\
    \ - 1 {\n                opt.add_edge(opt.id[i][j + 1], opt.id[i][j], i64::MAX);\n\
    \            }\n        }\n\n        opt\n    }\n\n    /// \u5B9A\u6570\u3092\u52A0\
    \u7B97\u3059\u308B  \n    ///\n    /// # \u5236\u7D04\n    /// - `|cost| <= 10^9`\n\
    \    ///\n    /// # \u5F15\u6570\n    /// - `cost`: \u52A0\u7B97\u3059\u308B\u5B9A\
    \u6570\u9805\u306E\u30B3\u30B9\u30C8\u3002\n    pub fn add_nullary(&mut self,\
    \ cost: i64) {\n        self.cost_0 += cost;\n    }\n\n    /// 1 \u5909\u6570\u95A2\
    \u6570\u3092\u52A0\u7B97\u3059\u308B  \n    /// +\u221E \u306F None \u3067\u8868\
    \u3059  \n    ///\n    /// # \u5236\u7D04\n    /// - \u6709\u9650\u5024\u304C\u9023\
    \u7D9A\u3059\u308B\u533A\u9593\u304C\u3061\u3087\u3046\u3069 1 \u3064\u5B58\u5728\
    \u3059\u308B  \n    /// - `|cost| <= 10^9` or `cost = None`\n    ///\n    ///\
    \ # \u5F15\u6570\n    /// - `i`: \u5BFE\u8C61\u306E\u5909\u6570\u306E\u30A4\u30F3\
    \u30C7\u30C3\u30AF\u30B9\u3002\n    /// - `cost`: \u5404\u9078\u629E\u80A2\u306B\
    \u5BFE\u3059\u308B\u30B3\u30B9\u30C8\u3092\u8FD4\u3059\u30AF\u30ED\u30FC\u30B8\
    \u30E3\u3002None\u306E\u5834\u5408\u306F+\u221E\u3068\u307F\u306A\u3059\u3002\n\
    \    pub fn add_unary(&mut self, i: usize, cost: impl FnMut(usize) -> Option<i64>)\
    \ {\n        let cost = (0..self.n_options[i]).map(cost).collect::<Vec<_>>();\n\
    \n        let mut appeared_finite_value = false;\n        if let Some(x0) = cost[0]\
    \ {\n            self.add_nullary(x0);\n            appeared_finite_value = true;\n\
    \        }\n\n        for mi in 1..self.n_options[i] {\n            match (cost[mi\
    \ - 1], cost[mi]) {\n                (None, None) => {}\n                (None,\
    \ Some(x1)) => {\n                    assert!(\n                        !appeared_finite_value,\n\
    \                        \"Finite cost values must form a contiguous segment\"\
    \n                    );\n                    appeared_finite_value = true;\n\
    \                    self.add_nullary(x1);\n                    self.add_edge(self.src,\
    \ self.id[i][mi], i64::MAX);\n                }\n                (Some(_), None)\
    \ => {\n                    self.add_edge(self.id[i][mi], self.dst, i64::MAX);\n\
    \                }\n                (Some(x0), Some(x1)) => {\n              \
    \      let diff = x1 - x0;\n                    if diff < 0 {\n              \
    \          self.add_nullary(diff);\n                        self.add_edge(self.src,\
    \ self.id[i][mi], -diff);\n                    } else {\n                    \
    \    self.add_edge(self.id[i][mi], self.dst, diff);\n                    }\n \
    \               }\n            }\n        }\n\n        assert!(\n            appeared_finite_value,\n\
    \            \"There must be at least one finite cost value\"\n        );\n  \
    \  }\n\n    /// 2 \u5909\u6570\u95A2\u6570\u3092\u52A0\u7B97\u3059\u308B  \n \
    \   /// +\u221E \u306F None \u3067\u8868\u3059  \n    ///\n    /// # \u5236\u7D04\
    \n    /// - Monge \u6027\u3092\u6E80\u305F\u3059  \n    /// - `|cost| <= 10^9`\
    \ or `cost = None`  \n    /// - `cost[0][0]` \u304B\u3089 `cost[h-1][w-1]` \u307E\
    \u3067\u304C\u6709\u9650\u5024\u306B\u3088\u3063\u3066\u9023\u7D50\u3067\u3042\
    \u308B  \n    /// - \u884C\u756A\u53F7\u304C\u5897\u3048\u308B\u65B9\u5411\u306B\
    \u3064\u3044\u3066\u3001\u6709\u9650\u5024\u3092\u3068\u308B\u533A\u9593\u306E\
    \u7AEF\u70B9\u306E\u4F4D\u7F6E\u304C\u5E83\u7FA9\u5358\u8ABF\u5897\u52A0\u3067\
    \u3042\u308B  \n    ///\n    /// # \u5F15\u6570\n    /// - `i`, `j`: \u5BFE\u8C61\
    \u306E2\u5909\u6570\u95A2\u6570\u306B\u9069\u7528\u3059\u308B\u5909\u6570\u306E\
    \u30A4\u30F3\u30C7\u30C3\u30AF\u30B9\u3002\n    /// - `cost`: \u5404\u7D44\u5408\
    \u305B\u306B\u5BFE\u3059\u308B\u30B3\u30B9\u30C8\u3092\u8FD4\u3059\u30AF\u30ED\
    \u30FC\u30B8\u30E3\u3002None\u306E\u5834\u5408\u306F+\u221E\u3068\u307F\u306A\u3059\
    \u3002\n    pub fn add_binary(\n        &mut self,\n        i: usize,\n      \
    \  j: usize,\n        mut cost: impl FnMut(usize, usize) -> Option<i64>,\n   \
    \ ) {\n        let h = self.n_options[i];\n        let w = self.n_options[j];\n\
    \        let mut cost = (0..h)\n            .map(|mi| {\n                (0..w)\n\
    \                    .map(|mj| cost(mi, mj).unwrap_or(i64::MAX))\n           \
    \         .collect::<Vec<_>>()\n            })\n            .collect::<Vec<_>>();\n\
    \n        // \u5404\u884C\u306E\u6709\u9650\u5024\u306E\u533A\u9593\u3092\u6C42\
    \u3081\u308B\n        let mut l = vec![w; h];\n        let mut r = vec![0; h];\n\
    \        for i in 0..h {\n            for j in 0..w {\n                if cost[i][j]\
    \ != i64::MAX {\n                    l[i] = l[i].min(j);\n                   \
    \ r[i] = r[i].max(j + 1);\n                }\n            }\n            assert!(\n\
    \                l[i] < r[i],\n                \"Each row must contain at least\
    \ one finite cost value\"\n            );\n            assert!(\n            \
    \    (l[i]..r[i]).all(|j| cost[i][j] != i64::MAX),\n                \"Finite cost\
    \ values in each row must appear in a contiguous block\"\n            );\n   \
    \     }\n\n        // \u5DE6\u4E0A\u3068\u53F3\u4E0B\u304C\u6709\u9650\u5024\u306B\
    \u3088\u3063\u3066\u9023\u7D50\u3067\u3042\u308B\u3053\u3068\u3092\u78BA\u8A8D\
    \u3059\u308B\n        // \u307E\u305F\u3001\u6709\u9650\u5024\u306E\u533A\u9593\
    \u304C\u4E0B\u306E\u884C\u306B\u3044\u304F\u306B\u3064\u308C\u3066\u53F3\u306B\
    \u79FB\u52D5\u3057\u3066\u3044\u304F\u3053\u3068\u3092\u78BA\u8A8D\u3059\u308B\
    \n        let mut is_connected = 0 < r[0];\n        for i in 1..h {\n        \
    \    is_connected &= l[i - 1] <= l[i] && l[i] < r[i - 1] && r[i - 1] <= r[i];\n\
    \        }\n        is_connected &= l[h - 1] < w && w - 1 < r[h - 1];\n      \
    \  assert!(\n            is_connected,\n            \"Finite cost values in the\
    \ matrix must form a single connected region that spans from cost[0][0] to cost[h-1][w-1]\"\
    \n        );\n\n        // monge \u6027\u3092\u6E80\u305F\u3059\u3088\u3046\u306B\
    \u7121\u52B9\u5024\u3092\u57CB\u3081\u308B\n        // \u305F\u3060\u3057\u3001\
    \u5024\u306F INF \u4EE5\u4E0A\u3068\u3059\u308B\n        // INF \u306F O(K) \u500D\
    \u3057\u3066\u3082\u30AA\u30FC\u30D0\u30FC\u30D5\u30ED\u30FC\u3057\u306A\u3044\
    \u7A0B\u5EA6\u306E\u5927\u304D\u306A\u5024\n        const INF: i64 = 1 << 40;\n\
    \        for i in (0..h - 1).rev() {\n            for j in r[i]..w {\n       \
    \         cost[i][j] = INF.max(cost[i][j - 1] + cost[i + 1][j] - cost[i + 1][j\
    \ - 1]);\n            }\n        }\n        for i in 1..h {\n            for j\
    \ in (0..l[i]).rev() {\n                cost[i][j] = INF.max(cost[i - 1][j] +\
    \ cost[i][j + 1] - cost[i - 1][j + 1]);\n            }\n        }\n\n        //\
    \ 1 \u5909\u6570\u95A2\u6570 \u03B8_j(mj) = \u03C6_ij(0, mj) \u3092\u30D5\u30ED\
    \u30FC\u306B\u8FFD\u52A0\u3057\u3001\u03C6 \u304B\u3089\u5F15\u304F\n        self.add_unary(j,\
    \ |mj| Some(cost[0][mj]));\n\n        // 1 \u5909\u6570\u95A2\u6570 \u03B8_i(mi)\
    \ = \u03C6_ij(mi, w-1) \u3092\u30D5\u30ED\u30FC\u306B\u8FFD\u52A0\u3057\u3001\u03C6\
    \ \u304B\u3089\u5F15\u304F\n        self.add_unary(i, |mi| Some(cost[mi][w - 1]\
    \ - cost[0][w - 1]));\n\n        // 2 \u5909\u6570\u95A2\u6570 \u03C6_ij(mi, mj)\
    \ \u3092\u30D5\u30ED\u30FC\u306B\u8FFD\u52A0\u3059\u308B\n        for mi in 1..h\
    \ {\n            for mj in 1..w {\n                let x = -cost[mi - 1][mj -\
    \ 1] + cost[mi - 1][mj] + cost[mi][mj - 1] - cost[mi][mj];\n                assert!(x\
    \ >= 0, \"Monge property is violated\");\n                self.add_edge(self.id[i][mi],\
    \ self.id[j][mj], x);\n            }\n        }\n    }\n\n    /// \u6700\u9069\
    \u5316\u554F\u984C\u3092\u89E3\u3044\u3066\u3001\u6700\u5C0F\u30B3\u30B9\u30C8\
    \u3068\u5404\u5909\u6570\u306E\u9078\u629E\u5024\u3092\u8FD4\u3059\u3002\n   \
    \ ///\n    /// # \u623B\u308A\u5024\n    /// - \u30BF\u30D7\u30EB (\u6700\u5C0F\
    \u30B3\u30B9\u30C8, \u5404\u5909\u6570\u306B\u5BFE\u3059\u308B\u9078\u629E\u3055\
    \u308C\u305F\u30A4\u30F3\u30C7\u30C3\u30AF\u30B9)\u3002\n    pub fn solve(&self)\
    \ -> (i64, Vec<usize>) {\n        let mut flow = MaxFlow::new();\n        flow.add_vertices(self.n_vertices);\n\
    \        for &(i, j, cost) in &self.edges {\n            flow.add_edge(i, j, cost);\n\
    \        }\n\n        let cost = self\n            .cost_0\n            .saturating_add(flow.max_flow(self.src,\
    \ self.dst, None));\n        let cut = flow.min_cut();\n\n        let mut choice\
    \ = vec![0; self.n_options.len()];\n        for i in 0..self.n_options.len() {\n\
    \            for j in 1..self.n_options[i] {\n                if cut[self.id[i][j]]\
    \ == 0 {\n                    choice[i] += 1;\n                }\n           \
    \ }\n        }\n\n        (cost, choice)\n    }\n\n    fn add_edge(&mut self,\
    \ i: usize, j: usize, cost: i64) {\n        assert!(cost >= 0);\n        if cost\
    \ == 0 {\n            return;\n        }\n        self.edges.push((i, j, cost));\n\
    \    }\n}\n"
  dependsOn:
  - crates/flow/max_flow/src/graph.rs
  - crates/flow/max_flow/src/lib.rs
  - crates/flow/max_flow/src/queue.rs
  isVerificationFile: false
  path: crates/flow/multivalued_optimization/src/lib.rs
  requiredBy:
  - verify/sandbox/abc397g/src/main.rs
  - verify/sandbox/abc347g/src/main.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yukicoder/flow/yuki0119_mopt/src/main.rs
documentation_of: crates/flow/multivalued_optimization/src/lib.rs
layout: document
redirect_from:
- /library/crates/flow/multivalued_optimization/src/lib.rs
- /library/crates/flow/multivalued_optimization/src/lib.rs.html
title: crates/flow/multivalued_optimization/src/lib.rs
---
