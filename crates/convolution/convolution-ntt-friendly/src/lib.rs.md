---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/convolution/convolution-naive/src/lib.rs
    title: crates/convolution/convolution-naive/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution-arbitrary-mod/src/lib.rs
    title: crates/convolution/convolution-arbitrary-mod/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution-u64/src/lib.rs
    title: crates/convolution/convolution-u64/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/bostan-mori/src/lib.rs
    title: crates/polynomial/bostan-mori/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/composition/src/lib.rs
    title: crates/polynomial/composition/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/compositional-inverse/src/lib.rs
    title: crates/polynomial/compositional-inverse/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal-power-series/src/lib.rs
    title: crates/polynomial/formal-power-series/src/lib.rs
  - icon: ':warning:'
    path: crates/polynomial/power-projection/src/lib.rs
    title: crates/polynomial/power-projection/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/string/wildcard-pattern-matching/src/lib.rs
    title: crates/string/wildcard-pattern-matching/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/convolution_mod/src/main.rs
    title: verify/convolution_mod/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/convolution_mod_1000000007/src/main.rs
    title: verify/convolution_mod_1000000007/src/main.rs
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
  code: "use convolution_naive::convolution_naive;\nuse modint::StaticModInt;\n\n\
    /// \u6570\u8AD6\u5909\u63DB (Number Theoretic Transform)\n///\n/// # \u5F15\u6570\
    \n/// - `a`: \u5165\u529B\u914D\u5217\u3002\u9577\u3055\u306F2\u306E\u51AA\u4E57\
    \u3067\u3042\u308B\u5FC5\u8981\u304C\u3042\u308B\n///\n/// # \u8A08\u7B97\u91CF\
    \n/// - O(N log N)\n///   - N: \u914D\u5217\u306E\u9577\u3055\n///\n/// # \u6CE8\
    \u610F\n/// - \u6CD5\u306F NTT-friendly (P-1 \u306E\u7D20\u56E0\u6570\u306B 2\
    \ \u3092\u591A\u304F\u542B\u3080) \u3067\u3042\u308B\u5FC5\u8981\u304C\u3042\u308B\
    \npub fn ntt<const P: u32>(a: &mut [StaticModInt<P>]) {\n    assert!(StaticModInt::<P>::IS_NTT_FRIENDLY);\n\
    \    let n = a.len();\n    assert_eq!(n.count_ones(), 1);\n    let h = n.trailing_zeros()\
    \ as usize;\n\n    let mut len = 0;\n    if (h - len) % 2 == 1 {\n        let\
    \ p = 1 << h - len - 1;\n        let mut rot = StaticModInt::raw(1);\n       \
    \ for (s, b) in a.chunks_exact_mut(p * 2).enumerate() {\n            let (b0,\
    \ b1) = b.split_at_mut(p);\n            for (x, y) in b0.iter_mut().zip(b1.iter_mut())\
    \ {\n                let l = *x;\n                let r = *y * rot;\n        \
    \        *x = l + r;\n                *y = l - r;\n            }\n           \
    \ rot *= StaticModInt::raw(StaticModInt::<P>::RATE2[(!s).trailing_zeros() as usize]);\n\
    \        }\n        len += 1;\n    }\n    let mod2 = (P as u64) * (P as u64);\n\
    \    while len < h {\n        let p = 1 << h - len - 2;\n        let mut rot =\
    \ StaticModInt::<P>::raw(1);\n        let imag = StaticModInt::<P>::raw(StaticModInt::<P>::ROOT[2]);\n\
    \        for (s, b) in a.chunks_exact_mut(p * 4).enumerate() {\n            let\
    \ rot2 = rot * rot;\n            let rot3 = rot2 * rot;\n            let (b0,\
    \ b1) = b.split_at_mut(p);\n            let (b1, b2) = b1.split_at_mut(p);\n \
    \           let (b2, b3) = b2.split_at_mut(p);\n            for (((x, y), z),\
    \ w) in b0\n                .iter_mut()\n                .zip(b1.iter_mut())\n\
    \                .zip(b2.iter_mut())\n                .zip(b3.iter_mut())\n  \
    \          {\n                let a0 = x.val() as u64;\n                let a1\
    \ = y.val() as u64 * rot.val() as u64;\n                let a2 = z.val() as u64\
    \ * rot2.val() as u64;\n                let a3 = w.val() as u64 * rot3.val() as\
    \ u64;\n                let a1na3imag =\n                    StaticModInt::<P>::from(a1\
    \ + mod2 - a3).val() as u64 * imag.val() as u64;\n                let na2 = mod2\
    \ - a2;\n                *x = StaticModInt::from(a0 + a2 + a1 + a3);\n       \
    \         *y = StaticModInt::from(a0 + a2 + (mod2 * 2 - (a1 + a3)));\n       \
    \         *z = StaticModInt::from(a0 + na2 + a1na3imag);\n                *w =\
    \ StaticModInt::from(a0 + na2 + mod2 - a1na3imag);\n            }\n          \
    \  rot *= StaticModInt::raw(StaticModInt::<P>::RATE3[(!s).trailing_zeros() as\
    \ usize]);\n        }\n        len += 2;\n    }\n}\n\n/// \u6570\u8AD6\u5909\u63DB\
    \u306E\u9006\u5909\u63DB\n/// \u6700\u5F8C\u306B\u914D\u5217\u9577 N \u3067\u5272\
    \u308B\n///\n/// # \u5F15\u6570\n/// - `a`: \u5165\u529B\u914D\u5217\u3002\u9577\
    \u3055\u306F2\u306E\u51AA\u4E57\u3067\u3042\u308B\u5FC5\u8981\u304C\u3042\u308B\
    \n///\n/// # \u8A08\u7B97\u91CF\n/// - O(N log N)\n///   - N: \u914D\u5217\u306E\
    \u9577\u3055\n///\n/// # \u6CE8\u610F\n/// - \u6CD5\u306F NTT-friendly (P-1 \u306E\
    \u7D20\u56E0\u6570\u306B 2 \u3092\u591A\u304F\u542B\u3080) \u3067\u3042\u308B\u5FC5\
    \u8981\u304C\u3042\u308B\npub fn ntt_inv<const P: u32>(a: &mut [StaticModInt<P>])\
    \ {\n    assert!(StaticModInt::<P>::IS_NTT_FRIENDLY);\n    let n = a.len();\n\
    \    assert_eq!(n.count_ones(), 1);\n    let h = n.trailing_zeros() as usize;\n\
    \n    let mut len = h;\n    if len % 2 == 1 {\n        let p = 1 << h - len;\n\
    \        let mut irot = StaticModInt::<P>::raw(1);\n        for (s, b) in a.chunks_exact_mut(p\
    \ * 2).enumerate() {\n            let (b0, b1) = b.split_at_mut(p);\n        \
    \    for (x, y) in b0.iter_mut().zip(b1.iter_mut()) {\n                let l =\
    \ *x;\n                let r = *y;\n                *x = l + r;\n            \
    \    *y = StaticModInt::<P>::from((P + l.val() - r.val()) as u64 * irot.val()\
    \ as u64);\n            }\n            irot *=\n                StaticModInt::<P>::raw(StaticModInt::<P>::IRATE2[(!s).trailing_zeros()\
    \ as usize]);\n        }\n        len -= 1;\n    }\n    while len > 0 {\n    \
    \    let p = 1 << h - len;\n        let mut irot = StaticModInt::<P>::raw(1);\n\
    \        let iimag = StaticModInt::<P>::raw(StaticModInt::<P>::IROOT[2]);\n  \
    \      for (s, b) in a.chunks_exact_mut(p * 4).enumerate() {\n            let\
    \ irot2 = irot * irot;\n            let irot3 = irot2 * irot;\n            let\
    \ (b0, b1) = b.split_at_mut(p);\n            let (b1, b2) = b1.split_at_mut(p);\n\
    \            let (b2, b3) = b2.split_at_mut(p);\n            for (((x, y), z),\
    \ w) in b0\n                .iter_mut()\n                .zip(b1.iter_mut())\n\
    \                .zip(b2.iter_mut())\n                .zip(b3.iter_mut())\n  \
    \          {\n                let a0 = x.val() as u64;\n                let a1\
    \ = y.val() as u64;\n                let a2 = z.val() as u64;\n              \
    \  let a3 = w.val() as u64;\n                let a2na3iimag =\n              \
    \      StaticModInt::<P>::from((P as u64 + a2 - a3) * iimag.val() as u64).val()\
    \ as u64;\n                *x = StaticModInt::<P>::from(a0 + a1 + a2 + a3);\n\
    \                *y = StaticModInt::<P>::from(\n                    (a0 + (P as\
    \ u64 - a1) + a2na3iimag) * irot.val() as u64,\n                );\n         \
    \       *z = StaticModInt::<P>::from(\n                    (a0 + a1 + (P as u64\
    \ - a2) + (P as u64 - a3)) * irot2.val() as u64,\n                );\n       \
    \         *w = StaticModInt::<P>::from(\n                    (a0 + (P as u64 -\
    \ a1) + (P as u64 - a2na3iimag)) * irot3.val() as u64,\n                );\n \
    \           }\n            irot *=\n                StaticModInt::<P>::raw(StaticModInt::<P>::IRATE3[(!s).trailing_zeros()\
    \ as usize]);\n        }\n        len -= 2;\n    }\n\n    let inv_n = StaticModInt::<P>::new(n).inv();\n\
    \    for x in a.iter_mut() {\n        *x *= inv_n;\n    }\n}\n\n/// \u9577\u3055\
    \ n \u306E DFT \u3092\u9577\u3055 2n \u306E DFT \u306B\u5909\u63DB\u3059\u308B\
    \npub fn ntt_doubling<const P: u32>(a: &mut Vec<StaticModInt<P>>) {\n    let n\
    \ = a.len();\n    a.append(&mut a.clone());\n    ntt_inv(&mut a[n..]);\n    let\
    \ mut r = StaticModInt::new(1);\n    let zeta = StaticModInt::new(StaticModInt::<P>::G).pow((P\
    \ - 1) as usize / (n << 1));\n    for i in n..n * 2 {\n        a[i] *= r;\n  \
    \      r *= zeta;\n    }\n    ntt(&mut a[n..]);\n}\n\n/// \u914D\u5217\u306E\u7573\
    \u307F\u8FBC\u307F\n///\n/// # \u6982\u8981\n/// 2\u3064\u306E\u914D\u5217 `a`,\
    \ `b` \u306B\u5BFE\u3057\u3001\u4EE5\u4E0B\u306E\u5F0F\u3067\u5B9A\u7FA9\u3055\
    \u308C\u308B\u7573\u307F\u8FBC\u307F\u3092\u8A08\u7B97\u3059\u308B\uFF1A\n///\
    \ ```text\n/// res[k] = \u03A3_{i + j = k} (a[i] * b[j])\n/// ```\n///\n/// #\
    \ \u5F15\u6570\n/// - `a`: \u5165\u529B\u914D\u5217\n/// - `b`: \u5165\u529B\u914D\
    \u5217\n///\n/// # \u623B\u308A\u5024\n/// - \u7573\u307F\u8FBC\u307F\u306E\u7D50\
    \u679C\n///\n/// # \u8A08\u7B97\u91CF\n/// - O(N log N)\n///   - N: \u914D\u5217\
    \u306E\u9577\u3055\n///\n/// # \u6CE8\u610F\n/// - \u6CD5\u306F NTT-friendly (P-1\
    \ \u306E\u7D20\u56E0\u6570\u306B 2 \u3092\u591A\u304F\u542B\u3080) \u3067\u3042\
    \u308B\u5FC5\u8981\u304C\u3042\u308B\npub fn convolution_ntt_friendly<const P:\
    \ u32>(\n    mut a: Vec<StaticModInt<P>>,\n    mut b: Vec<StaticModInt<P>>,\n\
    ) -> Vec<StaticModInt<P>> {\n    let n = a.len();\n    let m = b.len();\n    if\
    \ n == 0 || m == 0 {\n        return vec![];\n    }\n    if n.min(m) <= 60 {\n\
    \        return convolution_naive(&a, &b);\n    }\n    let len = n + m - 1;\n\
    \    let z = 1 << 64 - (len - 1).leading_zeros();\n    a.resize(z, 0.into());\n\
    \    b.resize(z, 0.into());\n    ntt(&mut a);\n    ntt(&mut b);\n    for i in\
    \ 0..z {\n        a[i] *= b[i];\n    }\n    ntt_inv(&mut a);\n    a.truncate(len);\n\
    \    a\n}\n"
  dependsOn:
  - crates/convolution/convolution-naive/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  isVerificationFile: false
  path: crates/convolution/convolution-ntt-friendly/src/lib.rs
  requiredBy:
  - crates/convolution/convolution-arbitrary-mod/src/lib.rs
  - crates/convolution/convolution-u64/src/lib.rs
  - crates/string/wildcard-pattern-matching/src/lib.rs
  - crates/polynomial/formal-power-series/src/lib.rs
  - crates/polynomial/composition/src/lib.rs
  - crates/polynomial/power-projection/src/lib.rs
  - crates/polynomial/bostan-mori/src/lib.rs
  - crates/polynomial/compositional-inverse/src/lib.rs
  timestamp: '2024-12-30 09:13:10+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/convolution_mod/src/main.rs
  - verify/convolution_mod_1000000007/src/main.rs
documentation_of: crates/convolution/convolution-ntt-friendly/src/lib.rs
layout: document
redirect_from:
- /library/crates/convolution/convolution-ntt-friendly/src/lib.rs
- /library/crates/convolution/convolution-ntt-friendly/src/lib.rs.html
title: crates/convolution/convolution-ntt-friendly/src/lib.rs
---
