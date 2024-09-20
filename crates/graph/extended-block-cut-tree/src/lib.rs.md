---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/biconnected_components/src/main.rs
    title: verify/biconnected_components/src/main.rs
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
  code: "use graph::Graph;\n\npub fn extended_block_cut_tree(g: &Graph<(), ()>) ->\
    \ Graph<(), ()> {\n    let n = g.len();\n    let mut next = vec![!0; n];\n   \
    \ let mut d = vec![!0; n];\n    let mut imos = vec![0; n];\n    for i in 0..n\
    \ {\n        if d[i] == !0 {\n            d[i] = 0;\n            dfs1(i, &g, &mut\
    \ next, &mut d, &mut imos);\n        }\n    }\n    let mut cnt = 0;\n    let mut\
    \ used = vec![false; n];\n    let mut edges = vec![];\n    for i in 0..n {\n \
    \       if d[i] == 0 {\n            dfs2(\n                i, cnt, &g, &mut d,\
    \ &mut imos, &mut used, &mut cnt, &mut edges,\n            );\n        }\n   \
    \     if g[i].is_empty() {\n            edges.push((i, n + cnt));\n          \
    \  cnt += 1;\n        }\n    }\n    Graph::from_unweighted_undirected_edges(n\
    \ + cnt, &edges)\n}\n\nfn dfs1(v: usize, g: &Graph<(), ()>, next: &mut [usize],\
    \ d: &mut [usize], imos: &mut [i32]) {\n    for &(u, _) in &g[v] {\n        if\
    \ d[u] == !0 {\n            d[u] = d[v] + 1;\n            next[v] = u;\n     \
    \       dfs1(u, g, next, d, imos);\n            imos[v] += imos[u];\n        }\
    \ else if d[u] + 1 < d[v] {\n            imos[v] += 1;\n            imos[next[u]]\
    \ -= 1;\n        }\n    }\n}\n\nfn dfs2(\n    v: usize,\n    b: usize,\n    g:\
    \ &Graph<(), ()>,\n    d: &mut [usize],\n    imos: &mut [i32],\n    used: &mut\
    \ [bool],\n    cnt: &mut usize,\n    edges: &mut Vec<(usize, usize)>,\n) {\n \
    \   let n = g.len();\n    used[v] = true;\n    let mut ok = false;\n    for &(u,\
    \ _) in &g[v] {\n        if d[u] == d[v] + 1 && !used[u] {\n            if imos[u]\
    \ > 0 {\n                if !ok {\n                    ok = true;\n          \
    \          edges.push((v, n + b));\n                }\n                dfs2(u,\
    \ b, g, d, imos, used, cnt, edges);\n            } else {\n                edges.push((v,\
    \ n + *cnt));\n                *cnt += 1;\n                dfs2(u, *cnt - 1, g,\
    \ d, imos, used, cnt, edges);\n            }\n        }\n    }\n    if !ok &&\
    \ d[v] > 0 {\n        edges.push((v, n + b));\n    }\n}\n"
  dependsOn:
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/graph/extended-block-cut-tree/src/lib.rs
  requiredBy: []
  timestamp: '2024-04-10 09:38:39+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/biconnected_components/src/main.rs
documentation_of: crates/graph/extended-block-cut-tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/extended-block-cut-tree/src/lib.rs
- /library/crates/graph/extended-block-cut-tree/src/lib.rs.html
title: crates/graph/extended-block-cut-tree/src/lib.rs
---
