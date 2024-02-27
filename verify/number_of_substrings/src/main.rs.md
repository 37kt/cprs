---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/string/suffix-array/src/lib.rs
    title: crates/string/suffix-array/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/number_of_substrings
    links:
    - https://judge.yosupo.jp/problem/number_of_substrings
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/number_of_substrings\n\
    \nuse proconio::{input, marker::Bytes};\nuse suffix_array::SuffixArray;\n\n#[proconio::fastout]\n\
    fn main() {\n    input! {\n        s: Bytes,\n    }\n    let sa = SuffixArray::build(&s);\n\
    \    let lcp = sa.lcp_array();\n    let n = s.len();\n    let res = n * (n + 1)\
    \ / 2 - lcp.iter().sum::<usize>();\n    println!(\"{}\", res);\n}\n"
  dependsOn:
  - crates/string/suffix-array/src/lib.rs
  isVerificationFile: true
  path: verify/number_of_substrings/src/main.rs
  requiredBy: []
  timestamp: '2023-06-13 17:07:21+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/number_of_substrings/src/main.rs
layout: document
redirect_from:
- /verify/verify/number_of_substrings/src/main.rs
- /verify/verify/number_of_substrings/src/main.rs.html
title: verify/number_of_substrings/src/main.rs
---
