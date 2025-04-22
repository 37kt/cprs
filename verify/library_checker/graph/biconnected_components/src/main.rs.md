---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/graph/extended_block_cut_tree/src/lib.rs
    title: crates/graph/extended_block_cut_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/builder.rs
    title: crates/graph/graph/src/builder.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/edge.rs
    title: crates/graph/graph/src/edge.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/biconnected_components
    links:
    - https://judge.yosupo.jp/problem/biconnected_components
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/biconnected_components\n\
    \nuse extended_block_cut_tree::extended_block_cut_tree;\nuse graph::{GraphBuilder,\
    \ UndirectedGraph};\nuse proconio::{fastout, input};\n\n#[fastout]\nfn main()\
    \ {\n    input! {\n        n: usize,\n        m: usize,\n        ab: [(usize,\
    \ usize); m],\n    }\n\n    let g = UndirectedGraph::from_edges(n, &ab);\n   \
    \ let ebct = extended_block_cut_tree(&g);\n    println!(\"{}\", ebct.len() - n);\n\
    \    for g in ebct.iter().skip(n) {\n        print!(\"{}\", g.len());\n      \
    \  for v in g {\n            print!(\" {}\", v);\n        }\n        println!();\n\
    \    }\n}\n"
  dependsOn:
  - crates/graph/extended_block_cut_tree/src/lib.rs
  - crates/graph/graph/src/builder.rs
  - crates/graph/graph/src/edge.rs
  - crates/graph/graph/src/lib.rs
  isVerificationFile: true
  path: verify/library_checker/graph/biconnected_components/src/main.rs
  requiredBy: []
  timestamp: '2025-03-26 03:07:13+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/graph/biconnected_components/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/graph/biconnected_components/src/main.rs
- /verify/verify/library_checker/graph/biconnected_components/src/main.rs.html
title: verify/library_checker/graph/biconnected_components/src/main.rs
---
