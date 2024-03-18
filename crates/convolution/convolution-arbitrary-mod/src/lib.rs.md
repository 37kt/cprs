---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/convolution/convolution-naive/src/lib.rs
    title: crates/convolution/convolution-naive/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution-ntt-friendly/src/lib.rs
    title: crates/convolution/convolution-ntt-friendly/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal-power-series/src/lib.rs
    title: crates/polynomial/formal-power-series/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/convolution_mod_1000000007/src/main.rs
    title: verify/convolution_mod_1000000007/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use convolution_naive::convolution_naive;\nuse convolution_ntt_friendly::convolution_ntt_friendly;\n\
    use modint::{ModInt, StaticModInt};\n\nconst M1: u32 = 167_772_161;\nconst M2:\
    \ u32 = 469_762_049;\nconst M3: u32 = 754_974_721;\ntype Fp1 = StaticModInt<M1>;\n\
    type Fp2 = StaticModInt<M2>;\ntype Fp3 = StaticModInt<M3>;\n\npub fn convolution_arbitrary_mod<T:\
    \ ModInt>(a: &[T], b: &[T]) -> Vec<T> {\n    if a.len().min(b.len()) < 60 {\n\
    \        return convolution_naive(a, b);\n    }\n    let a1 = a.iter().map(|&x|\
    \ Fp1::new(x.val())).collect::<Vec<_>>();\n    let a2 = a.iter().map(|&x| Fp2::new(x.val())).collect::<Vec<_>>();\n\
    \    let a3 = a.iter().map(|&x| Fp3::new(x.val())).collect::<Vec<_>>();\n    let\
    \ b1 = b.iter().map(|&x| Fp1::new(x.val())).collect::<Vec<_>>();\n    let b2 =\
    \ b.iter().map(|&x| Fp2::new(x.val())).collect::<Vec<_>>();\n    let b3 = b.iter().map(|&x|\
    \ Fp3::new(x.val())).collect::<Vec<_>>();\n    let a1 = convolution_ntt_friendly(a1,\
    \ b1);\n    let a2 = convolution_ntt_friendly(a2, b2);\n    let a3 = convolution_ntt_friendly(a3,\
    \ b3);\n    a1.iter()\n        .zip(a2.iter())\n        .zip(a3.iter())\n    \
    \    .map(|((&e1, &e2), &e3)| {\n            let x1 = e1;\n            let x2\
    \ = (e2 - Fp2::new(x1.val())) * Fp2::new(Fp1::modulus()).inv();\n            let\
    \ x3 = ((e3 - Fp3::new(x1.val())) * Fp3::new(Fp1::modulus()).inv()\n         \
    \       - Fp3::new(x2.val()))\n                * Fp3::new(Fp2::modulus()).inv();\n\
    \            T::from(x1.val())\n                + T::from(x2.val()) * T::from(Fp1::modulus())\n\
    \                + T::from(x3.val()) * T::from(Fp1::modulus()) * T::from(Fp2::modulus())\n\
    \        })\n        .collect()\n}\n"
  dependsOn:
  - crates/convolution/convolution-naive/src/lib.rs
  - crates/convolution/convolution-ntt-friendly/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  isVerificationFile: false
  path: crates/convolution/convolution-arbitrary-mod/src/lib.rs
  requiredBy:
  - crates/polynomial/formal-power-series/src/lib.rs
  timestamp: '2024-03-18 01:19:47+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/convolution_mod_1000000007/src/main.rs
documentation_of: crates/convolution/convolution-arbitrary-mod/src/lib.rs
layout: document
redirect_from:
- /library/crates/convolution/convolution-arbitrary-mod/src/lib.rs
- /library/crates/convolution/convolution-arbitrary-mod/src/lib.rs.html
title: crates/convolution/convolution-arbitrary-mod/src/lib.rs
---
