---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/partition/src/lib.rs
    title: crates/number-theory/partition/src/lib.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "/// \u4E94\u89D2\u6570\u5B9A\u7406\npub fn pentagonal_number_theorem<T>(n:\
    \ usize) -> Vec<T>\nwhere\n    T: Copy + From<i8>,\n{\n    let mut res = vec![0.into();\
    \ n];\n    res[0] = 1.into();\n    for k in 1.. {\n        let i1 = k * (3 * k\
    \ - 1) / 2;\n        let i2 = k * (3 * k + 1) / 2;\n        if i1 >= n {\n   \
    \         break;\n        }\n        let x = if k % 2 == 1 { (-1).into() } else\
    \ { 1.into() };\n        if i1 < n {\n            res[i1] = x;\n        }\n  \
    \      if i2 < n {\n            res[i2] = x;\n        }\n    }\n    res\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/number-theory/pentagonal-number-theorem/src/lib.rs
  requiredBy:
  - crates/number-theory/partition/src/lib.rs
  timestamp: '2024-12-30 09:13:10+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/number-theory/pentagonal-number-theorem/src/lib.rs
layout: document
redirect_from:
- /library/crates/number-theory/pentagonal-number-theorem/src/lib.rs
- /library/crates/number-theory/pentagonal-number-theorem/src/lib.rs.html
title: crates/number-theory/pentagonal-number-theorem/src/lib.rs
---
