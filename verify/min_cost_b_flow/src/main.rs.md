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
    PROBLEM: https://judge.yosupo.jp/problem/min_cost_b_flow
    links:
    - https://judge.yosupo.jp/problem/min_cost_b_flow
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.3/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.3/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/min_cost_b_flow\n\
    \nuse itertools::Itertools;\nuse min_cost_b_flow::MinCostBFlow;\nuse proconio::input;\n\
    \n#[proconio::fastout]\nfn main() {\n    input! {\n        n: usize,\n       \
    \ m: usize,\n    }\n    let mut mcf = MinCostBFlow::new();\n    let vs = mcf.add_vertices(n);\n\
    \    for &v in &vs {\n        input! {\n            b: i64,\n        }\n     \
    \   mcf.add_supply(v, b);\n    }\n    let mut es = vec![];\n    for _ in 0..m\
    \ {\n        input! {\n            s: usize,\n            t: usize,\n        \
    \    lower: i64,\n            upper: i64,\n            cost: i64,\n        }\n\
    \        es.push(mcf.add_edge(s, t, lower, upper, cost));\n    }\n    if let Ok(_)\
    \ = mcf.min_cost_b_flow() {\n        let z = mcf.get_result_value_i128();\n  \
    \      let f = mcf.get_edges().iter().map(|e| e.flow).collect::<Vec<_>>();\n \
    \       let p = mcf.get_potential();\n        println!(\"{}\", z);\n        if\
    \ p.len() > 0 {\n            println!(\"{}\", p.iter().join(\"\\n\"));\n     \
    \   }\n        if f.len() > 0 {\n            println!(\"{}\", f.iter().join(\"\
    \\n\"));\n        }\n    } else {\n        println!(\"infeasible\");\n    }\n\
    }\n"
  dependsOn:
  - crates/graph/min-cost-b-flow/src/lib.rs
  isVerificationFile: true
  path: verify/min_cost_b_flow/src/main.rs
  requiredBy: []
  timestamp: '2023-05-23 15:04:49+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/min_cost_b_flow/src/main.rs
layout: document
redirect_from:
- /verify/verify/min_cost_b_flow/src/main.rs
- /verify/verify/min_cost_b_flow/src/main.rs.html
title: verify/min_cost_b_flow/src/main.rs
---
