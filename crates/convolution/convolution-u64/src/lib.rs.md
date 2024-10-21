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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use convolution_ntt_friendly::convolution_ntt_friendly;\nuse modint::StaticModInt;\n\
    \nconst M1: u32 = 167_772_161;\nconst M2: u32 = 469_762_049;\nconst M3: u32 =\
    \ 754_974_721;\nconst M4: u32 = 880_803_841;\nconst M5: u32 = 998_244_353;\ntype\
    \ Fp1 = StaticModInt<M1>;\ntype Fp2 = StaticModInt<M2>;\ntype Fp3 = StaticModInt<M3>;\n\
    type Fp4 = StaticModInt<M4>;\ntype Fp5 = StaticModInt<M5>;\n\npub fn convolution_u64(a:\
    \ &[u64], b: &[u64]) -> Vec<u64> {\n    if a.is_empty() || b.is_empty() {\n  \
    \      return vec![];\n    }\n    let a1 = a.iter().map(|&x| Fp1::new(x)).collect::<Vec<_>>();\n\
    \    let a2 = a.iter().map(|&x| Fp2::new(x)).collect::<Vec<_>>();\n    let a3\
    \ = a.iter().map(|&x| Fp3::new(x)).collect::<Vec<_>>();\n    let a4 = a.iter().map(|&x|\
    \ Fp4::new(x)).collect::<Vec<_>>();\n    let a5 = a.iter().map(|&x| Fp5::new(x)).collect::<Vec<_>>();\n\
    \    let b1 = b.iter().map(|&x| Fp1::new(x)).collect::<Vec<_>>();\n    let b2\
    \ = b.iter().map(|&x| Fp2::new(x)).collect::<Vec<_>>();\n    let b3 = b.iter().map(|&x|\
    \ Fp3::new(x)).collect::<Vec<_>>();\n    let b4 = b.iter().map(|&x| Fp4::new(x)).collect::<Vec<_>>();\n\
    \    let b5 = b.iter().map(|&x| Fp5::new(x)).collect::<Vec<_>>();\n    let a1\
    \ = convolution_ntt_friendly(a1, b1);\n    let a2 = convolution_ntt_friendly(a2,\
    \ b2);\n    let a3 = convolution_ntt_friendly(a3, b3);\n    let a4 = convolution_ntt_friendly(a4,\
    \ b4);\n    let a5 = convolution_ntt_friendly(a5, b5);\n    a1.iter()\n      \
    \  .zip(a2.iter())\n        .zip(a3.iter())\n        .zip(a4.iter())\n       \
    \ .zip(a5.iter())\n        .map(|((((&e1, &e2), &e3), &e4), &e5)| {\n        \
    \    let x1 = e1;\n            let x2 = (e2 - Fp2::new(x1.val())) * Fp2::new(Fp1::modulus()).inv();\n\
    \            let x3 = ((e3 - Fp3::new(x1.val())) * Fp3::new(Fp1::modulus()).inv()\n\
    \                - Fp3::new(x2.val()))\n                * Fp3::new(Fp2::modulus()).inv();\n\
    \            let x4 = (((e4 - Fp4::new(x1.val())) * Fp4::new(Fp1::modulus()).inv()\n\
    \                - Fp4::new(x2.val()))\n                * Fp4::new(Fp2::modulus()).inv()\n\
    \                - Fp4::new(x3.val()))\n                * Fp4::new(Fp3::modulus()).inv();\n\
    \            let x5 = ((((e5 - Fp5::new(x1.val())) * Fp5::new(Fp1::modulus()).inv()\n\
    \                - Fp5::new(x2.val()))\n                * Fp5::new(Fp2::modulus()).inv()\n\
    \                - Fp5::new(x3.val()))\n                * Fp5::new(Fp3::modulus()).inv()\n\
    \                - Fp5::new(x4.val()))\n                * Fp5::new(Fp4::modulus()).inv();\n\
    \            (x1.val() as u64)\n                .wrapping_add((x2.val() as u64).wrapping_mul(Fp1::modulus()\
    \ as u64))\n                .wrapping_add(\n                    (x3.val() as u64)\n\
    \                        .wrapping_mul((Fp1::modulus() as u64).wrapping_mul(Fp2::modulus()\
    \ as u64)),\n                )\n                .wrapping_add(\n             \
    \       (x4.val() as u64).wrapping_mul(\n                        (Fp1::modulus()\
    \ as u64)\n                            .wrapping_mul(Fp2::modulus() as u64)\n\
    \                            .wrapping_mul(Fp3::modulus() as u64),\n         \
    \           ),\n                )\n                .wrapping_add(\n          \
    \          (x5.val() as u64).wrapping_mul(\n                        (Fp1::modulus()\
    \ as u64)\n                            .wrapping_mul(Fp2::modulus() as u64)\n\
    \                            .wrapping_mul(Fp3::modulus() as u64)\n          \
    \                  .wrapping_mul(Fp4::modulus() as u64),\n                   \
    \ ),\n                )\n        })\n        .collect()\n}\n"
  dependsOn:
  - crates/convolution/convolution-ntt-friendly/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  isVerificationFile: false
  path: crates/convolution/convolution-u64/src/lib.rs
  requiredBy: []
  timestamp: '2024-06-13 08:47:29+09:00'
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
