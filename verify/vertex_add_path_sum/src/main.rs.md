---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/segment-tree/src/lib.rs
    title: crates/data-structure/segment-tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/tree-query/src/lib.rs
    title: crates/data-structure/tree-query/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/vertex_add_path_sum
    links:
    - https://judge.yosupo.jp/problem/vertex_add_path_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_path_sum\n\
    \nuse algebraic::{algebra, monoid};\nuse graph::Graph;\nuse proconio::input;\n\
    use tree_query::TreeQueryVertex;\n\nalgebra!(M, i64);\nmonoid!(M, 0, |x, y| x\
    \ + y);\n\n#[proconio::fastout]\nfn main() {\n    input! {\n        n: usize,\n\
    \        q: usize,\n        a: [i64; n],\n        uv: [(usize, usize); n - 1],\n\
    \    }\n    let g = Graph::from_vertices_and_unweighted_undirected_edges(&a, &uv);\n\
    \    let mut tq = TreeQueryVertex::<M>::build(&g);\n    for _ in 0..q {\n    \
    \    input! {\n            ty: usize,\n        }\n        if ty == 0 {\n     \
    \       input! {\n                p: usize,\n                x: i64,\n       \
    \     }\n            let t = tq.get(p);\n            tq.set(p, t + x);\n     \
    \   } else {\n            input! {\n                u: usize,\n              \
    \  v: usize,\n            }\n            let t = tq.prod_path(u, v);\n       \
    \     println!(\"{}\", t);\n        }\n    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/data-structure/segment-tree/src/lib.rs
  - crates/data-structure/tree-query/src/lib.rs
  - crates/graph/graph/src/lib.rs
  isVerificationFile: true
  path: verify/vertex_add_path_sum/src/main.rs
  requiredBy: []
  timestamp: '2024-12-26 06:54:01+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/vertex_add_path_sum/src/main.rs
layout: document
redirect_from:
- /verify/verify/vertex_add_path_sum/src/main.rs
- /verify/verify/vertex_add_path_sum/src/main.rs.html
title: verify/vertex_add_path_sum/src/main.rs
---
