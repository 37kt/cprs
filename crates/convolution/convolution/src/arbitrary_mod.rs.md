---
data:
  _extendedDependsOn:
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
    path: crates/convolution/convolution/src/ntt_friendly.rs
    title: crates/convolution/convolution/src/ntt_friendly.rs
  _extendedRequiredBy:
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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use modint::ModInt;\nuse numeric_traits::Numeric;\nuse static_modint::StaticModInt;\n\
    \nuse crate::{convolution_naive, convolution_ntt_friendly, inv};\n\nconst NAIVE_THRESHOLD:\
    \ usize = 512; // \u96D1\n\nconst M1: u32 = 167_772_161;\nconst M2: u32 = 469_762_049;\n\
    const M3: u32 = 754_974_721;\ntype Fp1 = StaticModInt<M1>;\ntype Fp2 = StaticModInt<M2>;\n\
    type Fp3 = StaticModInt<M3>;\n\nconst M1INV_FP2: Fp2 = Fp2::from_raw(inv(M1, M2));\n\
    const M1INV_FP3: Fp3 = Fp3::from_raw(inv(M1, M3));\nconst M2INV_FP3: Fp3 = Fp3::from_raw(inv(M2,\
    \ M3));\n\npub fn convolution_arbitrary_mod<M: ModInt<Value = u32> + Numeric>(f:\
    \ &[M], g: &[M]) -> Vec<M> {\n    let n = f.len();\n    let m = g.len();\n   \
    \ if n.min(m) <= NAIVE_THRESHOLD {\n        return convolution_naive(f, g);\n\
    \    }\n\n    let f1 = f.iter().map(|&x| Fp1::new(x.val())).collect::<Vec<_>>();\n\
    \    let f2 = f.iter().map(|&x| Fp2::new(x.val())).collect::<Vec<_>>();\n    let\
    \ f3 = f.iter().map(|&x| Fp3::new(x.val())).collect::<Vec<_>>();\n    let g1 =\
    \ g.iter().map(|&x| Fp1::new(x.val())).collect::<Vec<_>>();\n    let g2 = g.iter().map(|&x|\
    \ Fp2::new(x.val())).collect::<Vec<_>>();\n    let g3 = g.iter().map(|&x| Fp3::new(x.val())).collect::<Vec<_>>();\n\
    \    let fg1 = convolution_ntt_friendly(&f1, &g1);\n    let fg2 = convolution_ntt_friendly(&f2,\
    \ &g2);\n    let fg3 = convolution_ntt_friendly(&f3, &g3);\n\n    let m1 = M::from(M1);\n\
    \    let m1m2 = m1 * M::from(M2);\n    fg1.into_iter()\n        .zip(fg2)\n  \
    \      .zip(fg3)\n        .map(|((e1, e2), e3)| {\n            let x1 = e1;\n\
    \            let x2 = (e2 - Fp2::from_raw(x1.val())) * M1INV_FP2;\n          \
    \  let x3 =\n                ((e3 - Fp3::from_raw(x1.val())) * M1INV_FP3 - Fp3::from_raw(x2.val()))\
    \ * M2INV_FP3;\n            M::from(x1.val()) + M::from(x2.val()) * m1 + M::from(x3.val())\
    \ * m1m2\n        })\n        .collect()\n}\n"
  dependsOn:
  - crates/convolution/convolution/src/lib.rs
  - crates/convolution/convolution/src/mod_2_64.rs
  - crates/convolution/convolution/src/naive.rs
  - crates/convolution/convolution/src/ntt.rs
  - crates/convolution/convolution/src/ntt_friendly.rs
  isVerificationFile: false
  path: crates/convolution/convolution/src/arbitrary_mod.rs
  requiredBy:
  - crates/polynomial/formal_power_series/src/lib.rs
  - crates/convolution/convolution/src/lib.rs
  - crates/convolution/convolution/src/ntt.rs
  - crates/convolution/convolution/src/naive.rs
  - crates/convolution/convolution/src/ntt_friendly.rs
  - crates/convolution/convolution/src/mod_2_64.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/tree/frequency_table_of_tree_distance/src/main.rs
  - verify/library_checker/convolution/convolution_mod_1000000007/src/main.rs
  - verify/library_checker/convolution/convolution_mod_2_64/src/main.rs
  - verify/library_checker/convolution/convolution_mod/src/main.rs
documentation_of: crates/convolution/convolution/src/arbitrary_mod.rs
layout: document
redirect_from:
- /library/crates/convolution/convolution/src/arbitrary_mod.rs
- /library/crates/convolution/convolution/src/arbitrary_mod.rs.html
title: crates/convolution/convolution/src/arbitrary_mod.rs
---
