---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/strongly-connected-components/src/lib.rs
    title: crates/graph/strongly-connected-components/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/scc
    links:
    - https://judge.yosupo.jp/problem/scc
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.1/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.1/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/scc\n\nuse\
    \ graph::Graph;\nuse itertools::Itertools;\nuse proconio::input;\nuse strongly_connected_components::strongly_connected_components;\n\
    \n#[proconio::fastout]\nfn main() {\n    input! {\n        n: usize,\n       \
    \ m: usize,\n    }\n    let mut g = Graph::<(), ()>::new(n);\n    for _ in 0..m\
    \ {\n        input! {\n            a: usize,\n            b: usize,\n        }\n\
    \        g.add_edge(a, b, ());\n    }\n    let h = strongly_connected_components(&g);\n\
    \    println!(\"{}\", h.size());\n    for i in 0..h.size() {\n        let vs =\
    \ &h.vertices()[i];\n        println!(\"{} {}\", vs.len(), vs.iter().join(\" \"\
    ));\n    }\n}\n"
  dependsOn:
  - crates/graph/graph/src/lib.rs
  - crates/graph/strongly-connected-components/src/lib.rs
  isVerificationFile: true
  path: verify/scc/src/main.rs
  requiredBy: []
  timestamp: '2023-05-17 16:30:46+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/scc/src/main.rs
layout: document
redirect_from:
- /verify/verify/scc/src/main.rs
- /verify/verify/scc/src/main.rs.html
title: verify/scc/src/main.rs
---
