---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution-arbitrary-mod/src/lib.rs
    title: crates/convolution/convolution-arbitrary-mod/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution-ntt-friendly/src/lib.rs
    title: crates/convolution/convolution-ntt-friendly/src/lib.rs
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
  code: "use modint::ModInt;\n\n/// \u7573\u307F\u8FBC\u307F\u3092\u8A08\u7B97\u3059\
    \u308B\n///\n/// # \u5F15\u6570\n/// - `a`: 1\u3064\u76EE\u306E\u914D\u5217\n\
    /// - `b`: 2\u3064\u76EE\u306E\u914D\u5217\n///\n/// # \u623B\u308A\u5024\n///\
    \ - \u7573\u307F\u8FBC\u307F\u306E\u7D50\u679C\uFF08\u9577\u3055\u306F `a.len()\
    \ + b.len() - 1`\uFF09\n///\n/// # \u8A08\u7B97\u91CF\n/// - O(NM)\n///   - N:\
    \ a.len()\n///   - M: b.len()\npub fn convolution_naive<T: ModInt>(a: &[T], b:\
    \ &[T]) -> Vec<T> {\n    let n = a.len();\n    let m = b.len();\n    if n == 0\
    \ || m == 0 {\n        return vec![];\n    }\n    let l = n + m - 1;\n    let\
    \ mut c = vec![0.into(); l];\n    if n > m {\n        for i in 0..n {\n      \
    \      for j in 0..m {\n                c[i + j] += a[i] * b[j];\n           \
    \ }\n        }\n    } else {\n        for j in 0..m {\n            for i in 0..n\
    \ {\n                c[i + j] += a[i] * b[j];\n            }\n        }\n    }\n\
    \    c\n}\n"
  dependsOn:
  - crates/number-theory/modint/src/lib.rs
  isVerificationFile: false
  path: crates/convolution/convolution-naive/src/lib.rs
  requiredBy:
  - crates/convolution/convolution-ntt-friendly/src/lib.rs
  - crates/convolution/convolution-arbitrary-mod/src/lib.rs
  timestamp: '2024-12-30 09:13:10+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/convolution/convolution-naive/src/lib.rs
layout: document
redirect_from:
- /library/crates/convolution/convolution-naive/src/lib.rs
- /library/crates/convolution/convolution-naive/src/lib.rs.html
title: crates/convolution/convolution-naive/src/lib.rs
---
