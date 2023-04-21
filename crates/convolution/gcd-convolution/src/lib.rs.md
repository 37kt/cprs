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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ops::{Add, MulAssign, Sub};\n\npub fn multiple_zeta<T>(a: &mut Vec<T>)\n\
    where\n    T: Clone + Add<Output = T>,\n{\n    let n = a.len() - 1;\n    let mut\
    \ is_prime = vec![true; n + 1];\n    for p in 2..=n {\n        if is_prime[p]\
    \ {\n            for q in (p * 2..=n).step_by(p) {\n                is_prime[q]\
    \ = false;\n            }\n            for i in (1..=n / p).rev() {\n        \
    \        a[i] = a[i].clone() + a[i * p].clone();\n            }\n        }\n \
    \   }\n}\n\npub fn multiple_moebius<T>(a: &mut Vec<T>)\nwhere\n    T: Clone +\
    \ Sub<Output = T>,\n{\n    let n = a.len() - 1;\n    let mut is_prime = vec![true;\
    \ n + 1];\n    for p in 2..=n {\n        if is_prime[p] {\n            for q in\
    \ (p * 2..=n).step_by(p) {\n                is_prime[q] = false;\n           \
    \ }\n            for i in 1..=n / p {\n                a[i] = a[i].clone() - a[i\
    \ * p].clone();\n            }\n        }\n    }\n}\n\npub fn gcd_convolution<T>(mut\
    \ a: Vec<T>, mut b: Vec<T>) -> Vec<T>\nwhere\n    T: Clone + Add<Output = T> +\
    \ Sub<Output = T> + MulAssign,\n{\n    assert_eq!(a.len(), b.len());\n    multiple_zeta(&mut\
    \ a);\n    multiple_zeta(&mut b);\n    for i in 1..a.len() {\n        a[i] *=\
    \ b[i].clone();\n    }\n    multiple_moebius(&mut a);\n    a\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/convolution/gcd-convolution/src/lib.rs
  requiredBy: []
  timestamp: '2023-04-14 16:37:19+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/convolution/gcd-convolution/src/lib.rs
layout: document
redirect_from:
- /library/crates/convolution/gcd-convolution/src/lib.rs
- /library/crates/convolution/gcd-convolution/src/lib.rs.html
title: crates/convolution/gcd-convolution/src/lib.rs
---
