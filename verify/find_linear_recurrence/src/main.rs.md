---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/berlekamp-massey/src/lib.rs
    title: crates/polynomial/berlekamp-massey/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/find_linear_recurrence
    links:
    - https://judge.yosupo.jp/problem/find_linear_recurrence
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/find_linear_recurrence\n\
    \nuse berlekamp_massey::berlekamp_massey;\nuse modint::ModInt998244353;\nuse proconio::input;\n\
    \ntype Mint = ModInt998244353;\n\n#[proconio::fastout]\nfn main() {\n    input!\
    \ {\n        n: usize,\n        a: [Mint; n],\n    }\n    let c = berlekamp_massey(&a);\n\
    \    println!(\"{}\", c.len() - 1);\n    for i in 1..c.len() {\n        print!(\"\
    {} \", -c[i]);\n    }\n    println!();\n}\n"
  dependsOn:
  - crates/number-theory/modint/src/lib.rs
  - crates/polynomial/berlekamp-massey/src/lib.rs
  isVerificationFile: true
  path: verify/find_linear_recurrence/src/main.rs
  requiredBy: []
  timestamp: '2024-12-30 09:13:10+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/find_linear_recurrence/src/main.rs
layout: document
redirect_from:
- /verify/verify/find_linear_recurrence/src/main.rs
- /verify/verify/find_linear_recurrence/src/main.rs.html
title: verify/find_linear_recurrence/src/main.rs
---
