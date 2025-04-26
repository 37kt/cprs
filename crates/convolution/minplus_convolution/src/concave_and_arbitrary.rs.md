---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/convolution/minplus_convolution/src/convex_and_arbitrary.rs
    title: crates/convolution/minplus_convolution/src/convex_and_arbitrary.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/minplus_convolution/src/convex_and_convex.rs
    title: crates/convolution/minplus_convolution/src/convex_and_convex.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/minplus_convolution/src/lib.rs
    title: crates/convolution/minplus_convolution/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/convolution/minplus_convolution/src/convex_and_arbitrary.rs
    title: crates/convolution/minplus_convolution/src/convex_and_arbitrary.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/minplus_convolution/src/convex_and_convex.rs
    title: crates/convolution/minplus_convolution/src/convex_and_convex.rs
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
  code: "use monotone_minima::monotone_minima;\nuse numeric_traits::{Inf, Integer,\
    \ Signed};\n\nuse crate::{is_concave, is_convex};\n\npub fn minplus_convolution_concave_and_arbitrary<T>(a:\
    \ &[T], b: &[T]) -> Vec<T>\nwhere\n    T: Integer + Inf,\n{\n    let n = a.len();\n\
    \    let m = b.len();\n    if n == 0 || m == 0 {\n        return vec![];\n   \
    \ }\n    assert!(is_concave(a));\n\n    let mut c = vec![T::MAX; n + m - 1];\n\
    \    divide_column(a, b, &mut c);\n    c\n}\n\npub fn minplus_convolution_arbitrary_and_concave<T>(a:\
    \ &[T], b: &[T]) -> Vec<T>\nwhere\n    T: Integer + Inf,\n{\n    minplus_convolution_concave_and_arbitrary(b,\
    \ a)\n}\n\n// \u9577\u65B9\u5F62\u9818\u57DF\u304C\u53D6\u308C\u308B\u3088\u3046\
    \u306B\u306A\u308B\u307E\u3067\u5217\u3092\u5206\u5272\nfn divide_column<T>(a:\
    \ &[T], b: &[T], c: &mut [T])\nwhere\n    T: Integer,\n{\n    let n = a.len();\n\
    \    let m = b.len();\n    if n == 0 || m == 0 {\n        return;\n    }\n\n \
    \   if n + 2 <= m {\n        let mm = m / 2;\n        divide_column(a, &b[..mm],\
    \ &mut c[..n + mm - 1]);\n        divide_column(a, &b[mm..], &mut c[mm..]);\n\
    \    } else {\n        solve_rectangle(a, b, &mut c[m - 1..m - 1 + n]);\n    \
    \    divide_lower_triangle(&a[..m - 1], &b[..m - 1], &mut c[..m - 1]);\n     \
    \   divide_upper_triangle(&a[n - (m - 1)..], &b[1..], &mut c[n..]);\n    }\n}\n\
    \nfn solve_rectangle<T>(a: &[T], b: &[T], c: &mut [T])\nwhere\n    T: Integer,\n\
    {\n    let n = a.len();\n    let m = b.len();\n    if n == 0 || m == 0 {\n   \
    \     return;\n    }\n\n    let f = |i: usize, j: usize| a[i + j] + b[m - 1 -\
    \ j];\n    let select = |i: usize, j: usize, k: usize| f(i, j) > f(i, k);\n  \
    \  let argmin = monotone_minima(n - (m - 1), m, select);\n    for i in 0..n -\
    \ (m - 1) {\n        c[i] = c[i].min(f(i, argmin[i]));\n    }\n}\n\n// \u4E0A\u5074\
    \u306E\u4E0B\u4E09\u89D2\u884C\u5217\u3092\u51E6\u7406\nfn divide_lower_triangle<T>(a:\
    \ &[T], b: &[T], c: &mut [T])\nwhere\n    T: Integer,\n{\n    let n = a.len();\n\
    \    let m = b.len();\n    assert_eq!(n, m);\n    assert_eq!(n, c.len());\n  \
    \  if n == 0 {\n        return;\n    } else if n == 1 {\n        c[0] = c[0].min(a[0]\
    \ + b[0]);\n        return;\n    }\n\n    let nm = n / 2;\n    solve_rectangle(a,\
    \ &b[..=nm], &mut c[nm..]);\n    divide_lower_triangle(&a[..nm], &b[..nm], &mut\
    \ c[..nm]);\n    divide_lower_triangle(&a[..n - nm - 1], &b[nm + 1..], &mut c[nm\
    \ + 1..]);\n}\n\n// \u4E0B\u5074\u306E\u4E0A\u4E09\u89D2\u884C\u5217\u3092\u51E6\
    \u7406\nfn divide_upper_triangle<T>(a: &[T], b: &[T], c: &mut [T])\nwhere\n  \
    \  T: Integer,\n{\n    let n = a.len();\n    let m = b.len();\n    assert_eq!(n,\
    \ m);\n    assert_eq!(n, c.len());\n    if n == 0 || m == 0 {\n        return;\n\
    \    } else if n == 1 {\n        c[0] = c[0].min(a[0] + b[0]);\n        return;\n\
    \    }\n\n    let nm = n / 2;\n    solve_rectangle(a, &b[nm..], &mut c[..=nm]);\n\
    \    divide_upper_triangle(&a[n - nm..], &b[..nm], &mut c[..nm]);\n    divide_upper_triangle(&a[nm\
    \ + 1..], &b[nm + 1..], &mut c[nm + 1..])\n}\n\npub fn maxplus_convolution_convex_and_arbitrary<T>(a:\
    \ &[T], b: &[T]) -> Vec<T>\nwhere\n    T: Integer + Signed + Inf,\n{\n    assert!(is_convex(a));\n\
    \    let a = a.iter().map(|x| -*x).collect::<Vec<_>>();\n    let b = b.iter().map(|x|\
    \ -*x).collect::<Vec<_>>();\n    let mut c = minplus_convolution_concave_and_arbitrary(&a,\
    \ &b);\n    c.iter_mut().for_each(|x| *x = -*x);\n    c\n}\n\npub fn maxplus_convolution_arbitrary_and_convex<T>(a:\
    \ &[T], b: &[T]) -> Vec<T>\nwhere\n    T: Integer + Signed + Inf,\n{\n    maxplus_convolution_convex_and_arbitrary(b,\
    \ a)\n}\n"
  dependsOn:
  - crates/convolution/minplus_convolution/src/convex_and_arbitrary.rs
  - crates/convolution/minplus_convolution/src/convex_and_convex.rs
  - crates/convolution/minplus_convolution/src/lib.rs
  isVerificationFile: false
  path: crates/convolution/minplus_convolution/src/concave_and_arbitrary.rs
  requiredBy:
  - crates/convolution/minplus_convolution/src/lib.rs
  - crates/convolution/minplus_convolution/src/convex_and_convex.rs
  - crates/convolution/minplus_convolution/src/convex_and_arbitrary.rs
  timestamp: '2025-03-20 09:27:03+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/convolution/min_plus_convolution_concave_arbitrary/src/main.rs
  - verify/library_checker/convolution/min_plus_convolution_convex_convex/src/main.rs
  - verify/library_checker/convolution/min_plus_convolution_convex_arbitrary/src/main.rs
documentation_of: crates/convolution/minplus_convolution/src/concave_and_arbitrary.rs
layout: document
redirect_from:
- /library/crates/convolution/minplus_convolution/src/concave_and_arbitrary.rs
- /library/crates/convolution/minplus_convolution/src/concave_and_arbitrary.rs.html
title: crates/convolution/minplus_convolution/src/concave_and_arbitrary.rs
---
