---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/vertex_add_path_sum
    links:
    - https://judge.yosupo.jp/problem/vertex_add_path_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_path_sum\n\
    \nuse algebraic::{algebra, monoid};\nuse graph::Graph;\nuse proconio::input;\n\
    use tree_query::TreeQueryVertex;\n\nalgebra!(M, i64);\nmonoid!(M, 0, |x, y| x\
    \ + y);\n\n#[proconio::fastout]\nfn main() {\n    input! {\n        n: usize,\n\
    \        q: usize,\n        a: [i64; n],\n        uv: [(usize, usize); n - 1],\n\
    \    }\n    let g = Graph::from_vertices_and_unweighted_undirected_edges(&a, &uv);\n\
    \    let mut tq = TreeQueryVertex::<M>::build(&g);\n    for _ in 0..q {\n    \
    \    input! {\n            ty: usize,\n        }\n        if ty == 0 {\n     \
    \       input! {\n                p: usize,\n                x: i64,\n       \
    \     }\n            let t = tq.get(p);\n            tq.set(p, t + x);\n     \
    \   } else {\n            input! {\n                u: usize,\n              \
    \  v: usize,\n            }\n            let t = tq.prod_path(u, v);\n       \
    \     println!(\"{}\", t);\n        }\n    }\n}\n"
  dependsOn: []
  isVerificationFile: true
  path: verify/vertex_add_path_sum/src/main.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/vertex_add_path_sum/src/main.rs
layout: document
redirect_from:
- /verify/verify/vertex_add_path_sum/src/main.rs
- /verify/verify/vertex_add_path_sum/src/main.rs.html
title: verify/vertex_add_path_sum/src/main.rs
---
