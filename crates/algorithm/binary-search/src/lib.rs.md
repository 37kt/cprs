---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
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
  code: "use algebraic::{One, Zero};\nuse std::ops::{Add, Div, Sub};\n\n/// \u534A\
    \u958B\u533A\u9593 `[l..r)` \u306B\u304A\u3044\u3066 `f(x) = false` \u3068\u306A\
    \u308B\u6700\u5C0F\u306E `x` \u3092\u63A2\u7D22\u3059\u308B\u3002\n///\n/// #\
    \ \u6982\u8981\n/// - \u4E8C\u5206\u63A2\u7D22\u306B\u3088\u308A\u3001\u6761\u4EF6\
    \u3092\u6E80\u305F\u3059\u6700\u5C0F\u306E\u5024\u3092\u6C42\u3081\u308B\n///\
    \ - `f(x)` \u306F\u5358\u8ABF\u3067\u3042\u308B\u5FC5\u8981\u304C\u3042\u308B\n\
    ///\n/// # \u623B\u308A\u5024\n/// - `f(x) = false` \u3068\u306A\u308B\u6700\u5C0F\
    \u306E `x`\n/// - \u305D\u306E\u3088\u3046\u306A `x` \u304C\u5B58\u5728\u3057\u306A\
    \u3044\u5834\u5408\u306F `r` \u3092\u8FD4\u3059\npub fn binary_search<I>(mut l:\
    \ I, mut r: I, mut f: impl FnMut(I) -> bool) -> I\nwhere\n    I: Copy + Add<Output\
    \ = I> + Sub<Output = I> + Div<Output = I> + Zero + One + PartialOrd,\n{\n   \
    \ let one = I::one();\n    let two = one + one;\n    if !f(l) {\n        return\
    \ l;\n    }\n    while l + one < r {\n        let m = (l + r) / two;\n       \
    \ if f(m) {\n            l = m;\n        } else {\n            r = m;\n      \
    \  }\n    }\n    r\n}\n\n/// \u533A\u9593 `(l..r)` \u3067 `f(x) = false` \u3068\
    \u306A\u308B\u6700\u5C0F\u306E `x` \u3092\u8FD4\u3059\u3002\n///\n/// # \u6982\
    \u8981\n/// - \u4E8C\u5206\u63A2\u7D22\u306B\u3088\u308A\u3001\u6761\u4EF6\u3092\
    \u6E80\u305F\u3059\u6700\u5C0F\u306E\u5024\u3092\u6C42\u3081\u308B\n/// - `f(x)`\
    \ \u306F\u5358\u8ABF\u3067\u3042\u308B\u5FC5\u8981\u304C\u3042\u308B\n///\n///\
    \ # \u623B\u308A\u5024\n/// - `f(x) = false` \u3068\u306A\u308B\u6700\u5C0F\u306E\
    \ `x`\n/// - \u305D\u306E\u3088\u3046\u306A `x` \u304C\u5B58\u5728\u3057\u306A\
    \u3044\u5834\u5408\u306F `r` \u3092\u8FD4\u3059\npub fn binary_search_f64(mut\
    \ l: f64, mut r: f64, mut f: impl FnMut(f64) -> bool) -> f64 {\n    for _ in 0..100\
    \ {\n        let m = (l + r) / 2.0;\n        if f(m) {\n            l = m;\n \
    \       } else {\n            r = m;\n        }\n    }\n    r\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  isVerificationFile: false
  path: crates/algorithm/binary-search/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-04 02:49:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/algorithm/binary-search/src/lib.rs
layout: document
redirect_from:
- /library/crates/algorithm/binary-search/src/lib.rs
- /library/crates/algorithm/binary-search/src/lib.rs.html
title: crates/algorithm/binary-search/src/lib.rs
---
