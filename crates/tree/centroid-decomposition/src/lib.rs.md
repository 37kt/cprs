---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/frequency_table_of_tree_distance/src/main.rs
    title: verify/frequency_table_of_tree_distance/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use graph::UndirectedGraph;\n\nfn dfs1(v: usize, p: usize, g: &UndirectedGraph<(),\
    \ ()>, sz: &mut [usize]) {\n    sz[v] = 1;\n    for &(u, _) in &g[v] {\n     \
    \   if u == p {\n            continue;\n        }\n        dfs1(u, v, g, sz);\n\
    \        sz[v] += sz[u];\n    }\n}\n\nfn dfs2(v: usize, p: usize, mid: usize,\
    \ g: &UndirectedGraph<(), ()>, sz: &[usize]) -> usize {\n    for &(u, _) in &g[v]\
    \ {\n        if u == p {\n            continue;\n        }\n        if sz[u] >\
    \ mid {\n            return dfs2(u, v, mid, g, sz);\n        }\n    }\n    v\n\
    }\n\nfn dfs4(\n    v: usize,\n    p: usize,\n    g: &UndirectedGraph<(), ()>,\n\
    \    pre_idx: &[usize],\n    idx: &mut Vec<usize>,\n    par: &mut Vec<usize>,\n\
    ) {\n    idx.push(pre_idx[v]);\n    par.push(pre_idx[p]);\n    for &(u, _) in\
    \ &g[v] {\n        if u == p {\n            continue;\n        }\n        dfs4(u,\
    \ v, g, pre_idx, idx, par);\n    }\n}\n\nfn dfs3(\n    v: usize,\n    g: &UndirectedGraph<(),\
    \ ()>,\n    pre_idx: &[usize],\n    conv: &mut [usize],\n    f: &mut impl FnMut(&[usize],\
    \ &[usize], usize),\n) {\n    let n = g.len();\n    let mut sz = vec![0; n];\n\
    \    dfs1(v, !0, g, &mut sz);\n    let c = dfs2(v, !0, sz[v] / 2, g, &sz);\n \
    \   dfs1(c, !0, g, &mut sz);\n    let n = sz[c];\n    if n <= 2 {\n        return;\n\
    \    }\n    let mut szsum = 0;\n    let mut rl = vec![];\n    let mut rr = vec![];\n\
    \    for &(u, _) in &g[c] {\n        if szsum + sz[u] <= (n - 1) / 2 {\n     \
    \       szsum += sz[u];\n            rl.push(u);\n        } else {\n         \
    \   rr.push(u);\n        }\n    }\n    conv[pre_idx[c]] = 0;\n    let mut idx_l\
    \ = vec![];\n    let mut par_l = vec![];\n    let mut es_l = vec![];\n    for\
    \ &u in &rl {\n        dfs4(u, c, g, pre_idx, &mut idx_l, &mut par_l);\n    }\n\
    \    for i in 0..idx_l.len() {\n        conv[idx_l[i]] = i + 1;\n        es_l.push((conv[par_l[i]],\
    \ i + 1));\n    }\n    let mut idx_r = vec![];\n    let mut par_r = vec![];\n\
    \    let mut es_r = vec![];\n    for &u in &rr {\n        dfs4(u, c, g, pre_idx,\
    \ &mut idx_r, &mut par_r);\n    }\n    for i in 0..idx_r.len() {\n        conv[idx_r[i]]\
    \ = i + 1;\n        es_r.push((conv[par_r[i]], i + 1));\n    }\n    let gl = UndirectedGraph::from_unweighted_edges(idx_l.len()\
    \ + 1, &es_l);\n    let gr = UndirectedGraph::from_unweighted_edges(idx_r.len()\
    \ + 1, &es_r);\n    let mut idx = vec![];\n    idx.append(&mut idx_l.clone());\n\
    \    idx.append(&mut idx_r.clone());\n    let mut par = vec![];\n    par.append(&mut\
    \ par_l);\n    par.append(&mut par_r);\n    f(&idx, &par, idx_l.len());\n    idx_l.insert(0,\
    \ pre_idx[c]);\n    idx_r.insert(0, pre_idx[c]);\n    dfs3(0, &gl, &idx_l, conv,\
    \ f);\n    dfs3(0, &gr, &idx_r, conv, f);\n}\n\n/// \u91CD\u5FC3\u5206\u89E3 \
    \ \n/// `f: fn f(idx: &[usize], par: &[usize], m: usize)`  \n/// `par[0]` \u3092\
    \u6839\u3068\u3059\u308B\u90E8\u5206\u6728\u304C\u30C8\u30DD\u30ED\u30B8\u30AB\
    \u30EB\u9806\u5E8F\u3067\u6E21\u3055\u308C\u308B  \n/// `idx`: \u9802\u70B9\u756A\
    \u53F7  \n/// `par`: \u89AA\u306E\u9802\u70B9\u756A\u53F7  \n/// `idx[..m]` \u304C\
    \u8D64\uFF0C`idx[m..]` \u304C\u9752  \n/// `f` \u5185\u3067\u3001`idx[..m]` \u3068\
    \ `idx[m..]` \u9593\u306E\u30D1\u30B9\u306B\u3064\u3044\u3066\u8A08\u7B97\u3059\
    \u308B\npub fn centroid_decomposition(\n    g: &UndirectedGraph<(), ()>,\n   \
    \ mut f: impl FnMut(&[usize], &[usize], usize),\n) {\n    let n = g.len();\n \
    \   let mut conv = vec![!0; n];\n    dfs3(0, g, &(0..n).collect::<Vec<_>>(), &mut\
    \ conv, &mut f);\n}\n"
  dependsOn:
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/tree/centroid-decomposition/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-14 05:25:42+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/frequency_table_of_tree_distance/src/main.rs
documentation_of: crates/tree/centroid-decomposition/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/centroid-decomposition/src/lib.rs
- /library/crates/tree/centroid-decomposition/src/lib.rs.html
title: crates/tree/centroid-decomposition/src/lib.rs
---
