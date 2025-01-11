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
  code: "use algebraic::{One, Zero};\nuse std::ops::{Add, Div, Sub};\n\n/// \u533A\
    \u9593\u4E0A\u306E\u51F8\u95A2\u6570\u306E\u6700\u5C0F\u5024\u3092\u4E09\u5206\
    \u63A2\u7D22\u3067\u6C42\u3081\u308B\n///\n/// # \u6982\u8981\n/// - \u533A\u9593\
    \ `[l..r]` \u4E0A\u3067\u4E0B\u306B\u51F8\u306A\u95A2\u6570 `f` \u306E\u6700\u5C0F\
    \u5024\u3092\u6C42\u3081\u308B\n/// - \u95A2\u6570\u306E\u6700\u5C0F\u5024\u3092\
    \u53D6\u308B\u4F4D\u7F6E `x` \u3068\u3001\u305D\u306E\u5024 `f(x)` \u3092\u8FD4\
    \u3059\n///\n/// # \u5F15\u6570\n/// - `l`: \u63A2\u7D22\u533A\u9593\u306E\u5DE6\
    \u7AEF\n/// - `r`: \u63A2\u7D22\u533A\u9593\u306E\u53F3\u7AEF\n/// - `f`: \u76EE\
    \u7684\u95A2\u6570\uFF08\u4E0B\u306B\u51F8\u3067\u3042\u308B\u5FC5\u8981\u304C\
    \u3042\u308B\uFF09\n///\n/// # \u578B\u30D1\u30E9\u30E1\u30FC\u30BF\n/// - `I`:\
    \ \u5EA7\u6A19\u306E\u578B\uFF08\u6574\u6570\u578B\u3092\u60F3\u5B9A\uFF09\n///\
    \ - `T`: \u95A2\u6570\u5024\u306E\u578B\n///\n/// # \u623B\u308A\u5024\n/// -\
    \ `(x, f(x))`: \u6700\u5C0F\u5024\u3092\u53D6\u308B\u4F4D\u7F6E\u3068\u305D\u306E\
    \u5024\n///\n/// # \u5236\u7D04\n/// - `l <= r` \u3067\u3042\u308B\u3053\u3068\
    \n/// - `f` \u306F\u533A\u9593 `[l..r]` \u3067\u4E0B\u306B\u51F8\u3067\u3042\u308B\
    \u3053\u3068\n///\n/// # \u8A08\u7B97\u91CF\n/// - O(log(r - l))\npub fn ternary_search<I,\
    \ T>(mut l: I, mut r: I, mut f: impl FnMut(I) -> T) -> (I, T)\nwhere\n    I: Copy\
    \ + Add<Output = I> + Sub<Output = I> + Div<Output = I> + Zero + One + PartialOrd,\n\
    \    T: Copy + PartialOrd,\n{\n    assert!(l <= r);\n    let one = I::one();\n\
    \    let two = one + one;\n    let three = two + one;\n    while l + two < r {\n\
    \        let m1 = (l + l + r) / three;\n        let m2 = (l + r + r) / three;\n\
    \        if f(m1) < f(m2) {\n            r = m2;\n        } else {\n         \
    \   l = m1;\n        }\n    }\n    let mut mn = f(l);\n    let mut i = l;\n  \
    \  if l + one <= r && f(l + one) < mn {\n        mn = f(l + one);\n        i =\
    \ l + one;\n    }\n    if l + two <= r && f(l + two) < mn {\n        mn = f(l\
    \ + two);\n        i = l + two;\n    }\n    (i, mn)\n}\n\n/// \u5B9F\u6570\u533A\
    \u9593\u4E0A\u306E\u51F8\u95A2\u6570\u306E\u6700\u5C0F\u5024\u3092\u4E09\u5206\
    \u63A2\u7D22\u3067\u6C42\u3081\u308B\n///\n/// # \u6982\u8981\n/// - \u533A\u9593\
    \ `[l..r]` \u3067\u4E0B\u306B\u51F8\u306A\u95A2\u6570 `f` \u306E\u6700\u5C0F\u5024\
    \u3092\u6C42\u3081\u308B\n/// - \u95A2\u6570\u306E\u6700\u5C0F\u5024\u3092\u53D6\
    \u308B\u4F4D\u7F6E `x` \u3068\u3001\u305D\u306E\u5024 `f(x)` \u3092\u8FD4\u3059\
    \n///\n/// # \u5F15\u6570\n/// - `l`: \u63A2\u7D22\u533A\u9593\u306E\u5DE6\u7AEF\
    \uFF08\u5B9F\u6570\uFF09\n/// - `r`: \u63A2\u7D22\u533A\u9593\u306E\u53F3\u7AEF\
    \uFF08\u5B9F\u6570\uFF09\n/// - `f`: \u76EE\u7684\u95A2\u6570\uFF08\u4E0B\u306B\
    \u51F8\u3067\u3042\u308B\u5FC5\u8981\u304C\u3042\u308B\uFF09\n///\n/// # \u578B\
    \u30D1\u30E9\u30E1\u30FC\u30BF\n/// - `T`: \u95A2\u6570\u5024\u306E\u578B\n///\n\
    /// # \u623B\u308A\u5024\n/// - `(x, f(x))`: \u6700\u5C0F\u5024\u3092\u53D6\u308B\
    \u4F4D\u7F6E\u3068\u305D\u306E\u5024\n///\n/// # \u5B9F\u88C5\u8A73\u7D30\n///\
    \ - 100\u56DE\u306E\u53CD\u5FA9\u3067\u5341\u5206\u306A\u7CBE\u5EA6\u3092\u5F97\
    \u3089\u308C\u308B\n/// - \u7CBE\u5EA6\u306F\u7D04 2^(-100) \u2248 10^(-30)\n\
    pub fn ternary_search_f64<T>(mut l: f64, mut r: f64, mut f: impl FnMut(f64) ->\
    \ T) -> (f64, T)\nwhere\n    T: Copy + PartialOrd,\n{\n    for _ in 0..100 {\n\
    \        let m1 = (l * 2.0 + r) / 3.0;\n        let m2 = (l + 2.0 * r) / 3.0;\n\
    \        if f(m1) < f(m2) {\n            r = m2;\n        } else {\n         \
    \   l = m1;\n        }\n    }\n    (l, f(l))\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  isVerificationFile: false
  path: crates/algorithm/ternary-search/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-11 09:03:35+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/algorithm/ternary-search/src/lib.rs
layout: document
redirect_from:
- /library/crates/algorithm/ternary-search/src/lib.rs
- /library/crates/algorithm/ternary-search/src/lib.rs.html
title: crates/algorithm/ternary-search/src/lib.rs
---
