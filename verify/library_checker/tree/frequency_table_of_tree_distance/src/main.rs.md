---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/arbitrary_mod.rs
    title: crates/convolution/convolution/src/arbitrary_mod.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/lib.rs
    title: crates/convolution/convolution/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/mod_2_64.rs
    title: crates/convolution/convolution/src/mod_2_64.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/naive.rs
    title: crates/convolution/convolution/src/naive.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/ntt.rs
    title: crates/convolution/convolution/src/ntt.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/ntt_friendly.rs
    title: crates/convolution/convolution/src/ntt_friendly.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/builder.rs
    title: crates/graph/graph/src/builder.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/edge.rs
    title: crates/graph/graph/src/edge.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/centroid_decomposition/src/lib.rs
    title: crates/tree/centroid_decomposition/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/frequency_table_of_tree_distance
    links:
    - https://judge.yosupo.jp/problem/frequency_table_of_tree_distance
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/frequency_table_of_tree_distance\n\
    \nuse centroid_decomposition::CentroidDecomposition;\nuse convolution::convolution_mod_2_64;\n\
    use graph::{GraphBuilder, UndirectedGraph};\nuse proconio::{fastout, input};\n\
    \n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n        ab: [(usize,\
    \ usize); n - 1],\n    }\n    let g = UndirectedGraph::from_edges(n, ab);\n  \
    \  let mut res = vec![0; n];\n    let mut depth = vec![0; n];\n    CentroidDecomposition::solve(&g,\
    \ |tr| {\n        depth[tr.root] = 0;\n        let mut f = vec![vec![]; 2];\n\
    \        for c in 0..2 {\n            for (&v, &p) in tr.vs[c].iter().zip(tr.par[c].iter())\
    \ {\n                depth[v] = depth[p] + 1;\n                if f[c].len() <=\
    \ depth[v] {\n                    f[c].resize(depth[v] + 1, 0);\n            \
    \    }\n                f[c][depth[v]] += 1;\n            }\n        }\n     \
    \   let g = convolution_mod_2_64(&f[0], &f[1]);\n        for i in 0..g.len() {\n\
    \            res[i] += g[i];\n        }\n    });\n    res[1] += (n - 1) as u64;\n\
    \    for x in &res[1..] {\n        print!(\"{} \", x);\n    }\n    println!();\n\
    }\n"
  dependsOn:
  - crates/convolution/convolution/src/arbitrary_mod.rs
  - crates/convolution/convolution/src/lib.rs
  - crates/convolution/convolution/src/mod_2_64.rs
  - crates/convolution/convolution/src/naive.rs
  - crates/convolution/convolution/src/ntt.rs
  - crates/convolution/convolution/src/ntt_friendly.rs
  - crates/graph/graph/src/builder.rs
  - crates/graph/graph/src/edge.rs
  - crates/graph/graph/src/lib.rs
  - crates/tree/centroid_decomposition/src/lib.rs
  isVerificationFile: true
  path: verify/library_checker/tree/frequency_table_of_tree_distance/src/main.rs
  requiredBy: []
  timestamp: '2025-03-27 00:50:57+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/tree/frequency_table_of_tree_distance/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/tree/frequency_table_of_tree_distance/src/main.rs
- /verify/verify/library_checker/tree/frequency_table_of_tree_distance/src/main.rs.html
title: verify/library_checker/tree/frequency_table_of_tree_distance/src/main.rs
---
