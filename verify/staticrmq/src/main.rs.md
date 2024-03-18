---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  - icon: ':x:'
    path: crates/data-structure/disjoint-sparse-table/src/lib.rs
    title: crates/data-structure/disjoint-sparse-table/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/staticrmq
    links:
    - https://judge.yosupo.jp/problem/staticrmq
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq\n\
    \nuse algebraic::{algebra, monoid};\nuse disjoint_sparse_table::DisjointSparseTable;\n\
    use proconio::input;\n\nalgebra!(M, i64);\nmonoid!(M, 1 << 60, |x: &i64, y: &i64|\
    \ *x.min(y));\n\n#[proconio::fastout]\nfn main() {\n    input! {\n        n: usize,\n\
    \        q: usize,\n        a: [i64; n],\n    }\n    let spt = DisjointSparseTable::<M>::new(&a);\n\
    \    for _ in 0..q {\n        input! {\n            l: usize,\n            r:\
    \ usize,\n        }\n        println!(\"{}\", spt.prod(l, r));\n    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/data-structure/disjoint-sparse-table/src/lib.rs
  isVerificationFile: true
  path: verify/staticrmq/src/main.rs
  requiredBy: []
  timestamp: '2024-03-18 01:19:47+09:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: verify/staticrmq/src/main.rs
layout: document
redirect_from:
- /verify/verify/staticrmq/src/main.rs
- /verify/verify/staticrmq/src/main.rs.html
title: verify/staticrmq/src/main.rs
---
