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
    path: verify/library_checker/convolution/bitwise_xor_convolution/src/main.rs
    title: verify/library_checker/convolution/bitwise_xor_convolution/src/main.rs
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
  code: "use algebraic_traits::{AbelianGroup, Magma, Ring};\nuse numeric_traits::Integer;\n\
    \npub fn hadamard_transform<G>(f: &mut [G::Value])\nwhere\n    G: AbelianGroup,\n\
    {\n    let n = f.len();\n    let lg = n.floor_log2();\n    assert_eq!(n, 1 <<\
    \ lg, \"n must be a power of 2\");\n\n    for i in 0..lg {\n        let w = 1\
    \ << i;\n        for f in f.chunks_exact_mut(w << 1) {\n            let (f0, f1)\
    \ = f.split_at_mut(w);\n            for (x, y) in f0.iter_mut().zip(f1) {\n  \
    \              let nx = G::op(x, y);\n                let ny = G::op(x, &G::inv(y));\n\
    \                *x = nx;\n                *y = ny;\n            }\n        }\n\
    \    }\n}\n\npub fn hadamard_transform_inverse<G>(\n    f: &mut [G::Value],\n\
    \    mut divide_by_2: impl FnMut(G::Value) -> G::Value,\n) where\n    G: AbelianGroup,\n\
    {\n    let n = f.len();\n    let lg = n.floor_log2();\n    assert_eq!(n, 1 <<\
    \ lg, \"n must be a power of 2\");\n\n    for i in 0..lg {\n        let w = 1\
    \ << i;\n        for f in f.chunks_exact_mut(w << 1) {\n            let (f0, f1)\
    \ = f.split_at_mut(w);\n            for (x, y) in f0.iter_mut().zip(f1) {\n  \
    \              let nx = divide_by_2(G::op(x, y));\n                let ny = divide_by_2(G::op(x,\
    \ &G::inv(y)));\n                *x = nx;\n                *y = ny;\n        \
    \    }\n        }\n    }\n}\n\npub fn bitwise_xor_convolution<R>(\n    f: &[R::Value],\n\
    \    g: &[R::Value],\n    divide_by_2: impl FnMut(R::Value) -> R::Value,\n) ->\
    \ Vec<R::Value>\nwhere\n    R: Ring,\n    R::Additive: AbelianGroup,\n    R::Value:\
    \ Clone,\n{\n    assert_eq!(f.len(), g.len(), \"f and g must have the same length\"\
    );\n    let mut f = f.to_vec();\n    let mut g = g.to_vec();\n    hadamard_transform::<R::Additive>(&mut\
    \ f);\n    hadamard_transform::<R::Additive>(&mut g);\n    for (x, y) in f.iter_mut().zip(&g)\
    \ {\n        *x = R::Multiplicative::op(x, y);\n    }\n    hadamard_transform_inverse::<R::Additive>(&mut\
    \ f, divide_by_2);\n    f\n}\n"
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
  path: crates/convolution/bitwise_xor_convolution/src/lib.rs
  requiredBy: []
  timestamp: '2025-03-20 09:27:03+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/convolution/bitwise_xor_convolution/src/main.rs
documentation_of: crates/convolution/bitwise_xor_convolution/src/lib.rs
layout: document
redirect_from:
- /library/crates/convolution/bitwise_xor_convolution/src/lib.rs
- /library/crates/convolution/bitwise_xor_convolution/src/lib.rs.html
title: crates/convolution/bitwise_xor_convolution/src/lib.rs
---
