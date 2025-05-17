---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/arbitrary_mod.rs
    title: crates/convolution/convolution/src/arbitrary_mod.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/lib.rs
    title: crates/convolution/convolution/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/mod_2_64.rs
    title: crates/convolution/convolution/src/mod_2_64.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/ntt.rs
    title: crates/convolution/convolution/src/ntt.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/ntt_friendly.rs
    title: crates/convolution/convolution/src/ntt_friendly.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/arbitrary_mod.rs
    title: crates/convolution/convolution/src/arbitrary_mod.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/lib.rs
    title: crates/convolution/convolution/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/mod_2_64.rs
    title: crates/convolution/convolution/src/mod_2_64.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/ntt.rs
    title: crates/convolution/convolution/src/ntt.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/ntt_friendly.rs
    title: crates/convolution/convolution/src/ntt_friendly.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/lib.rs
    title: crates/polynomial/formal_power_series/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/convolution/convolution_mod/src/main.rs
    title: verify/library_checker/convolution/convolution_mod/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/convolution/convolution_mod_1000000007/src/main.rs
    title: verify/library_checker/convolution/convolution_mod_1000000007/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/convolution/convolution_mod_2_64/src/main.rs
    title: verify/library_checker/convolution/convolution_mod_2_64/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/frequency_table_of_tree_distance/src/main.rs
    title: verify/library_checker/tree/frequency_table_of_tree_distance/src/main.rs
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
  code: "use numeric_traits::Numeric;\n\npub fn convolution_naive<T: Numeric>(f: &[T],\
    \ g: &[T]) -> Vec<T> {\n    let n = f.len();\n    let m = g.len();\n    if n >\
    \ m {\n        return convolution_naive(g, f);\n    } else if n == 0 {\n     \
    \   return vec![];\n    }\n\n    let mut h = vec![T::zero(); n + m - 1];\n   \
    \ for i in 0..n {\n        for j in 0..m {\n            h[i + j] += f[i] * g[j];\n\
    \        }\n    }\n    h\n}\n"
  dependsOn:
  - crates/convolution/convolution/src/arbitrary_mod.rs
  - crates/convolution/convolution/src/lib.rs
  - crates/convolution/convolution/src/mod_2_64.rs
  - crates/convolution/convolution/src/ntt.rs
  - crates/convolution/convolution/src/ntt_friendly.rs
  isVerificationFile: false
  path: crates/convolution/convolution/src/naive.rs
  requiredBy:
  - crates/convolution/convolution/src/mod_2_64.rs
  - crates/convolution/convolution/src/lib.rs
  - crates/convolution/convolution/src/arbitrary_mod.rs
  - crates/convolution/convolution/src/ntt.rs
  - crates/convolution/convolution/src/ntt_friendly.rs
  - crates/polynomial/formal_power_series/src/lib.rs
  timestamp: '2025-05-17 08:18:49+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/convolution/convolution_mod/src/main.rs
  - verify/library_checker/convolution/convolution_mod_2_64/src/main.rs
  - verify/library_checker/convolution/convolution_mod_1000000007/src/main.rs
  - verify/library_checker/tree/frequency_table_of_tree_distance/src/main.rs
documentation_of: crates/convolution/convolution/src/naive.rs
layout: document
redirect_from:
- /library/crates/convolution/convolution/src/naive.rs
- /library/crates/convolution/convolution/src/naive.rs.html
title: crates/convolution/convolution/src/naive.rs
---
