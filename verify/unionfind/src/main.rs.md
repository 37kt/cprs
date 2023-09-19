---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/union-find/src/lib.rs
    title: crates/data-structure/union-find/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/unionfind
    links:
    - https://judge.yosupo.jp/problem/unionfind
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.5/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.5/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind\n\
    \nuse proconio::input;\nuse union_find::UnionFind;\n\n#[proconio::fastout]\nfn\
    \ main() {\n    input! {\n        n: usize,\n        q: usize,\n    }\n    let\
    \ mut uf = UnionFind::new(n);\n    for _ in 0..q {\n        input! {\n       \
    \     t: usize,\n            u: usize,\n            v: usize,\n        }\n   \
    \     if t == 0 {\n            uf.merge(u, v);\n        } else {\n           \
    \ println!(\"{}\", uf.same(u, v) as i64);\n        }\n    }\n}\n"
  dependsOn:
  - crates/data-structure/union-find/src/lib.rs
  isVerificationFile: true
  path: verify/unionfind/src/main.rs
  requiredBy: []
  timestamp: '2023-07-10 15:59:10+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/unionfind/src/main.rs
layout: document
redirect_from:
- /verify/verify/unionfind/src/main.rs
- /verify/verify/unionfind/src/main.rs.html
title: verify/unionfind/src/main.rs
---
