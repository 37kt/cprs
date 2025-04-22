---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy_light_decomposition/src/compress.rs
    title: crates/tree/heavy_light_decomposition/src/compress.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy_light_decomposition/src/construct.rs
    title: crates/tree/heavy_light_decomposition/src/construct.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy_light_decomposition/src/lib.rs
    title: crates/tree/heavy_light_decomposition/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/jump_on_tree
    links:
    - https://judge.yosupo.jp/problem/jump_on_tree
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/jump_on_tree\n\
    \nuse heavy_light_decomposition::HeavyLightDecomposition;\nuse proconio::{fastout,\
    \ input};\n\n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n      \
    \  q: usize,\n        ab: [(usize, usize); n - 1],\n    }\n    let hld = HeavyLightDecomposition::from_edges(&ab,\
    \ 0);\n    for _ in 0..q {\n        input! {\n            s: usize,\n        \
    \    t: usize,\n            d: usize,\n        }\n        if let Some(v) = hld.jump(s,\
    \ t, d) {\n            println!(\"{}\", v);\n        } else {\n            println!(\"\
    -1\");\n        }\n    }\n}\n"
  dependsOn:
  - crates/tree/heavy_light_decomposition/src/compress.rs
  - crates/tree/heavy_light_decomposition/src/construct.rs
  - crates/tree/heavy_light_decomposition/src/lib.rs
  isVerificationFile: true
  path: verify/library_checker/tree/jump_on_tree/src/main.rs
  requiredBy: []
  timestamp: '2025-04-22 05:57:06+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/tree/jump_on_tree/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/tree/jump_on_tree/src/main.rs
- /verify/verify/library_checker/tree/jump_on_tree/src/main.rs.html
title: verify/library_checker/tree/jump_on_tree/src/main.rs
---
