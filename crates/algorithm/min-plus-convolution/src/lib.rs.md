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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use monotone_minima::monotone_minima;\n\nfn check_convex<T>(a: &[T]) -> bool\n\
    where\n    T: Copy + std::ops::Add<Output = T> + std::cmp::Ord,\n{\n    for i\
    \ in 2..a.len() {\n        if a[i - 1] + a[i - 1] > a[i] + a[i - 2] {\n      \
    \      return false;\n        }\n    }\n    true\n}\n\n/// \u4EFB\u610F\u306E\u95A2\
    \u6570\u3068\u51F8\u95A2\u6570\u306E min-plus convolution\n///\n/// # \u6982\u8981\
    \n/// - \u4EFB\u610F\u306E\u95A2\u6570 `a` \u3068\u51F8\u95A2\u6570 `b` \u306E\
    \ min-plus convolution \u3092\u8A08\u7B97\n/// - \u51F8\u95A2\u6570 `b` \u306F\
    \ `b[i+1] - b[i] \u2264 b[i+2] - b[i+1]` \u3092\u6E80\u305F\u3059\n///\n/// #\
    \ \u623B\u308A\u5024    \n/// - `res[i] = min(a[j] + b[i-j]) (0 \u2264 j \u2264\
    \ i)`\n///\n/// # \u8A08\u7B97\u91CF\n/// (TODO)\npub fn min_plus_convolution_arbitrary_convex<T>(a:\
    \ &[T], b: &[T]) -> Vec<T>\nwhere\n    T: Copy + std::ops::Add<Output = T> + std::cmp::Ord,\n\
    {\n    let n = a.len();\n    let m = b.len();\n    if n == 0 || m == 0 {\n   \
    \     return vec![];\n    }\n    assert!(check_convex(b));\n    let cmp = |i:\
    \ usize, j: usize, k: usize| {\n        if i < k {\n            false\n      \
    \  } else if i >= m + j {\n            true\n        } else {\n            a[j]\
    \ + b[i - j] >= a[k] + b[i - k]\n        }\n    };\n    let c = monotone_minima(n\
    \ + m - 1, n, cmp);\n    (0..n + m - 1).map(|i| a[c[i]] + b[i - c[i]]).collect()\n\
    }\n\n/// \u51F8\u95A2\u6570\u3068\u4EFB\u610F\u306E\u95A2\u6570\u306E min-plus\
    \ convolution\n///\n/// # \u6982\u8981\n/// - \u51F8\u95A2\u6570 `a` \u3068\u4EFB\
    \u610F\u306E\u95A2\u6570 `b` \u306E min-plus convolution \u3092\u8A08\u7B97\n\
    /// - \u51F8\u95A2\u6570 `a` \u306F `a[i+1] - a[i] \u2264 a[i+2] - a[i+1]` \u3092\
    \u6E80\u305F\u3059\n///\n/// # \u623B\u308A\u5024\n/// - `res[i] = min(a[j] +\
    \ b[i-j]) (0 \u2264 j \u2264 i)`\n///\n/// # \u8A08\u7B97\u91CF\n/// (TODO)\n\
    pub fn min_plus_convolution_convex_arbitrary<T>(a: &[T], b: &[T]) -> Vec<T>\n\
    where\n    T: Copy + std::ops::Add<Output = T> + std::cmp::Ord,\n{\n    assert!(check_convex(a));\n\
    \    min_plus_convolution_arbitrary_convex(b, a)\n}\n\n/// \u51F8\u95A2\u6570\u3068\
    \u51F8\u95A2\u6570\u306E min-plus convolution\n///\n/// # \u6982\u8981\n/// -\
    \ \u51F8\u95A2\u6570 `a` \u3068\u51F8\u95A2\u6570 `b` \u306E min-plus convolution\
    \ \u3092\u8A08\u7B97\n/// - \u51F8\u95A2\u6570 `a` \u306F `a[i+1] - a[i] \u2264\
    \ a[i+2] - a[i+1]` \u3092\u6E80\u305F\u3059\n/// - \u51F8\u95A2\u6570 `b` \u306F\
    \ `b[i+1] - b[i] \u2264 b[i+2] - b[i+1]` \u3092\u6E80\u305F\u3059\n///\n/// #\
    \ \u623B\u308A\u5024\n/// - `res[i] = min(a[j] + b[i-j]) (0 \u2264 j \u2264 i)`\n\
    ///\n/// # \u8A08\u7B97\u91CF\n/// - O(n + m)\npub fn min_plus_convolution_convex_convex<T>(a:\
    \ &[T], b: &[T]) -> Vec<T>\nwhere\n    T: Copy + std::ops::Add<Output = T> + std::cmp::Ord,\n\
    {\n    let n = a.len();\n    let m = b.len();\n    if n == 0 || m == 0 {\n   \
    \     return vec![];\n    }\n    assert!(check_convex(a));\n    assert!(check_convex(b));\n\
    \    let mut c = vec![a[0] + b[0]; n + m - 1];\n    let mut i = 0;\n    let mut\
    \ j = 0;\n    for k in 1..n + m - 1 {\n        if j == m - 1 || (i != n - 1 &&\
    \ a[i + 1] + b[j] < a[i] + b[j + 1]) {\n            i += 1;\n        } else {\n\
    \            j += 1;\n        }\n        c[k] = a[i] + b[j];\n    }\n    c\n}\n"
  dependsOn:
  - crates/algorithm/monotone-minima/src/lib.rs
  isVerificationFile: false
  path: crates/algorithm/min-plus-convolution/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-25 03:34:39+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/min_plus_convolution_convex_arbitrary/src/main.rs
  - verify/min_plus_convolution_convex_convex/src/main.rs
documentation_of: crates/algorithm/min-plus-convolution/src/lib.rs
layout: document
redirect_from:
- /library/crates/algorithm/min-plus-convolution/src/lib.rs
- /library/crates/algorithm/min-plus-convolution/src/lib.rs.html
title: crates/algorithm/min-plus-convolution/src/lib.rs
---
