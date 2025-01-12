---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
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
  code: "use graph::UndirectedGraph;\nuse std::mem::swap;\n\n/// \u91CD\u8EFD\u5206\
    \u89E3\n/// t_in[v]: Euler Tour \u306B\u304A\u3051\u308B\u90E8\u5206\u6728 v \u306E\
    \u59CB\u70B9\n/// t_out[v]: Euler Tour \u306B\u304A\u3051\u308B\u90E8\u5206\u6728\
    \ v \u306E\u7D42\u70B9\n/// ord: Euler Tour \u306E\u9806\u5E8F\n/// size[v]: \u90E8\
    \u5206\u6728 v \u306E\u30B5\u30A4\u30BA\n/// heavy[v]: v \u306E heavy-edge \u306B\
    \u7E4B\u304C\u308B\u5B50\n/// head[v]: v \u3092\u542B\u3080 heavy-path \u306E\u5148\
    \u982D\n/// par[v]: v \u306E\u89AA\n/// depth[v]: v \u306E\u6DF1\u3055\n#[derive(Clone)]\n\
    pub struct HeavyLightDecomposition {\n    pub t_in: Vec<usize>,\n    pub t_out:\
    \ Vec<usize>,\n    pub ord: Vec<usize>,\n    pub size: Vec<usize>,\n    pub heavy:\
    \ Vec<usize>,\n    pub head: Vec<usize>,\n    pub par: Vec<usize>,\n    pub depth:\
    \ Vec<usize>,\n}\n\nimpl HeavyLightDecomposition {\n    /// \u91CD\u8EFD\u5206\
    \u89E3\u3092\u69CB\u7BC9\u3059\u308B\u3002\n    ///\n    /// # \u5165\u529B\n\
    \    ///\n    /// - `g`: \u9023\u7D50\u7121\u5411\u30B0\u30E9\u30D5\n    ///\n\
    \    /// # \u8A08\u7B97\u91CF\n    ///\n    /// O(n)\n    pub fn new<V, E>(g:\
    \ &UndirectedGraph<V, E>) -> Self\n    where\n        V: Clone,\n        E: Clone,\n\
    \    {\n        let n = g.len();\n        let mut hld = HeavyLightDecomposition\
    \ {\n            t_in: vec![0; n],\n            t_out: vec![0; n],\n         \
    \   ord: vec![],\n            size: vec![0; n],\n            heavy: vec![!0; n],\n\
    \            head: vec![0; n],\n            par: vec![!0; n],\n            depth:\
    \ vec![0; n],\n        };\n        hld.dfs_sz(g, 0);\n        hld.dfs_hld(g, 0,\
    \ &mut 0);\n        hld\n    }\n\n    fn dfs_sz<V, E>(&mut self, g: &UndirectedGraph<V,\
    \ E>, v: usize)\n    where\n        V: Clone,\n        E: Clone,\n    {\n    \
    \    self.size[v] = 1;\n        for &(u, _) in &g[v] {\n            if u == self.par[v]\
    \ {\n                continue;\n            }\n            self.par[u] = v;\n\
    \            self.depth[u] = self.depth[v] + 1;\n            self.dfs_sz(g, u);\n\
    \            self.size[v] += self.size[u];\n            if self.heavy[v] == !0\
    \ || self.size[u] > self.size[self.heavy[v]] {\n                self.heavy[v]\
    \ = u;\n            }\n        }\n    }\n\n    fn dfs_hld<V, E>(&mut self, g:\
    \ &UndirectedGraph<V, E>, v: usize, t: &mut usize)\n    where\n        V: Clone,\n\
    \        E: Clone,\n    {\n        self.t_in[v] = *t;\n        self.ord.push(v);\n\
    \        *t += 1;\n        if self.heavy[v] != !0 {\n            let u = self.heavy[v];\n\
    \            self.head[u] = self.head[v];\n            self.dfs_hld(g, u, t);\n\
    \        }\n        for &(u, _) in &g[v] {\n            if u == self.par[v] {\n\
    \                continue;\n            }\n            if u == self.heavy[v] {\n\
    \                continue;\n            }\n            self.head[u] = u;\n   \
    \         self.dfs_hld(g, u, t);\n        }\n        self.t_out[v] = *t;\n   \
    \ }\n}\n\nimpl HeavyLightDecomposition {\n    /// \u9802\u70B9 v \u306E k \u500B\
    \u4E0A\u306E\u7956\u5148\u3092\u53D6\u5F97\u3059\u308B\u3002\n    /// \u5B58\u5728\
    \u3057\u306A\u3044\u5834\u5408\u306F !0 \u3092\u8FD4\u3059\u3002\n    ///\n  \
    \  /// # \u8A08\u7B97\u91CF\n    ///\n    /// O(log n)\n    pub fn kth_ancestor(&self,\
    \ mut v: usize, mut k: usize) -> usize {\n        if self.depth[v] < k {\n   \
    \         return !0;\n        }\n        loop {\n            let u = self.head[v];\n\
    \            if self.t_in[v] - k >= self.t_in[u] {\n                return self.ord[self.t_in[v]\
    \ - k];\n            }\n            k -= 1 + self.t_in[v] - self.t_in[u];\n  \
    \          v = self.par[u];\n        }\n    }\n\n    /// \u9802\u70B9 u \u3068\
    \u9802\u70B9 v \u306E LCA \u3092\u53D6\u5F97\u3059\u308B\u3002\n    ///\n    ///\
    \ # \u8A08\u7B97\u91CF\n    ///\n    /// O(log n)\n    pub fn lca(&self, mut u:\
    \ usize, mut v: usize) -> usize {\n        loop {\n            if self.t_in[u]\
    \ > self.t_in[v] {\n                swap(&mut u, &mut v);\n            }\n   \
    \         if self.head[u] == self.head[v] {\n                return u;\n     \
    \       }\n            v = self.par[self.head[v]];\n        }\n    }\n\n    ///\
    \ \u9802\u70B9 u \u3068\u9802\u70B9 v \u306E\u8DDD\u96E2\u3092\u53D6\u5F97\u3059\
    \u308B\u3002\n    ///\n    /// # \u8A08\u7B97\u91CF\n    ///\n    /// O(log n)\n\
    \    pub fn dist(&self, u: usize, v: usize) -> usize {\n        let l = self.lca(u,\
    \ v);\n        self.depth[u] + self.depth[v] - self.depth[l] * 2\n    }\n\n  \
    \  /// \u9802\u70B9 u \u304B\u3089\u9802\u70B9 v \u306B k \u3060\u3051\u9032\u3093\
    \u3060\u3068\u304D\u306E\u9802\u70B9\u3092\u53D6\u5F97\u3059\u308B\u3002\n   \
    \ /// v \u3092\u8D85\u3048\u3066\u3057\u307E\u3046\u5834\u5408\u306F !0 \u3092\
    \u8FD4\u3059\u3002\n    ///\n    /// # \u8A08\u7B97\u91CF\n    ///\n    /// O(log\
    \ n)\n    pub fn jump(&self, u: usize, v: usize, k: usize) -> usize {\n      \
    \  if k == 0 {\n            return u;\n        }\n        let l = self.lca(u,\
    \ v);\n        let d_lu = self.depth[u] - self.depth[l];\n        let d_lv = self.depth[v]\
    \ - self.depth[l];\n        if k > d_lu + d_lv {\n            !0\n        } else\
    \ if k <= d_lu {\n            self.kth_ancestor(u, k)\n        } else {\n    \
    \        self.kth_ancestor(v, d_lu + d_lv - k)\n        }\n    }\n\n    /// \u9802\
    \u70B9 v \u306E HLD \u4E0A\u306E\u5834\u6240\u3092\u53D6\u5F97\u3059\u308B\u3002\
    \n    ///\n    /// # \u8A08\u7B97\u91CF\n    ///\n    /// O(1)\n    pub fn vertex(&self,\
    \ v: usize) -> usize {\n        self.t_in[v]\n    }\n\n    /// \u8FBA (u, v) \u306E\
    \ HLD \u4E0A\u306E\u5834\u6240\u3092\u53D6\u5F97\u3059\u308B\u3002\n    ///\n\
    \    /// # \u8A08\u7B97\u91CF\n    ///\n    /// O(1)\n    pub fn edge(&self, u:\
    \ usize, v: usize) -> usize {\n        if self.depth[u] < self.depth[v] {\n  \
    \          assert!(self.par[v] == u);\n            self.t_in[v]\n        } else\
    \ {\n            assert!(self.par[u] == v);\n            self.t_in[u]\n      \
    \  }\n    }\n\n    /// \u9802\u70B9 u \u304B\u3089\u9802\u70B9 v \u306E\u30D1\u30B9\
    \u3092 (up \u30D1\u30B9\u306E\u5217, down \u30D1\u30B9\u306E\u5217) \u306B\u5206\
    \u89E3\u3059\u308B\u3002  \n    /// \u3053\u308C\u3089\u306E\u5217\u3092 SegmentTree\
    \ \u7B49\u306B\u4E0E\u3048\u308B\u3053\u3068\u3067\uFF0C\n    /// \u30D1\u30B9\
    \u306B\u5BFE\u3059\u308B\u30AF\u30A8\u30EA\u3092\u51E6\u7406\u3059\u308B\u3053\
    \u3068\u304C\u3067\u304D\u308B\u3002  \n    /// \u975E\u53EF\u63DB\u306E\u5834\
    \u5408\u306F up \u306E\u90E8\u5206\u306E\u6F14\u7B97\u3092\u53CD\u8EE2\u3055\u305B\
    \u308B\u5FC5\u8981\u304C\u3042\u308B\u3002\n    ///\n    /// # \u5F15\u6570\n\
    \    ///\n    /// - `u`: \u9802\u70B9 u\n    /// - `v`: \u9802\u70B9 v\n    ///\
    \ - `edge`: true \u306E\u5834\u5408\uFF0C\u8FBA\u30AF\u30A8\u30EA\u3068\u3057\u3066\
    \u51E6\u7406\u3059\u308B\u3002\n    ///\n    /// # \u623B\u308A\u5024\n    ///\n\
    \    /// - `up`: up \u30D1\u30B9\u306E\u5217\n    /// - `down`: down \u30D1\u30B9\
    \u306E\u5217\n    ///\n    /// # \u8A08\u7B97\u91CF\n    ///\n    /// O(log n)\n\
    \    pub fn path(\n        &self,\n        mut u: usize,\n        mut v: usize,\n\
    \        edge: bool,\n    ) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {\n\
    \        let mut up = vec![];\n        let mut down = vec![];\n        let e =\
    \ if edge { 1 } else { 0 };\n        while self.head[u] != self.head[v] {\n  \
    \          if self.t_in[u] < self.t_in[v] {\n                down.push((self.t_in[self.head[v]],\
    \ self.t_in[v] + 1));\n                v = self.par[self.head[v]];\n         \
    \   } else {\n                up.push((self.t_in[self.head[u]], self.t_in[u] +\
    \ 1));\n                u = self.par[self.head[u]];\n            }\n        }\n\
    \        if self.t_in[u] < self.t_in[v] {\n            down.push((self.t_in[u]\
    \ + e, self.t_in[v] + 1));\n        } else if self.t_in[u] >= self.t_in[v] + e\
    \ {\n            up.push((self.t_in[v] + e, self.t_in[u] + 1));\n        }\n \
    \       down.reverse();\n        (up, down)\n    }\n\n    /// \u9802\u70B9 v \u3092\
    \u6839\u3068\u3059\u308B\u90E8\u5206\u6728\u306E HLD \u4E0A\u306E\u7BC4\u56F2\u3092\
    \u53D6\u5F97\u3059\u308B\u3002\n    /// \u3053\u306E\u7BC4\u56F2\u3092 SegmentTree\
    \ \u7B49\u306B\u4E0E\u3048\u308B\u3053\u3068\u3067\uFF0C\n    /// \u90E8\u5206\
    \u6728\u306B\u5BFE\u3059\u308B\u30AF\u30A8\u30EA\u3092\u51E6\u7406\u3059\u308B\
    \u3053\u3068\u304C\u3067\u304D\u308B\u3002\n    ///\n    /// # \u5F15\u6570\n\
    \    ///\n    /// - `v`: \u9802\u70B9\n    /// - `edge`: true \u306E\u5834\u5408\
    \uFF0C\u8FBA\u30AF\u30A8\u30EA\u3068\u3057\u3066\u51E6\u7406\u3059\u308B\u3002\
    \n    ///\n    /// # \u623B\u308A\u5024\n    ///\n    /// - `l`: \u90E8\u5206\u6728\
    \u306E HLD \u4E0A\u306E\u59CB\u70B9\n    /// - `r`: \u90E8\u5206\u6728\u306E HLD\
    \ \u4E0A\u306E\u7D42\u70B9\n    ///\n    /// # \u8A08\u7B97\u91CF\n    ///\n \
    \   /// O(1)\n    pub fn subtree(&self, v: usize, edge: bool) -> (usize, usize)\
    \ {\n        let e = if edge { 1 } else { 0 };\n        (self.t_in[v] + e, self.t_out[v])\n\
    \    }\n}\n"
  dependsOn:
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/data-structure/heavy-light-decomposition-old/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-12 04:36:01+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data-structure/heavy-light-decomposition-old/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/heavy-light-decomposition-old/src/lib.rs
- /library/crates/data-structure/heavy-light-decomposition-old/src/lib.rs.html
title: crates/data-structure/heavy-light-decomposition-old/src/lib.rs
---
