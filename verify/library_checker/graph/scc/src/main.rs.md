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
    path: crates/graph/strongly_connected_components/src/lib.rs
    title: crates/graph/strongly_connected_components/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/scc
    links:
    - https://judge.yosupo.jp/problem/scc
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/scc\n\nuse\
    \ graph::{DirectedGraph, GraphBuilder};\nuse proconio::{fastout, input};\nuse\
    \ strongly_connected_components::StronglyConnectedComponents;\n\n#[fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        m: usize,\n        ab: [(usize,\
    \ usize); m],\n    }\n\n    let g = DirectedGraph::from_edges(n, &ab);\n    let\
    \ scc = StronglyConnectedComponents::new(&g);\n    let groups = scc.groups;\n\n\
    \    println!(\"{}\", groups.len());\n    for g in &groups {\n        print!(\"\
    {}\", g.len());\n        for v in g {\n            print!(\" {}\", v);\n     \
    \   }\n        println!();\n    }\n}\n"
  dependsOn:
  - crates/graph/graph/src/builder.rs
  - crates/graph/graph/src/edge.rs
  - crates/graph/graph/src/lib.rs
  - crates/graph/strongly_connected_components/src/lib.rs
  isVerificationFile: true
  path: verify/library_checker/graph/scc/src/main.rs
  requiredBy: []
  timestamp: '2025-03-06 00:54:38+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/graph/scc/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/graph/scc/src/main.rs
- /verify/verify/library_checker/graph/scc/src/main.rs.html
title: verify/library_checker/graph/scc/src/main.rs
---
