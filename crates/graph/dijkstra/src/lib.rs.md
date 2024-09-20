---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/shortest_path/src/main.rs
    title: verify/shortest_path/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{cmp::Reverse, collections::BinaryHeap, ops::Add};\n\nuse graph::Graph;\n\
    \npub struct DijkstraResult<T>\nwhere\n    T: Clone + Ord + Add<Output = T> +\
    \ Default,\n{\n    pub dist: Vec<T>,\n    pub prev: Vec<usize>,\n}\n\npub fn dijkstra<V,\
    \ T>(g: &Graph<V, T>, starts: &[usize], inf: T) -> DijkstraResult<T>\nwhere\n\
    \    V: Clone,\n    T: Clone + Ord + Add<Output = T> + Default,\n{\n    assert!(starts.len()\
    \ > 0);\n    let mut dist = vec![inf.clone(); g.len()];\n    let mut prev = vec![!0;\
    \ g.len()];\n    let mut pq = BinaryHeap::new();\n    for &s in starts {\n   \
    \     dist[s] = T::default();\n        pq.push(Reverse((T::default(), s)));\n\
    \    }\n    while let Some(Reverse((s, v))) = pq.pop() {\n        if dist[v] <\
    \ s {\n            continue;\n        }\n        for (u, w) in &g[v] {\n     \
    \       assert!(w.clone() >= T::default());\n            if dist[*u] > dist[v].clone()\
    \ + w.clone() {\n                dist[*u] = dist[v].clone() + w.clone();\n   \
    \             prev[*u] = v;\n                pq.push(Reverse((dist[*u].clone(),\
    \ *u)));\n            }\n        }\n    }\n    DijkstraResult { dist, prev }\n\
    }\n\nimpl<T> DijkstraResult<T>\nwhere\n    T: Clone + Ord + Add<Output = T> +\
    \ Default,\n{\n    pub fn path(&self, mut v: usize) -> Option<Vec<usize>> {\n\
    \        if self.dist[v].clone() != T::default() && self.prev[v] == !0 {\n   \
    \         return None;\n        }\n        let mut path = vec![];\n        while\
    \ v != !0 {\n            path.push(v);\n            v = self.prev[v];\n      \
    \  }\n        path.reverse();\n        Some(path)\n    }\n}\n"
  dependsOn:
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/graph/dijkstra/src/lib.rs
  requiredBy: []
  timestamp: '2024-05-13 10:35:48+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/shortest_path/src/main.rs
documentation_of: crates/graph/dijkstra/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/dijkstra/src/lib.rs
- /library/crates/graph/dijkstra/src/lib.rs.html
title: crates/graph/dijkstra/src/lib.rs
---
