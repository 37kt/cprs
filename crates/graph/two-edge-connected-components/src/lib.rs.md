---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/two_edge_connected_components/src/main.rs
    title: verify/two_edge_connected_components/src/main.rs
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
  code: "use graph::Graph;\n\nfn dfs(\n    v: usize,\n    g: &Graph<(), ()>,\n   \
    \ ord: &mut Vec<usize>,\n    par: &mut Vec<usize>,\n    imos: &mut Vec<i32>,\n\
    \    used: &mut Vec<bool>,\n) {\n    ord.push(v);\n    for i in 0..g[v].len()\
    \ {\n        let u = g[v][i].0;\n        let e = g.edge_id(v, i);\n        if\
    \ used[e] {\n            continue;\n        }\n        if par[u] != !1 {\n   \
    \         imos[v] += 1;\n            imos[u] -= 1;\n            used[e] = true;\n\
    \        } else {\n            used[e] = true;\n            par[u] = v;\n    \
    \        dfs(u, g, ord, par, imos, used);\n        }\n    }\n}\n\npub fn two_edge_connected_components(g:\
    \ &Graph<(), ()>) -> (Vec<Vec<usize>>, Vec<usize>) {\n    let n = g.len();\n \
    \   let m = g.edges_count();\n    let mut ord = vec![];\n    let mut par = vec![!1;\
    \ n];\n    let mut imos = vec![0; n];\n    let mut used = vec![false; m];\n  \
    \  for v in 0..n {\n        if par[v] == !1 {\n            par[v] = !0;\n    \
    \        dfs(v, g, &mut ord, &mut par, &mut imos, &mut used);\n        }\n   \
    \ }\n    for &v in ord.iter().rev() {\n        if par[v] != !0 {\n           \
    \ imos[par[v]] += imos[v];\n        }\n    }\n    let mut comp = vec![!0; n];\n\
    \    let mut comp_cnt = 0;\n    for &v in &ord {\n        if imos[v] == 0 {\n\
    \            comp[v] = comp_cnt;\n            comp_cnt += 1;\n        } else {\n\
    \            comp[v] = comp[par[v]];\n        }\n    }\n    let mut groups = vec![vec![];\
    \ comp_cnt];\n    for v in 0..n {\n        groups[comp[v]].push(v);\n    }\n \
    \   (groups, comp)\n}\n"
  dependsOn:
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/graph/two-edge-connected-components/src/lib.rs
  requiredBy: []
  timestamp: '2024-04-10 09:38:39+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/two_edge_connected_components/src/main.rs
documentation_of: crates/graph/two-edge-connected-components/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/two-edge-connected-components/src/lib.rs
- /library/crates/graph/two-edge-connected-components/src/lib.rs.html
title: crates/graph/two-edge-connected-components/src/lib.rs
---
