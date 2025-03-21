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
    path: crates/number_theory/eratosthenes/src/lib.rs
    title: crates/number_theory/eratosthenes/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/convolution/lcm_convolution/src/main.rs
    title: verify/library_checker/convolution/lcm_convolution/src/main.rs
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
  code: "use algebraic_traits::{AbelianGroup, CommutativeMonoid, Magma, Ring};\nuse\
    \ eratosthenes::Eratosthenes;\n\n/// \u7D04\u6570\u306E\u65B9\u5411\u306B\u7D2F\
    \u7A4D\u548C\u3092\u53D6\u308B\npub fn divisor_zeta_transform<M: CommutativeMonoid>(f:\
    \ &mut [M::Value]) {\n    let n = f.len() - 1;\n    let sieve = Eratosthenes::new(n);\n\
    \    for p in sieve.primes() {\n        for i in 1..=n / p {\n            f[i\
    \ * p] = M::op(&f[i * p], &f[i]);\n        }\n    }\n}\n\n/// \u7D04\u6570\u306E\
    \u65B9\u5411\u306B\u53D6\u3089\u308C\u305F\u7D2F\u7A4D\u548C\u304B\u3089\u5FA9\
    \u5143\u3059\u308B\npub fn divisor_moebius_transform<G: AbelianGroup>(f: &mut\
    \ [G::Value]) {\n    let n = f.len() - 1;\n    let sieve = Eratosthenes::new(n);\n\
    \    for p in sieve.primes() {\n        for i in (1..=n / p).rev() {\n       \
    \     f[i * p] = G::op(&f[i * p], &G::inv(&f[i]));\n        }\n    }\n}\n\n///\
    \ h\\[k\\] = sum_{lcm(i, j) = k} f\\[i\\] * g\\[j\\]\npub fn lcm_convolution<R>(f:\
    \ &[R::Value], g: &[R::Value]) -> Vec<R::Value>\nwhere\n    R: Ring,\n    R::Additive:\
    \ AbelianGroup,\n    R::Value: Clone,\n{\n    assert_eq!(f.len(), g.len(), \"\
    f and g must have the same length\");\n    let mut f = f.to_vec();\n    let mut\
    \ g = g.to_vec();\n    if f.len() == 0 {\n        return vec![];\n    }\n    divisor_zeta_transform::<R::Additive>(&mut\
    \ f);\n    divisor_zeta_transform::<R::Additive>(&mut g);\n    for (x, y) in f.iter_mut().zip(&g).skip(1)\
    \ {\n        *x = R::Multiplicative::op(x, y);\n    }\n    divisor_moebius_transform::<R::Additive>(&mut\
    \ f);\n    f\n}\n"
  dependsOn:
  - crates/algebra/algebraic_traits/src/lib.rs
  - crates/algebra/algebraic_traits/src/macros.rs
  - crates/algebra/algebraic_traits/src/traits.rs
  - crates/number_theory/eratosthenes/src/lib.rs
  isVerificationFile: false
  path: crates/convolution/lcm_convolution/src/lib.rs
  requiredBy: []
  timestamp: '2025-03-07 00:14:11+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/convolution/lcm_convolution/src/main.rs
documentation_of: crates/convolution/lcm_convolution/src/lib.rs
layout: document
redirect_from:
- /library/crates/convolution/lcm_convolution/src/lib.rs
- /library/crates/convolution/lcm_convolution/src/lib.rs.html
title: crates/convolution/lcm_convolution/src/lib.rs
---
