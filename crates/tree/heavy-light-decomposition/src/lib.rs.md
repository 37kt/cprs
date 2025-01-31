---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/data-structure/csr-array/src/lib.rs
    title: crates/data-structure/csr-array/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/data-structure/range-contour-query/src/lib.rs
    title: crates/data-structure/range-contour-query/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/dynamic-rerooting-tree-dp/src/lib.rs
    title: crates/tree/dynamic-rerooting-tree-dp/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/dynamic-tree-dp/src/lib.rs
    title: crates/tree/dynamic-tree-dp/src/lib.rs
  - icon: ':warning:'
    path: crates/tree/static-top-tree/src/lib.rs
    title: crates/tree/static-top-tree/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/jump_on_tree/src/main.rs
    title: verify/jump_on_tree/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/lca/src/main.rs
    title: verify/lca/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/vertex_add_path_sum/src/main.rs
    title: verify/vertex_add_path_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/vertex_add_subtree_sum/src/main.rs
    title: verify/vertex_add_subtree_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/vertex_set_path_composite/src/main.rs
    title: verify/vertex_set_path_composite/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://noya2ruler.github.io/noya2_Library/tree/heavy_light_decomposition.hpp
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// https://noya2ruler.github.io/noya2_Library/tree/heavy_light_decomposition.hpp\n\
    \nuse csr_array::CSRArray;\nuse graph::Graph;\n\n#[derive(Clone)]\npub struct\
    \ HeavyLightDecomposition {\n    n: usize,\n    root: usize,\n    down: Vec<i32>,\n\
    \    next: Vec<i32>,\n    sub: Vec<i32>,\n    tour: Vec<i32>,\n}\n\nimpl HeavyLightDecomposition\
    \ {\n    pub fn new<V: Clone, E: Clone>(g: &Graph<V, E, false>, root: usize) ->\
    \ Self {\n        let n = g.len();\n        let mut down = vec![0; n];\n     \
    \   let mut next = vec![0; n];\n        let mut sub = vec![1; n];\n        let\
    \ mut tour = vec![-1; n];\n\n        for u in 0..n {\n            for &(v, _)\
    \ in &g[u] {\n                if u > v {\n                    continue;\n    \
    \            }\n                down[u] += 1;\n                down[v] += 1;\n\
    \                next[u] ^= v as i32;\n                next[v] ^= u as i32;\n\
    \            }\n        }\n\n        let mut back = 0;\n        for u in 0..n\
    \ {\n            if u != root && down[u] == 1 {\n                tour[back] =\
    \ u as i32;\n                back += 1;\n            }\n        }\n        for\
    \ front in 0..n - 1 {\n            let u = tour[front] as usize;\n           \
    \ down[u] = -1;\n            let v = next[u] as usize;\n            next[v] ^=\
    \ u as i32;\n            down[v] -= 1;\n            if down[v] == 1 && v != root\
    \ {\n                tour[back] = v as i32;\n                back += 1;\n    \
    \        }\n        }\n        tour.pop();\n\n        for &u in &tour {\n    \
    \        let u = u as usize;\n            let v = next[u] as usize;\n        \
    \    sub[v] += sub[u];\n            down[v] = down[v].max(sub[u]);\n        }\n\
    \        for &u in &tour {\n            let u = u as usize;\n            let v\
    \ = next[u] as usize;\n            if down[v] == sub[u] {\n                sub[u]\
    \ = !sub[u];\n                down[v] = !down[v];\n            }\n        }\n\n\
    \        sub[root] = !down[root] + 1;\n        down[root] = 0;\n        next[root]\
    \ = -1;\n        for &u in tour.iter().rev() {\n            let u = u as usize;\n\
    \            let v = next[u] as usize;\n            let nsub = !down[u] + 1;\n\
    \            if sub[u] < 0 {\n                down[u] = down[v] + 1;\n       \
    \         next[u] = if next[v] < 0 { v as i32 } else { next[v] };\n          \
    \  } else {\n                down[u] = down[v] + sub[v];\n                sub[v]\
    \ += sub[u];\n                next[u] = !(v as i32);\n            }\n        \
    \    sub[u] = nsub;\n        }\n\n        tour.push(0);\n        for u in 0..n\
    \ {\n            tour[down[u] as usize] = u as i32;\n        }\n\n        Self\
    \ {\n            n,\n            root,\n            down,\n            next,\n\
    \            sub,\n            tour,\n        }\n    }\n\n    /// par[i] < i,\
    \ par[0] == !0\n    pub fn from_parents(par: &[usize]) -> Self {\n        let\
    \ n = par.len();\n        let mut down = vec![-1; n];\n        let mut next: Vec<_>\
    \ = par.iter().map(|&u| u as i32).collect();\n        let mut sub = vec![1; n];\n\
    \        let mut tour = vec![-1; n];\n\n        for u in (1..n).rev() {\n    \
    \        let v = next[u] as usize;\n            sub[v] += sub[u];\n          \
    \  down[v] = down[v].max(sub[u]);\n        }\n        for u in (1..n).rev() {\n\
    \            let v = next[u] as usize;\n            if down[v] == sub[u] {\n \
    \               sub[u] = !sub[u];\n                down[v] = !down[v];\n     \
    \       }\n        }\n\n        sub[0] = !down[0] + 1;\n        down[0] = 0;\n\
    \        for u in 1..n {\n            let v = next[u] as usize;\n            let\
    \ nsub = !down[u] + 1;\n            if sub[u] < 0 {\n                down[u] =\
    \ down[v] + 1;\n                next[u] = if next[v] < 0 { v as i32 } else { next[v]\
    \ };\n            } else {\n                down[u] = down[v] + sub[v];\n    \
    \            sub[v] += sub[u];\n                next[u] = !(v as i32);\n     \
    \       }\n            sub[u] = nsub;\n        }\n\n        for u in 0..n {\n\
    \            tour[down[u] as usize] = u as i32;\n        }\n\n        Self {\n\
    \            n,\n            root: 0,\n            down,\n            next,\n\
    \            sub,\n            tour,\n        }\n    }\n\n    pub fn root(&self)\
    \ -> usize {\n        self.root\n    }\n\n    pub fn len(&self) -> usize {\n \
    \       self.n\n    }\n\n    pub fn leader(&self, v: usize) -> usize {\n     \
    \   if self.next[v] < 0 {\n            v\n        } else {\n            self.next[v]\
    \ as usize\n        }\n    }\n\n    pub fn la(&self, mut v: usize, mut d: usize)\
    \ -> usize {\n        while v != !0 {\n            let u = self.leader(v);\n \
    \           if self.down[v] as usize >= self.down[u] as usize + d {\n        \
    \        v = self.tour[self.down[v] as usize - d] as usize;\n                break;\n\
    \            }\n            d -= 1 + (self.down[v] - self.down[u]) as usize;\n\
    \            v = if u == self.root {\n                !0\n            } else {\n\
    \                (!self.next[u]) as usize\n            };\n        }\n       \
    \ v\n    }\n\n    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {\n \
    \       let mut du = self.down[u] as usize;\n        let mut dv = self.down[v]\
    \ as usize;\n        if du > dv {\n            std::mem::swap(&mut du, &mut dv);\n\
    \            std::mem::swap(&mut u, &mut v);\n        }\n        if dv < du +\
    \ self.sub[u] as usize {\n            return u;\n        }\n        while du <\
    \ dv {\n            v = (!self.next[self.leader(v)]) as usize;\n            dv\
    \ = self.down[v] as usize;\n        }\n        v\n    }\n\n    pub fn dist(&self,\
    \ mut u: usize, mut v: usize) -> usize {\n        let mut d = 0;\n        while\
    \ self.leader(u) != self.leader(v) {\n            if self.down[u] > self.down[v]\
    \ {\n                std::mem::swap(&mut u, &mut v);\n            }\n        \
    \    d += 1 + self.down[v] - self.down[self.leader(v)];\n            v = (!self.next[self.leader(v)])\
    \ as usize;\n        }\n        (d + (self.down[u] - self.down[v]).abs()) as usize\n\
    \    }\n\n    pub fn jump(&self, u: usize, v: usize, d: usize) -> usize {\n  \
    \      let mut d = d as i32;\n        let mut uu = u;\n        let mut vv = v;\n\
    \        let mut dist_u_lca = 0;\n        let mut dist_v_lca = 0;\n        while\
    \ self.leader(uu) != self.leader(vv) {\n            if self.down[uu] > self.down[vv]\
    \ {\n                dist_u_lca += 1 + self.down[uu] - self.down[self.leader(uu)];\n\
    \                uu = (!self.next[self.leader(uu)]) as usize;\n            } else\
    \ {\n                dist_v_lca += 1 + self.down[vv] - self.down[self.leader(vv)];\n\
    \                vv = (!self.next[self.leader(vv)]) as usize;\n            }\n\
    \        }\n        if self.down[uu] > self.down[vv] {\n            dist_u_lca\
    \ += self.down[uu] - self.down[vv];\n        } else {\n            dist_v_lca\
    \ += self.down[vv] - self.down[uu];\n        }\n        if d <= dist_u_lca {\n\
    \            return self.la(u, d as usize);\n        }\n        d -= dist_u_lca;\n\
    \        if d <= dist_v_lca {\n            self.la(v, (dist_v_lca - d) as usize)\n\
    \        } else {\n            !0\n        }\n    }\n\n    pub fn parent(&self,\
    \ v: usize) -> usize {\n        if v == self.root {\n            !0\n        }\
    \ else if self.next[v] < 0 {\n            (!self.next[v]) as usize\n        }\
    \ else {\n            self.tour[self.down[v] as usize - 1] as usize\n        }\n\
    \    }\n\n    pub fn vertex_index(&self, v: usize) -> usize {\n        self.down[v]\
    \ as usize\n    }\n\n    pub fn edge_index(&self, u: usize, v: usize) -> usize\
    \ {\n        self.down[u].max(self.down[v]) as usize - 1\n    }\n\n    pub fn\
    \ subtree_range(&self, v: usize) -> (usize, usize) {\n        (self.down[v] as\
    \ usize, (self.down[v] + self.sub[v]) as usize)\n    }\n\n    pub fn is_in_subtree(&self,\
    \ r: usize, v: usize) -> bool {\n        let (rl, rr) = self.subtree_range(r);\n\
    \        let (vl, vr) = self.subtree_range(v);\n        rl <= vl && vr <= rr\n\
    \    }\n\n    pub fn dist_table(&self, mut s: usize) -> Vec<usize> {\n       \
    \ let mut dist = vec![!0; self.n];\n        dist[s] = 0;\n        while s != self.root\
    \ {\n            dist[self.parent(s)] = dist[s] + 1;\n            s = self.parent(s);\n\
    \        }\n        for &v in &self.tour {\n            let v = v as usize;\n\
    \            if dist[v] == !0 {\n                dist[v] = dist[self.parent(v)]\
    \ + 1;\n            }\n        }\n        dist\n    }\n\n    pub fn diameter(&self)\
    \ -> (usize, usize, usize) {\n        let depth = self.dist_table(self.root);\n\
    \        let u = depth.iter().enumerate().max_by_key(|&(_, &d)| d).unwrap().0;\n\
    \        let dist_u = self.dist_table(u);\n        let v = dist_u\n          \
    \  .iter()\n            .enumerate()\n            .max_by_key(|&(_, &d)| d)\n\
    \            .unwrap()\n            .0;\n        (dist_u[v], u, v)\n    }\n\n\
    \    pub fn path(&self, mut u: usize, mut v: usize) -> Vec<usize> {\n        let\
    \ d = self.dist(u, v);\n        let mut path = vec![0; d + 1];\n        let mut\
    \ front = 0;\n        let mut back = d;\n        while u != v {\n            if\
    \ self.down[u] > self.down[v] {\n                path[front] = u;\n          \
    \      front += 1;\n                u = self.parent(u);\n            } else {\n\
    \                path[back] = v;\n                back -= 1;\n               \
    \ v = self.parent(v);\n            }\n        }\n        path[front] = u;\n  \
    \      path\n    }\n\n    /// f: (l, r, reverse)\n    pub fn path_query(\n   \
    \     &self,\n        mut u: usize,\n        mut v: usize,\n        mut f: impl\
    \ FnMut(usize, usize, bool),\n        vertex_query: bool,\n    ) {\n        let\
    \ mut up_path = vec![];\n        let mut down_path = vec![];\n        while self.leader(u)\
    \ != self.leader(v) {\n            if self.down[u] < self.down[v] {\n        \
    \        let l = self.down[self.leader(v)] as usize;\n                let r =\
    \ self.down[v] as usize + 1;\n                assert!(l < r);\n              \
    \  down_path.push((l, r));\n                v = (!self.next[self.leader(v)]) as\
    \ usize;\n            } else {\n                let l = self.down[self.leader(u)]\
    \ as usize;\n                let r = self.down[u] as usize + 1;\n            \
    \    assert!(l < r);\n                up_path.push((l, r));\n                u\
    \ = (!self.next[self.leader(u)]) as usize;\n            }\n        }\n       \
    \ if vertex_query {\n            let du = self.down[u] as usize;\n           \
    \ let dv = self.down[v] as usize;\n            if du < dv {\n                down_path.push((du,\
    \ dv + 1));\n            } else {\n                up_path.push((dv, du + 1));\n\
    \            }\n        } else {\n            let du = self.down[u] as usize;\n\
    \            let dv = self.down[v] as usize;\n            if du < dv {\n     \
    \           down_path.push((du + 1, dv + 1));\n            } else {\n        \
    \        up_path.push((dv + 1, du + 1));\n            }\n        }\n\n       \
    \ if !vertex_query {\n            for (l, r) in up_path.iter_mut() {\n       \
    \         *l -= 1;\n                *r -= 1;\n            }\n            for (l,\
    \ r) in down_path.iter_mut() {\n                *l -= 1;\n                *r -=\
    \ 1;\n            }\n        }\n        for &(l, r) in &up_path {\n          \
    \  f(l, r, true);\n        }\n        for &(l, r) in down_path.iter().rev() {\n\
    \            f(l, r, false);\n        }\n    }\n\n    pub fn euler_tour(&self)\
    \ -> Vec<usize> {\n        self.tour.iter().map(|&u| u as usize).collect()\n \
    \   }\n\n    pub fn heavy_child(&self, v: usize) -> usize {\n        if (self.down[v]\
    \ + 1) as usize >= self.n {\n            return !0;\n        }\n        let u\
    \ = self.tour[self.down[v] as usize + 1] as usize;\n        if self.parent(u)\
    \ == v {\n            u\n        } else {\n            !0\n        }\n    }\n\n\
    \    pub fn parents(&self) -> Vec<usize> {\n        (0..self.n).map(|i| self.parent(i)).collect()\n\
    \    }\n\n    pub fn children(&self) -> CSRArray<usize> {\n        let children\
    \ = self\n            .tour\n            .iter()\n            .skip(1)\n     \
    \       .map(|&v| (self.parent(v as usize), v as usize))\n            .collect::<Vec<_>>();\n\
    \        CSRArray::new(self.n, &children)\n    }\n}\n"
  dependsOn:
  - crates/data-structure/csr-array/src/lib.rs
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/tree/heavy-light-decomposition/src/lib.rs
  requiredBy:
  - crates/data-structure/range-contour-query/src/lib.rs
  - crates/tree/static-top-tree/src/lib.rs
  - crates/tree/dynamic-tree-dp/src/lib.rs
  - crates/tree/dynamic-rerooting-tree-dp/src/lib.rs
  timestamp: '2025-01-15 06:25:46+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/vertex_add_path_sum/src/main.rs
  - verify/lca/src/main.rs
  - verify/jump_on_tree/src/main.rs
  - verify/vertex_add_subtree_sum/src/main.rs
  - verify/vertex_set_path_composite/src/main.rs
documentation_of: crates/tree/heavy-light-decomposition/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/heavy-light-decomposition/src/lib.rs
- /library/crates/tree/heavy-light-decomposition/src/lib.rs.html
title: crates/tree/heavy-light-decomposition/src/lib.rs
---
