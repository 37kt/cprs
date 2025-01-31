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
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy-light-decomposition/src/lib.rs
    title: crates/tree/heavy-light-decomposition/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/vertex_add_path_sum
    links:
    - https://judge.yosupo.jp/problem/vertex_add_path_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_path_sum\n\
    \nuse algebraic::Monoid;\nuse algebraic::{algebra, monoid};\nuse graph::UndirectedGraph;\n\
    use heavy_light_decomposition::HeavyLightDecomposition;\nuse proconio::fastout;\n\
    use proconio::input;\nuse segment_tree::SegmentTree;\n\nalgebra!(M, i64);\nmonoid!(M,\
    \ 0, |x, y| x + y);\n\n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n\
    \        q: usize,\n        a: [i64; n],\n        uv: [(usize, usize); n - 1],\n\
    \    }\n    let g = UndirectedGraph::from_vertices_and_unweighted_edges(&a, &uv);\n\
    \    let hld = HeavyLightDecomposition::new(&g, 0);\n    let mut seg = SegmentTree::<M>::new(n);\n\
    \    for i in 0..n {\n        seg.set(hld.vertex_index(i), a[i]);\n    }\n   \
    \ for _ in 0..q {\n        input! {\n            ty: usize,\n        }\n     \
    \   if ty == 0 {\n            input! {\n                p: usize,\n          \
    \      x: i64,\n            }\n            let i = hld.vertex_index(p);\n    \
    \        let y = seg.get(i) + x;\n            seg.set(i, y);\n        } else {\n\
    \            input! {\n                u: usize,\n                v: usize,\n\
    \            }\n            let mut s = M::e();\n            hld.path_query(\n\
    \                u,\n                v,\n                |l, r, _| {\n       \
    \             s = M::op(&s, &seg.prod(l..r));\n                },\n          \
    \      true,\n            );\n            println!(\"{}\", s);\n        }\n  \
    \  }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/data-structure/segment-tree/src/lib.rs
  - crates/graph/graph/src/lib.rs
  - crates/tree/heavy-light-decomposition/src/lib.rs
  isVerificationFile: true
  path: verify/vertex_add_path_sum/src/main.rs
  requiredBy: []
  timestamp: '2025-01-15 06:25:46+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/vertex_add_path_sum/src/main.rs
layout: document
redirect_from:
- /verify/verify/vertex_add_path_sum/src/main.rs
- /verify/verify/vertex_add_path_sum/src/main.rs.html
title: verify/vertex_add_path_sum/src/main.rs
---
