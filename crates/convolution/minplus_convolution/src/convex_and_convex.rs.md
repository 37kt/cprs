---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/convolution/minplus_convolution/src/concave_and_arbitrary.rs
    title: crates/convolution/minplus_convolution/src/concave_and_arbitrary.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/minplus_convolution/src/convex_and_arbitrary.rs
    title: crates/convolution/minplus_convolution/src/convex_and_arbitrary.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/minplus_convolution/src/lib.rs
    title: crates/convolution/minplus_convolution/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/convolution/minplus_convolution/src/concave_and_arbitrary.rs
    title: crates/convolution/minplus_convolution/src/concave_and_arbitrary.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/minplus_convolution/src/convex_and_arbitrary.rs
    title: crates/convolution/minplus_convolution/src/convex_and_arbitrary.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/minplus_convolution/src/lib.rs
    title: crates/convolution/minplus_convolution/src/lib.rs
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
  code: "use numeric_traits::{Integer, Signed};\n\nuse crate::{is_concave, is_convex};\n\
    \npub fn minplus_convolution_convex_and_convex<T>(a: &[T], b: &[T]) -> Vec<T>\n\
    where\n    T: Integer,\n{\n    let n = a.len();\n    let m = b.len();\n    if\
    \ n == 0 || m == 0 {\n        return vec![];\n    }\n    assert!(is_convex(a));\n\
    \    assert!(is_convex(b));\n\n    let mut c = Vec::with_capacity(n + m - 1);\n\
    \    c.push(a[0] + b[0]);\n    let mut i = 0;\n    let mut j = 0;\n    for _ in\
    \ 1..n + m - 1 {\n        if j == m - 1 || (i < n - 1 && a[i + 1] + b[j] < a[i]\
    \ + b[j + 1]) {\n            i += 1;\n        } else {\n            j += 1;\n\
    \        }\n        c.push(a[i] + b[j]);\n    }\n    c\n}\n\npub fn maxplus_convolution_concave_and_concave<T>(a:\
    \ &[T], b: &[T]) -> Vec<T>\nwhere\n    T: Integer + Signed,\n{\n    assert!(is_concave(a));\n\
    \    assert!(is_concave(b));\n\n    let a = a.iter().map(|x| -*x).collect::<Vec<_>>();\n\
    \    let b = b.iter().map(|x| -*x).collect::<Vec<_>>();\n    let mut c = minplus_convolution_convex_and_convex(&a,\
    \ &b);\n    c.iter_mut().for_each(|x| *x = -*x);\n    c\n}\n"
  dependsOn:
  - crates/convolution/minplus_convolution/src/concave_and_arbitrary.rs
  - crates/convolution/minplus_convolution/src/convex_and_arbitrary.rs
  - crates/convolution/minplus_convolution/src/lib.rs
  isVerificationFile: false
  path: crates/convolution/minplus_convolution/src/convex_and_convex.rs
  requiredBy:
  - crates/convolution/minplus_convolution/src/lib.rs
  - crates/convolution/minplus_convolution/src/concave_and_arbitrary.rs
  - crates/convolution/minplus_convolution/src/convex_and_arbitrary.rs
  timestamp: '2025-03-20 09:27:03+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/convolution/min_plus_convolution_concave_arbitrary/src/main.rs
  - verify/library_checker/convolution/min_plus_convolution_convex_convex/src/main.rs
  - verify/library_checker/convolution/min_plus_convolution_convex_arbitrary/src/main.rs
documentation_of: crates/convolution/minplus_convolution/src/convex_and_convex.rs
layout: document
redirect_from:
- /library/crates/convolution/minplus_convolution/src/convex_and_convex.rs
- /library/crates/convolution/minplus_convolution/src/convex_and_convex.rs.html
title: crates/convolution/minplus_convolution/src/convex_and_convex.rs
---
