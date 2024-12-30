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
  code: "/// \u30E9\u30F3\u30EC\u30F3\u30B0\u30B9\u5727\u7E2E  \n/// (\u5024, \u51FA\
    \u73FE\u56DE\u6570) \u306E\u30DA\u30A2\u306E Vec \u3092\u8FD4\u3059\u3002\npub\
    \ fn run_length_encoding<T>(a: &[T]) -> Vec<(T, usize)>\nwhere\n    T: Clone +\
    \ Eq,\n{\n    if a.len() == 0 {\n        return vec![];\n    }\n    let mut res\
    \ = vec![(a[0].clone(), 0)];\n    for x in a {\n        if res.last().unwrap().0\
    \ == *x {\n            res.last_mut().unwrap().1 += 1;\n        } else {\n   \
    \         res.push((x.clone(), 1));\n        }\n    }\n    res\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/sequence/run-length-encoding/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-30 09:13:10+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/sequence/run-length-encoding/src/lib.rs
layout: document
redirect_from:
- /library/crates/sequence/run-length-encoding/src/lib.rs
- /library/crates/sequence/run-length-encoding/src/lib.rs.html
title: crates/sequence/run-length-encoding/src/lib.rs
---
