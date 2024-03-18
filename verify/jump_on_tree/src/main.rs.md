---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/heavy-light-decomposition/src/lib.rs
    title: crates/data-structure/heavy-light-decomposition/src/lib.rs
  - icon: ':question:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/jump_on_tree
    links:
    - https://judge.yosupo.jp/problem/jump_on_tree
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/jump_on_tree\n\
    \nuse graph::Graph;\nuse heavy_light_decomposition::HeavyLightDecomposition;\n\
    use proconio::input;\n\n#[proconio::fastout]\nfn main() {\n    input! {\n    \
    \    n: usize,\n        q: usize,\n    }\n    let mut g = Graph::<(), ()>::new(n);\n\
    \    for _ in 1..n {\n        input! {\n            a: usize,\n            b:\
    \ usize,\n        }\n        g.add_undirected_edge(a, b, ());\n    }\n    let\
    \ hld = HeavyLightDecomposition::new(&g);\n    for _ in 0..q {\n        input!\
    \ {\n            s: usize,\n            t: usize,\n            i: usize,\n   \
    \     }\n        let v = hld.jump(s, t, i);\n        println!(\"{}\", v as i64);\n\
    \    }\n}\n"
  dependsOn:
  - crates/data-structure/heavy-light-decomposition/src/lib.rs
  - crates/graph/graph/src/lib.rs
  isVerificationFile: true
  path: verify/jump_on_tree/src/main.rs
  requiredBy: []
  timestamp: '2023-05-17 16:30:46+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/jump_on_tree/src/main.rs
layout: document
redirect_from:
- /verify/verify/jump_on_tree/src/main.rs
- /verify/verify/jump_on_tree/src/main.rs.html
title: verify/jump_on_tree/src/main.rs
---
