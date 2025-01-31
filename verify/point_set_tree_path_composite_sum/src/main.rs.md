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
    path: crates/tree/dynamic-rerooting-tree-dp/src/lib.rs
    title: crates/tree/dynamic-rerooting-tree-dp/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/point_set_tree_path_composite_sum
    links:
    - https://judge.yosupo.jp/problem/point_set_tree_path_composite_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_tree_path_composite_sum\n\
    \nuse dynamic_rerooting_tree_dp::DynamicRerootingTreeDP;\nuse dynamic_rerooting_tree_dp::DynamicRerootingTreeDPOperator;\n\
    use graph::UndirectedGraph;\nuse modint::ModInt998244353 as Mint;\nuse proconio::fastout;\n\
    use proconio::input;\n\n#[derive(Clone, Copy)]\nstruct S {\n    a: Mint,\n   \
    \ b: Mint,\n    cnt: Mint,\n    sum: Mint,\n}\n\nenum Op {}\nimpl DynamicRerootingTreeDPOperator\
    \ for Op {\n    type V = Mint;\n    type E = (Mint, Mint);\n    type X = S;\n\n\
    \    fn e() -> Self::X {\n        S {\n            a: 1.into(),\n            b:\
    \ 0.into(),\n            cnt: 0.into(),\n            sum: 0.into(),\n        }\n\
    \    }\n\n    fn single(&v: &Self::V, e: Option<&Self::E>) -> (Self::X, Self::X)\
    \ {\n        let &(a, b) = e.unwrap_or(&(1.into(), 0.into()));\n        (\n  \
    \          S {\n                a,\n                b,\n                cnt: 1.into(),\n\
    \                sum: a * v + b,\n            },\n            S {\n          \
    \      a,\n                b,\n                cnt: 1.into(),\n              \
    \  sum: v,\n            },\n        )\n    }\n\n    fn rake(&l: &Self::X, &r:\
    \ &Self::X) -> Self::X {\n        S {\n            a: l.a,\n            b: l.b,\n\
    \            cnt: l.cnt + r.cnt,\n            sum: l.sum + r.sum,\n        }\n\
    \    }\n\n    fn rake2(&l: &Self::X, &r: &Self::X) -> Self::X {\n        S {\n\
    \            a: l.a,\n            b: l.b,\n            cnt: l.cnt + r.cnt,\n \
    \           sum: l.sum + l.a * r.sum + l.b * r.cnt,\n        }\n    }\n\n    fn\
    \ rake3(p: &Self::X, c: &Self::X) -> Self::X {\n        Self::rake(p, c)\n   \
    \ }\n\n    fn compress(&p: &Self::X, &c: &Self::X) -> Self::X {\n        S {\n\
    \            a: p.a * c.a,\n            b: p.a * c.b + p.b,\n            cnt:\
    \ p.cnt + c.cnt,\n            sum: p.sum + p.a * c.sum + p.b * c.cnt,\n      \
    \  }\n    }\n\n    fn compress2(p: &Self::X, c: &Self::X) -> Self::X {\n     \
    \   Self::compress(c, p)\n    }\n}\n\n#[fastout]\nfn main() {\n    input! {\n\
    \        n: usize,\n        q: usize,\n        a: [Mint; n],\n        uvbc: [(usize,\
    \ usize, (Mint, Mint)); n - 1],\n    }\n    let g = UndirectedGraph::from_vertices_and_edges(&a,\
    \ &uvbc);\n    let mut dp = DynamicRerootingTreeDP::<Op>::new(&g, 0);\n    for\
    \ _ in 0..q {\n        input! {\n            t: usize,\n        }\n        if\
    \ t == 0 {\n            input! {\n                v: usize,\n                x:\
    \ Mint,\n            }\n            dp.set_vertex(v, x);\n        } else {\n \
    \           input! {\n                e: usize,\n                a: Mint,\n  \
    \              b: Mint,\n            }\n            let (u, v, _) = uvbc[e];\n\
    \            dp.set_edge(u, v, (a, b));\n        }\n\n        input! {\n     \
    \       r: usize,\n        }\n        let S { sum, .. } = dp.prod(r);\n      \
    \  println!(\"{}\", sum);\n    }\n}\n"
  dependsOn:
  - crates/graph/graph/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  - crates/tree/dynamic-rerooting-tree-dp/src/lib.rs
  isVerificationFile: true
  path: verify/point_set_tree_path_composite_sum/src/main.rs
  requiredBy: []
  timestamp: '2025-01-18 04:01:37+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/point_set_tree_path_composite_sum/src/main.rs
layout: document
redirect_from:
- /verify/verify/point_set_tree_path_composite_sum/src/main.rs
- /verify/verify/point_set_tree_path_composite_sum/src/main.rs.html
title: verify/point_set_tree_path_composite_sum/src/main.rs
---
