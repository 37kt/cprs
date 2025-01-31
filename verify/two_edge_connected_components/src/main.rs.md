---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/two-edge-connected-components/src/lib.rs
    title: crates/graph/two-edge-connected-components/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/two_edge_connected_components
    links:
    - https://judge.yosupo.jp/problem/two_edge_connected_components
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/two_edge_connected_components\n\
    \nuse graph::UndirectedGraph;\nuse itertools::Itertools;\nuse proconio::input;\n\
    use two_edge_connected_components::two_edge_connected_components;\n\n#[proconio::fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        m: usize,\n        ab: [(usize,\
    \ usize); m]\n    }\n    let g = UndirectedGraph::from_unweighted_edges(n, &ab);\n\
    \    let (groups, _) = two_edge_connected_components(&g);\n    println!(\"{}\"\
    , groups.len());\n    for group in groups.iter() {\n        println!(\"{} {}\"\
    , group.len(), group.iter().join(\" \"));\n    }\n}\n"
  dependsOn:
  - crates/graph/graph/src/lib.rs
  - crates/graph/two-edge-connected-components/src/lib.rs
  isVerificationFile: true
  path: verify/two_edge_connected_components/src/main.rs
  requiredBy: []
  timestamp: '2025-01-14 05:59:50+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/two_edge_connected_components/src/main.rs
layout: document
redirect_from:
- /verify/verify/two_edge_connected_components/src/main.rs
- /verify/verify/two_edge_connected_components/src/main.rs.html
title: verify/two_edge_connected_components/src/main.rs
---
