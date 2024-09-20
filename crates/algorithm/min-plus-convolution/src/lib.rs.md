---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algorithm/monotone-minima/src/lib.rs
    title: crates/algorithm/monotone-minima/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/min_plus_convolution_convex_arbitrary/src/main.rs
    title: verify/min_plus_convolution_convex_arbitrary/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/min_plus_convolution_convex_convex/src/main.rs
    title: verify/min_plus_convolution_convex_convex/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use monotone_minima::monotone_minima;\n\n/// b[i+1] - b[i] <= b[i+2] - b[i+1]\n\
    pub fn min_plus_convolution_arbitrary_convex<T>(a: &[T], b: &[T]) -> Vec<T>\n\
    where\n    T: Copy + std::ops::Add<Output = T> + std::cmp::Ord,\n{\n    let n\
    \ = a.len();\n    let m = b.len();\n    let cmp = |i: usize, j: usize, k: usize|\
    \ {\n        if i < k {\n            false\n        } else if i >= m + j {\n \
    \           true\n        } else {\n            a[j] + b[i - j] >= a[k] + b[i\
    \ - k]\n        }\n    };\n    let c = monotone_minima(n + m - 1, n, cmp);\n \
    \   (0..n + m - 1).map(|i| a[c[i]] + b[i - c[i]]).collect()\n}\n\n/// a[i+1] -\
    \ a[i] <= a[i+2] - a[i+1]\npub fn min_plus_convolution_convex_arbitrary<T>(a:\
    \ &[T], b: &[T]) -> Vec<T>\nwhere\n    T: Copy + std::ops::Add<Output = T> + std::cmp::Ord,\n\
    {\n    min_plus_convolution_arbitrary_convex(b, a)\n}\n\n/// a[i+1] - a[i] <=\
    \ a[i+2] - a[i+1]\n/// b[i+1] - b[i] <= b[i+2] - b[i+1]\npub fn min_plus_convolution_convex_convex<T>(a:\
    \ &[T], b: &[T]) -> Vec<T>\nwhere\n    T: Copy + std::ops::Add<Output = T> + std::cmp::Ord,\n\
    {\n    let n = a.len();\n    let m = b.len();\n    let mut c = vec![a[0] + b[0];\
    \ n + m - 1];\n    let mut i = 0;\n    let mut j = 0;\n    for k in 1..n + m -\
    \ 1 {\n        if j == m - 1 || (i != n - 1 && a[i + 1] + b[j] < a[i] + b[j +\
    \ 1]) {\n            i += 1;\n        } else {\n            j += 1;\n        }\n\
    \        c[k] = a[i] + b[j];\n    }\n    c\n}\n"
  dependsOn:
  - crates/algorithm/monotone-minima/src/lib.rs
  isVerificationFile: false
  path: crates/algorithm/min-plus-convolution/src/lib.rs
  requiredBy: []
  timestamp: '2024-04-07 09:46:12+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/min_plus_convolution_convex_convex/src/main.rs
  - verify/min_plus_convolution_convex_arbitrary/src/main.rs
documentation_of: crates/algorithm/min-plus-convolution/src/lib.rs
layout: document
redirect_from:
- /library/crates/algorithm/min-plus-convolution/src/lib.rs
- /library/crates/algorithm/min-plus-convolution/src/lib.rs.html
title: crates/algorithm/min-plus-convolution/src/lib.rs
---
