---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/add.rs
    title: crates/algebra/algebraic_structure/src/add.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/affine.rs
    title: crates/algebra/algebraic_structure/src/affine.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/count_sum.rs
    title: crates/algebra/algebraic_structure/src/count_sum.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/countsum_affine.rs
    title: crates/algebra/algebraic_structure/src/countsum_affine.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/lib.rs
    title: crates/algebra/algebraic_structure/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/max.rs
    title: crates/algebra/algebraic_structure/src/max.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/min.rs
    title: crates/algebra/algebraic_structure/src/min.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/mul.rs
    title: crates/algebra/algebraic_structure/src/mul.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/semiring.rs
    title: crates/algebra/algebraic_structure/src/semiring.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/trivial_group.rs
    title: crates/algebra/algebraic_structure/src/trivial_group.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/xor.rs
    title: crates/algebra/algebraic_structure/src/xor.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/fenwick_tree/fenwick_tree/src/lib.rs
    title: crates/data_structure/fenwick_tree/fenwick_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/builder.rs
    title: crates/graph/graph/src/builder.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/edge.rs
    title: crates/graph/graph/src/edge.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/tree_contour_range/src/lib.rs
    title: crates/tree/tree_contour_range/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/vertex_add_range_contour_sum_on_tree
    links:
    - https://judge.yosupo.jp/problem/vertex_add_range_contour_sum_on_tree
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_range_contour_sum_on_tree\n\
    \nuse algebraic_structure::magma::AddOperator;\nuse fenwick_tree::FenwickTree;\n\
    use graph::{GraphBuilder, UndirectedGraph};\nuse proconio::{fastout, input};\n\
    use tree_contour_range::TreeContourRange;\n\n#[fastout]\nfn main() {\n    input!\
    \ {\n        n: usize,\n        q: usize,\n        mut a: [i64; n],\n        uv:\
    \ [(usize, usize); n - 1],\n    }\n\n    let g = UndirectedGraph::from_edges(n,\
    \ &uv);\n    let cr = TreeContourRange::new(&g);\n    let mut seg = FenwickTree::<AddOperator<i64>>::new(cr.len());\n\
    \    for (v, &x) in a.iter().enumerate() {\n        for i in cr.point(v) {\n \
    \           seg.add(i, x);\n        }\n    }\n    for _ in 0..q {\n        input!\
    \ {\n            t: usize,\n            p: usize,\n        }\n        if t ==\
    \ 0 {\n            input! {\n                x: i64,\n            }\n        \
    \    for i in cr.point(p) {\n                seg.add(i, x);\n            }\n \
    \           a[p] += x;\n        } else {\n            input! {\n             \
    \   l: usize,\n                r: usize,\n            }\n            let mut res\
    \ = cr.range(p, l..r).map(|range| seg.fold(range)).sum::<i64>();\n           \
    \ if l == 0 && 0 < r {\n                res += a[p];\n            }\n        \
    \    if n == 2 {\n                let q = p ^ 1;\n                if l <= 1 &&\
    \ 1 < r {\n                    res += a[q];\n                }\n            }\n\
    \            println!(\"{}\", res);\n        }\n    }\n}\n"
  dependsOn:
  - crates/algebra/algebraic_structure/src/add.rs
  - crates/algebra/algebraic_structure/src/affine.rs
  - crates/algebra/algebraic_structure/src/count_sum.rs
  - crates/algebra/algebraic_structure/src/countsum_affine.rs
  - crates/algebra/algebraic_structure/src/lib.rs
  - crates/algebra/algebraic_structure/src/max.rs
  - crates/algebra/algebraic_structure/src/min.rs
  - crates/algebra/algebraic_structure/src/mul.rs
  - crates/algebra/algebraic_structure/src/semiring.rs
  - crates/algebra/algebraic_structure/src/trivial_group.rs
  - crates/algebra/algebraic_structure/src/xor.rs
  - crates/data_structure/fenwick_tree/fenwick_tree/src/lib.rs
  - crates/graph/graph/src/builder.rs
  - crates/graph/graph/src/edge.rs
  - crates/graph/graph/src/lib.rs
  - crates/tree/tree_contour_range/src/lib.rs
  isVerificationFile: true
  path: verify/library_checker/tree/vertex_add_range_contour_sum_on_tree/src/main.rs
  requiredBy: []
  timestamp: '2025-05-08 12:53:34+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/tree/vertex_add_range_contour_sum_on_tree/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/tree/vertex_add_range_contour_sum_on_tree/src/main.rs
- /verify/verify/library_checker/tree/vertex_add_range_contour_sum_on_tree/src/main.rs.html
title: verify/library_checker/tree/vertex_add_range_contour_sum_on_tree/src/main.rs
---
