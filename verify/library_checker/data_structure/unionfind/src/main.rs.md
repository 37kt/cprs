---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/union_find/src/lib.rs
    title: crates/data_structure/union_find/union_find/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/union_find/src/potentialized_union_find.rs
    title: crates/data_structure/union_find/union_find/src/potentialized_union_find.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/union_find/src/union_find.rs
    title: crates/data_structure/union_find/union_find/src/union_find.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/union_find/src/union_find_component_sum.rs
    title: crates/data_structure/union_find/union_find/src/union_find_component_sum.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/union_find/src/union_find_impl.rs
    title: crates/data_structure/union_find/union_find/src/union_find_impl.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/unionfind
    links:
    - https://judge.yosupo.jp/problem/unionfind
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind\n\
    \nuse proconio::fastout;\nuse proconio::input;\nuse union_find::UnionFind;\n\n\
    #[fastout]\nfn main() {\n    input! {\n        n: usize,\n        q: usize,\n\
    \    }\n\n    let mut uf = UnionFind::new(n);\n    for _ in 0..q {\n        input!\
    \ {\n            t: usize,\n            u: usize,\n            v: usize,\n   \
    \     }\n        match t {\n            0 => {\n                uf.merge(u, v);\n\
    \            }\n            1 => println!(\"{}\", uf.same(u, v) as i32),\n   \
    \         _ => unreachable!(),\n        }\n    }\n}\n"
  dependsOn:
  - crates/data_structure/union_find/union_find/src/lib.rs
  - crates/data_structure/union_find/union_find/src/potentialized_union_find.rs
  - crates/data_structure/union_find/union_find/src/union_find.rs
  - crates/data_structure/union_find/union_find/src/union_find_component_sum.rs
  - crates/data_structure/union_find/union_find/src/union_find_impl.rs
  isVerificationFile: true
  path: verify/library_checker/data_structure/unionfind/src/main.rs
  requiredBy: []
  timestamp: '2025-03-07 01:17:39+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/data_structure/unionfind/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/data_structure/unionfind/src/main.rs
- /verify/verify/library_checker/data_structure/unionfind/src/main.rs.html
title: verify/library_checker/data_structure/unionfind/src/main.rs
---
