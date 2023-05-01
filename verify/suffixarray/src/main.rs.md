---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/suffixarray
    links:
    - https://judge.yosupo.jp/problem/suffixarray
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/suffixarray\n\
    \nuse itertools::Itertools;\nuse proconio::{input, marker::Bytes};\nuse suffix_array::SuffixArray;\n\
    \n#[proconio::fastout]\nfn main() {\n    input! {\n        s: Bytes,\n    }\n\
    \    let sa = SuffixArray::build(&s);\n    println!(\"{}\", sa.suffix_array().iter().join(\"\
    \ \"));\n}\n"
  dependsOn: []
  isVerificationFile: true
  path: verify/suffixarray/src/main.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: verify/suffixarray/src/main.rs
layout: document
redirect_from:
- /verify/verify/suffixarray/src/main.rs
- /verify/verify/suffixarray/src/main.rs.html
title: verify/suffixarray/src/main.rs
---
