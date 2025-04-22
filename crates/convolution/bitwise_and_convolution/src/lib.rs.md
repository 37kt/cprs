---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_traits/src/lib.rs
    title: crates/algebra/algebraic_traits/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_traits/src/macros.rs
    title: crates/algebra/algebraic_traits/src/macros.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_traits/src/traits.rs
    title: crates/algebra/algebraic_traits/src/traits.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/cast.rs
    title: crates/algebra/numeric_traits/src/cast.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/inf.rs
    title: crates/algebra/numeric_traits/src/inf.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/integer.rs
    title: crates/algebra/numeric_traits/src/integer.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/lib.rs
    title: crates/algebra/numeric_traits/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/numeric.rs
    title: crates/algebra/numeric_traits/src/numeric.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/signed.rs
    title: crates/algebra/numeric_traits/src/signed.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/zero_one.rs
    title: crates/algebra/numeric_traits/src/zero_one.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/convolution/bitwise_and_convolution/src/main.rs
    title: verify/library_checker/convolution/bitwise_and_convolution/src/main.rs
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
  code: "use algebraic_traits::{AbelianGroup, CommutativeMonoid, Magma, Ring};\nuse\
    \ numeric_traits::Integer;\n\n/// \u90E8\u5206\u96C6\u5408\u306E\u65B9\u5411\u306B\
    \u7D2F\u7A4D\u548C\u3092\u53D6\u308B\npub fn superset_zeta_transform<M: CommutativeMonoid>(f:\
    \ &mut [M::Value]) {\n    let n = f.len();\n    let lg = n.floor_log2();\n   \
    \ assert_eq!(n, 1 << lg, \"n must be a power of 2\");\n\n    for i in 0..lg {\n\
    \        let w = 1 << i;\n        for f in f.chunks_exact_mut(w << 1) {\n    \
    \        let (f0, f1) = f.split_at_mut(w);\n            for (x, y) in f0.iter_mut().zip(f1)\
    \ {\n                *x = M::op(x, y);\n            }\n        }\n    }\n}\n\n\
    /// \u90E8\u5206\u96C6\u5408\u306E\u65B9\u5411\u306B\u53D6\u3089\u308C\u305F\u7D2F\
    \u7A4D\u548C\u304B\u3089\u5FA9\u5143\u3059\u308B\npub fn superset_moebius_transform<G:\
    \ AbelianGroup>(f: &mut [G::Value]) {\n    let n = f.len();\n    let lg = n.floor_log2();\n\
    \    assert_eq!(n, 1 << lg, \"n must be a power of 2\");\n\n    for i in 0..lg\
    \ {\n        let w = 1 << i;\n        for f in f.chunks_exact_mut(w << 1) {\n\
    \            let (f0, f1) = f.split_at_mut(w);\n            for (x, y) in f0.iter_mut().zip(f1)\
    \ {\n                *x = G::op(x, &G::inv(y));\n            }\n        }\n  \
    \  }\n}\n\n/// h\\[k\\] = sum_{i & j = k} f\\[i\\] * g\\[j\\]\npub fn bitwise_and_convolution<R>(f:\
    \ &[R::Value], g: &[R::Value]) -> Vec<R::Value>\nwhere\n    R: Ring,\n    R::Additive:\
    \ AbelianGroup,\n    R::Value: Clone,\n{\n    assert_eq!(f.len(), g.len(), \"\
    f and g must have the same length\");\n    let mut f = f.to_vec();\n    let mut\
    \ g = g.to_vec();\n    superset_zeta_transform::<R::Additive>(&mut f);\n    superset_zeta_transform::<R::Additive>(&mut\
    \ g);\n    for (x, y) in f.iter_mut().zip(&g) {\n        *x = R::Multiplicative::op(x,\
    \ y);\n    }\n    superset_moebius_transform::<R::Additive>(&mut f);\n    f\n\
    }\n"
  dependsOn:
  - crates/algebra/algebraic_traits/src/lib.rs
  - crates/algebra/algebraic_traits/src/macros.rs
  - crates/algebra/algebraic_traits/src/traits.rs
  - crates/algebra/numeric_traits/src/cast.rs
  - crates/algebra/numeric_traits/src/inf.rs
  - crates/algebra/numeric_traits/src/integer.rs
  - crates/algebra/numeric_traits/src/lib.rs
  - crates/algebra/numeric_traits/src/numeric.rs
  - crates/algebra/numeric_traits/src/signed.rs
  - crates/algebra/numeric_traits/src/zero_one.rs
  isVerificationFile: false
  path: crates/convolution/bitwise_and_convolution/src/lib.rs
  requiredBy: []
  timestamp: '2025-04-22 06:09:15+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/convolution/bitwise_and_convolution/src/main.rs
documentation_of: crates/convolution/bitwise_and_convolution/src/lib.rs
layout: document
redirect_from:
- /library/crates/convolution/bitwise_and_convolution/src/lib.rs
- /library/crates/convolution/bitwise_and_convolution/src/lib.rs.html
title: crates/convolution/bitwise_and_convolution/src/lib.rs
---
