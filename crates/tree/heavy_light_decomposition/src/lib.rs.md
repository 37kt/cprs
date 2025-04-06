---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/csr_array/src/builder.rs
    title: crates/data_structure/csr_array/src/builder.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/csr_array/src/csr_array.rs
    title: crates/data_structure/csr_array/src/csr_array.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/csr_array/src/lib.rs
    title: crates/data_structure/csr_array/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy_light_decomposition/src/compress.rs
    title: crates/tree/heavy_light_decomposition/src/compress.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy_light_decomposition/src/construct.rs
    title: crates/tree/heavy_light_decomposition/src/construct.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/tree/dynamic_rerooting_tree_dp/src/lib.rs
    title: crates/tree/dynamic_rerooting_tree_dp/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/dynamic_tree_dp/src/lib.rs
    title: crates/tree/dynamic_tree_dp/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy_light_decomposition/src/compress.rs
    title: crates/tree/heavy_light_decomposition/src/compress.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy_light_decomposition/src/construct.rs
    title: crates/tree/heavy_light_decomposition/src/construct.rs
  - icon: ':warning:'
    path: crates/tree/static_top_tree/src/lib.rs
    title: crates/tree/static_top_tree/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/jump_on_tree/src/main.rs
    title: verify/library_checker/tree/jump_on_tree/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/lca/src/main.rs
    title: verify/library_checker/tree/lca/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/vertex_add_path_sum/src/main.rs
    title: verify/library_checker/tree/vertex_add_path_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/vertex_add_subtree_sum/src/main.rs
    title: verify/library_checker/tree/vertex_add_subtree_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/vertex_set_path_composite/src/main.rs
    title: verify/library_checker/tree/vertex_set_path_composite/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yukicoder/tree/yuki0901_aux/src/main.rs
    title: verify/yukicoder/tree/yuki0901_aux/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://noya2ruler.github.io/noya2_Library/tree/heavy_light_decomposition.hpp
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// https://noya2ruler.github.io/noya2_Library/tree/heavy_light_decomposition.hpp\n\
    \npub mod compress;\npub mod construct;\n\nuse std::iter::FusedIterator;\n\nuse\
    \ csr_array::{CsrArray, CsrArrayBuilder};\n\npub struct HeavyLightDecomposition\
    \ {\n    n: usize,\n    root: usize,\n    down: Vec<i32>,\n    next: Vec<i32>,\n\
    \    sub: Vec<i32>,\n    tour: Vec<i32>,\n    edge_ord: Vec<usize>,\n}\n\nimpl\
    \ HeavyLightDecomposition {\n    pub fn is_empty(&self) -> bool {\n        self.n\
    \ == 0\n    }\n\n    pub fn len(&self) -> usize {\n        self.n\n    }\n\n \
    \   pub fn root(&self) -> usize {\n        self.root\n    }\n\n    /// \u5C5E\u3059\
    \u308B heavy path \u306E\u5148\u982D\n    pub fn head(&self, v: usize) -> usize\
    \ {\n        if self.next[v] < 0 {\n            v\n        } else {\n        \
    \    self.next[v] as usize\n        }\n    }\n\n    /// \u9802\u70B9 v \u306E\
    \ d \u500B\u89AA\n    pub fn la(&self, mut v: usize, d: usize) -> Option<usize>\
    \ {\n        let mut d = d as i32;\n        while v != !0 {\n            let u\
    \ = self.head(v);\n            if self.down[v] - d >= self.down[u] {\n       \
    \         v = self.tour[(self.down[v] - d) as usize] as usize;\n             \
    \   break;\n            }\n            d -= self.down[v] - self.down[u] + 1;\n\
    \            v = if u == self.root {\n                !0\n            } else {\n\
    \                (!self.next[u]) as usize\n            };\n        }\n       \
    \ if v == !0 {\n            None\n        } else {\n            Some(v)\n    \
    \    }\n    }\n\n    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {\n\
    \        let mut du = self.down[u];\n        let mut dv = self.down[v];\n    \
    \    if du > dv {\n            std::mem::swap(&mut u, &mut v);\n            std::mem::swap(&mut\
    \ du, &mut dv);\n        }\n        if dv < du + self.sub[u] {\n            return\
    \ u;\n        }\n        while du < dv {\n            v = !self.next[self.head(v)]\
    \ as usize;\n            dv = self.down[v];\n        }\n        v\n    }\n\n \
    \   /// u \u3092\u6839\u3068\u3057\u305F\u3068\u304D\u306E lca(v, w)\n    pub\
    \ fn meet(&self, u: usize, v: usize, w: usize) -> usize {\n        self.lca(u,\
    \ v) ^ self.lca(v, w) ^ self.lca(w, u)\n    }\n\n    pub fn dist(&self, mut u:\
    \ usize, mut v: usize) -> usize {\n        let mut dist = 0;\n        while self.head(u)\
    \ != self.head(v) {\n            if self.down[u] > self.down[v] {\n          \
    \      std::mem::swap(&mut u, &mut v);\n            }\n            dist += self.down[v]\
    \ - self.down[self.head(v)] + 1;\n            v = !self.next[self.head(v)] as\
    \ usize;\n        }\n        dist += (self.down[u] - self.down[v]).abs();\n  \
    \      dist as usize\n    }\n\n    pub fn jump(&self, mut s: usize, mut t: usize,\
    \ d: usize) -> Option<usize> {\n        let (ss, tt) = (s, t);\n        let (mut\
    \ dist_sl, mut dist_tl) = (0, 0);\n        while self.head(s) != self.head(t)\
    \ {\n            if self.down[s] > self.down[t] {\n                dist_sl +=\
    \ self.down[s] - self.down[self.head(s)] + 1;\n                s = !self.next[self.head(s)]\
    \ as usize;\n            } else {\n                dist_tl += self.down[t] - self.down[self.head(t)]\
    \ + 1;\n                t = !self.next[self.head(t)] as usize;\n            }\n\
    \        }\n        if self.down[s] > self.down[t] {\n            dist_sl += self.down[s]\
    \ - self.down[t];\n        } else {\n            dist_tl += self.down[t] - self.down[s];\n\
    \        }\n        let mut d = d as i32;\n        if d <= dist_sl {\n       \
    \     return Some(self.la(ss, d as usize).unwrap());\n        }\n        d -=\
    \ dist_sl;\n        if d <= dist_tl {\n            return Some(self.la(tt, (dist_tl\
    \ - d) as usize).unwrap());\n        }\n        None\n    }\n\n    pub fn parent(&self,\
    \ v: usize) -> Option<usize> {\n        if v == self.root {\n            None\n\
    \        } else if self.next[v] < 0 {\n            Some(!self.next[v] as usize)\n\
    \        } else {\n            Some(self.tour[(self.down[v] - 1) as usize] as\
    \ usize)\n        }\n    }\n\n    /// \\[0, n)\n    pub fn vertex_index(&self,\
    \ v: usize) -> usize {\n        self.down[v] as usize\n    }\n\n    /// \\[0,\
    \ n-1)\n    pub fn edge_index(&self, u: usize, v: usize) -> usize {\n        debug_assert!(self.parent(u)\
    \ == Some(v) || self.parent(v) == Some(u));\n        (self.down[u].max(self.down[v])\
    \ - 1) as usize\n    }\n\n    pub fn subtree_size(&self, v: usize) -> usize {\n\
    \        self.sub[v] as usize\n    }\n\n    // TODO: l..r \u3092\u8FD4\u3059\u306E\
    \u3068\u3069\u3063\u3061\u304C\u3044\u3044\uFF1F\n    pub fn subtree_range(&self,\
    \ v: usize) -> (usize, usize) {\n        let l = self.down[v] as usize;\n    \
    \    let r = (self.down[v] + self.sub[v]) as usize;\n        (l, r)\n    }\n\n\
    \    pub fn in_subtree(&self, r: usize, v: usize) -> bool {\n        let (l1,\
    \ r1) = self.subtree_range(r);\n        let (l2, r2) = self.subtree_range(v);\n\
    \        l1 <= l2 && r2 <= r1\n    }\n\n    pub fn dist_table(&self, mut s: usize)\
    \ -> Vec<usize> {\n        let mut dist = vec![!0; self.n];\n        dist[s] =\
    \ 0;\n        while let Some(p) = self.parent(s) {\n            dist[p] = dist[s]\
    \ + 1;\n            s = p;\n        }\n        for v in 0..self.n {\n        \
    \    if dist[v] == !0 {\n                dist[v] = dist[self.parent(v).unwrap()]\
    \ + 1;\n            }\n        }\n        dist\n    }\n\n    /// (dist, (u, v))\n\
    \    pub fn diameter(&self) -> (usize, (usize, usize)) {\n        let depth =\
    \ self.dist_table(self.root);\n        let (_, u) = depth.iter().zip(0..).max().unwrap();\n\
    \        let from_u = self.dist_table(u);\n        let (_, v) = from_u.iter().zip(0..).max().unwrap();\n\
    \        (from_u[v], (u, v))\n    }\n\n    /// \\[s, .., t\\]\n    pub fn path(&self,\
    \ mut s: usize, mut t: usize) -> Vec<usize> {\n        let d = self.dist(s, t);\n\
    \        let mut path = vec![!0; d + 1];\n        let (mut i, mut j) = (0, d);\n\
    \        while s != t {\n            if self.down[s] > self.down[t] {\n      \
    \          path[i] = s;\n                i += 1;\n                s = self.parent(s).unwrap();\n\
    \            } else {\n                path[j] = t;\n                j -= 1;\n\
    \                t = self.parent(t).unwrap();\n            }\n        }\n    \
    \    path[i] = s;\n        path\n    }\n\n    /// f: (l, r, reverse)\n    pub\
    \ fn path_query(\n        &self,\n        mut s: usize,\n        mut t: usize,\n\
    \        vertex_query: bool,\n        mut f: impl FnMut(usize, usize, bool),\n\
    \    ) {\n        let mut f = |l, r, reverse| {\n            if vertex_query {\n\
    \                f(l, r, reverse);\n            } else {\n                f(l\
    \ - 1, r - 1, reverse);\n            }\n        };\n\n        let mut down_query\
    \ = vec![];\n        while self.head(s) != self.head(t) {\n            if self.down[s]\
    \ < self.down[t] {\n                let l = self.down[self.head(t)];\n       \
    \         let r = self.down[t] + 1;\n                down_query.push((l, r));\n\
    \                t = !self.next[self.head(t)] as _;\n            } else {\n  \
    \              let l = self.down[self.head(s)];\n                let r = self.down[s]\
    \ + 1;\n                f(l as _, r as _, true);\n                s = !self.next[self.head(s)]\
    \ as _;\n            }\n        }\n\n        if vertex_query {\n            if\
    \ self.down[s] < self.down[t] {\n                let l = self.down[s];\n     \
    \           let r = self.down[t] + 1;\n                f(l as _, r as _, false);\n\
    \            } else {\n                let l = self.down[t];\n               \
    \ let r = self.down[s] + 1;\n                f(l as _, r as _, true);\n      \
    \      }\n        } else if self.down[s] < self.down[t] {\n            let l =\
    \ self.down[s] + 1;\n            let r = self.down[t] + 1;\n            f(l as\
    \ _, r as _, false);\n        } else if self.down[s] > self.down[t] {\n      \
    \      let l = self.down[t] + 1;\n            let r = self.down[s] + 1;\n    \
    \        f(l as _, r as _, true);\n        }\n\n        for &(l, r) in down_query.iter().rev()\
    \ {\n            f(l as _, r as _, false);\n        }\n    }\n\n    pub fn euler_tour(\n\
    \        &self,\n    ) -> impl Iterator<Item = usize> + FusedIterator + ExactSizeIterator\
    \ + DoubleEndedIterator + '_\n    {\n        self.tour.iter().map(|&v| v as usize)\n\
    \    }\n\n    pub fn edges_order(\n        &self,\n    ) -> impl Iterator<Item\
    \ = usize> + FusedIterator + ExactSizeIterator + DoubleEndedIterator + '_\n  \
    \  {\n        self.edge_ord.iter().copied()\n    }\n\n    /// \u5404\u9802\u70B9\
    \u306E\u5B50\u306E\u30EA\u30B9\u30C8  \n    /// heavy child \u304C\u5148\u982D\
    \n    pub fn children(&self) -> CsrArray<usize> {\n        let mut csr = CsrArrayBuilder::new(self.n);\n\
    \        for v in self.euler_tour().skip(1) {\n            csr.push(self.parent(v).unwrap(),\
    \ v);\n        }\n        csr.build()\n    }\n}\n\n#[doc(hidden)]\npub trait Edge\
    \ {\n    fn endpoints(&self) -> (usize, usize);\n}\n\n#[doc(hidden)]\nimpl Edge\
    \ for (usize, usize) {\n    fn endpoints(&self) -> (usize, usize) {\n        *self\n\
    \    }\n}\n\n#[doc(hidden)]\nimpl<T> Edge for (usize, usize, T) {\n    fn endpoints(&self)\
    \ -> (usize, usize) {\n        (self.0, self.1)\n    }\n}\n"
  dependsOn:
  - crates/data_structure/csr_array/src/builder.rs
  - crates/data_structure/csr_array/src/csr_array.rs
  - crates/data_structure/csr_array/src/lib.rs
  - crates/tree/heavy_light_decomposition/src/compress.rs
  - crates/tree/heavy_light_decomposition/src/construct.rs
  isVerificationFile: false
  path: crates/tree/heavy_light_decomposition/src/lib.rs
  requiredBy:
  - crates/tree/dynamic_rerooting_tree_dp/src/lib.rs
  - crates/tree/dynamic_tree_dp/src/lib.rs
  - crates/tree/heavy_light_decomposition/src/compress.rs
  - crates/tree/heavy_light_decomposition/src/construct.rs
  - crates/tree/static_top_tree/src/lib.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/tree/lca/src/main.rs
  - verify/library_checker/tree/vertex_add_path_sum/src/main.rs
  - verify/library_checker/tree/vertex_set_path_composite/src/main.rs
  - verify/library_checker/tree/vertex_add_subtree_sum/src/main.rs
  - verify/library_checker/tree/jump_on_tree/src/main.rs
  - verify/yukicoder/tree/yuki0901_aux/src/main.rs
documentation_of: crates/tree/heavy_light_decomposition/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/heavy_light_decomposition/src/lib.rs
- /library/crates/tree/heavy_light_decomposition/src/lib.rs.html
title: crates/tree/heavy_light_decomposition/src/lib.rs
---
