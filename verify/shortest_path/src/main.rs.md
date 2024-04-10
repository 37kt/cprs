---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/graph/dijkstra/src/lib.rs
    title: crates/graph/dijkstra/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/shortest_path
    links:
    - https://judge.yosupo.jp/problem/shortest_path
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/shortest_path\n\
    \nuse dijkstra::dijkstra;\nuse graph::Graph;\nuse proconio::input;\n\n#[proconio::fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        m: usize,\n        s: usize,\n\
    \        t: usize,\n        abc: [(usize, usize, i64); m],\n    }\n    let g =\
    \ Graph::from_directed_edges(n, &abc);\n    let dijkstra_result = dijkstra(&g,\
    \ &[s], 1 << 60);\n    if let Some(path) = dijkstra_result.path(t) {\n       \
    \ println!(\"{} {}\", dijkstra_result.dist[t], path.len() - 1);\n        for i\
    \ in 0..path.len() - 1 {\n            println!(\"{} {}\", path[i], path[i + 1]);\n\
    \        }\n    } else {\n        println!(\"-1\");\n    }\n}\n"
  dependsOn:
  - crates/graph/dijkstra/src/lib.rs
  - crates/graph/graph/src/lib.rs
  isVerificationFile: true
  path: verify/shortest_path/src/main.rs
  requiredBy: []
  timestamp: '2024-04-10 09:38:39+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/shortest_path/src/main.rs
layout: document
redirect_from:
- /verify/verify/shortest_path/src/main.rs
- /verify/verify/shortest_path/src/main.rs.html
title: verify/shortest_path/src/main.rs
---