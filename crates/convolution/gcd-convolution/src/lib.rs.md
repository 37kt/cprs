---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/gcd_convolution/src/main.rs
    title: verify/gcd_convolution/src/main.rs
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
  code: "use std::ops::{Add, Mul, Sub};\n\n/// \u500D\u6570\u30BC\u30FC\u30BF\u5909\
    \u63DB\n///\n/// # \u6982\u8981\n/// - i \u306E\u500D\u6570 j \u306B\u3064\u3044\
    \u3066\u306E f\\[j\\] \u306E\u7DCF\u548C\u3092\u8A08\u7B97\n///\n/// # \u5F15\u6570\
    \n/// - `f`: \u5165\u529B\u914D\u5217\n///\n/// # \u8A08\u7B97\u5F0F\n/// g\\\
    [i\\] = \u03A3_{i | j} f\\[j\\]\n///\n/// # \u8A08\u7B97\u91CF\n/// - O(n log\
    \ log n)\n///   - n: \u914D\u5217\u306E\u9577\u3055\npub fn multiple_zeta<T>(f:\
    \ &mut [T])\nwhere\n    T: Clone + Add<Output = T>,\n{\n    let n = f.len() -\
    \ 1;\n    let mut is_prime = vec![true; n + 1];\n    for p in 2..=n {\n      \
    \  if is_prime[p] {\n            for q in (p * 2..=n).step_by(p) {\n         \
    \       is_prime[q] = false;\n            }\n            for i in (1..=n / p).rev()\
    \ {\n                f[i] = f[i].clone() + f[i * p].clone();\n            }\n\
    \        }\n    }\n}\n\n/// \u500D\u6570\u30E1\u30D3\u30A6\u30B9\u5909\u63DB\n\
    ///\n/// # \u6982\u8981\n/// - \u500D\u6570\u30BC\u30FC\u30BF\u5909\u63DB\u306E\
    \u9006\u5909\u63DB\n///\n/// # \u5F15\u6570\n/// - `g`: \u5165\u529B\u914D\u5217\
    \n///\n/// # \u8A08\u7B97\u5F0F\n/// g\\[i\\] = \u03A3_{i | j} f\\[j\\]\n///\n\
    /// # \u8A08\u7B97\u91CF\n/// - O(n log log n)\n///   - n: \u914D\u5217\u306E\u9577\
    \u3055\npub fn multiple_moebius<T>(g: &mut [T])\nwhere\n    T: Clone + Sub<Output\
    \ = T>,\n{\n    let n = g.len() - 1;\n    let mut is_prime = vec![true; n + 1];\n\
    \    for p in 2..=n {\n        if is_prime[p] {\n            for q in (p * 2..=n).step_by(p)\
    \ {\n                is_prime[q] = false;\n            }\n            for i in\
    \ 1..=n / p {\n                g[i] = g[i].clone() - g[i * p].clone();\n     \
    \       }\n        }\n    }\n}\n\n/// GCD \u7573\u307F\u8FBC\u307F\n///\n/// #\
    \ \u6982\u8981\n/// - 2\u3064\u306E\u914D\u5217 `a`, `b` \u306B\u5BFE\u3057\u3001\
    \u4EE5\u4E0B\u306E\u5F0F\u3067\u5B9A\u7FA9\u3055\u308C\u308B\u7573\u307F\u8FBC\
    \u307F\u3092\u8A08\u7B97\u3059\u308B\uFF1A\n/// ```text\n/// res[k] = \u03A3_{k\
    \ | k=gcd(i, j)} (a[i] * b[j])\n/// ```\n///\n/// # \u5F15\u6570\n/// - `a`: 1\u3064\
    \u76EE\u306E\u914D\u5217\n/// - `b`: 2\u3064\u76EE\u306E\u914D\u5217\n///\n///\
    \ # \u623B\u308A\u5024\n/// - GCD \u7573\u307F\u8FBC\u307F\u306E\u7D50\u679C\uFF08\
    \u9577\u3055\u306F `a.len() + b.len() - 1`\uFF09\n///\n/// # \u8A08\u7B97\u91CF\
    \n/// - O(N log log N)\n///   - N: max(a.len(), b.len())\npub fn gcd_convolution<T>(mut\
    \ a: Vec<T>, mut b: Vec<T>) -> Vec<T>\nwhere\n    T: Clone + Add<Output = T> +\
    \ Sub<Output = T> + Mul<Output = T>,\n{\n    assert_eq!(a.len(), b.len());\n \
    \   multiple_zeta(&mut a);\n    multiple_zeta(&mut b);\n    a = a\n        .into_iter()\n\
    \        .zip(b.into_iter())\n        .map(|(x, y)| x * y)\n        .collect();\n\
    \    multiple_moebius(&mut a);\n    a\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/convolution/gcd-convolution/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-25 07:02:27+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/gcd_convolution/src/main.rs
documentation_of: crates/convolution/gcd-convolution/src/lib.rs
layout: document
redirect_from:
- /library/crates/convolution/gcd-convolution/src/lib.rs
- /library/crates/convolution/gcd-convolution/src/lib.rs.html
title: crates/convolution/gcd-convolution/src/lib.rs
---
