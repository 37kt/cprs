---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy_light_decomposition/src/compress.rs
    title: crates/tree/heavy_light_decomposition/src/compress.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy_light_decomposition/src/lib.rs
    title: crates/tree/heavy_light_decomposition/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/tree/dynamic_rerooting_tree_dp/src/lib.rs
    title: crates/tree/dynamic_rerooting_tree_dp/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/dynamic_tree_dp/src/lib.rs
    title: crates/tree/dynamic_tree_dp/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy_light_decomposition/src/compress.rs
    title: crates/tree/heavy_light_decomposition/src/compress.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy_light_decomposition/src/lib.rs
    title: crates/tree/heavy_light_decomposition/src/lib.rs
  - icon: ':warning:'
    path: crates/tree/static_top_tree/src/lib.rs
    title: crates/tree/static_top_tree/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/jump_on_tree/src/main.rs
    title: verify/library_checker/tree/jump_on_tree/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/lca/src/main.rs
    title: verify/library_checker/tree/lca/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/vertex_add_path_sum/src/main.rs
    title: verify/library_checker/tree/vertex_add_path_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/vertex_add_subtree_sum/src/main.rs
    title: verify/library_checker/tree/vertex_add_subtree_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/vertex_set_path_composite/src/main.rs
    title: verify/library_checker/tree/vertex_set_path_composite/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yukicoder/tree/yuki0901_aux/src/main.rs
    title: verify/yukicoder/tree/yuki0901_aux/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use crate::{Edge, HeavyLightDecomposition};\n\nimpl HeavyLightDecomposition\
    \ {\n    /// \u89AA\u914D\u5217\u304B\u3089\u69CB\u7BC9  \n    /// `0` \u304C\u6839\
    \u3002 `i > 0` \u306B\u3064\u3044\u3066 `par[i] < i`  \n    /// `par[0]` \u306E\
    \u5024\u306F\u7121\u8996\u3055\u308C\u308B\n    pub fn from_parents(par: &[usize])\
    \ -> Self {\n        let n = par.len();\n        let root = 0;\n\n        let\
    \ mut down = vec![-1; n];\n        let mut next = std::iter::once(-1)\n      \
    \      .chain(par.iter().skip(1).map(|&p| p as i32))\n            .collect::<Vec<_>>();\n\
    \        let mut sub = vec![1; n];\n\n        for v in (1..n).rev() {\n      \
    \      let p = next[v] as usize;\n            sub[p] += sub[v];\n            down[p]\
    \ = down[p].max(sub[v]);\n        }\n        for v in (1..n).rev() {\n       \
    \     let p = next[v] as usize;\n            if down[p] == sub[v] {\n        \
    \        sub[v] = !sub[v];\n                down[p] = !down[p];\n            }\n\
    \        }\n\n        sub[root] = !down[root] + 1;\n        down[root] = 0;\n\n\
    \        for v in 1..n {\n            let p = next[v] as usize;\n            let\
    \ nsub = !down[v] + 1;\n            if sub[v] < 0 {\n                down[v] =\
    \ down[p] + 1;\n                next[v] = if next[p] < 0 { p as i32 } else { next[p]\
    \ };\n            } else {\n                down[v] = down[p] + sub[p];\n    \
    \            sub[p] += sub[v];\n                next[v] = !(p as i32);\n     \
    \       }\n            sub[v] = nsub;\n        }\n\n        let mut tour = vec![-1;\
    \ n];\n        for (v, &d) in down.iter().enumerate() {\n            let t = d\
    \ as usize;\n            tour[t] = v as i32;\n        }\n\n        let mut edge_ord\
    \ = vec![0; n - 1];\n        for (e, &d) in down.iter().enumerate().skip(1) {\n\
    \            let i = d as usize - 1;\n            edge_ord[i] = e;\n        }\n\
    \n        Self {\n            n,\n            root,\n            down,\n     \
    \       next,\n            sub,\n            tour,\n            edge_ord,\n  \
    \      }\n    }\n\n    /// &[(usize, usize)] \u304B &[(usize, usize, T)] \u304B\
    \u3089\u69CB\u7BC9\n    pub fn from_edges(edges: &[impl Edge], root: usize) ->\
    \ Self {\n        let n = edges.len() + 1;\n        assert!(root < n);\n\n   \
    \     let mut down = vec![0; n];\n        let mut next = vec![0; n];\n       \
    \ for e in edges {\n            let (u, v) = e.endpoints();\n            down[u]\
    \ += 1;\n            down[v] += 1;\n            next[u] ^= v as i32;\n       \
    \     next[v] ^= u as i32;\n        }\n\n        let mut tour = Vec::with_capacity(n);\n\
    \        for (v, &d) in down.iter().enumerate() {\n            if v != root &&\
    \ d == 1 {\n                tour.push(v as i32);\n            }\n        }\n \
    \       for i in 0..n - 1 {\n            let v = tour[i] as usize;\n         \
    \   down[v] = -1;\n            let u = next[v] as usize;\n            next[u]\
    \ ^= v as i32;\n            down[u] -= 1;\n            if down[u] == 1 && u !=\
    \ root {\n                tour.push(u as i32);\n            }\n        }\n\n \
    \       let mut sub = vec![1; n];\n        for &v in &tour {\n            let\
    \ v = v as usize;\n            let p = next[v] as usize;\n            sub[p] +=\
    \ sub[v];\n            down[p] = down[p].max(sub[v]);\n        }\n        for\
    \ &v in &tour {\n            let v = v as usize;\n            let p = next[v]\
    \ as usize;\n            if down[p] == sub[v] {\n                sub[v] = !sub[v];\n\
    \                down[p] = !down[p];\n            }\n        }\n\n        sub[root]\
    \ = !down[root] + 1;\n        down[root] = 0;\n        next[root] = -1;\n\n  \
    \      for &v in tour.iter().rev() {\n            let v = v as usize;\n      \
    \      let p = next[v] as usize;\n            let nsub = !down[v] + 1;\n     \
    \       if sub[v] < 0 {\n                down[v] = down[p] + 1;\n            \
    \    next[v] = if next[p] < 0 { p as i32 } else { next[p] };\n            } else\
    \ {\n                down[v] = down[p] + sub[p];\n                sub[p] += sub[v];\n\
    \                next[v] = !(p as i32);\n            }\n            sub[v] = nsub;\n\
    \        }\n\n        tour.resize(n, -1);\n        for (v, &d) in down.iter().enumerate()\
    \ {\n            let t = d as usize;\n            tour[t] = v as i32;\n      \
    \  }\n\n        let mut edge_ord = vec![0; n - 1];\n        for (e, edge) in edges.iter().enumerate()\
    \ {\n            let (u, v) = edge.endpoints();\n            let i = down[u].max(down[v])\
    \ as usize - 1;\n            edge_ord[i] = e;\n        }\n\n        Self {\n \
    \           n,\n            root,\n            down,\n            next,\n    \
    \        sub,\n            tour,\n            edge_ord,\n        }\n    }\n}\n"
  dependsOn:
  - crates/tree/heavy_light_decomposition/src/compress.rs
  - crates/tree/heavy_light_decomposition/src/lib.rs
  isVerificationFile: false
  path: crates/tree/heavy_light_decomposition/src/construct.rs
  requiredBy:
  - crates/tree/dynamic_rerooting_tree_dp/src/lib.rs
  - crates/tree/dynamic_tree_dp/src/lib.rs
  - crates/tree/heavy_light_decomposition/src/lib.rs
  - crates/tree/heavy_light_decomposition/src/compress.rs
  - crates/tree/static_top_tree/src/lib.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/tree/lca/src/main.rs
  - verify/library_checker/tree/vertex_add_path_sum/src/main.rs
  - verify/library_checker/tree/vertex_set_path_composite/src/main.rs
  - verify/library_checker/tree/vertex_add_subtree_sum/src/main.rs
  - verify/library_checker/tree/jump_on_tree/src/main.rs
  - verify/yukicoder/tree/yuki0901_aux/src/main.rs
documentation_of: crates/tree/heavy_light_decomposition/src/construct.rs
layout: document
redirect_from:
- /library/crates/tree/heavy_light_decomposition/src/construct.rs
- /library/crates/tree/heavy_light_decomposition/src/construct.rs.html
title: crates/tree/heavy_light_decomposition/src/construct.rs
---
