---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/heavy-light-decomposition/src/lib.rs
    title: crates/data-structure/heavy-light-decomposition/src/lib.rs
  - icon: ':question:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/lca
    links:
    - https://judge.yosupo.jp/problem/lca
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/lca\n\nuse\
    \ graph::Graph;\nuse heavy_light_decomposition::HeavyLightDecomposition;\nuse\
    \ proconio::input;\n\n#[proconio::fastout]\nfn main() {\n    input! {\n      \
    \  n: usize,\n        q: usize,\n    }\n    let mut g = Graph::<(), ()>::new(n);\n\
    \    for v in 1..n {\n        input! {\n            p: usize,\n        }\n   \
    \     g.add_undirected_edge(v, p, ());\n    }\n    let hld = HeavyLightDecomposition::new(&g);\n\
    \    for _ in 0..q {\n        input! {\n            u: usize,\n            v:\
    \ usize,\n        }\n        println!(\"{}\", hld.lca(u, v));\n    }\n}\n"
  dependsOn:
  - crates/data-structure/heavy-light-decomposition/src/lib.rs
  - crates/graph/graph/src/lib.rs
  isVerificationFile: true
  path: verify/lca/src/main.rs
  requiredBy: []
  timestamp: '2023-04-24 12:50:05+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/lca/src/main.rs
layout: document
redirect_from:
- /verify/verify/lca/src/main.rs
- /verify/verify/lca/src/main.rs.html
title: verify/lca/src/main.rs
---
