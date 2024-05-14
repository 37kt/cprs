---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algorithm/static-rectangle-add-rectangle-sum/src/lib.rs
    title: crates/algorithm/static-rectangle-add-rectangle-sum/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/static_rectangle_add_rectangle_sum
    links:
    - https://judge.yosupo.jp/problem/static_rectangle_add_rectangle_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.3/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.3/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_rectangle_add_rectangle_sum\n\
    \nuse modint::ModInt998244353 as Mint;\nuse proconio::input;\nuse static_rectangle_add_rectangle_sum::static_rectangle_add_rectangle_sum;\n\
    \n#[proconio::fastout]\nfn main() {\n    input! {\n        n: usize,\n       \
    \ q: usize,\n    }\n    let mut add = vec![];\n    let mut sum = vec![];\n   \
    \ for _ in 0..n {\n        input! {\n            l: usize,\n            d: usize,\n\
    \            r: usize,\n            u: usize,\n            w: Mint,\n        }\n\
    \        add.push((l, r, d, u, w));\n    }\n    for _ in 0..q {\n        input!\
    \ {\n            l: usize,\n            d: usize,\n            r: usize,\n   \
    \         u: usize,\n        }\n        sum.push((l, r, d, u));\n    }\n    let\
    \ res = static_rectangle_add_rectangle_sum(&add, &sum);\n    for r in res {\n\
    \        println!(\"{}\", r);\n    }\n}\n"
  dependsOn:
  - crates/algorithm/static-rectangle-add-rectangle-sum/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  isVerificationFile: true
  path: verify/static_rectangle_add_rectangle_sum/src/main.rs
  requiredBy: []
  timestamp: '2024-05-14 13:30:35+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/static_rectangle_add_rectangle_sum/src/main.rs
layout: document
redirect_from:
- /verify/verify/static_rectangle_add_rectangle_sum/src/main.rs
- /verify/verify/static_rectangle_add_rectangle_sum/src/main.rs.html
title: verify/static_rectangle_add_rectangle_sum/src/main.rs
---
