---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
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
  code: "pub fn euler_phi(n: usize) -> Vec<usize> {\n    let mut p = vec![0; n + 1];\n\
    \    for i in 0..=n {\n        p[i] = i;\n    }\n    for i in 2..=n {\n      \
    \  if p[i] != i {\n            continue;\n        }\n        for j in (i..=n).step_by(i)\
    \ {\n            p[j] /= i;\n            p[j] *= i - 1;\n        }\n    }\n  \
    \  p\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/math/euler-phi/src/lib.rs
  requiredBy: []
  timestamp: '2023-12-24 10:25:28+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/math/euler-phi/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/euler-phi/src/lib.rs
- /library/crates/math/euler-phi/src/lib.rs.html
title: crates/math/euler-phi/src/lib.rs
---
