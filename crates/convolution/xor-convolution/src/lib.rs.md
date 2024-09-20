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
  code: "use std::ops::{Add, Div, DivAssign, MulAssign, Sub};\n\npub fn hadamard<T>(a:\
    \ &mut [T], inv: bool)\nwhere\n    T: Clone\n        + Default\n        + Eq\n\
    \        + Add<Output = T>\n        + Sub<Output = T>\n        + From<i64>\n \
    \       + MulAssign\n        + Div<Output = T>\n        + DivAssign,\n{\n    let\
    \ log = a.len().trailing_zeros();\n    assert_eq!(1 << log, a.len());\n    for\
    \ i in 0..log {\n        let i = 1 << i;\n        for j in (0..a.len()).step_by(i\
    \ << 1) {\n            for k in 0..i {\n                let x = a[j + k].clone();\n\
    \                let y = a[i + j + k].clone();\n                a[j + k] = x.clone()\
    \ + y.clone();\n                a[i + j + k] = x - y;\n            }\n       \
    \ }\n    }\n    if inv {\n        let inv_n = T::from(1) / T::from(a.len() as\
    \ i64);\n        if inv_n == T::default() {\n            let n = T::from(a.len()\
    \ as i64);\n            for i in 0..a.len() {\n                a[i] /= n.clone();\n\
    \            }\n        } else {\n            for i in 0..a.len() {\n        \
    \        a[i] *= inv_n.clone();\n            }\n        }\n    }\n}\n\npub fn\
    \ xor_convolution<T>(mut a: Vec<T>, mut b: Vec<T>) -> Vec<T>\nwhere\n    T: Clone\n\
    \        + Default\n        + Eq\n        + Add<Output = T>\n        + Sub<Output\
    \ = T>\n        + From<i64>\n        + MulAssign\n        + Div<Output = T>\n\
    \        + DivAssign,\n{\n    assert_eq!(a.len(), b.len());\n    hadamard(&mut\
    \ a, false);\n    hadamard(&mut b, false);\n    for i in 0..a.len() {\n      \
    \  a[i] *= b[i].clone();\n    }\n    hadamard(&mut a, true);\n    a\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/convolution/xor-convolution/src/lib.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/convolution/xor-convolution/src/lib.rs
layout: document
redirect_from:
- /library/crates/convolution/xor-convolution/src/lib.rs
- /library/crates/convolution/xor-convolution/src/lib.rs.html
title: crates/convolution/xor-convolution/src/lib.rs
---
