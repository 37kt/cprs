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
  code: "use std::ops::Add;\n\nuse graph::Graph;\n\n/// \u30D9\u30EB\u30DE\u30F3\u30D5\
    \u30A9\u30FC\u30C9\u6CD5  \n/// \u59CB\u70B9\u96C6\u5408\u304B\u3089\u306E\u6700\
    \u77ED\u8DDD\u96E2\u3092\u6C42\u3081\u308B\u3002  \n/// \u8CA0\u306E\u9589\u8DEF\
    \u304C\u5B58\u5728\u3059\u308B\u5834\u5408\u306F None \u3092\u8FD4\u3059\u3002\
    \npub fn bellman_ford<T>(g: &Graph<(), T>, starts: &[usize], inf: T) -> Option<Vec<T>>\n\
    where\n    T: Clone + PartialOrd + Add<Output = T> + Default,\n{\n    let n =\
    \ g.len();\n    let mut dist = vec![inf.clone(); n];\n    for &s in starts {\n\
    \        dist[s] = T::default();\n    }\n    for i in 0..n {\n        for v in\
    \ 0..n {\n            for (u, w) in &g[v] {\n                if dist[*u] > dist[v].clone()\
    \ + w.clone() {\n                    if i == n - 1 {\n                       \
    \ return None;\n                    }\n                    dist[*u] = dist[v].clone()\
    \ + w.clone();\n                }\n            }\n        }\n    }\n    Some(dist)\n\
    }\n"
  dependsOn:
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/graph/bellman-ford/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-27 03:53:35+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/graph/bellman-ford/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/bellman-ford/src/lib.rs
- /library/crates/graph/bellman-ford/src/lib.rs.html
title: crates/graph/bellman-ford/src/lib.rs
---
