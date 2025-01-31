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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "/// a \u306B\u542B\u307E\u308C\u306A\u3044\u6700\u5C0F\u306E\u975E\u8CA0\u6574\
    \u6570\u3092\u6C42\u3081\u308B\u3002\npub fn mex(a: &[usize]) -> usize {\n   \
    \ let mut a = a.to_vec();\n    for i in 0..a.len() {\n        while i < a.len()\
    \ && a[i] != i {\n            if a[i] >= a.len() || a[i] == a[a[i]] {\n      \
    \          a.swap_remove(i);\n            } else {\n                let j = a[i];\n\
    \                a.swap(i, j);\n            }\n        }\n    }\n    a.iter()\n\
    \        .enumerate()\n        .position(|(i, &x)| x != i)\n        .unwrap_or(a.len())\n\
    }\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/sequence/mex/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-25 11:38:56+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/sequence/mex/src/lib.rs
layout: document
redirect_from:
- /library/crates/sequence/mex/src/lib.rs
- /library/crates/sequence/mex/src/lib.rs.html
title: crates/sequence/mex/src/lib.rs
---
