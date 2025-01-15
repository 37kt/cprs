---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy-light-decomposition/src/lib.rs
    title: crates/tree/heavy-light-decomposition/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/jump_on_tree
    links:
    - https://judge.yosupo.jp/problem/jump_on_tree
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/jump_on_tree\n\
    \nuse graph::UndirectedGraph;\nuse heavy_light_decomposition::HeavyLightDecomposition;\n\
    use proconio::fastout;\nuse proconio::input;\n\n#[fastout]\nfn main() {\n    input!\
    \ {\n        n: usize,\n        q: usize,\n        ab: [(usize, usize); n - 1],\n\
    \    }\n    let g = UndirectedGraph::from_unweighted_edges(n, &ab);\n    let hld\
    \ = HeavyLightDecomposition::new(&g, 0);\n    for _ in 0..q {\n        input!\
    \ {\n            s: usize,\n            t: usize,\n            i: usize,\n   \
    \     }\n        let v = hld.jump(s, t, i);\n        println!(\"{}\", v as i64);\n\
    \    }\n}\n"
  dependsOn:
  - crates/graph/graph/src/lib.rs
  - crates/tree/heavy-light-decomposition/src/lib.rs
  isVerificationFile: true
  path: verify/jump_on_tree/src/main.rs
  requiredBy: []
  timestamp: '2025-01-15 04:45:47+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/jump_on_tree/src/main.rs
layout: document
redirect_from:
- /verify/verify/jump_on_tree/src/main.rs
- /verify/verify/jump_on_tree/src/main.rs.html
title: verify/jump_on_tree/src/main.rs
---
