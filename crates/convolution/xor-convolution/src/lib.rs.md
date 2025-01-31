---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/bitwise_xor_convolution/src/main.rs
    title: verify/bitwise_xor_convolution/src/main.rs
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
  code: "use std::ops::{Add, Div, DivAssign, MulAssign, Sub};\n\n/// \u9AD8\u901F\u30A2\
    \u30C0\u30DE\u30FC\u30EB\u5909\u63DB\n///\n/// # \u6982\u8981\n/// - \u914D\u5217\
    \ `a` \u306B\u5BFE\u3057\u3001\u4EE5\u4E0B\u306E\u5F0F\u3067\u5B9A\u7FA9\u3055\
    \u308C\u308B\u5909\u63DB\u3092\u8A08\u7B97\u3059\u308B\uFF1A\n/// ```text\n///\
    \ a[i] = \u03A3_{j=0}^{n-1} a[j] * (-1)^{popcount(i&j)}\n/// ```\n/// - \u9006\
    \u5909\u63DB\u306E\u3042\u3068\u306B n \u3067\u5272\u308B\n///\n/// # \u5F15\u6570\
    \n/// - `a`: \u5165\u529B\u914D\u5217\n/// - `inv`: \u9006\u5909\u63DB\u306E\u5834\
    \u5408\u306F `true`\npub fn hadamard<T>(a: &mut [T], inv: bool)\nwhere\n    T:\
    \ Clone\n        + Default\n        + Eq\n        + Add<Output = T>\n        +\
    \ Sub<Output = T>\n        + From<i64>\n        + MulAssign\n        + Div<Output\
    \ = T>\n        + DivAssign,\n{\n    let log = a.len().trailing_zeros();\n   \
    \ assert_eq!(1 << log, a.len());\n    for i in 0..log {\n        let i = 1 <<\
    \ i;\n        for j in (0..a.len()).step_by(i << 1) {\n            for k in 0..i\
    \ {\n                let x = a[j + k].clone();\n                let y = a[i +\
    \ j + k].clone();\n                a[j + k] = x.clone() + y.clone();\n       \
    \         a[i + j + k] = x - y;\n            }\n        }\n    }\n    if inv {\n\
    \        let inv_n = T::from(1) / T::from(a.len() as i64);\n        if inv_n ==\
    \ T::default() {\n            let n = T::from(a.len() as i64);\n            for\
    \ i in 0..a.len() {\n                a[i] /= n.clone();\n            }\n     \
    \   } else {\n            for i in 0..a.len() {\n                a[i] *= inv_n.clone();\n\
    \            }\n        }\n    }\n}\n\n/// XOR \u7573\u307F\u8FBC\u307F\n///\n\
    /// # \u6982\u8981\n/// - 2\u3064\u306E\u914D\u5217 `a`, `b` \u306B\u5BFE\u3057\
    \u3001\u4EE5\u4E0B\u306E\u5F0F\u3067\u5B9A\u7FA9\u3055\u308C\u308B\u7573\u307F\
    \u8FBC\u307F\u3092\u8A08\u7B97\u3059\u308B\uFF1A\n/// ```text\n/// res[k] = \u03A3\
    _{k=i^j} (a[i] * b[j])\n/// ```\n/// - \u8A08\u7B97\u91CF\u306F O(N log N)\npub\
    \ fn xor_convolution<T>(mut a: Vec<T>, mut b: Vec<T>) -> Vec<T>\nwhere\n    T:\
    \ Clone\n        + Default\n        + Eq\n        + Add<Output = T>\n        +\
    \ Sub<Output = T>\n        + From<i64>\n        + MulAssign\n        + Div<Output\
    \ = T>\n        + DivAssign,\n{\n    assert_eq!(a.len(), b.len());\n    hadamard(&mut\
    \ a, false);\n    hadamard(&mut b, false);\n    for i in 0..a.len() {\n      \
    \  a[i] *= b[i].clone();\n    }\n    hadamard(&mut a, true);\n    a\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/convolution/xor-convolution/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-25 07:02:27+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/bitwise_xor_convolution/src/main.rs
documentation_of: crates/convolution/xor-convolution/src/lib.rs
layout: document
redirect_from:
- /library/crates/convolution/xor-convolution/src/lib.rs
- /library/crates/convolution/xor-convolution/src/lib.rs.html
title: crates/convolution/xor-convolution/src/lib.rs
---
