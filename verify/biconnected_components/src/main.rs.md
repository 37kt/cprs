---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/graph/extended-block-cut-tree/src/lib.rs
    title: crates/graph/extended-block-cut-tree/src/lib.rs
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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/biconnected_components\n\
    \nuse extended_block_cut_tree::extended_block_cut_tree;\nuse graph::Graph;\nuse\
    \ itertools::Itertools;\nuse proconio::input;\n\n#[proconio::fastout]\nfn main()\
    \ {\n    input! {\n        n: usize,\n        m: usize,\n        ab: [(usize,\
    \ usize); m]\n    }\n    let g = Graph::from_unweighted_undirected_edges(n, &ab);\n\
    \    let bct = extended_block_cut_tree(&g);\n    println!(\"{}\", bct.len() -\
    \ n);\n    for i in n..bct.len() {\n        println!(\n            \"{} {}\",\n\
    \            bct[i].len(),\n            bct[i].iter().map(|&(i, _)| i).join(\"\
    \ \")\n        );\n    }\n}\n"
  dependsOn:
  - crates/graph/extended-block-cut-tree/src/lib.rs
  - crates/graph/graph/src/lib.rs
  isVerificationFile: true
  path: verify/biconnected_components/src/main.rs
  requiredBy: []
  timestamp: '2024-04-10 09:38:39+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/biconnected_components/src/main.rs
layout: document
redirect_from:
- /verify/verify/biconnected_components/src/main.rs
- /verify/verify/biconnected_components/src/main.rs.html
title: verify/biconnected_components/src/main.rs
---
