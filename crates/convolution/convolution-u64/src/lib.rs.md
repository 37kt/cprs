---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution-ntt-friendly/src/lib.rs
    title: crates/convolution/convolution-ntt-friendly/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/convolution_mod_2_64/src/main.rs
    title: verify/convolution_mod_2_64/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/frequency_table_of_tree_distance/src/main.rs
    title: verify/frequency_table_of_tree_distance/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use convolution_ntt_friendly::convolution_ntt_friendly;\nuse modint::StaticModInt;\n\
    \nconst M1: u32 = 167_772_161;\nconst M2: u32 = 469_762_049;\nconst M3: u32 =\
    \ 754_974_721;\nconst M4: u32 = 880_803_841;\nconst M5: u32 = 998_244_353;\ntype\
    \ Fp1 = StaticModInt<M1>;\ntype Fp2 = StaticModInt<M2>;\ntype Fp3 = StaticModInt<M3>;\n\
    type Fp4 = StaticModInt<M4>;\ntype Fp5 = StaticModInt<M5>;\n\nconst fn pow(x:\
    \ u32, mut n: u32, m: u32) -> u32 {\n    if m == 1 {\n        return 0;\n    }\n\
    \    let mut r = 1u64;\n    let mut y = (x % m) as u64;\n    while n != 0 {\n\
    \        if n & 1 != 0 {\n            r = r * y % m as u64;\n        }\n     \
    \   y = y * y % m as u64;\n        n >>= 1;\n    }\n    r as u32\n}\n\nconst fn\
    \ inv(x: u32, m: u32) -> u32 {\n    pow(x, m - 2, m)\n}\n\nconst M1INV_FP2: Fp2\
    \ = Fp2::raw(inv(M1, M2));\nconst M1INV_FP3: Fp3 = Fp3::raw(inv(M1, M3));\nconst\
    \ M1INV_FP4: Fp4 = Fp4::raw(inv(M1, M4));\nconst M1INV_FP5: Fp5 = Fp5::raw(inv(M1,\
    \ M5));\nconst M2INV_FP3: Fp3 = Fp3::raw(inv(M2, M3));\nconst M2INV_FP4: Fp4 =\
    \ Fp4::raw(inv(M2, M4));\nconst M2INV_FP5: Fp5 = Fp5::raw(inv(M2, M5));\nconst\
    \ M3INV_FP4: Fp4 = Fp4::raw(inv(M3, M4));\nconst M3INV_FP5: Fp5 = Fp5::raw(inv(M3,\
    \ M5));\nconst M4INV_FP5: Fp5 = Fp5::raw(inv(M4, M5));\nconst P_M1: u64 = M1 as\
    \ u64;\nconst P_M1M2: u64 = P_M1.wrapping_mul(M2 as u64);\nconst P_M1M2M3: u64\
    \ = P_M1M2.wrapping_mul(M3 as u64);\nconst P_M1M2M3M4: u64 = P_M1M2M3.wrapping_mul(M4\
    \ as u64);\n\npub fn convolution_u64(a: &[u64], b: &[u64]) -> Vec<u64> {\n   \
    \ if a.is_empty() || b.is_empty() {\n        return vec![];\n    }\n    let a1\
    \ = a.iter().map(|&x| Fp1::new(x)).collect::<Vec<_>>();\n    let a2 = a.iter().map(|&x|\
    \ Fp2::new(x)).collect::<Vec<_>>();\n    let a3 = a.iter().map(|&x| Fp3::new(x)).collect::<Vec<_>>();\n\
    \    let a4 = a.iter().map(|&x| Fp4::new(x)).collect::<Vec<_>>();\n    let a5\
    \ = a.iter().map(|&x| Fp5::new(x)).collect::<Vec<_>>();\n    let b1 = b.iter().map(|&x|\
    \ Fp1::new(x)).collect::<Vec<_>>();\n    let b2 = b.iter().map(|&x| Fp2::new(x)).collect::<Vec<_>>();\n\
    \    let b3 = b.iter().map(|&x| Fp3::new(x)).collect::<Vec<_>>();\n    let b4\
    \ = b.iter().map(|&x| Fp4::new(x)).collect::<Vec<_>>();\n    let b5 = b.iter().map(|&x|\
    \ Fp5::new(x)).collect::<Vec<_>>();\n    let a1 = convolution_ntt_friendly(a1,\
    \ b1);\n    let a2 = convolution_ntt_friendly(a2, b2);\n    let a3 = convolution_ntt_friendly(a3,\
    \ b3);\n    let a4 = convolution_ntt_friendly(a4, b4);\n    let a5 = convolution_ntt_friendly(a5,\
    \ b5);\n    a1.iter()\n        .zip(a2.iter())\n        .zip(a3.iter())\n    \
    \    .zip(a4.iter())\n        .zip(a5.iter())\n        .map(|((((&e1, &e2), &e3),\
    \ &e4), &e5)| {\n            let x1 = e1;\n            let x2 = (e2 - Fp2::raw(x1.val()))\
    \ * M1INV_FP2;\n            let x3 = ((e3 - Fp3::raw(x1.val())) * M1INV_FP3 -\
    \ Fp3::raw(x2.val())) * M2INV_FP3;\n            let x4 = (((e4 - Fp4::raw(x1.val()))\
    \ * M1INV_FP4 - Fp4::raw(x2.val())) * M2INV_FP4\n                - Fp4::raw(x3.val()))\n\
    \                * M3INV_FP4;\n            let x5 = ((((e5 - Fp5::raw(x1.val()))\
    \ * M1INV_FP5 - Fp5::raw(x2.val())) * M2INV_FP5\n                - Fp5::raw(x3.val()))\n\
    \                * M3INV_FP5\n                - Fp5::raw(x4.val()))\n        \
    \        * M4INV_FP5;\n            (x1.val() as u64)\n                .wrapping_add((x2.val()\
    \ as u64).wrapping_mul(P_M1))\n                .wrapping_add((x3.val() as u64).wrapping_mul(P_M1M2))\n\
    \                .wrapping_add((x4.val() as u64).wrapping_mul(P_M1M2M3))\n   \
    \             .wrapping_add((x5.val() as u64).wrapping_mul(P_M1M2M3M4))\n    \
    \    })\n        .collect()\n}\n"
  dependsOn:
  - crates/convolution/convolution-ntt-friendly/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  isVerificationFile: false
  path: crates/convolution/convolution-u64/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-23 05:51:44+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/convolution_mod_2_64/src/main.rs
  - verify/frequency_table_of_tree_distance/src/main.rs
documentation_of: crates/convolution/convolution-u64/src/lib.rs
layout: document
redirect_from:
- /library/crates/convolution/convolution-u64/src/lib.rs
- /library/crates/convolution/convolution-u64/src/lib.rs.html
title: crates/convolution/convolution-u64/src/lib.rs
---
