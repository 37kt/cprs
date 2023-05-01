---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/vertex_add_subtree_sum
    links:
    - https://judge.yosupo.jp/problem/vertex_add_subtree_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_subtree_sum\n\
    \nuse algebraic::{algebra, monoid};\nuse graph::Graph;\nuse proconio::input;\n\
    use tree_query::TreeQueryVertex;\n\nalgebra!(M, i64);\nmonoid!(M, 0, |x, y| x\
    \ + y);\n\n#[proconio::fastout]\nfn main() {\n    input! {\n        n: usize,\n\
    \        q: usize,\n        a: [i64; n],\n    }\n    let mut g = Graph::from(a);\n\
    \    for v in 1..n {\n        input! {\n            p: usize,\n        }\n   \
    \     g.add_undirected_edge(p, v, ());\n    }\n    let mut tq = TreeQueryVertex::<M>::build(&g);\n\
    \    for _ in 0..q {\n        input! {\n            ty: usize,\n        }\n  \
    \      if ty == 0 {\n            input! {\n                p: usize,\n       \
    \         x: i64\n            }\n            let t = tq.get(p);\n            tq.set(p,\
    \ t + x);\n        } else {\n            input! {\n                v: usize,\n\
    \            }\n            let t = tq.prod_subtree(v);\n            println!(\"\
    {}\", t);\n        }\n    }\n}\n"
  dependsOn: []
  isVerificationFile: true
  path: verify/vertex_add_subtree_sum/src/main.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: verify/vertex_add_subtree_sum/src/main.rs
layout: document
redirect_from:
- /verify/verify/vertex_add_subtree_sum/src/main.rs
- /verify/verify/vertex_add_subtree_sum/src/main.rs.html
title: verify/vertex_add_subtree_sum/src/main.rs
---
