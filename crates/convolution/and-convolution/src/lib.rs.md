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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ops::{Add, MulAssign, Sub};\n\npub fn superset_zeta<T>(a: &mut [T])\n\
    where\n    T: Clone + Add<Output = T>,\n{\n    let log = a.len().trailing_zeros();\n\
    \    assert_eq!(1 << log, a.len());\n    for i in 0..log {\n        let i = 1\
    \ << i;\n        for j in (0..a.len()).step_by(i << 1) {\n            for k in\
    \ 0..i {\n                a[j + k] = a[j + k].clone() + a[i + j + k].clone();\n\
    \            }\n        }\n    }\n}\n\npub fn superset_moebius<T>(a: &mut [T])\n\
    where\n    T: Clone + Sub<Output = T>,\n{\n    let log = a.len().trailing_zeros();\n\
    \    assert_eq!(1 << log, a.len());\n    for i in 0..log {\n        let i = 1\
    \ << i;\n        for j in (0..a.len()).step_by(i << 1) {\n            for k in\
    \ 0..i {\n                a[j + k] = a[j + k].clone() - a[i + j + k].clone();\n\
    \            }\n        }\n    }\n}\n\npub fn and_convolution<T>(mut a: Vec<T>,\
    \ mut b: Vec<T>) -> Vec<T>\nwhere\n    T: Clone + Add<Output = T> + Sub<Output\
    \ = T> + MulAssign,\n{\n    assert_eq!(a.len(), b.len());\n    superset_zeta(&mut\
    \ a);\n    superset_zeta(&mut b);\n    for i in 0..a.len() {\n        a[i] *=\
    \ b[i].clone();\n    }\n    superset_moebius(&mut a);\n    a\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/convolution/and-convolution/src/lib.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/convolution/and-convolution/src/lib.rs
layout: document
redirect_from:
- /library/crates/convolution/and-convolution/src/lib.rs
- /library/crates/convolution/and-convolution/src/lib.rs.html
title: crates/convolution/and-convolution/src/lib.rs
---
