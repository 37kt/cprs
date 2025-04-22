---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/cast.rs
    title: crates/algebra/numeric_traits/src/cast.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/inf.rs
    title: crates/algebra/numeric_traits/src/inf.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/integer.rs
    title: crates/algebra/numeric_traits/src/integer.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/lib.rs
    title: crates/algebra/numeric_traits/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/numeric.rs
    title: crates/algebra/numeric_traits/src/numeric.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/signed.rs
    title: crates/algebra/numeric_traits/src/signed.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/zero_one.rs
    title: crates/algebra/numeric_traits/src/zero_one.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/csr_array/src/builder.rs
    title: crates/data_structure/csr_array/src/builder.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/csr_array/src/csr_array.rs
    title: crates/data_structure/csr_array/src/csr_array.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/csr_array/src/lib.rs
    title: crates/data_structure/csr_array/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/graph/shortest_path/src/main.rs
    title: verify/library_checker/graph/shortest_path/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{cmp::Ordering, collections::BinaryHeap};\n\nuse csr_array::CsrArray;\n\
    use numeric_traits::{Inf, Numeric};\n\npub struct Dijkstra<T>\nwhere\n    T: Numeric\
    \ + Inf,\n{\n    pub dist: Vec<T>,\n    pub prev: Vec<usize>,\n}\n\n#[derive(PartialEq)]\n\
    struct State<T>\nwhere\n    T: Numeric + Inf,\n{\n    cost: T,\n    v: u32,\n\
    }\n\nimpl<T> Eq for State<T> where T: Numeric + Inf {}\n\nimpl<T> Ord for State<T>\n\
    where\n    T: Numeric + Inf,\n{\n    fn cmp(&self, other: &Self) -> Ordering {\n\
    \        self.partial_cmp(other).unwrap()\n    }\n}\n\nimpl<T> PartialOrd for\
    \ State<T>\nwhere\n    T: Numeric + Inf,\n{\n    fn partial_cmp(&self, other:\
    \ &Self) -> Option<Ordering> {\n        other.cost.partial_cmp(&self.cost)\n \
    \   }\n}\n\nimpl<T> Dijkstra<T>\nwhere\n    T: Numeric + Inf,\n{\n    pub fn new<'a>(g:\
    \ &CsrArray<(usize, T)>, starts: impl IntoIterator<Item = &'a usize>) -> Self\
    \ {\n        let n = g.len();\n        let mut dist = vec![T::inf(); n];\n   \
    \     let mut prev = vec![!0; n];\n        let mut pq = BinaryHeap::<State<T>>::new();\n\
    \        for &s in starts {\n            dist[s] = T::zero();\n            pq.push(State\
    \ {\n                cost: T::zero(),\n                v: s as u32,\n        \
    \    });\n        }\n        while let Some(State { cost, v }) = pq.pop() {\n\
    \            let v = v as usize;\n            if dist[v] < cost {\n          \
    \      continue;\n            }\n            for &(u, w) in &g[v] {\n        \
    \        assert!(w >= T::zero(), \"Weight must be non-negative\");\n\n       \
    \         let new_cost = cost + w;\n                if dist[u] > new_cost {\n\
    \                    dist[u] = new_cost;\n                    prev[u] = v;\n \
    \                   pq.push(State {\n                        cost: new_cost,\n\
    \                        v: u as u32,\n                    });\n             \
    \   }\n            }\n        }\n        Self { dist, prev }\n    }\n\n    pub\
    \ fn dist(&self, v: usize) -> Option<T> {\n        if self.dist[v] >= T::inf()\
    \ {\n            None\n        } else {\n            Some(self.dist[v])\n    \
    \    }\n    }\n\n    pub fn path(&self, mut v: usize) -> Option<Vec<usize>> {\n\
    \        if self.dist[v] >= T::inf() {\n            return None;\n        }\n\
    \        let mut path = vec![v];\n        while self.prev[v] != !0 {\n       \
    \     v = self.prev[v];\n            path.push(v);\n        }\n        path.reverse();\n\
    \        Some(path)\n    }\n}\n"
  dependsOn:
  - crates/algebra/numeric_traits/src/cast.rs
  - crates/algebra/numeric_traits/src/inf.rs
  - crates/algebra/numeric_traits/src/integer.rs
  - crates/algebra/numeric_traits/src/lib.rs
  - crates/algebra/numeric_traits/src/numeric.rs
  - crates/algebra/numeric_traits/src/signed.rs
  - crates/algebra/numeric_traits/src/zero_one.rs
  - crates/data_structure/csr_array/src/builder.rs
  - crates/data_structure/csr_array/src/csr_array.rs
  - crates/data_structure/csr_array/src/lib.rs
  isVerificationFile: false
  path: crates/graph/dijkstra/src/lib.rs
  requiredBy: []
  timestamp: '2025-03-20 09:27:03+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/graph/shortest_path/src/main.rs
documentation_of: crates/graph/dijkstra/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/dijkstra/src/lib.rs
- /library/crates/graph/dijkstra/src/lib.rs.html
title: crates/graph/dijkstra/src/lib.rs
---
