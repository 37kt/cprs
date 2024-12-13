---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/graph/min-cost-b-flow/src/lib.rs
    title: crates/graph/min-cost-b-flow/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/assignment
    links:
    - https://judge.yosupo.jp/problem/assignment
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/assignment\n\
    \nuse itertools::Itertools;\nuse min_cost_b_flow::MinCostBFlow;\nuse proconio::input;\n\
    \n#[proconio::fastout]\nfn main() {\n    input! {\n        n: usize,\n    }\n\
    \    let mut mcf = MinCostBFlow::new();\n    let a = mcf.add_vertices(n);\n  \
    \  let b = mcf.add_vertices(n);\n    let mut es = vec![vec![0; n]; n];\n    for\
    \ &v in &a {\n        mcf.add_supply(v, 1);\n    }\n    for &v in &b {\n     \
    \   mcf.add_demand(v, 1);\n    }\n    for i in 0..n {\n        for j in 0..n {\n\
    \            input! {\n                cost: i64,\n            }\n           \
    \ es[i][j] = mcf.add_edge(a[i], b[j], 0, 1, cost);\n        }\n    }\n    let\
    \ value = mcf.min_cost_b_flow().unwrap();\n    println!(\"{}\", value);\n    let\
    \ mut res = vec![];\n    for i in 0..n {\n        for j in 0..n {\n          \
    \  if mcf.get_edge(es[i][j]).flow > 0 {\n                res.push(j);\n      \
    \      }\n        }\n    }\n    println!(\"{}\", res.iter().join(\" \"));\n}\n"
  dependsOn:
  - crates/graph/min-cost-b-flow/src/lib.rs
  isVerificationFile: true
  path: verify/assignment/src/main.rs
  requiredBy: []
  timestamp: '2023-05-23 15:04:49+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/assignment/src/main.rs
layout: document
redirect_from:
- /verify/verify/assignment/src/main.rs
- /verify/verify/assignment/src/main.rs.html
title: verify/assignment/src/main.rs
---
