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
  code: "use std::collections::VecDeque;\n\nuse graph::Graph;\n\nconst INF: usize\
    \ = 1 << 60;\n\npub fn complement_graph_bfs(g: &Graph<(), ()>, start: usize) ->\
    \ Vec<usize> {\n    let n = g.len();\n    let mut dist = vec![INF; n];\n    let\
    \ mut q = VecDeque::new();\n    dist[start] = 0;\n    q.push_back(start);\n  \
    \  let mut s = (0..start).chain(start + 1..n).collect();\n    let mut f = vec![false;\
    \ n];\n    while let Some(v) = q.pop_front() {\n        let mut l = vec![];\n\
    \        for &(u, _) in &g[v] {\n            f[u] = true;\n        }\n       \
    \ for &u in &s {\n            if f[u] {\n                l.push(u);\n        \
    \    } else {\n                dist[u] = dist[v] + 1;\n                q.push_back(u);\n\
    \            }\n        }\n        for &(u, _) in &g[v] {\n            f[u] =\
    \ false;\n        }\n        s = l;\n    }\n    dist\n}\n"
  dependsOn:
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/graph/complement-graph-bfs/src/lib.rs
  requiredBy: []
  timestamp: '2024-04-10 09:38:39+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/graph/complement-graph-bfs/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/complement-graph-bfs/src/lib.rs
- /library/crates/graph/complement-graph-bfs/src/lib.rs.html
title: crates/graph/complement-graph-bfs/src/lib.rs
---
