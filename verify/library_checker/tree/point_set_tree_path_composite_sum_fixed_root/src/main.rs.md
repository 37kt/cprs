---
data:
  _extendedDependsOn:
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
    path: crates/tree/dynamic_tree_dp/src/lib.rs
    title: crates/tree/dynamic_tree_dp/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/point_set_tree_path_composite_sum_fixed_root
    links:
    - https://judge.yosupo.jp/problem/point_set_tree_path_composite_sum_fixed_root
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_tree_path_composite_sum_fixed_root\n\
    \nuse dynamic_tree_dp::{DynamicTreeDp, DynamicTreeDpOperator};\nuse proconio::{fastout,\
    \ input};\nuse static_modint::ModInt998244353 as Mint;\n\n#[derive(Clone, Copy)]\n\
    struct S {\n    a: Mint,\n    b: Mint,\n    cnt: Mint,\n    sum: Mint,\n}\n\n\
    enum Op {}\nimpl DynamicTreeDpOperator for Op {\n    type Value = S;\n    type\
    \ Vertex = Mint;\n    type Edge = (Mint, Mint);\n\n    fn unit() -> Self::Value\
    \ {\n        S {\n            a: 1.into(),\n            b: 0.into(),\n       \
    \     cnt: 0.into(),\n            sum: 0.into(),\n        }\n    }\n\n    fn add_vertex(x:\
    \ &Self::Value, v: &Self::Vertex) -> Self::Value {\n        S {\n            a:\
    \ x.a,\n            b: x.b,\n            cnt: x.cnt + 1,\n            sum: x.sum\
    \ + v,\n        }\n    }\n\n    fn add_edge(x: &Self::Value, e: &Self::Edge) ->\
    \ Self::Value {\n        S {\n            a: e.0,\n            b: e.1,\n     \
    \       cnt: 1.into(),\n            sum: e.0 * x.sum + e.1 * x.cnt,\n        }\n\
    \    }\n\n    fn rake(l: &Self::Value, r: &Self::Value) -> Self::Value {\n   \
    \     S {\n            a: l.a,\n            b: l.b,\n            cnt: l.cnt +\
    \ r.cnt,\n            sum: l.sum + r.sum,\n        }\n    }\n\n    fn compress(p:\
    \ &Self::Value, c: &Self::Value) -> Self::Value {\n        S {\n            a:\
    \ p.a * c.a,\n            b: p.a * c.b + p.b,\n            cnt: p.cnt + c.cnt,\n\
    \            sum: p.sum + p.a * c.sum + p.b * c.cnt,\n        }\n    }\n}\n\n\
    #[fastout]\nfn main() {\n    input! {\n        n: usize,\n        q: usize,\n\
    \        a: [Mint; n],\n        edges: [(usize, usize, (Mint, Mint)); n - 1],\n\
    \    }\n    let mut dp = DynamicTreeDp::<Op>::with_vertices(&edges, &a, 0);\n\
    \    for _ in 0..q {\n        input! {\n            ty: usize,\n        }\n  \
    \      match ty {\n            0 => {\n                input! {\n            \
    \        v: usize,\n                    x: Mint,\n                }\n        \
    \        dp.set_vertex(v, x);\n            }\n            1 => {\n           \
    \     input! {\n                    e: usize,\n                    a: Mint,\n\
    \                    b: Mint,\n                }\n                let (u, v, _)\
    \ = edges[e];\n                dp.set_edge(u, v, (a, b));\n            }\n   \
    \         _ => unreachable!(),\n        }\n\n        println!(\"{}\", dp.fold().sum);\n\
    \    }\n}\n"
  dependsOn:
  - crates/number_theory/modint/static_modint/src/lib.rs
  - crates/number_theory/modint/static_modint/src/mod_arithmetic.rs
  - crates/number_theory/modint/static_modint/src/ntt_precalc.rs
  - crates/number_theory/modint/static_modint/src/numeric.rs
  - crates/number_theory/modint/static_modint/src/ops.rs
  - crates/tree/dynamic_tree_dp/src/lib.rs
  isVerificationFile: true
  path: verify/library_checker/tree/point_set_tree_path_composite_sum_fixed_root/src/main.rs
  requiredBy: []
  timestamp: '2025-03-27 07:31:57+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/tree/point_set_tree_path_composite_sum_fixed_root/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/tree/point_set_tree_path_composite_sum_fixed_root/src/main.rs
- /verify/verify/library_checker/tree/point_set_tree_path_composite_sum_fixed_root/src/main.rs.html
title: verify/library_checker/tree/point_set_tree_path_composite_sum_fixed_root/src/main.rs
---
