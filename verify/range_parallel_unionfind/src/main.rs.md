---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/range-union-find/src/lib.rs
    title: crates/data-structure/range-union-find/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/range_parallel_unionfind
    links:
    - https://judge.yosupo.jp/problem/range_parallel_unionfind
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.3/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.3/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_parallel_unionfind\n\
    \nuse modint::ModInt998244353 as Mint;\nuse proconio::input;\nuse range_union_find::RangeUnionFind;\n\
    \n#[proconio::fastout]\nfn main() {\n    input! {\n        n: usize,\n       \
    \ q: usize,\n        mut x: [Mint; n],\n    }\n    let mut sum = Mint::new(0);\n\
    \    let mut uf = RangeUnionFind::new(n);\n    for _ in 0..q {\n        input!\
    \ {\n            k: usize,\n            a: usize,\n            b: usize,\n   \
    \     }\n        for (u, v) in uf.merge_range(a..a + k, b..b + k) {\n        \
    \    sum += x[u] * x[v];\n            x[u] = x[u] + x[v];\n        }\n       \
    \ println!(\"{}\", sum);\n    }\n}\n"
  dependsOn:
  - crates/data-structure/range-union-find/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  isVerificationFile: true
  path: verify/range_parallel_unionfind/src/main.rs
  requiredBy: []
  timestamp: '2024-06-13 09:46:47+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/range_parallel_unionfind/src/main.rs
layout: document
redirect_from:
- /verify/verify/range_parallel_unionfind/src/main.rs
- /verify/verify/range_parallel_unionfind/src/main.rs.html
title: verify/range_parallel_unionfind/src/main.rs
---
