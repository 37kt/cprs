---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/subset_convolution/src/main.rs
    title: verify/subset_convolution/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.1/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.1/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ops::{Add, Mul, Sub};\n\npub fn ranked_zeta<T>(a: &[T]) -> Vec<[T;\
    \ 21]>\nwhere\n    T: Default + Copy + Add<Output = T> + Sub<Output = T> + Mul<Output\
    \ = T>,\n{\n    let n = a.len();\n    let logn = 63 - n.leading_zeros() as usize;\n\
    \    assert_eq!(1 << logn, n);\n    let mut b = vec![[T::default(); 21]; n];\n\
    \    for s in 0..n {\n        b[s][s.count_ones() as usize] = a[s].clone();\n\
    \    }\n    for i in 0..logn {\n        let w = 1 << i;\n        for p in (0..n).step_by(w\
    \ * 2) {\n            for s in p..p + w {\n                let t = s | 1 << i;\n\
    \                for d in 0..=logn {\n                    b[t][d] = b[t][d] +\
    \ b[s][d];\n                }\n            }\n        }\n    }\n    b\n}\n\npub\
    \ fn ranked_moebius<T>(mut a: Vec<[T; 21]>) -> Vec<T>\nwhere\n    T: Default +\
    \ Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,\n{\n    let n =\
    \ a.len();\n    let logn = 63 - n.leading_zeros() as usize;\n    assert_eq!(1\
    \ << logn, n);\n    for i in 0..logn {\n        let w = 1 << i;\n        for p\
    \ in (0..n).step_by(w * 2) {\n            for s in p..p + w {\n              \
    \  let t = s | 1 << i;\n                for d in 0..=logn {\n                \
    \    a[t][d] = a[t][d] - a[s][d];\n                }\n            }\n        }\n\
    \    }\n    (0..n)\n        .into_iter()\n        .map(|s| a[s][s.count_ones()\
    \ as usize])\n        .collect()\n}\n\npub fn subset_convolution<T>(a: &[T], b:\
    \ &[T]) -> Vec<T>\nwhere\n    T: Default + Copy + Add<Output = T> + Sub<Output\
    \ = T> + Mul<Output = T>,\n{\n    assert_eq!(a.len(), b.len());\n    let logn\
    \ = 63 - a.len().leading_zeros() as usize;\n    let mut ra = ranked_zeta(&a);\n\
    \    let rb = ranked_zeta(&b);\n    for (f, g) in ra.iter_mut().zip(&rb) {\n \
    \       for d in (0..=logn).rev() {\n            let mut x = T::default();\n \
    \           for i in 0..=d {\n                x = x + f[i] * g[d - i];\n     \
    \       }\n            f[d] = x;\n        }\n    }\n    ranked_moebius(ra)\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/convolution/subset-convolution/src/lib.rs
  requiredBy: []
  timestamp: '2023-05-06 16:57:25+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/subset_convolution/src/main.rs
documentation_of: crates/convolution/subset-convolution/src/lib.rs
layout: document
redirect_from:
- /library/crates/convolution/subset-convolution/src/lib.rs
- /library/crates/convolution/subset-convolution/src/lib.rs.html
title: crates/convolution/subset-convolution/src/lib.rs
---
