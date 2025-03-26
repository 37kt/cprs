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
    path: crates/graph/two_edge_connected_components/src/lib.rs
    title: crates/graph/two_edge_connected_components/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/two_edge_connected_components
    links:
    - https://judge.yosupo.jp/problem/two_edge_connected_components
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/two_edge_connected_components\n\
    \nuse graph::{GraphBuilder, UndirectedGraph};\nuse proconio::{fastout, input};\n\
    use two_edge_connected_components::TwoEdgeConnectedComponents;\n\n#[fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        m: usize,\n        ab: [(usize,\
    \ usize); m],\n    }\n\n    let g = UndirectedGraph::from_edges(n, &ab);\n   \
    \ let tecc = TwoEdgeConnectedComponents::new(&g);\n\n    println!(\"{}\", tecc.groups.len());\n\
    \    for g in &tecc.groups {\n        print!(\"{}\", g.len());\n        for v\
    \ in g {\n            print!(\" {}\", v);\n        }\n        println!();\n  \
    \  }\n}\n"
  dependsOn:
  - crates/graph/graph/src/builder.rs
  - crates/graph/graph/src/edge.rs
  - crates/graph/graph/src/lib.rs
  - crates/graph/two_edge_connected_components/src/lib.rs
  isVerificationFile: true
  path: verify/library_checker/graph/two_edge_connected_components/src/main.rs
  requiredBy: []
  timestamp: '2025-03-26 02:28:07+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/graph/two_edge_connected_components/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/graph/two_edge_connected_components/src/main.rs
- /verify/verify/library_checker/graph/two_edge_connected_components/src/main.rs.html
title: verify/library_checker/graph/two_edge_connected_components/src/main.rs
---
