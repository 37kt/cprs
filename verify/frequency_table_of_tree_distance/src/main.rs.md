---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution-u64/src/lib.rs
    title: crates/convolution/convolution-u64/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/centroid-decomposition/src/lib.rs
    title: crates/tree/centroid-decomposition/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/frequency_table_of_tree_distance
    links:
    - https://judge.yosupo.jp/problem/frequency_table_of_tree_distance
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/frequency_table_of_tree_distance\n\
    \nuse centroid_decomposition::centroid_decomposition;\nuse convolution_u64::convolution_u64;\n\
    use graph::Graph;\nuse itertools::Itertools;\nuse proconio::input;\n\n#[proconio::fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        uv: [(usize, usize); n -\
    \ 1],\n    }\n    let g = Graph::from_unweighted_undirected_edges(n, &uv);\n \
    \   let mut res = vec![0; n];\n    let mut dist = vec![0; n];\n    let mut f =\
    \ |idx: &[usize], par: &[usize], m: usize| {\n        let n = idx.len();\n   \
    \     let r = par[0];\n        dist[r] = 0;\n        let mut mx = 0;\n       \
    \ for i in 0..n {\n            let v = idx[i];\n            let p = par[i];\n\
    \            dist[v] = dist[p] + 1;\n            mx = mx.max(dist[v]);\n     \
    \   }\n        let mut a = vec![0; mx + 1];\n        let mut b = vec![0; mx +\
    \ 1];\n        for i in 0..m {\n            let v = idx[i];\n            a[dist[v]]\
    \ += 1;\n        }\n        for i in m..n {\n            let v = idx[i];\n   \
    \         b[dist[v]] += 1;\n        }\n        let c = convolution_u64(&a, &b);\n\
    \        for i in 0..c.len().min(res.len()) {\n            res[i] += c[i];\n \
    \       }\n    };\n    centroid_decomposition(&g, &mut f);\n    res[1] += n as\
    \ u64 - 1;\n    println!(\"{}\", res[1..].iter().join(\" \"));\n}\n"
  dependsOn:
  - crates/convolution/convolution-u64/src/lib.rs
  - crates/graph/graph/src/lib.rs
  - crates/tree/centroid-decomposition/src/lib.rs
  isVerificationFile: true
  path: verify/frequency_table_of_tree_distance/src/main.rs
  requiredBy: []
  timestamp: '2024-09-20 11:13:45+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/frequency_table_of_tree_distance/src/main.rs
layout: document
redirect_from:
- /verify/verify/frequency_table_of_tree_distance/src/main.rs
- /verify/verify/frequency_table_of_tree_distance/src/main.rs.html
title: verify/frequency_table_of_tree_distance/src/main.rs
---
