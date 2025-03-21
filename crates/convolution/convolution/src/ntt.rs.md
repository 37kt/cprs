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
    path: crates/convolution/convolution/src/naive.rs
    title: crates/convolution/convolution/src/naive.rs
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
  code: "use numeric_traits::Integer;\nuse static_modint::StaticModInt;\n\npub fn\
    \ ntt<const P: u32>(f: &mut [StaticModInt<P>]) {\n    assert!(StaticModInt::<P>::IS_NTT_FRIENDLY);\n\
    \n    let n = f.len();\n    let lg = n.floor_log2();\n    assert_eq!(n, 1 << lg,\
    \ \"length must be a power of 2\");\n\n    let mut l = 0;\n    if (lg - l) % 2\
    \ == 1 {\n        let p = 1 << lg - l - 1;\n        let mut rot = StaticModInt::from_raw(1);\n\
    \        for (s, b) in f.chunks_exact_mut(p << 1).enumerate() {\n            let\
    \ (b0, b1) = b.split_at_mut(p);\n            for (x, y) in b0.iter_mut().zip(b1)\
    \ {\n                let l = *x;\n                let r = *y * rot;\n        \
    \        *x = l + r;\n                *y = l - r;\n            }\n           \
    \ rot *= StaticModInt::from_raw(\n                StaticModInt::<P>::NTT_PRECALC.rate2[s.trailing_ones()\
    \ as usize],\n            );\n        }\n        l += 1;\n    }\n\n    let mod2\
    \ = (P as u64) * (P as u64);\n    while l < lg {\n        let p = 1 << lg - l\
    \ - 2;\n        let mut rot = StaticModInt::<P>::from_raw(1);\n        let imag\
    \ = StaticModInt::<P>::from_raw(StaticModInt::<P>::NTT_PRECALC.root[2]);\n   \
    \     for (s, b) in f.chunks_exact_mut(p << 2).enumerate() {\n            let\
    \ rot2 = rot * rot;\n            let rot3 = rot2 * rot;\n            let (b0,\
    \ b1) = b.split_at_mut(p);\n            let (b1, b2) = b1.split_at_mut(p);\n \
    \           let (b2, b3) = b2.split_at_mut(p);\n            for (((x, y), z),\
    \ w) in b0.iter_mut().zip(b1).zip(b2).zip(b3) {\n                let a0 = x.val()\
    \ as u64;\n                let a1 = y.val() as u64 * rot.val() as u64;\n     \
    \           let a2 = z.val() as u64 * rot2.val() as u64;\n                let\
    \ a3 = w.val() as u64 * rot3.val() as u64;\n                let a1na3imag =\n\
    \                    StaticModInt::<P>::from(a1 + mod2 - a3).val() as u64 * imag.val()\
    \ as u64;\n                let na2 = mod2 - a2;\n                *x = StaticModInt::from(a0\
    \ + a2 + a1 + a3);\n                *y = StaticModInt::from(a0 + a2 + (mod2 *\
    \ 2 - (a1 + a3)));\n                *z = StaticModInt::from(a0 + na2 + a1na3imag);\n\
    \                *w = StaticModInt::from(a0 + na2 + mod2 - a1na3imag);\n     \
    \       }\n            rot *= StaticModInt::from_raw(\n                StaticModInt::<P>::NTT_PRECALC.rate3[s.trailing_ones()\
    \ as usize],\n            );\n        }\n        l += 2;\n    }\n}\n\npub fn ntt_inv<const\
    \ P: u32>(f: &mut [StaticModInt<P>]) {\n    assert!(StaticModInt::<P>::IS_NTT_FRIENDLY);\n\
    \n    let n = f.len();\n    let lg = n.floor_log2();\n    assert_eq!(n, 1 << lg,\
    \ \"length must be a power of 2\");\n\n    let mut l = lg;\n    if l % 2 == 1\
    \ {\n        let p = 1 << lg - l;\n        let mut irot = StaticModInt::<P>::from_raw(1);\n\
    \        for (s, b) in f.chunks_exact_mut(p << 1).enumerate() {\n            let\
    \ (b0, b1) = b.split_at_mut(p);\n            for (x, y) in b0.iter_mut().zip(b1)\
    \ {\n                let l = *x;\n                let r = *y;\n              \
    \  *x = l + r;\n                *y = StaticModInt::<P>::from((P + l.val() - r.val())\
    \ as u64 * irot.val() as u64);\n            }\n            irot *= StaticModInt::<P>::from_raw(\n\
    \                StaticModInt::<P>::NTT_PRECALC.irate2[s.trailing_ones() as usize],\n\
    \            );\n        }\n        l -= 1;\n    }\n\n    while l > 0 {\n    \
    \    let p = 1 << lg - l;\n        let mut irot = StaticModInt::<P>::from_raw(1);\n\
    \        let iimag = StaticModInt::<P>::from_raw(StaticModInt::<P>::NTT_PRECALC.iroot[2]);\n\
    \        for (s, b) in f.chunks_exact_mut(p << 2).enumerate() {\n            let\
    \ irot2 = irot * irot;\n            let irot3 = irot2 * irot;\n            let\
    \ (b0, b1) = b.split_at_mut(p);\n            let (b1, b2) = b1.split_at_mut(p);\n\
    \            let (b2, b3) = b2.split_at_mut(p);\n            for (((x, y), z),\
    \ w) in b0.iter_mut().zip(b1).zip(b2).zip(b3) {\n                let a0 = x.val()\
    \ as u64;\n                let a1 = y.val() as u64;\n                let a2 =\
    \ z.val() as u64;\n                let a3 = w.val() as u64;\n                let\
    \ a2na3iimag =\n                    StaticModInt::<P>::from((P as u64 + a2 - a3)\
    \ * iimag.val() as u64).val() as u64;\n                *x = StaticModInt::<P>::from(a0\
    \ + a1 + a2 + a3);\n                *y = StaticModInt::<P>::from(\n          \
    \          (a0 + (P as u64 - a1) + a2na3iimag) * irot.val() as u64,\n        \
    \        );\n                *z = StaticModInt::<P>::from(\n                 \
    \   (a0 + a1 + (P as u64 - a2) + (P as u64 - a3)) * irot2.val() as u64,\n    \
    \            );\n                *w = StaticModInt::<P>::from(\n             \
    \       (a0 + (P as u64 - a1) + (P as u64 - a2na3iimag)) * irot3.val() as u64,\n\
    \                );\n            }\n            irot *= StaticModInt::<P>::from_raw(\n\
    \                StaticModInt::<P>::NTT_PRECALC.irate3[s.trailing_ones() as usize],\n\
    \            );\n        }\n        l -= 2;\n    }\n\n    let recip_n = StaticModInt::<P>::from_raw(n\
    \ as u32).recip();\n    for x in f.iter_mut() {\n        *x *= recip_n;\n    }\n\
    }\n\n/// \u9577\u3055 n \u306E DFT \u304B\u3089\u9577\u3055 2n \u306E DFT \u3092\
    \u6C42\u3081\u308B\npub fn ntt_doubling<const P: u32>(f: &mut Vec<StaticModInt<P>>)\
    \ {\n    let n = f.len();\n    f.extend_from_within(..);\n    ntt_inv(&mut f[n..]);\n\
    \    let mut r = StaticModInt::from_raw(1);\n    let zeta = StaticModInt::from_raw(StaticModInt::<P>::NTT_PRECALC.primitive_root)\n\
    \        .pow((P - 1) as usize / (n << 1));\n    for x in &mut f[n..] {\n    \
    \    *x *= r;\n        r *= zeta;\n    }\n    ntt(&mut f[n..]);\n}\n"
  dependsOn:
  - crates/convolution/convolution/src/arbitrary_mod.rs
  - crates/convolution/convolution/src/lib.rs
  - crates/convolution/convolution/src/mod_2_64.rs
  - crates/convolution/convolution/src/naive.rs
  - crates/convolution/convolution/src/ntt_friendly.rs
  isVerificationFile: false
  path: crates/convolution/convolution/src/ntt.rs
  requiredBy:
  - crates/polynomial/formal_power_series/src/lib.rs
  - crates/convolution/convolution/src/lib.rs
  - crates/convolution/convolution/src/arbitrary_mod.rs
  - crates/convolution/convolution/src/naive.rs
  - crates/convolution/convolution/src/ntt_friendly.rs
  - crates/convolution/convolution/src/mod_2_64.rs
  timestamp: '2025-03-08 02:08:27+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/convolution/convolution_mod_1000000007/src/main.rs
  - verify/library_checker/convolution/convolution_mod_2_64/src/main.rs
  - verify/library_checker/convolution/convolution_mod/src/main.rs
documentation_of: crates/convolution/convolution/src/ntt.rs
layout: document
redirect_from:
- /library/crates/convolution/convolution/src/ntt.rs
- /library/crates/convolution/convolution/src/ntt.rs.html
title: crates/convolution/convolution/src/ntt.rs
---
