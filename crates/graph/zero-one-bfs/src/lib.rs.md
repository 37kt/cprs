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
  code: "use std::{collections::VecDeque, ops::Add};\n\nuse graph::Graph;\n\npub struct\
    \ ZeroOneBFSResult<T>\nwhere\n    T: Clone + Ord + Add<Output = T> + Default,\n\
    {\n    pub dist: Vec<T>,\n    pub prev: Vec<usize>,\n}\n\npub fn zero_one_bfs<V,\
    \ T>(g: &Graph<V, T>, starts: &[usize], inf: T) -> ZeroOneBFSResult<T>\nwhere\n\
    \    V: Clone,\n    T: Clone + Ord + Add<Output = T> + Default + From<u8>,\n{\n\
    \    assert!(starts.len() > 0);\n    let zero = T::from(0);\n    let one = T::from(1);\n\
    \    let mut dist = vec![inf.clone(); g.len()];\n    let mut prev = vec![!0; g.len()];\n\
    \    let mut dq = VecDeque::new();\n    for &s in starts {\n        dist[s] =\
    \ T::default();\n        dq.push_back((zero.clone(), s));\n    }\n    while let\
    \ Some((s, v)) = dq.pop_front() {\n        if dist[v] < s {\n            continue;\n\
    \        }\n        for (u, w) in &g[v] {\n            assert!(*w == zero || *w\
    \ == one);\n            let t = dist[v].clone() + w.clone();\n            if dist[*u]\
    \ > t {\n                dist[*u] = t.clone();\n                prev[*u] = v;\n\
    \                if *w == zero {\n                    dq.push_front((t.clone(),\
    \ *u));\n                } else {\n                    dq.push_back((t.clone(),\
    \ *u));\n                }\n            }\n        }\n    }\n    ZeroOneBFSResult\
    \ { dist, prev }\n}\n\nimpl<T> ZeroOneBFSResult<T>\nwhere\n    T: Clone + Ord\
    \ + Add<Output = T> + Default,\n{\n    pub fn path(&self, mut v: usize) -> Option<Vec<usize>>\
    \ {\n        if self.dist[v].clone() != T::default() && self.prev[v] == !0 {\n\
    \            return None;\n        }\n        let mut path = vec![];\n       \
    \ while v != !0 {\n            path.push(v);\n            v = self.prev[v];\n\
    \        }\n        path.reverse();\n        Some(path)\n    }\n}\n"
  dependsOn:
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/graph/zero-one-bfs/src/lib.rs
  requiredBy: []
  timestamp: '2024-05-13 10:35:48+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/graph/zero-one-bfs/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/zero-one-bfs/src/lib.rs
- /library/crates/graph/zero-one-bfs/src/lib.rs.html
title: crates/graph/zero-one-bfs/src/lib.rs
---
