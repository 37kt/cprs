---
data:
  _extendedDependsOn:
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
    path: crates/number_theory/modint/static_modint/src/lib.rs
    title: crates/number_theory/modint/static_modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/static_modint/src/mod_arithmetic.rs
    title: crates/number_theory/modint/static_modint/src/mod_arithmetic.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/static_modint/src/ntt_precalc.rs
    title: crates/number_theory/modint/static_modint/src/ntt_precalc.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/static_modint/src/numeric.rs
    title: crates/number_theory/modint/static_modint/src/numeric.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/static_modint/src/ops.rs
    title: crates/number_theory/modint/static_modint/src/ops.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/rerooting_tree_dp/src/lib.rs
    title: crates/tree/rerooting_tree_dp/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/tree_path_composite_sum
    links:
    - https://judge.yosupo.jp/problem/tree_path_composite_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/tree_path_composite_sum\n\
    \nuse graph::{GraphBuilder, UndirectedGraph};\nuse proconio::{fastout, input};\n\
    use rerooting_tree_dp::{RerootingTreeDp, TreeDpOperator};\nuse static_modint::ModInt998244353\
    \ as Mint;\n\nenum Op {}\nimpl TreeDpOperator for Op {\n    type Value = (Mint,\
    \ Mint);\n    type Vertex = Mint;\n    type Edge = (Mint, Mint);\n\n    fn unit()\
    \ -> Self::Value {\n        (0.into(), 0.into())\n    }\n\n    fn add_vertex(&(c,\
    \ s): &Self::Value, v: &Self::Vertex) -> Self::Value {\n        (c + 1, s + v)\n\
    \    }\n\n    fn add_edge(&(c, s): &Self::Value, &(a, b): &Self::Edge) -> Self::Value\
    \ {\n        (c, a * s + b * c)\n    }\n\n    fn rake(&(c1, s1): &Self::Value,\
    \ &(c2, s2): &Self::Value) -> Self::Value {\n        (c1 + c2, s1 + s2)\n    }\n\
    }\n\n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n        a: [Mint;\
    \ n],\n        edges: [(usize, usize, (Mint, Mint)); n - 1],\n    }\n    let g\
    \ = UndirectedGraph::from_edges(n, edges);\n    let dp = RerootingTreeDp::<Op>::with_vertices(&g,\
    \ &a);\n    for (_, x) in &dp {\n        print!(\"{} \", x);\n    }\n    println!();\n\
    }\n"
  dependsOn:
  - crates/graph/graph/src/builder.rs
  - crates/graph/graph/src/edge.rs
  - crates/graph/graph/src/lib.rs
  - crates/number_theory/modint/static_modint/src/lib.rs
  - crates/number_theory/modint/static_modint/src/mod_arithmetic.rs
  - crates/number_theory/modint/static_modint/src/ntt_precalc.rs
  - crates/number_theory/modint/static_modint/src/numeric.rs
  - crates/number_theory/modint/static_modint/src/ops.rs
  - crates/tree/rerooting_tree_dp/src/lib.rs
  isVerificationFile: true
  path: verify/library_checker/tree/tree_path_composite_sum/src/main.rs
  requiredBy: []
  timestamp: '2025-05-12 06:37:24+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/tree/tree_path_composite_sum/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/tree/tree_path_composite_sum/src/main.rs
- /verify/verify/library_checker/tree/tree_path_composite_sum/src/main.rs.html
title: verify/library_checker/tree/tree_path_composite_sum/src/main.rs
---
