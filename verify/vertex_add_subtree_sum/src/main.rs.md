---
data:
  _extendedDependsOn:
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
    PROBLEM: https://judge.yosupo.jp/problem/vertex_add_subtree_sum
    links:
    - https://judge.yosupo.jp/problem/vertex_add_subtree_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_subtree_sum\n\
    \nuse ac_library::Monoid;\nuse graph::Graph;\nuse proconio::input;\nuse tree_query::TreeQueryVertex;\n\
    \nenum M {}\nimpl Monoid for M {\n    type S = i64;\n    fn identity() -> Self::S\
    \ {\n        0\n    }\n    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S\
    \ {\n        a + b\n    }\n}\n\n#[proconio::fastout]\nfn main() {\n    input!\
    \ {\n        n: usize,\n        q: usize,\n        a: [i64; n],\n    }\n    let\
    \ mut g = Graph::from(a);\n    for v in 1..n {\n        input! {\n           \
    \ p: usize,\n        }\n        g.add_undirected_edge(p, v, ());\n    }\n    let\
    \ mut tq = TreeQueryVertex::<M>::build(&g);\n    for _ in 0..q {\n        input!\
    \ {\n            ty: usize,\n        }\n        if ty == 0 {\n            input!\
    \ {\n                p: usize,\n                x: i64\n            }\n      \
    \      let t = tq.get(p);\n            tq.set(p, t + x);\n        } else {\n \
    \           input! {\n                v: usize,\n            }\n            let\
    \ t = tq.prod_subtree(v);\n            println!(\"{}\", t);\n        }\n    }\n\
    }\n"
  dependsOn:
  - crates/data-structure/tree-query/src/lib.rs
  - crates/graph/graph/src/lib.rs
  isVerificationFile: true
  path: verify/vertex_add_subtree_sum/src/main.rs
  requiredBy: []
  timestamp: '2023-04-21 11:20:46+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/vertex_add_subtree_sum/src/main.rs
layout: document
redirect_from:
- /verify/verify/vertex_add_subtree_sum/src/main.rs
- /verify/verify/vertex_add_subtree_sum/src/main.rs.html
title: verify/vertex_add_subtree_sum/src/main.rs
---
