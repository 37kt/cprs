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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ops::{Add, Mul, Sub};\n\npub fn ranked_zeta<T>(a: &[T]) -> Vec<[T;\
    \ 21]>\nwhere\n    T: Default + Copy + Add<Output = T> + Sub<Output = T> + Mul<Output\
    \ = T>,\n{\n    let n = a.len();\n    let logn = 63 - n.leading_zeros() as usize;\n\
    \    assert_eq!(1 << logn, n);\n    let mut b = vec![[T::default(); 21]; n];\n\
    \    for s in 0..n {\n        b[s][s.count_ones() as usize] = a[s].clone();\n\
    \    }\n    for i in 0..logn {\n        let w = 1 << i;\n        for p in (0..n).step_by(w\
    \ * 2) {\n            for s in p..p + w {\n                let t = s | w;\n  \
    \              for d in 0..=logn {\n                    b[t][d] = b[t][d] + b[s][d];\n\
    \                }\n            }\n        }\n    }\n    b\n}\n\npub fn ranked_moebius<T>(a:\
    \ &[[T; 21]]) -> Vec<T>\nwhere\n    T: Default + Copy + Add<Output = T> + Sub<Output\
    \ = T> + Mul<Output = T>,\n{\n    let mut a = a.to_vec();\n    let n = a.len();\n\
    \    let logn = 63 - n.leading_zeros() as usize;\n    assert_eq!(1 << logn, n);\n\
    \    for i in 0..logn {\n        let w = 1 << i;\n        for p in (0..n).step_by(w\
    \ * 2) {\n            for s in p..p + w {\n                let t = s | w;\n  \
    \              for d in 0..=logn {\n                    a[t][d] = a[t][d] - a[s][d];\n\
    \                }\n            }\n        }\n    }\n    (0..n)\n        .into_iter()\n\
    \        .map(|s| a[s][s.count_ones() as usize])\n        .collect()\n}\n\n///\
    \ Subset Convolution\n///\n/// # \u6982\u8981\n/// - 2\u3064\u306E\u914D\u5217\
    \ `a`, `b` \u306B\u5BFE\u3057\u3001\u4EE5\u4E0B\u306E\u5F0F\u3067\u5B9A\u7FA9\u3055\
    \u308C\u308B\u7573\u307F\u8FBC\u307F\u3092\u8A08\u7B97\u3059\u308B\uFF1A\n///\
    \ ```text\n/// res[s] = \u03A3_{t\u2286s} (a[t] * b[s\\t])\n/// ```\n///\n///\
    \ # \u5F15\u6570\n/// - `a`: 1\u3064\u76EE\u306E\u914D\u5217\n/// - `b`: 2\u3064\
    \u76EE\u306E\u914D\u5217\n///\n/// # \u623B\u308A\u5024\n/// - Subset Convolution\
    \ \u306E\u7D50\u679C\npub fn subset_convolution<T>(a: &[T], b: &[T]) -> Vec<T>\n\
    where\n    T: Default + Copy + Add<Output = T> + Sub<Output = T> + Mul<Output\
    \ = T>,\n{\n    assert_eq!(a.len(), b.len());\n    let logn = 63 - a.len().leading_zeros()\
    \ as usize;\n    let mut ra = ranked_zeta(&a);\n    let rb = ranked_zeta(&b);\n\
    \    for (f, g) in ra.iter_mut().zip(&rb) {\n        for d in (0..=logn).rev()\
    \ {\n            let mut x = T::default();\n            for i in 0..=d {\n   \
    \             x = x + f[i] * g[d - i];\n            }\n            f[d] = x;\n\
    \        }\n    }\n    ranked_moebius(&ra)\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/convolution/subset-convolution/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-25 07:02:27+00:00'
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
