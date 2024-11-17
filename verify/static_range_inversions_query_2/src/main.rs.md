---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/static-range-inversions-query/src/lib.rs
    title: crates/data-structure/static-range-inversions-query/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/static_range_inversions_query
    links:
    - https://judge.yosupo.jp/problem/static_range_inversions_query
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_inversions_query\n\
    \nuse proconio::input;\nuse static_range_inversions_query::StaticRangeInversionsQuery;\n\
    \n#[proconio::fastout]\nfn main() {\n    input! {\n        n: usize,\n       \
    \ q: usize,\n        a: [usize; n],\n    }\n    let riq = StaticRangeInversionsQuery::new(&a);\n\
    \    for _ in 0..q {\n        input! {\n            l: usize,\n            r:\
    \ usize,\n        }\n        println!(\"{}\", riq.inversions(l..r));\n    }\n\
    }\n"
  dependsOn:
  - crates/data-structure/static-range-inversions-query/src/lib.rs
  isVerificationFile: true
  path: verify/static_range_inversions_query_2/src/main.rs
  requiredBy: []
  timestamp: '2024-11-17 18:56:30+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/static_range_inversions_query_2/src/main.rs
layout: document
redirect_from:
- /verify/verify/static_range_inversions_query_2/src/main.rs
- /verify/verify/static_range_inversions_query_2/src/main.rs.html
title: verify/static_range_inversions_query_2/src/main.rs
---
