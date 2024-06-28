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
    path: crates/tree/static-top-tree-dp/src/lib.rs
    title: crates/tree/static-top-tree-dp/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/point_set_tree_path_composite_sum_fixed_root
    links:
    - https://judge.yosupo.jp/problem/point_set_tree_path_composite_sum_fixed_root
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.4/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.4/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_tree_path_composite_sum_fixed_root\n\
    \nuse graph::Graph;\nuse modint::ModInt998244353 as Mint;\nuse proconio::input;\n\
    use static_top_tree_dp::{StaticTopTreeDP, TreeDPOperator};\n\nenum Op {}\nimpl\
    \ TreeDPOperator for Op {\n    type Path = (Mint, Mint, Mint, Mint);\n    type\
    \ Point = (Mint, Mint);\n    type V = Mint;\n    type E = (Mint, Mint);\n\n  \
    \  fn vertex(&v: &Self::V) -> Self::Path {\n        (v, 1.into(), 1.into(), 0.into())\n\
    \    }\n\n    fn compress(p: &Self::Path, c: &Self::Path, e: &Self::E) -> Self::Path\
    \ {\n        let &(ps, pn, pa, pb) = p;\n        let &(cs, cn, ca, cb) = c;\n\
    \        let &(a, b) = e;\n        let (cs, cn, ca, cb) = (cs * a + cn * b, cn,\
    \ ca * a, cb * a + b);\n        (ps + cs * pa + cn * pb, pn + cn, pa * ca, pa\
    \ * cb + pb)\n    }\n\n    fn rake(l: &Self::Point, r: &Self::Point) -> Self::Point\
    \ {\n        let &(ls, ln) = l;\n        let &(rs, rn) = r;\n        (ls + rs,\
    \ ln + rn)\n    }\n\n    fn add_edge(d: &Self::Path, e: &Self::E) -> Self::Point\
    \ {\n        let &(s, n, _, _) = d;\n        let &(a, b) = e;\n        (s * a\
    \ + n * b, n)\n    }\n\n    fn add_vertex(d: &Self::Point, v: &Self::V) -> Self::Path\
    \ {\n        let &(s, n) = d;\n        (s + v, n + 1, 1.into(), 0.into())\n  \
    \  }\n}\n\n#[proconio::fastout]\nfn main() {\n    input! {\n        n: usize,\n\
    \        q: usize,\n        a: [Mint; n],\n        uvbc: [(usize, usize, (Mint,\
    \ Mint)); n - 1],\n    }\n    let g = Graph::from_vertices_and_undirected_edges(&a,\
    \ &uvbc);\n    let mut dp = StaticTopTreeDP::<Op>::new(&g);\n    for _ in 0..q\
    \ {\n        input! {\n            t: usize,\n        }\n        if t == 0 {\n\
    \            input! {\n                w: usize,\n                x: Mint,\n \
    \           }\n            dp.set_vertex(w, x);\n        } else {\n          \
    \  input! {\n                e: usize,\n                y: Mint,\n           \
    \     z: Mint,\n            }\n            dp.set_edge(e, (y, z));\n        }\n\
    \        println!(\"{}\", dp.prod().0);\n    }\n}\n"
  dependsOn:
  - crates/graph/graph/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  - crates/tree/static-top-tree-dp/src/lib.rs
  isVerificationFile: true
  path: verify/point_set_tree_path_composite_sum_fixed_root/src/main.rs
  requiredBy: []
  timestamp: '2024-06-13 08:47:29+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/point_set_tree_path_composite_sum_fixed_root/src/main.rs
layout: document
redirect_from:
- /verify/verify/point_set_tree_path_composite_sum_fixed_root/src/main.rs
- /verify/verify/point_set_tree_path_composite_sum_fixed_root/src/main.rs.html
title: verify/point_set_tree_path_composite_sum_fixed_root/src/main.rs
---
