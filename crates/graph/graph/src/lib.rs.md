---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/heavy-light-decomposition/src/lib.rs
    title: crates/data-structure/heavy-light-decomposition/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/tree-query/src/lib.rs
    title: crates/data-structure/tree-query/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/strongly-connected-components/src/lib.rs
    title: crates/graph/strongly-connected-components/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/math/two-satisfiability/src/lib.rs
    title: crates/math/two-satisfiability/src/lib.rs
  - icon: ':warning:'
    path: crates/tree/centroid-decomposition/src/lib.rs
    title: crates/tree/centroid-decomposition/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/re-rooting-dp/src/lib.rs
    title: crates/tree/re-rooting-dp/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/jump_on_tree/src/main.rs
    title: verify/jump_on_tree/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/lca/src/main.rs
    title: verify/lca/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/scc/src/main.rs
    title: verify/scc/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/vertex_add_path_sum/src/main.rs
    title: verify/vertex_add_path_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/vertex_add_subtree_sum/src/main.rs
    title: verify/vertex_add_subtree_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/vertex_set_path_composite/src/main.rs
    title: verify/vertex_set_path_composite/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yuki1333/src/main.rs
    title: verify/yuki1333/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "#[derive(Clone)]\npub struct Graph<V, E>\nwhere\n    V: Clone,\n    E: Clone,\n\
    {\n    vertices: Vec<V>,\n    head: Vec<usize>,\n    next: Vec<usize>,\n    edges:\
    \ Vec<(usize, E)>,\n}\n\npub const GRID_NEIGHBOURS_4: &[(usize, usize)] = &[(!0,\
    \ 0), (0, !0), (1, 0), (0, 1)];\npub const GRID_NEIGHBOURS_8: &[(usize, usize)]\
    \ = &[\n    (!0, 0),\n    (0, !0),\n    (1, 0),\n    (0, 1),\n    (!0, !0),\n\
    \    (!0, 1),\n    (1, !0),\n    (1, 1),\n];\n\nimpl<V, E> Graph<V, E>\nwhere\n\
    \    V: Clone + Default,\n    E: Clone,\n{\n    pub fn new(n: usize) -> Self {\n\
    \        Self {\n            vertices: vec![Default::default(); n],\n        \
    \    head: vec![!0; n],\n            next: vec![],\n            edges: vec![],\n\
    \        }\n    }\n}\n\nimpl<V, E> From<Vec<V>> for Graph<V, E>\nwhere\n    V:\
    \ Clone,\n    E: Clone,\n{\n    fn from(vertices: Vec<V>) -> Self {\n        Self\
    \ {\n            head: vec![!0; vertices.len()],\n            vertices,\n    \
    \        next: vec![],\n            edges: vec![],\n        }\n    }\n}\n\nimpl<V,\
    \ E> Graph<V, E>\nwhere\n    V: Clone,\n    E: Clone,\n{\n    pub fn size(&self)\
    \ -> usize {\n        self.vertices.len()\n    }\n\n    pub fn set_vertex(&mut\
    \ self, v: usize, w: V) {\n        self.vertices[v] = w;\n    }\n\n    pub fn\
    \ add_vertex(&mut self, w: V) -> usize {\n        self.vertices.push(w);\n   \
    \     self.head.push(!0);\n        self.vertices.len() - 1\n    }\n\n    pub fn\
    \ add_edge(&mut self, u: usize, v: usize, w: E) {\n        self.next.push(self.head[u]);\n\
    \        self.head[u] = self.edges.len();\n        self.edges.push((v, w));\n\
    \    }\n\n    pub fn add_undirected_edge(&mut self, u: usize, v: usize, w: E)\
    \ {\n        self.add_edge(u, v, w.clone());\n        self.add_edge(v, u, w);\n\
    \    }\n\n    pub fn vertices(&self) -> &[V] {\n        &self.vertices\n    }\n\
    \n    pub fn out_edges(&self, v: usize) -> impl Iterator<Item = &(usize, E)> {\n\
    \        let mut e = self.head[v];\n        std::iter::from_fn(move || {\n   \
    \         if e != !0 {\n                let res = &self.edges[e];\n          \
    \      e = self.next[e];\n                Some(res)\n            } else {\n  \
    \              None\n            }\n        })\n    }\n\n    pub fn from_grid(\n\
    \        grid: &Vec<Vec<V>>,\n        neighbours: &[(usize, usize)],\n       \
    \ cost: impl Fn(&V, &V) -> Option<E>,\n    ) -> Self {\n        let h = grid.len();\n\
    \        let w = grid[0].len();\n        let mut g = Self::from(grid.into_iter().flatten().cloned().collect::<Vec<_>>());\n\
    \        for i in 0..h {\n            for j in 0..w {\n                for &(di,\
    \ dj) in neighbours {\n                    let ni = i.wrapping_add(di);\n    \
    \                let nj = j.wrapping_add(dj);\n                    if ni >= h\
    \ || nj >= w {\n                        continue;\n                    }\n   \
    \                 if let Some(c) = cost(&grid[i][j], &grid[ni][nj]) {\n      \
    \                  g.add_edge(i * w + j, ni * w + nj, c);\n                  \
    \  }\n                }\n            }\n        }\n        g\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/graph/graph/src/lib.rs
  requiredBy:
  - crates/tree/centroid-decomposition/src/lib.rs
  - crates/tree/re-rooting-dp/src/lib.rs
  - crates/math/two-satisfiability/src/lib.rs
  - crates/data-structure/tree-query/src/lib.rs
  - crates/data-structure/heavy-light-decomposition/src/lib.rs
  - crates/graph/strongly-connected-components/src/lib.rs
  timestamp: '2023-05-17 16:30:46+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/lca/src/main.rs
  - verify/scc/src/main.rs
  - verify/vertex_add_path_sum/src/main.rs
  - verify/vertex_add_subtree_sum/src/main.rs
  - verify/jump_on_tree/src/main.rs
  - verify/yuki1333/src/main.rs
  - verify/vertex_set_path_composite/src/main.rs
documentation_of: crates/graph/graph/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/graph/src/lib.rs
- /library/crates/graph/graph/src/lib.rs.html
title: crates/graph/graph/src/lib.rs
---
