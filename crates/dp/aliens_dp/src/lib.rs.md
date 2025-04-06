---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/cast.rs
    title: crates/algebra/numeric_traits/src/cast.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/inf.rs
    title: crates/algebra/numeric_traits/src/inf.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/integer.rs
    title: crates/algebra/numeric_traits/src/integer.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/lib.rs
    title: crates/algebra/numeric_traits/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/numeric.rs
    title: crates/algebra/numeric_traits/src/numeric.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/signed.rs
    title: crates/algebra/numeric_traits/src/signed.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/zero_one.rs
    title: crates/algebra/numeric_traits/src/zero_one.rs
  - icon: ':warning:'
    path: crates/misc/as_half_open_range/src/lib.rs
    title: crates/misc/as_half_open_range/src/lib.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: verify/sandbox/abc218h/src/main.rs
    title: verify/sandbox/abc218h/src/main.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// TODO: verify\n\nuse std::ops::RangeBounds;\n\nuse as_half_open_range::AsHalfOpenRange;\n\
    use numeric_traits::{Cast, Inf, Integer, NegInf, Signed};\n\n/// Aliens DP  \n\
    /// \u51F8\u95A2\u6570 `f` \u306E `f(x)` \u3092\u6C42\u3081\u308B\n///\n/// -\
    \ `g(p) := f(x) + px`  \n/// - `p_range` \u306E\u7BC4\u56F2\u3092\u63A2\u7D22\u3059\
    \u308B\npub fn aliens_dp<T>(x: usize, p_range: impl RangeBounds<T>, mut g: impl\
    \ FnMut(T) -> T) -> T\nwhere\n    T: Integer + Signed + Inf + NegInf,\n    usize:\
    \ Cast<T>,\n{\n    let one = T::one();\n\n    let x = x.cast();\n    let (mut\
    \ l, mut r) = p_range.as_half_open_range(T::neg_inf(), T::inf());\n    l -= one;\n\
    \    r -= one;\n    while l + one < r {\n        let p = l + ((r - l) >> 1);\n\
    \        let c = g(p + one) - g(p);\n        if c <= x {\n            r = p;\n\
    \        } else {\n            l = p;\n        }\n    }\n\n    g(r) - r * x\n\
    }\n"
  dependsOn:
  - crates/algebra/numeric_traits/src/cast.rs
  - crates/algebra/numeric_traits/src/inf.rs
  - crates/algebra/numeric_traits/src/integer.rs
  - crates/algebra/numeric_traits/src/lib.rs
  - crates/algebra/numeric_traits/src/numeric.rs
  - crates/algebra/numeric_traits/src/signed.rs
  - crates/algebra/numeric_traits/src/zero_one.rs
  - crates/misc/as_half_open_range/src/lib.rs
  isVerificationFile: false
  path: crates/dp/aliens_dp/src/lib.rs
  requiredBy:
  - verify/sandbox/abc218h/src/main.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/dp/aliens_dp/src/lib.rs
layout: document
redirect_from:
- /library/crates/dp/aliens_dp/src/lib.rs
- /library/crates/dp/aliens_dp/src/lib.rs.html
title: crates/dp/aliens_dp/src/lib.rs
---
