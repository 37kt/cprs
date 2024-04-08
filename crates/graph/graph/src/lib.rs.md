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
  - icon: ':warning:'
    path: crates/graph/bellman-ford/src/lib.rs
    title: crates/graph/bellman-ford/src/lib.rs
  - icon: ':warning:'
    path: crates/graph/complement-graph-bfs/src/lib.rs
    title: crates/graph/complement-graph-bfs/src/lib.rs
  - icon: ':warning:'
    path: crates/graph/compressed-tree/src/lib.rs
    title: crates/graph/compressed-tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/dijkstra/src/lib.rs
    title: crates/graph/dijkstra/src/lib.rs
  - icon: ':warning:'
    path: crates/graph/low-link/src/lib.rs
    title: crates/graph/low-link/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/range-edge-graph/src/lib.rs
    title: crates/graph/range-edge-graph/src/lib.rs
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
    path: verify/shortest_path/src/main.rs
    title: verify/shortest_path/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/tree_path_composite_sum/src/main.rs
    title: verify/tree_path_composite_sum/src/main.rs
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
    path: verify/yuki1014/src/main.rs
    title: verify/yuki1014/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ops::Index;\n\n#[derive(Clone)]\npub struct Graph<V, E>\nwhere\n\
    \    V: Clone,\n    E: Clone,\n{\n    vertices: Vec<V>,\n    edges: Vec<(usize,\
    \ E)>,\n    pos: Vec<usize>,\n}\n\npub const GRID_NEIGHBOURS_4: &[(usize, usize)]\
    \ = &[(!0, 0), (0, !0), (1, 0), (0, 1)];\npub const GRID_NEIGHBOURS_8: &[(usize,\
    \ usize)] = &[\n    (!0, 0),\n    (0, !0),\n    (1, 0),\n    (0, 1),\n    (!0,\
    \ !0),\n    (!0, 1),\n    (1, !0),\n    (1, 1),\n];\n\nimpl<V, E> Graph<V, E>\n\
    where\n    V: Clone,\n    E: Clone,\n{\n    pub fn from_vertices_and_directed_edges(vertices:\
    \ &[V], edges: &[(usize, usize, E)]) -> Self {\n        if edges.is_empty() {\n\
    \            return Self {\n                vertices: vertices.to_vec(),\n   \
    \             edges: vec![],\n                pos: vec![0; vertices.len() + 1],\n\
    \            };\n        }\n\n        let n = vertices.len();\n        let mut\
    \ pos = vec![0; n + 1];\n        for &(u, _, _) in edges {\n            pos[u]\
    \ += 1;\n        }\n        for i in 1..=n {\n            pos[i] += pos[i - 1];\n\
    \        }\n        let mut ord = vec![0; edges.len()];\n        for i in (0..edges.len()).rev()\
    \ {\n            let u = edges[i].0;\n            pos[u] -= 1;\n            ord[pos[u]]\
    \ = i;\n        }\n\n        Self {\n            vertices: vertices.to_vec(),\n\
    \            edges: ord\n                .into_iter()\n                .map(|i|\
    \ (edges[i].1, edges[i].2.clone()))\n                .collect(),\n           \
    \ pos,\n        }\n    }\n\n    pub fn from_vertices_and_undirected_edges(vertices:\
    \ &[V], edges: &[(usize, usize, E)]) -> Self {\n        if edges.is_empty() {\n\
    \            return Self {\n                vertices: vertices.to_vec(),\n   \
    \             edges: vec![],\n                pos: vec![0; vertices.len() + 1],\n\
    \            };\n        }\n\n        let n = vertices.len();\n        let mut\
    \ pos = vec![0; n + 1];\n        for &(u, v, _) in edges {\n            pos[u]\
    \ += 1;\n            pos[v] += 1;\n        }\n        for i in 1..=n {\n     \
    \       pos[i] += pos[i - 1];\n        }\n        let mut ord = vec![0; edges.len()\
    \ * 2];\n        for i in (0..edges.len() * 2).rev() {\n            if i & 1 ==\
    \ 0 {\n                let u = edges[i >> 1].0;\n                pos[u] -= 1;\n\
    \                ord[pos[u]] = i;\n            } else {\n                let v\
    \ = edges[i >> 1].1;\n                pos[v] -= 1;\n                ord[pos[v]]\
    \ = i;\n            }\n        }\n\n        Self {\n            vertices: vertices.to_vec(),\n\
    \            edges: ord\n                .into_iter()\n                .map(|i|\
    \ {\n                    (\n                        if i & 1 == 0 {\n        \
    \                    edges[i >> 1].1\n                        } else {\n     \
    \                       edges[i >> 1].0\n                        },\n        \
    \                edges[i >> 1].2.clone(),\n                    )\n           \
    \     })\n                .collect(),\n            pos,\n        }\n    }\n\n\
    \    pub fn len(&self) -> usize {\n        self.vertices.len()\n    }\n\n    pub\
    \ fn vertex(&self, v: usize) -> &V {\n        &self.vertices[v]\n    }\n\n   \
    \ pub fn edges(&self, v: usize) -> &[(usize, E)] {\n        let l = self.pos[v];\n\
    \        let r = self.pos[v + 1];\n        &self.edges[l..r]\n    }\n\n    ///\
    \ (i, j) -> i * w + j\n    pub fn from_grid(\n        grid: &Vec<Vec<V>>,\n  \
    \      neighbours: &[(usize, usize)],\n        cost: impl Fn(&V, &V) -> Option<E>,\n\
    \    ) -> Self {\n        let h = grid.len();\n        let w = grid[0].len();\n\
    \        let mut edges = vec![];\n        for i in 0..h {\n            for j in\
    \ 0..w {\n                for &(di, dj) in neighbours {\n                    let\
    \ ni = i.wrapping_add(di);\n                    let nj = j.wrapping_add(dj);\n\
    \                    if ni >= h || nj >= w {\n                        continue;\n\
    \                    }\n                    if let Some(c) = cost(&grid[i][j],\
    \ &grid[ni][nj]) {\n                        edges.push((i * w + j, ni * w + nj,\
    \ c));\n                    }\n                }\n            }\n        }\n \
    \       Self::from_vertices_and_directed_edges(\n            &grid.into_iter().flatten().cloned().collect::<Vec<_>>(),\n\
    \            &edges,\n        )\n    }\n}\n\nimpl<V, E> Index<usize> for Graph<V,\
    \ E>\nwhere\n    V: Clone,\n    E: Clone,\n{\n    type Output = [(usize, E)];\n\
    \n    fn index(&self, v: usize) -> &[(usize, E)] {\n        self.edges(v)\n  \
    \  }\n}\n\nimpl<E> Graph<(), E>\nwhere\n    E: Clone,\n{\n    pub fn from_directed_edges(n:\
    \ usize, edges: &[(usize, usize, E)]) -> Self {\n        Self::from_vertices_and_directed_edges(&vec![();\
    \ n], edges)\n    }\n\n    pub fn from_undirected_edges(n: usize, edges: &[(usize,\
    \ usize, E)]) -> Self {\n        Self::from_vertices_and_undirected_edges(&vec![();\
    \ n], edges)\n    }\n}\n\nimpl<V> Graph<V, ()>\nwhere\n    V: Clone,\n{\n    pub\
    \ fn from_vertices_and_unweighted_directed_edges(\n        vertices: &[V],\n \
    \       edges: &[(usize, usize)],\n    ) -> Self {\n        Self::from_vertices_and_directed_edges(\n\
    \            vertices,\n            &edges.iter().map(|&(u, v)| (u, v, ())).collect::<Vec<_>>(),\n\
    \        )\n    }\n\n    pub fn from_vertices_and_unweighted_undirected_edges(\n\
    \        vertices: &[V],\n        edges: &[(usize, usize)],\n    ) -> Self {\n\
    \        Self::from_vertices_and_undirected_edges(\n            vertices,\n  \
    \          &edges.iter().map(|&(u, v)| (u, v, ())).collect::<Vec<_>>(),\n    \
    \    )\n    }\n}\n\nimpl Graph<(), ()> {\n    pub fn from_unweighted_directed_edges(n:\
    \ usize, edges: &[(usize, usize)]) -> Self {\n        Self::from_directed_edges(\n\
    \            n,\n            &edges.iter().map(|&(u, v)| (u, v, ())).collect::<Vec<_>>(),\n\
    \        )\n    }\n\n    pub fn from_unweighted_undirected_edges(n: usize, edges:\
    \ &[(usize, usize)]) -> Self {\n        Self::from_undirected_edges(\n       \
    \     n,\n            &edges.iter().map(|&(u, v)| (u, v, ())).collect::<Vec<_>>(),\n\
    \        )\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/graph/graph/src/lib.rs
  requiredBy:
  - crates/tree/re-rooting-dp/src/lib.rs
  - crates/tree/centroid-decomposition/src/lib.rs
  - crates/math/two-satisfiability/src/lib.rs
  - crates/graph/compressed-tree/src/lib.rs
  - crates/graph/low-link/src/lib.rs
  - crates/graph/complement-graph-bfs/src/lib.rs
  - crates/graph/strongly-connected-components/src/lib.rs
  - crates/graph/dijkstra/src/lib.rs
  - crates/graph/range-edge-graph/src/lib.rs
  - crates/graph/bellman-ford/src/lib.rs
  - crates/data-structure/tree-query/src/lib.rs
  - crates/data-structure/heavy-light-decomposition/src/lib.rs
  timestamp: '2024-04-07 08:56:09+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/lca/src/main.rs
  - verify/vertex_set_path_composite/src/main.rs
  - verify/tree_path_composite_sum/src/main.rs
  - verify/jump_on_tree/src/main.rs
  - verify/scc/src/main.rs
  - verify/yuki1014/src/main.rs
  - verify/shortest_path/src/main.rs
  - verify/vertex_add_path_sum/src/main.rs
  - verify/vertex_add_subtree_sum/src/main.rs
documentation_of: crates/graph/graph/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/graph/src/lib.rs
- /library/crates/graph/graph/src/lib.rs.html
title: crates/graph/graph/src/lib.rs
---
