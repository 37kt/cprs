---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/tree-query/src/lib.rs
    title: crates/data-structure/tree-query/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/jump_on_tree/src/main.rs
    title: verify/jump_on_tree/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/lca/src/main.rs
    title: verify/lca/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.1/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.1/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use graph::Graph;\nuse std::mem::swap;\n\npub struct HeavyLightDecomposition\
    \ {\n    t_in: Vec<usize>,\n    t_out: Vec<usize>,\n    ord: Vec<usize>,\n   \
    \ size: Vec<usize>,\n    heavy: Vec<usize>,\n    head: Vec<usize>,\n    par: Vec<usize>,\n\
    \    depth: Vec<usize>,\n}\n\nimpl HeavyLightDecomposition {\n    pub fn new<V,\
    \ E>(g: &Graph<V, E>) -> Self\n    where\n        V: Clone,\n        E: Clone,\n\
    \    {\n        let n = g.size();\n        let mut hld = HeavyLightDecomposition\
    \ {\n            t_in: vec![0; n],\n            t_out: vec![0; n],\n         \
    \   ord: vec![],\n            size: vec![0; n],\n            heavy: vec![!0; n],\n\
    \            head: vec![0; n],\n            par: vec![!0; n],\n            depth:\
    \ vec![0; n],\n        };\n        hld.dfs_sz(g, 0);\n        hld.dfs_hld(g, 0,\
    \ &mut 0);\n        hld\n    }\n\n    fn dfs_sz<V, E>(&mut self, g: &Graph<V,\
    \ E>, v: usize)\n    where\n        V: Clone,\n        E: Clone,\n    {\n    \
    \    self.size[v] = 1;\n        for &(u, _) in g.out_edges(v) {\n            if\
    \ u == self.par[v] {\n                continue;\n            }\n            self.par[u]\
    \ = v;\n            self.depth[u] = self.depth[v] + 1;\n            self.dfs_sz(g,\
    \ u);\n            self.size[v] += self.size[u];\n            if self.heavy[v]\
    \ == !0 || self.size[u] > self.size[self.heavy[v]] {\n                self.heavy[v]\
    \ = u;\n            }\n        }\n    }\n\n    fn dfs_hld<V, E>(&mut self, g:\
    \ &Graph<V, E>, v: usize, t: &mut usize)\n    where\n        V: Clone,\n     \
    \   E: Clone,\n    {\n        self.t_in[v] = *t;\n        self.ord.push(v);\n\
    \        *t += 1;\n        if self.heavy[v] != !0 {\n            let u = self.heavy[v];\n\
    \            self.head[u] = self.head[v];\n            self.dfs_hld(g, u, t);\n\
    \        }\n        for &(u, _) in g.out_edges(v) {\n            if u == self.par[v]\
    \ {\n                continue;\n            }\n            if u == self.heavy[v]\
    \ {\n                continue;\n            }\n            self.head[u] = u;\n\
    \            self.dfs_hld(g, u, t);\n        }\n        self.t_out[v] = *t;\n\
    \    }\n}\n\nimpl HeavyLightDecomposition {\n    // \u9802\u70B9v\u306Ek\u500B\
    \u89AA\n    pub fn kth_ancestor(&self, mut v: usize, mut k: usize) -> usize {\n\
    \        if self.depth[v] < k {\n            return !0;\n        }\n        loop\
    \ {\n            let u = self.head[v];\n            if self.t_in[v] - k >= self.t_in[u]\
    \ {\n                return self.ord[self.t_in[v] - k];\n            }\n     \
    \       k -= 1 + self.t_in[v] - self.t_in[u];\n            v = self.par[u];\n\
    \        }\n    }\n\n    // \u9802\u70B9u\u3068\u9802\u70B9v\u306ELCA\n    pub\
    \ fn lca(&self, mut u: usize, mut v: usize) -> usize {\n        loop {\n     \
    \       if self.t_in[u] > self.t_in[v] {\n                swap(&mut u, &mut v);\n\
    \            }\n            if self.head[u] == self.head[v] {\n              \
    \  return u;\n            }\n            v = self.par[self.head[v]];\n       \
    \ }\n    }\n\n    // \u9802\u70B9u\u3068\u9802\u70B9v\u306E\u8DDD\u96E2\n    pub\
    \ fn dist(&self, u: usize, v: usize) -> usize {\n        let l = self.lca(u, v);\n\
    \        self.depth[u] + self.depth[v] - self.depth[l] * 2\n    }\n\n    // \u9802\
    \u70B9u\u304B\u3089\u9802\u70B9v\u306Bk\u3060\u3051\u9032\u3093\u3060\u3068\u304D\
    \u306E\u9802\u70B9\n    pub fn jump(&self, u: usize, v: usize, k: usize) -> usize\
    \ {\n        if k == 0 {\n            return u;\n        }\n        let l = self.lca(u,\
    \ v);\n        let d_lu = self.depth[u] - self.depth[l];\n        let d_lv = self.depth[v]\
    \ - self.depth[l];\n        if k > d_lu + d_lv {\n            !0\n        } else\
    \ if k <= d_lu {\n            self.kth_ancestor(u, k)\n        } else {\n    \
    \        self.kth_ancestor(v, d_lu + d_lv - k)\n        }\n    }\n\n    // \u9802\
    \u70B9v\u306EHLD\u4E0A\u306E\u5834\u6240\n    pub fn vertex(&self, v: usize) ->\
    \ usize {\n        self.t_in[v]\n    }\n\n    // \u8FBA(u,v)\u306EHLD\u4E0A\u306E\
    \u5834\u6240\n    pub fn edge(&self, u: usize, v: usize) -> usize {\n        if\
    \ self.depth[u] < self.depth[v] {\n            self.t_in[v]\n        } else {\n\
    \            self.t_in[u]\n        }\n    }\n\n    // u->v\u306E\u30D1\u30B9\u3092\
    (up\u30D1\u30B9\u306E\u5217,down\u30D1\u30B9\u306E\u5217)\u306B\u5206\u89E3\u3059\
    \u308B\n    // \u30BB\u30B0\u6728\u306B\u6295\u3052\u308B\u306A\u308A\u3057\u3066\
    \u4F7F\u3046\n    // \u975E\u53EF\u63DB\u306E\u3068\u304D\u306Fup\u306E\u90E8\u5206\
    \u306E\u6F14\u7B97\u3092\u53CD\u8EE2\u3055\u305B\u308B\u5FC5\u8981\u304C\u3042\
    \u308B\n    pub fn path(\n        &self,\n        mut u: usize,\n        mut v:\
    \ usize,\n        edge: bool,\n    ) -> (Vec<(usize, usize)>, Vec<(usize, usize)>)\
    \ {\n        let mut up = vec![];\n        let mut down = vec![];\n        let\
    \ e = if edge { 1 } else { 0 };\n        while self.head[u] != self.head[v] {\n\
    \            if self.t_in[u] < self.t_in[v] {\n                down.push((self.t_in[self.head[v]],\
    \ self.t_in[v] + 1));\n                v = self.par[self.head[v]];\n         \
    \   } else {\n                up.push((self.t_in[self.head[u]], self.t_in[u] +\
    \ 1));\n                u = self.par[self.head[u]];\n            }\n        }\n\
    \        if self.t_in[u] < self.t_in[v] {\n            down.push((self.t_in[u]\
    \ + e, self.t_in[v] + 1));\n        } else if self.t_in[u] >= self.t_in[v] + e\
    \ {\n            up.push((self.t_in[v] + e, self.t_in[u] + 1));\n        }\n \
    \       down.reverse();\n        (up, down)\n    }\n\n    // v\u3092\u6839\u3068\
    \u3059\u308B\u90E8\u5206\u6728\u306EHLD\u4E0A\u306E\u7BC4\u56F2\n    pub fn subtree(&self,\
    \ v: usize, edge: bool) -> (usize, usize) {\n        let e = if edge { 1 } else\
    \ { 0 };\n        (self.t_in[v] + e, self.t_out[v])\n    }\n}\n"
  dependsOn:
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/data-structure/heavy-light-decomposition/src/lib.rs
  requiredBy:
  - crates/data-structure/tree-query/src/lib.rs
  timestamp: '2023-05-17 16:30:46+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/jump_on_tree/src/main.rs
  - verify/lca/src/main.rs
documentation_of: crates/data-structure/heavy-light-decomposition/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/heavy-light-decomposition/src/lib.rs
- /library/crates/data-structure/heavy-light-decomposition/src/lib.rs.html
title: crates/data-structure/heavy-light-decomposition/src/lib.rs
---
