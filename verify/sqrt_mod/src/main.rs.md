---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/sqrt_mod
    links:
    - https://judge.yosupo.jp/problem/sqrt_mod
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/sqrt_mod\n\
    \nuse modint::DynamicModInt as Mint;\nuse proconio::input;\n\n#[proconio::fastout]\n\
    fn main() {\n    input! {\n        t: usize,\n    }\n    for _ in 0..t {\n   \
    \     input! {\n            y: u32,\n            p: u32,\n        }\n        Mint::set_modulus(p);\n\
    \        if let Some(res) = Mint::new(y).sqrt() {\n            println!(\"{}\"\
    , res);\n        } else {\n            println!(\"-1\");\n        }\n    }\n}\n"
  dependsOn:
  - crates/number-theory/modint/src/lib.rs
  isVerificationFile: true
  path: verify/sqrt_mod/src/main.rs
  requiredBy: []
  timestamp: '2024-06-13 08:47:29+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/sqrt_mod/src/main.rs
layout: document
redirect_from:
- /verify/verify/sqrt_mod/src/main.rs
- /verify/verify/sqrt_mod/src/main.rs.html
title: verify/sqrt_mod/src/main.rs
---
