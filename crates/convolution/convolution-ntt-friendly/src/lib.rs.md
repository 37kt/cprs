---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/convolution/convolution-naive/src/lib.rs
    title: crates/convolution/convolution-naive/src/lib.rs
  - icon: ':question:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution-arbitrary-mod/src/lib.rs
    title: crates/convolution/convolution-arbitrary-mod/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/bostan-mori/src/lib.rs
    title: crates/polynomial/bostan-mori/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal-power-series/src/lib.rs
    title: crates/polynomial/formal-power-series/src/lib.rs
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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.5/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.5/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use convolution_naive::convolution_naive;\nuse modint::StaticModInt;\n\n\
    pub fn ntt<const P: u32>(a: &mut [StaticModInt<P>]) {\n    assert!(StaticModInt::<P>::IS_NTT_FRIENDLY);\n\
    \    let n = a.len();\n    assert_eq!(n.count_ones(), 1);\n    let h = n.trailing_zeros()\
    \ as usize;\n\n    let mut len = 0;\n    while len < h {\n        if h - len ==\
    \ 1 {\n            let p = 1 << h - len - 1;\n            let mut rot = StaticModInt::raw(1);\n\
    \            for s in 0..1 << len {\n                let offset = s << h - len;\n\
    \                for i in 0..p {\n                    let l = a[i + offset];\n\
    \                    let r = a[i + offset + p] * rot;\n                    a[i\
    \ + offset] = l + r;\n                    a[i + offset + p] = l - r;\n       \
    \         }\n                if s + 1 != 1 << len {\n                    rot *=\n\
    \                        StaticModInt::raw(StaticModInt::<P>::RATE2[(!s).trailing_zeros()\
    \ as usize]);\n                }\n            }\n            len += 1;\n     \
    \   } else {\n            let p = 1 << h - len - 2;\n            let mut rot =\
    \ StaticModInt::<P>::raw(1);\n            let imag = StaticModInt::<P>::raw(StaticModInt::<P>::ROOT[2]);\n\
    \            for s in 0..1 << len {\n                let rot2 = rot * rot;\n \
    \               let rot3 = rot2 * rot;\n                let offset = s << h -\
    \ len;\n                for i in 0..p {\n                    let mod2 = (StaticModInt::<P>::modulus()\
    \ as u64).pow(2);\n                    let a0 = a[i + offset].val() as u64;\n\
    \                    let a1 = a[i + offset + p].val() as u64 * rot.val() as u64;\n\
    \                    let a2 = a[i + offset + p * 2].val() as u64 * rot2.val()\
    \ as u64;\n                    let a3 = a[i + offset + p * 3].val() as u64 * rot3.val()\
    \ as u64;\n                    let a1na3imag =\n                        StaticModInt::<P>::from(a1\
    \ + mod2 - a3).val() as u64 * imag.val() as u64;\n                    let na2\
    \ = mod2 - a2;\n                    a[i + offset] = StaticModInt::from(a0 + a2\
    \ + a1 + a3);\n                    a[i + offset + p] = StaticModInt::from(a0 +\
    \ a2 + (mod2 * 2 - (a1 + a3)));\n                    a[i + offset + p * 2] = StaticModInt::from(a0\
    \ + na2 + a1na3imag);\n                    a[i + offset + p * 3] = StaticModInt::from(a0\
    \ + na2 + mod2 - a1na3imag);\n                }\n                if s + 1 != 1\
    \ << len {\n                    rot *=\n                        StaticModInt::raw(StaticModInt::<P>::RATE3[(!s).trailing_zeros()\
    \ as usize]);\n                }\n            }\n            len += 2;\n     \
    \   }\n    }\n}\n\npub fn ntt_inv<const P: u32>(a: &mut [StaticModInt<P>]) {\n\
    \    assert!(StaticModInt::<P>::IS_NTT_FRIENDLY);\n    let n = a.len();\n    assert_eq!(n.count_ones(),\
    \ 1);\n    let h = n.trailing_zeros() as usize;\n\n    let mut len = h;\n    while\
    \ len > 0 {\n        if len == 1 {\n            let p = 1 << h - len;\n      \
    \      let mut irot = StaticModInt::<P>::raw(1);\n            for s in 0..1 <<\
    \ len - 1 {\n                let offset = s << h - len + 1;\n                for\
    \ i in 0..p {\n                    let l = a[i + offset];\n                  \
    \  let r = a[i + offset + p];\n                    a[i + offset] = l + r;\n  \
    \                  a[i + offset + p] = StaticModInt::<P>::from(\n            \
    \            (StaticModInt::<P>::modulus() + l.val() - r.val()) as u64\n     \
    \                       * irot.val() as u64,\n                    );\n       \
    \         }\n                if s + 1 != 1 << len - 1 {\n                    irot\
    \ *= StaticModInt::<P>::raw(\n                        StaticModInt::<P>::IRATE2[(!s).trailing_zeros()\
    \ as usize],\n                    );\n                }\n            }\n     \
    \       len -= 1;\n        } else {\n            let p = 1 << h - len;\n     \
    \       let mut irot = StaticModInt::<P>::raw(1);\n            let iimag = StaticModInt::<P>::raw(StaticModInt::<P>::IROOT[2]);\n\
    \            for s in 0..1 << len - 2 {\n                let irot2 = irot * irot;\n\
    \                let irot3 = irot2 * irot;\n                let offset = s <<\
    \ h - len + 2;\n                for i in 0..p {\n                    let a0 =\
    \ a[i + offset].val() as u64;\n                    let a1 = a[i + offset + p].val()\
    \ as u64;\n                    let a2 = a[i + offset + p * 2].val() as u64;\n\
    \                    let a3 = a[i + offset + p * 3].val() as u64;\n          \
    \          let a2na3iimag = StaticModInt::<P>::from(\n                       \
    \ (StaticModInt::<P>::modulus() as u64 + a2 - a3) * iimag.val() as u64,\n    \
    \                )\n                    .val() as u64;\n                    a[i\
    \ + offset] = StaticModInt::<P>::from(a0 + a1 + a2 + a3);\n                  \
    \  a[i + offset + p] = StaticModInt::<P>::from(\n                        (a0 +\
    \ (StaticModInt::<P>::modulus() as u64 - a1) + a2na3iimag)\n                 \
    \           * irot.val() as u64,\n                    );\n                   \
    \ a[i + offset + p * 2] = StaticModInt::<P>::from(\n                        (a0\
    \ + a1\n                            + (StaticModInt::<P>::modulus() as u64 - a2)\n\
    \                            + (StaticModInt::<P>::modulus() as u64 - a3))\n \
    \                           * irot2.val() as u64,\n                    );\n  \
    \                  a[i + offset + p * 3] = StaticModInt::<P>::from(\n        \
    \                (a0 + (StaticModInt::<P>::modulus() as u64 - a1)\n          \
    \                  + (StaticModInt::<P>::modulus() as u64 - a2na3iimag))\n   \
    \                         * irot3.val() as u64,\n                    );\n    \
    \            }\n                if s + 1 != 1 << len - 2 {\n                 \
    \   irot *= StaticModInt::<P>::raw(\n                        StaticModInt::<P>::IRATE3[(!s).trailing_zeros()\
    \ as usize],\n                    );\n                }\n            }\n     \
    \       len -= 2;\n        }\n    }\n\n    let inv_n = StaticModInt::<P>::new(n).inv();\n\
    \    for x in a.iter_mut() {\n        *x *= inv_n;\n    }\n}\n\npub fn ntt_doubling<const\
    \ P: u32>(a: &mut Vec<StaticModInt<P>>) {\n    let n = a.len();\n    a.append(&mut\
    \ a.clone());\n    ntt_inv(&mut a[n..]);\n    let mut r = StaticModInt::new(1);\n\
    \    let zeta = StaticModInt::new(StaticModInt::<P>::G).pow((P - 1) as usize /\
    \ (n << 1));\n    for i in n..n * 2 {\n        a[i] *= r;\n        r *= zeta;\n\
    \    }\n    ntt(&mut a[n..]);\n}\n\npub fn convolution_ntt_friendly<const P: u32>(\n\
    \    mut a: Vec<StaticModInt<P>>,\n    mut b: Vec<StaticModInt<P>>,\n) -> Vec<StaticModInt<P>>\
    \ {\n    let n = a.len();\n    let m = b.len();\n    if n == 0 || m == 0 {\n \
    \       return vec![];\n    } else if n.min(m) <= 60 {\n        return convolution_naive(&a,\
    \ &b);\n    }\n    let len = n + m - 1;\n    let z = 1 << 64 - (len - 1).leading_zeros();\n\
    \    a.resize(z, 0.into());\n    b.resize(z, 0.into());\n    ntt(&mut a);\n  \
    \  ntt(&mut b);\n    for i in 0..z {\n        a[i] *= b[i];\n    }\n    ntt_inv(&mut\
    \ a);\n    a.truncate(len);\n    a\n}\n"
  dependsOn:
  - crates/convolution/convolution-naive/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  isVerificationFile: false
  path: crates/convolution/convolution-ntt-friendly/src/lib.rs
  requiredBy:
  - crates/polynomial/formal-power-series/src/lib.rs
  - crates/polynomial/bostan-mori/src/lib.rs
  - crates/convolution/convolution-arbitrary-mod/src/lib.rs
  timestamp: '2023-07-15 18:59:53+09:00'
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
