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
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy-light-decomposition/src/lib.rs
    title: crates/tree/heavy-light-decomposition/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/vertex_set_path_composite
    links:
    - https://judge.yosupo.jp/problem/vertex_set_path_composite
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_set_path_composite\n\
    \nuse algebraic::{algebra, monoid, Monoid, ReversedMonoid};\nuse graph::UndirectedGraph;\n\
    use heavy_light_decomposition::HeavyLightDecomposition;\nuse modint::ModInt998244353\
    \ as Mint;\nuse proconio::fastout;\nuse proconio::input;\nuse segment_tree::SegmentTree;\n\
    \nalgebra!(M, (Mint, Mint));\nmonoid!(M, (1.into(), 0.into()), |&(a, b), &(c,\
    \ d)| (\n    a * c,\n    b * c + d\n));\n\ntype RM = ReversedMonoid<M>;\n\n#[fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        q: usize,\n        a: [(Mint,\
    \ Mint); n],\n        uv: [(usize, usize); n - 1],\n    }\n    let g = UndirectedGraph::from_vertices_and_unweighted_edges(&a,\
    \ &uv);\n    let mut seg = SegmentTree::<M>::new(n);\n    let mut rev_seg = SegmentTree::<RM>::new(n);\n\
    \    let hld = HeavyLightDecomposition::new(&g, 0);\n    for i in 0..n {\n   \
    \     seg.set(hld.vertex_index(i), a[i]);\n        rev_seg.set(hld.vertex_index(i),\
    \ a[i]);\n    }\n    for _ in 0..q {\n        input! {\n            ty: usize,\n\
    \        }\n        if ty == 0 {\n            input! {\n                p: usize,\n\
    \                c: Mint,\n                d: Mint,\n            }\n         \
    \   seg.set(hld.vertex_index(p), (c, d));\n            rev_seg.set(hld.vertex_index(p),\
    \ (c, d));\n        } else {\n            input! {\n                u: usize,\n\
    \                v: usize,\n                x: Mint,\n            }\n        \
    \    let mut ab = M::e();\n            hld.path_query(\n                u,\n \
    \               v,\n                |l, r, rev| {\n                    if rev\
    \ {\n                        ab = M::op(&ab, &rev_seg.prod(l..r));\n         \
    \           } else {\n                        ab = M::op(&ab, &seg.prod(l..r));\n\
    \                    }\n                },\n                true,\n          \
    \  );\n            let (a, b) = ab;\n            println!(\"{}\", a * x + b);\n\
    \        }\n    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/data-structure/segment-tree/src/lib.rs
  - crates/graph/graph/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  - crates/tree/heavy-light-decomposition/src/lib.rs
  isVerificationFile: true
  path: verify/vertex_set_path_composite/src/main.rs
  requiredBy: []
  timestamp: '2025-01-12 04:36:01+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/vertex_set_path_composite/src/main.rs
layout: document
redirect_from:
- /verify/verify/vertex_set_path_composite/src/main.rs
- /verify/verify/vertex_set_path_composite/src/main.rs.html
title: verify/vertex_set_path_composite/src/main.rs
---
