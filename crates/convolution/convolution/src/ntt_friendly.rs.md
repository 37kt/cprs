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
    path: crates/convolution/convolution/src/naive.rs
    title: crates/convolution/convolution/src/naive.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/ntt.rs
    title: crates/convolution/convolution/src/ntt.rs
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
    path: crates/convolution/convolution/src/naive.rs
    title: crates/convolution/convolution/src/naive.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/ntt.rs
    title: crates/convolution/convolution/src/ntt.rs
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
  code: "use numeric_traits::Integer;\nuse static_modint::StaticModInt;\n\nuse crate::{convolution_naive,\
    \ ntt, ntt_inv};\n\nconst NAIVE_THRESHOLD: usize = 128; // \u96D1\n\npub fn convolution_ntt_friendly<const\
    \ P: u32>(\n    f: &[StaticModInt<P>],\n    g: &[StaticModInt<P>],\n) -> Vec<StaticModInt<P>>\
    \ {\n    let n = f.len();\n    let m = g.len();\n    if n == 0 || m == 0 {\n \
    \       return vec![];\n    } else if n.min(m) <= NAIVE_THRESHOLD {\n        return\
    \ convolution_naive(f, g);\n    }\n\n    let l = n + m - 1;\n    let sz = l.ceil_pow2();\n\
    \    let mut f = f.to_vec();\n    let mut g = g.to_vec();\n    f.resize(sz, 0.into());\n\
    \    g.resize(sz, 0.into());\n    ntt(&mut f);\n    ntt(&mut g);\n    for (x,\
    \ y) in f.iter_mut().zip(g) {\n        *x *= y;\n    }\n    ntt_inv(&mut f);\n\
    \    f.truncate(l);\n    f\n}\n"
  dependsOn:
  - crates/convolution/convolution/src/arbitrary_mod.rs
  - crates/convolution/convolution/src/lib.rs
  - crates/convolution/convolution/src/mod_2_64.rs
  - crates/convolution/convolution/src/naive.rs
  - crates/convolution/convolution/src/ntt.rs
  isVerificationFile: false
  path: crates/convolution/convolution/src/ntt_friendly.rs
  requiredBy:
  - crates/polynomial/formal_power_series/src/lib.rs
  - crates/convolution/convolution/src/lib.rs
  - crates/convolution/convolution/src/arbitrary_mod.rs
  - crates/convolution/convolution/src/ntt.rs
  - crates/convolution/convolution/src/naive.rs
  - crates/convolution/convolution/src/mod_2_64.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/tree/frequency_table_of_tree_distance/src/main.rs
  - verify/library_checker/convolution/convolution_mod_1000000007/src/main.rs
  - verify/library_checker/convolution/convolution_mod_2_64/src/main.rs
  - verify/library_checker/convolution/convolution_mod/src/main.rs
documentation_of: crates/convolution/convolution/src/ntt_friendly.rs
layout: document
redirect_from:
- /library/crates/convolution/convolution/src/ntt_friendly.rs
- /library/crates/convolution/convolution/src/ntt_friendly.rs.html
title: crates/convolution/convolution/src/ntt_friendly.rs
---
