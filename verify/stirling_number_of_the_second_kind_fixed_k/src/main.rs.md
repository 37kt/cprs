---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/stirling-second-fixed-k/src/lib.rs
    title: crates/number-theory/stirling-second-fixed-k/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/stirling_number_of_the_second_kind_fixed_k
    links:
    - https://judge.yosupo.jp/problem/stirling_number_of_the_second_kind_fixed_k
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/stirling_number_of_the_second_kind_fixed_k\n\
    \nuse proconio::input;\nuse stirling_second_fixed_k::stirling_second_fixed_k;\n\
    \n#[proconio::fastout]\nfn main() {\n    input! {\n        n: usize,\n       \
    \ k: usize,\n    }\n    let mut st = stirling_second_fixed_k::<998244353>(n, k);\n\
    \    st.drain(..k);\n    println!(\"{}\", st);\n}\n"
  dependsOn:
  - crates/number-theory/modint/src/lib.rs
  - crates/number-theory/stirling-second-fixed-k/src/lib.rs
  isVerificationFile: true
  path: verify/stirling_number_of_the_second_kind_fixed_k/src/main.rs
  requiredBy: []
  timestamp: '2025-01-26 00:19:46+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/stirling_number_of_the_second_kind_fixed_k/src/main.rs
layout: document
redirect_from:
- /verify/verify/stirling_number_of_the_second_kind_fixed_k/src/main.rs
- /verify/verify/stirling_number_of_the_second_kind_fixed_k/src/main.rs.html
title: verify/stirling_number_of_the_second_kind_fixed_k/src/main.rs
---
