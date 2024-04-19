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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.3/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.3/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ops::Add;\n\nuse graph::Graph;\n\npub fn bellman_ford<T>(g: &Graph<(),\
    \ T>, starts: &[usize], inf: T) -> Option<Vec<T>>\nwhere\n    T: Clone + PartialOrd\
    \ + Add<Output = T> + Default,\n{\n    let n = g.len();\n    let mut dist = vec![inf.clone();\
    \ n];\n    for &s in starts {\n        dist[s] = T::default();\n    }\n    for\
    \ i in 0..n {\n        for v in 0..n {\n            for (u, w) in &g[v] {\n  \
    \              if dist[*u] > dist[v].clone() + w.clone() {\n                 \
    \   if i == n - 1 {\n                        return None;\n                  \
    \  }\n                    dist[*u] = dist[v].clone() + w.clone();\n          \
    \      }\n            }\n        }\n    }\n    Some(dist)\n}\n"
  dependsOn:
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/graph/bellman-ford/src/lib.rs
  requiredBy: []
  timestamp: '2024-04-10 09:38:39+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/graph/bellman-ford/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/bellman-ford/src/lib.rs
- /library/crates/graph/bellman-ford/src/lib.rs.html
title: crates/graph/bellman-ford/src/lib.rs
---
