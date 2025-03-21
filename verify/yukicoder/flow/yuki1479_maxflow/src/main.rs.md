---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/flow/max_flow/src/graph.rs
    title: crates/flow/max_flow/src/graph.rs
  - icon: ':heavy_check_mark:'
    path: crates/flow/max_flow/src/lib.rs
    title: crates/flow/max_flow/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/flow/max_flow/src/queue.rs
    title: crates/flow/max_flow/src/queue.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://yukicoder.me/problems/no/1479
    links:
    - https://yukicoder.me/problems/no/1479
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://yukicoder.me/problems/no/1479\n\n\
    use max_flow::MaxFlow;\nuse proconio::{fastout, input};\n\nconst M: usize = 500_001;\n\
    \n#[fastout]\nfn main() {\n    input! {\n        h: usize,\n        w: usize,\n\
    \        a: [[usize; w]; h],\n    }\n    let mut zh = vec![vec![]; M];\n    let\
    \ mut zw = vec![vec![]; M];\n    let mut b = vec![vec![]; M];\n    for i in 0..h\
    \ {\n        for j in 0..w {\n            zh[a[i][j]].push(i);\n            zw[a[i][j]].push(j);\n\
    \            b[a[i][j]].push((i, j));\n        }\n    }\n    for i in 0..M {\n\
    \        zh[i].sort_unstable();\n        zh[i].dedup();\n        zw[i].sort_unstable();\n\
    \        zw[i].dedup();\n    }\n    let mut res = 0;\n    for x in 1..M {\n  \
    \      if zh[x].is_empty() {\n            continue;\n        }\n        let mut\
    \ g = MaxFlow::new();\n        let ls = g.add_vertices(zh[x].len());\n       \
    \ let rs = g.add_vertices(zw[x].len());\n        let src = g.add_vertex();\n \
    \       let dst = g.add_vertex();\n        for &v in &ls {\n            g.add_edge(src,\
    \ v, 1);\n        }\n        for &v in &rs {\n            g.add_edge(v, dst, 1);\n\
    \        }\n        for &(i, j) in &b[x] {\n            let p = zh[x].binary_search(&i).unwrap();\n\
    \            let q = zw[x].binary_search(&j).unwrap();\n            g.add_edge(ls[p],\
    \ rs[q], 1);\n        }\n        res += g.max_flow(src, dst, None);\n    }\n \
    \   println!(\"{}\", res);\n}\n"
  dependsOn:
  - crates/flow/max_flow/src/graph.rs
  - crates/flow/max_flow/src/lib.rs
  - crates/flow/max_flow/src/queue.rs
  isVerificationFile: true
  path: verify/yukicoder/flow/yuki1479_maxflow/src/main.rs
  requiredBy: []
  timestamp: '2025-03-18 02:28:18+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yukicoder/flow/yuki1479_maxflow/src/main.rs
layout: document
redirect_from:
- /verify/verify/yukicoder/flow/yuki1479_maxflow/src/main.rs
- /verify/verify/yukicoder/flow/yuki1479_maxflow/src/main.rs.html
title: verify/yukicoder/flow/yuki1479_maxflow/src/main.rs
---
