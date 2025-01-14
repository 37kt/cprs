---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/data-structure/csr-array/src/lib.rs
    title: crates/data-structure/csr-array/src/lib.rs
  - icon: ':question:'
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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use csr_array::CSRArray;\nuse graph::UndirectedGraph;\n\nfn dfs(\n    v:\
    \ usize,\n    p: usize,\n    g: &UndirectedGraph<(), ()>,\n    comp: &mut [usize],\n\
    \    comp_cnt: &mut usize,\n    ord: &mut [usize],\n    low: &mut [usize],\n \
    \   vs: &mut Vec<usize>,\n    t: &mut usize,\n) {\n    ord[v] = *t;\n    low[v]\
    \ = *t;\n    *t += 1;\n    vs.push(v);\n    let mut f = false;\n    for &(u, _)\
    \ in &g[v] {\n        if ord[u] == !0 {\n            dfs(u, v, g, comp, comp_cnt,\
    \ ord, low, vs, t);\n            low[v] = low[v].min(low[u]);\n        } else\
    \ if u == p && !f {\n            f = true;\n        } else {\n            low[v]\
    \ = low[v].min(ord[u]);\n        }\n    }\n    if ord[v] == low[v] {\n       \
    \ while comp[v] == !0 {\n            comp[vs.pop().unwrap()] = *comp_cnt;\n  \
    \      }\n        *comp_cnt += 1;\n    }\n}\n\n/// \u4E8C\u8FBA\u9023\u7D50\u6210\
    \u5206\u5206\u89E3\u3092\u3059\u308B  \n/// \u6A4B\u3092\u9664\u3044\u305F\u3068\
    \u304D\u306E\u9023\u7D50\u6210\u5206\u3092\u6C42\u3081\u308B\n///\n/// # \u623B\
    \u308A\u5024\n///\n/// (groups, comp)\n/// - groups: \u4E8C\u8FBA\u9023\u7D50\u6210\
    \u5206\u306E\u30B0\u30EB\u30FC\u30D7\n/// - comp: \u5404\u9802\u70B9\u304C\u5C5E\
    \u3059\u308B\u4E8C\u8FBA\u9023\u7D50\u6210\u5206\u306E\u756A\u53F7\npub fn two_edge_connected_components(g:\
    \ &UndirectedGraph<(), ()>) -> (CSRArray<usize>, Vec<usize>) {\n    let n = g.len();\n\
    \    let mut comp = vec![!0; n];\n    let mut comp_cnt = 0;\n    let mut ord =\
    \ vec![!0; n];\n    let mut low = vec![!0; n];\n    let mut vs = vec![];\n   \
    \ let mut t = 0;\n    for v in 0..n {\n        if comp[v] == !0 {\n          \
    \  dfs(\n                v,\n                !0,\n                g,\n       \
    \         &mut comp,\n                &mut comp_cnt,\n                &mut ord,\n\
    \                &mut low,\n                &mut vs,\n                &mut t,\n\
    \            );\n        }\n    }\n\n    let groups = comp\n        .iter()\n\
    \        .enumerate()\n        .map(|(v, &c)| (c, v))\n        .collect::<Vec<_>>();\n\
    \    let groups = CSRArray::new(comp_cnt, &groups);\n\n    (groups, comp)\n}\n"
  dependsOn:
  - crates/data-structure/csr-array/src/lib.rs
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/graph/two-edge-connected-components/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-14 05:59:50+00:00'
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
