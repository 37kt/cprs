---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/flow/min_cost_b_flow/src/lib.rs
    title: crates/flow/min_cost_b_flow/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/assignment
    links:
    - https://judge.yosupo.jp/problem/assignment
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/assignment\n\
    \nuse min_cost_b_flow::MinCostBFlow;\nuse proconio::{fastout, input};\n\n#[fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n    }\n\n    let mut mcf = MinCostBFlow::new();\n\
    \    let a = mcf.add_vertices(n).collect::<Vec<_>>();\n    let b = mcf.add_vertices(n).collect::<Vec<_>>();\n\
    \    for &v in &a {\n        mcf.add_supply(v, 1);\n    }\n    for &v in &b {\n\
    \        mcf.add_demand(v, 1);\n    }\n\n    let es = (0..n)\n        .map(|i|\
    \ {\n            (0..n)\n                .map(|j| {\n                    input!\
    \ {\n                        cost: i64,\n                    }\n             \
    \       mcf.add_edge(a[i], b[j], 0, 1, cost)\n                })\n           \
    \     .collect::<Vec<_>>()\n        })\n        .collect::<Vec<_>>();\n\n    let\
    \ res = mcf.min_cost_b_flow().unwrap();\n    println!(\"{}\", res);\n\n    for\
    \ i in 0..n {\n        for j in 0..n {\n            if mcf.edge(es[i][j]).flow\
    \ > 0 {\n                print!(\"{} \", j);\n            }\n        }\n    }\n\
    \    println!();\n}\n"
  dependsOn:
  - crates/flow/min_cost_b_flow/src/lib.rs
  isVerificationFile: true
  path: verify/library_checker/graph/assignment/src/main.rs
  requiredBy: []
  timestamp: '2025-03-15 09:42:03+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/graph/assignment/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/graph/assignment/src/main.rs
- /verify/verify/library_checker/graph/assignment/src/main.rs.html
title: verify/library_checker/graph/assignment/src/main.rs
---
