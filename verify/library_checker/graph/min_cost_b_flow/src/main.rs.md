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
    PROBLEM: https://judge.yosupo.jp/problem/min_cost_b_flow
    links:
    - https://judge.yosupo.jp/problem/min_cost_b_flow
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/min_cost_b_flow\n\
    \nuse min_cost_b_flow::MinCostBFlow;\nuse proconio::{fastout, input};\n\n#[fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        m: usize,\n    }\n\n   \
    \ let mut mcf = MinCostBFlow::new();\n    let vs = mcf.add_vertices(n).collect::<Vec<_>>();\n\
    \n    for &v in &vs {\n        input! {\n            b: i64,\n        }\n    \
    \    mcf.add_supply(v, b);\n    }\n\n    let es = (0..m)\n        .map(|_| {\n\
    \            input! {\n                s: usize,\n                t: usize,\n\
    \                lower: i64,\n                upper: i64,\n                cost:\
    \ i64,\n            }\n            mcf.add_edge(vs[s], vs[t], lower, upper, cost)\n\
    \        })\n        .collect::<Vec<_>>();\n\n    if let Ok(z) = mcf.min_cost_b_flow()\
    \ {\n        println!(\"{}\", z);\n\n        let potentials = mcf.potentials();\n\
    \        for v in vs {\n            println!(\"{}\", potentials[v]);\n       \
    \ }\n\n        let edges = mcf.edges().collect::<Vec<_>>();\n        for e in\
    \ es {\n            println!(\"{}\", edges[e].flow);\n        }\n    } else {\n\
    \        println!(\"infeasible\");\n    }\n}\n"
  dependsOn:
  - crates/flow/min_cost_b_flow/src/lib.rs
  isVerificationFile: true
  path: verify/library_checker/graph/min_cost_b_flow/src/main.rs
  requiredBy: []
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/graph/min_cost_b_flow/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/graph/min_cost_b_flow/src/main.rs
- /verify/verify/library_checker/graph/min_cost_b_flow/src/main.rs.html
title: verify/library_checker/graph/min_cost_b_flow/src/main.rs
---
