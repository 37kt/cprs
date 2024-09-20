---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/two_edge_connected_components
    links:
    - https://judge.yosupo.jp/problem/two_edge_connected_components
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/two_edge_connected_components\n\
    \nuse graph::Graph;\nuse itertools::Itertools;\nuse proconio::input;\nuse two_edge_connected_components::two_edge_connected_components;\n\
    \n#[proconio::fastout]\nfn main() {\n    input! {\n        n: usize,\n       \
    \ m: usize,\n        ab: [(usize, usize); m]\n    }\n    let g = Graph::from_unweighted_undirected_edges(n,\
    \ &ab);\n    let (groups, _) = two_edge_connected_components(&g);\n    println!(\"\
    {}\", groups.len());\n    for group in groups {\n        println!(\"{} {}\", group.len(),\
    \ group.iter().join(\" \"));\n    }\n}\n"
  dependsOn: []
  isVerificationFile: true
  path: verify/two_edge_connected_components/src/main.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/two_edge_connected_components/src/main.rs
layout: document
redirect_from:
- /verify/verify/two_edge_connected_components/src/main.rs
- /verify/verify/two_edge_connected_components/src/main.rs.html
title: verify/two_edge_connected_components/src/main.rs
---
