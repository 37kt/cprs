---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/range-edge-graph/src/lib.rs
    title: crates/graph/range-edge-graph/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/macros/chminmax/src/lib.rs
    title: crates/macros/chminmax/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://yukicoder.me/problems/no/1014
    links:
    - https://yukicoder.me/problems/no/1014
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://yukicoder.me/problems/no/1014\n\n\
    use chminmax::{chmax, chmin};\nuse graph::Graph;\nuse proconio::input;\nuse range_edge_graph::RangeEdgeGraph;\n\
    \nconst INF: i64 = 1 << 60;\n\nfn dfs(v: usize, g: &Graph<(), i64>, res: &mut\
    \ [Option<i64>], b: &[i64]) -> i64 {\n    if let Some(res) = res[v] {\n      \
    \  return res;\n    }\n    res[v] = Some(INF);\n    let w = if v < b.len() { b[v]\
    \ } else { 0 };\n    let mut r = w;\n    for &(u, _) in &g[v] {\n        chmax!(r,\
    \ dfs(u, g, res, b) + w);\n        chmin!(r, INF);\n    }\n    res[v] = Some(r);\n\
    \    r\n}\n\nfn main() {\n    input! {\n        n: usize,\n        abc: [(i64,\
    \ i64, i64); n],\n    }\n    let mut ord: Vec<usize> = (0..n).collect();\n   \
    \ ord.sort_by_key(|&i| abc[i].0);\n    let mut rev = vec![0; n];\n    let mut\
    \ bs = vec![0; n];\n    for i in 0..n {\n        rev[ord[i]] = i;\n        bs[i]\
    \ = abc[ord[i]].1;\n    }\n    let mut g = RangeEdgeGraph::new(n);\n    for i\
    \ in 0..n {\n        let (_, b, c) = abc[ord[i]];\n        let j = ord.partition_point(|&i|\
    \ abc[i].0 <= b - c);\n        if j <= i {\n            g.add_edge(i..=i, 0..j,\
    \ 0);\n        } else {\n            g.add_edge(i..=i, 0..i, 0);\n           \
    \ g.add_edge(i..=i, i + 1..j, 0);\n        }\n    }\n    let h = g.build();\n\
    \    let mut res = vec![None; h.len()];\n    for i in 0..n {\n        let r =\
    \ dfs(rev[i], &h, &mut res, &bs);\n        if r >= INF {\n            println!(\"\
    BAN\");\n        } else {\n            println!(\"{r}\");\n        }\n    }\n\
    }\n"
  dependsOn:
  - crates/graph/graph/src/lib.rs
  - crates/graph/range-edge-graph/src/lib.rs
  - crates/macros/chminmax/src/lib.rs
  isVerificationFile: true
  path: verify/yuki1014/src/main.rs
  requiredBy: []
  timestamp: '2024-04-10 09:38:39+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yuki1014/src/main.rs
layout: document
redirect_from:
- /verify/verify/yuki1014/src/main.rs
- /verify/verify/yuki1014/src/main.rs.html
title: verify/yuki1014/src/main.rs
---
