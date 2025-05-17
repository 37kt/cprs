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
  - icon: ':heavy_check_mark:'
    path: crates/convolution/minplus_convolution/src/concave_and_arbitrary.rs
    title: crates/convolution/minplus_convolution/src/concave_and_arbitrary.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/minplus_convolution/src/convex_and_arbitrary.rs
    title: crates/convolution/minplus_convolution/src/convex_and_arbitrary.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/minplus_convolution/src/convex_and_convex.rs
    title: crates/convolution/minplus_convolution/src/convex_and_convex.rs
  - icon: ':heavy_check_mark:'
    path: crates/dp/monotone_minima/src/lib.rs
    title: crates/dp/monotone_minima/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/convolution/minplus_convolution/src/concave_and_arbitrary.rs
    title: crates/convolution/minplus_convolution/src/concave_and_arbitrary.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/minplus_convolution/src/convex_and_arbitrary.rs
    title: crates/convolution/minplus_convolution/src/convex_and_arbitrary.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/minplus_convolution/src/convex_and_convex.rs
    title: crates/convolution/minplus_convolution/src/convex_and_convex.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/convolution/min_plus_convolution_concave_arbitrary/src/main.rs
    title: verify/library_checker/convolution/min_plus_convolution_concave_arbitrary/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/convolution/min_plus_convolution_convex_arbitrary/src/main.rs
    title: verify/library_checker/convolution/min_plus_convolution_convex_arbitrary/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/convolution/min_plus_convolution_convex_convex/src/main.rs
    title: verify/library_checker/convolution/min_plus_convolution_convex_convex/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "mod convex_and_convex;\npub use convex_and_convex::*;\n\nmod convex_and_arbitrary;\n\
    pub use convex_and_arbitrary::*;\n\nmod concave_and_arbitrary;\npub use concave_and_arbitrary::*;\n\
    use numeric_traits::Integer;\n\npub fn is_convex<T>(a: &[T]) -> bool\nwhere\n\
    \    T: Integer,\n{\n    a.windows(3).all(|a| a[1] + a[1] <= a[2] + a[0])\n}\n\
    \npub fn is_concave<T>(a: &[T]) -> bool\nwhere\n    T: Integer,\n{\n    a.windows(3).all(|a|\
    \ a[1] + a[1] >= a[2] + a[0])\n}\n"
  dependsOn:
  - crates/algebra/numeric_traits/src/cast.rs
  - crates/algebra/numeric_traits/src/inf.rs
  - crates/algebra/numeric_traits/src/integer.rs
  - crates/algebra/numeric_traits/src/lib.rs
  - crates/algebra/numeric_traits/src/numeric.rs
  - crates/algebra/numeric_traits/src/signed.rs
  - crates/algebra/numeric_traits/src/zero_one.rs
  - crates/convolution/minplus_convolution/src/concave_and_arbitrary.rs
  - crates/convolution/minplus_convolution/src/convex_and_arbitrary.rs
  - crates/convolution/minplus_convolution/src/convex_and_convex.rs
  - crates/dp/monotone_minima/src/lib.rs
  isVerificationFile: false
  path: crates/convolution/minplus_convolution/src/lib.rs
  requiredBy:
  - crates/convolution/minplus_convolution/src/concave_and_arbitrary.rs
  - crates/convolution/minplus_convolution/src/convex_and_arbitrary.rs
  - crates/convolution/minplus_convolution/src/convex_and_convex.rs
  timestamp: '2025-03-20 09:27:03+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/convolution/min_plus_convolution_convex_convex/src/main.rs
  - verify/library_checker/convolution/min_plus_convolution_concave_arbitrary/src/main.rs
  - verify/library_checker/convolution/min_plus_convolution_convex_arbitrary/src/main.rs
documentation_of: crates/convolution/minplus_convolution/src/lib.rs
layout: document
redirect_from:
- /library/crates/convolution/minplus_convolution/src/lib.rs
- /library/crates/convolution/minplus_convolution/src/lib.rs.html
title: crates/convolution/minplus_convolution/src/lib.rs
---
