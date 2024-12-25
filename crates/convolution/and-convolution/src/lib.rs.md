---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/bitwise_and_convolution/src/main.rs
    title: verify/bitwise_and_convolution/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ops::{Add, Mul, Sub};\n\n/// \u9AD8\u901F\u30BC\u30FC\u30BF\u5909\
    \u63DB\n///\n/// # \u6982\u8981\n/// - \u96C6\u5408 s \u3092\u542B\u3080\u4E0A\
    \u4F4D\u96C6\u5408 t \u306B\u3064\u3044\u3066\u306E f\\[t\\] \u306E\u7DCF\u548C\
    \u3092\u8A08\u7B97\n///\n/// # \u8A08\u7B97\u5F0F\n/// g\\[s\\] = \u03A3_{s \u2286\
    \ t} f\\[t\\]\n///\n/// # \u8A08\u7B97\u91CF\n/// - O(n 2^n)\n///   - n: \u96C6\
    \u5408\u306E\u8981\u7D20\u6570\npub fn superset_zeta<T>(f: &mut [T])\nwhere\n\
    \    T: Clone + Add<Output = T>,\n{\n    let log = f.len().trailing_zeros();\n\
    \    assert_eq!(1 << log, f.len());\n    for i in 0..log {\n        let w = 1\
    \ << i;\n        for f in f.chunks_exact_mut(w << 1) {\n            let (f0, f1)\
    \ = f.split_at_mut(w);\n            for (x, y) in f0.iter_mut().zip(f1) {\n  \
    \              *x = x.clone() + y.clone();\n            }\n        }\n    }\n\
    }\n\n/// \u9AD8\u901F\u30E1\u30D3\u30A6\u30B9\u5909\u63DB\n///\n/// # \u6982\u8981\
    \n/// - \u9AD8\u901F\u30BC\u30FC\u30BF\u5909\u63DB\u306E\u9006\u5909\u63DB\u3092\
    \u8A08\u7B97\n/// - \u96C6\u5408 s \u3092\u542B\u3080\u4E0A\u4F4D\u96C6\u5408\
    \ t \u306B\u3064\u3044\u3066\u306E\u5305\u9664\u539F\u7406\u3092\u9069\u7528\n\
    ///\n/// # \u8A08\u7B97\u5F0F\n/// f\\[s\\] = \u03A3_{s \u2286 t} (-1)^{|t| -\
    \ |s|} g\\[t\\]\n///\n/// # \u8A08\u7B97\u91CF\n/// - O(n 2^n)\n///   - n: \u96C6\
    \u5408\u306E\u8981\u7D20\u6570\npub fn superset_moebius<T>(g: &mut [T])\nwhere\n\
    \    T: Clone + Sub<Output = T>,\n{\n    let log = g.len().trailing_zeros();\n\
    \    assert_eq!(1 << log, g.len());\n    for i in 0..log {\n        let w = 1\
    \ << i;\n        for g in g.chunks_exact_mut(w << 1) {\n            let (g0, g1)\
    \ = g.split_at_mut(w);\n            for (x, y) in g0.iter_mut().zip(g1) {\n  \
    \              *x = x.clone() - y.clone();\n            }\n        }\n    }\n\
    }\n\n/// \u30D3\u30C3\u30C8\u5358\u4F4D\u306EAND\u7573\u307F\u8FBC\u307F\u3092\
    \u8A08\u7B97\u3059\u308B\n///\n/// # \u6982\u8981\n/// 2\u3064\u306E\u914D\u5217\
    \ `a`, `b` \u306B\u5BFE\u3057\u3001\u4EE5\u4E0B\u306E\u5F0F\u3067\u5B9A\u7FA9\u3055\
    \u308C\u308B\u7573\u307F\u8FBC\u307F\u3092\u8A08\u7B97\u3059\u308B\uFF1A\n///\
    \ ```text\n/// res[k] = \u03A3_{i & j = k} (a[i] * b[j])\n/// ```\n///\n/// #\
    \ \u5F15\u6570\n/// - `a`: 1\u3064\u76EE\u306E\u914D\u5217\uFF08\u9577\u3055\u306F\
    2\u306E\u51AA\u4E57\u3067\u3042\u308B\u5FC5\u8981\u304C\u3042\u308B\uFF09\n///\
    \ - `b`: 2\u3064\u76EE\u306E\u914D\u5217\uFF08`a`\u3068\u540C\u3058\u9577\u3055\
    \uFF09\n///\n/// # \u623B\u308A\u5024\n/// - AND\u7573\u307F\u8FBC\u307F\u306E\
    \u7D50\u679C\uFF08\u9577\u3055\u306F\u5165\u529B\u3068\u540C\u3058\uFF09\n///\n\
    /// # \u5236\u7D04\n/// - `a.len() == b.len()`\n/// - \u914D\u5217\u306E\u9577\
    \u3055\u306F2\u306E\u51AA\u4E57\n///\n/// # \u8A08\u7B97\u91CF\n/// - O(N log\
    \ N)\n///   - N: \u914D\u5217\u306E\u9577\u3055\npub fn and_convolution<T>(mut\
    \ a: Vec<T>, mut b: Vec<T>) -> Vec<T>\nwhere\n    T: Clone + Add<Output = T> +\
    \ Sub<Output = T> + Mul<Output = T>,\n{\n    assert_eq!(a.len(), b.len());\n \
    \   superset_zeta(&mut a);\n    superset_zeta(&mut b);\n    a = a\n        .into_iter()\n\
    \        .zip(b.into_iter())\n        .map(|(a, b)| a * b)\n        .collect();\n\
    \    superset_moebius(&mut a);\n    a\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/convolution/and-convolution/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-25 07:02:27+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/bitwise_and_convolution/src/main.rs
documentation_of: crates/convolution/and-convolution/src/lib.rs
layout: document
redirect_from:
- /library/crates/convolution/and-convolution/src/lib.rs
- /library/crates/convolution/and-convolution/src/lib.rs.html
title: crates/convolution/and-convolution/src/lib.rs
---
