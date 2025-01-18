---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/dynamic-tree-dp/src/lib.rs
    title: crates/tree/dynamic-tree-dp/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/point_set_tree_path_composite_sum_fixed_root
    links:
    - https://judge.yosupo.jp/problem/point_set_tree_path_composite_sum_fixed_root
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_tree_path_composite_sum_fixed_root\n\
    \nuse dynamic_tree_dp::DynamicTreeDP;\nuse dynamic_tree_dp::DynamicTreeDPOperator;\n\
    use graph::UndirectedGraph;\nuse modint::ModInt998244353 as Mint;\nuse proconio::fastout;\n\
    use proconio::input;\n\n#[derive(Clone, Copy)]\nstruct S {\n    a: Mint,\n   \
    \ b: Mint,\n    cnt: Mint,\n    sum: Mint,\n}\n\nenum Op {}\nimpl DynamicTreeDPOperator\
    \ for Op {\n    type V = Mint;\n    type E = (Mint, Mint);\n    type X = S;\n\n\
    \    fn e() -> Self::X {\n        S {\n            a: 1.into(),\n            b:\
    \ 0.into(),\n            cnt: 0.into(),\n            sum: 0.into(),\n        }\n\
    \    }\n\n    fn single(&v: &Self::V, e: Option<&Self::E>) -> Self::X {\n    \
    \    let &(a, b) = e.unwrap_or(&(1.into(), 0.into()));\n        S {\n        \
    \    a,\n            b,\n            cnt: 1.into(),\n            sum: a * v +\
    \ b,\n        }\n    }\n\n    fn rake(&l: &Self::X, &r: &Self::X) -> Self::X {\n\
    \        S {\n            a: l.a,\n            b: l.b,\n            cnt: l.cnt\
    \ + r.cnt,\n            sum: l.sum + r.sum,\n        }\n    }\n\n    fn compress(&p:\
    \ &Self::X, &c: &Self::X) -> Self::X {\n        S {\n            a: p.a * c.a,\n\
    \            b: p.a * c.b + p.b,\n            cnt: p.cnt + c.cnt,\n          \
    \  sum: p.sum + p.a * c.sum + p.b * c.cnt,\n        }\n    }\n}\n\n#[fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        q: usize,\n        a: [Mint;\
    \ n],\n        uvbc: [(usize, usize, (Mint, Mint)); n - 1],\n    }\n    let g\
    \ = UndirectedGraph::from_vertices_and_edges(&a, &uvbc);\n    let mut dp = DynamicTreeDP::<Op>::new(&g,\
    \ 0);\n    for _ in 0..q {\n        input! {\n            t: usize,\n        }\n\
    \        if t == 0 {\n            input! {\n                w: usize,\n      \
    \          x: Mint,\n            }\n            dp.set_vertex(w, x);\n       \
    \ } else {\n            input! {\n                e: usize,\n                y:\
    \ Mint,\n                z: Mint,\n            }\n            let (u, v, _) =\
    \ uvbc[e];\n            dp.set_edge(u, v, (y, z));\n        }\n        let S {\
    \ sum, .. } = dp.prod();\n        println!(\"{}\", sum);\n    }\n}\n"
  dependsOn:
  - crates/graph/graph/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  - crates/tree/dynamic-tree-dp/src/lib.rs
  isVerificationFile: true
  path: verify/point_set_tree_path_composite_sum_fixed_root/src/main.rs
  requiredBy: []
  timestamp: '2025-01-18 04:01:37+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/point_set_tree_path_composite_sum_fixed_root/src/main.rs
layout: document
redirect_from:
- /verify/verify/point_set_tree_path_composite_sum_fixed_root/src/main.rs
- /verify/verify/point_set_tree_path_composite_sum_fixed_root/src/main.rs.html
title: verify/point_set_tree_path_composite_sum_fixed_root/src/main.rs
---
