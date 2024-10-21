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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "/// \u30CA\u30A4\u30FC\u30D6\u306Bmex\u3092\u6C42\u3081\u308B\u3002\u5B9F\
    \u9A13\u7528\npub fn mex(a: &[usize]) -> usize {\n    let mut a = a.to_vec();\n\
    \    a.sort();\n    a.dedup();\n    for i in 0..a.len() {\n        if a[i] !=\
    \ i {\n            return i;\n        }\n    }\n    a.len()\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/sequence/mex/src/lib.rs
  requiredBy: []
  timestamp: '2023-04-22 22:39:06+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/sequence/mex/src/lib.rs
layout: document
redirect_from:
- /library/crates/sequence/mex/src/lib.rs
- /library/crates/sequence/mex/src/lib.rs.html
title: crates/sequence/mex/src/lib.rs
---
