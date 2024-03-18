---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/splay-tree/src/lib.rs
    title: crates/data-structure/splay-tree/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/range_reverse_range_sum
    links:
    - https://judge.yosupo.jp/problem/range_reverse_range_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_reverse_range_sum\n\
    \nuse algebraic::{act, algebra, monoid};\nuse proconio::input;\nuse splay_tree::SplayTree;\n\
    \nalgebra!(M, i64);\nmonoid!(M, 0, |&s1, &s2| s1 + s2);\n\nalgebra!(F, ());\n\
    act!(F, i64, |_, &s| s);\nmonoid!(F, (), |_, _| ());\n\n#[proconio::fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        q: usize,\n        a: [i64;\
    \ n],\n    }\n    let mut sp = SplayTree::<M, F>::from(&a[..]);\n    for _ in\
    \ 0..q {\n        input! {\n            ty: usize,\n            l: usize,\n  \
    \          r: usize,\n        }\n        if ty == 0 {\n            sp.reverse(l..r);\n\
    \        } else {\n            println!(\"{}\", sp.prod(l..r));\n        }\n \
    \   }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/data-structure/splay-tree/src/lib.rs
  isVerificationFile: true
  path: verify/range_reverse_range_sum/src/main.rs
  requiredBy: []
  timestamp: '2024-03-18 01:19:47+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/range_reverse_range_sum/src/main.rs
layout: document
redirect_from:
- /verify/verify/range_reverse_range_sum/src/main.rs
- /verify/verify/range_reverse_range_sum/src/main.rs.html
title: verify/range_reverse_range_sum/src/main.rs
---
