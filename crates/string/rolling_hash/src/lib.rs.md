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
  - icon: ':warning:'
    path: crates/misc/as_half_open_range/src/lib.rs
    title: crates/misc/as_half_open_range/src/lib.rs
  - icon: ':warning:'
    path: crates/misc/random/src/lib.rs
    title: crates/misc/random/src/lib.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/modint_61/src/lib.rs
    title: crates/number_theory/modint/modint_61/src/lib.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/modint_61/src/numeric.rs
    title: crates/number_theory/modint/modint_61/src/numeric.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/modint_61/src/ops.rs
    title: crates/number_theory/modint/modint_61/src/ops.rs
  - icon: ':heavy_check_mark:'
    path: crates/string/rolling_hash/src/monoid.rs
    title: crates/string/rolling_hash/src/monoid.rs
  - icon: ':heavy_check_mark:'
    path: crates/string/rolling_hash/src/sequence.rs
    title: crates/string/rolling_hash/src/sequence.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/string/rolling_hash/src/monoid.rs
    title: crates/string/rolling_hash/src/monoid.rs
  - icon: ':heavy_check_mark:'
    path: crates/string/rolling_hash/src/sequence.rs
    title: crates/string/rolling_hash/src/sequence.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/string/zalgorithm_rh/src/main.rs
    title: verify/library_checker/string/zalgorithm_rh/src/main.rs
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
  code: "use std::{\n    cell::{OnceCell, RefCell},\n    fmt::Debug,\n};\n\nuse modint_61::ModInt61;\n\
    use random::Pcg64Fast;\n\nmod monoid;\npub use monoid::*;\n\nmod sequence;\npub\
    \ use sequence::*;\n\n#[derive(Clone, Copy, PartialEq, Eq, Hash)]\npub struct\
    \ RollingHash {\n    pub hash: ModInt61,\n    pub base_pow: ModInt61, // base^len\n\
    }\n\nimpl RollingHash {\n    pub fn base() -> ModInt61 {\n        random::<0x_BA5E_0000>()\n\
    \    }\n}\n\nimpl Default for RollingHash {\n    fn default() -> Self {\n    \
    \    Self {\n            hash: ModInt61::from_raw(0),\n            base_pow: ModInt61::from_raw(1),\n\
    \        }\n    }\n}\n\nimpl Debug for RollingHash {\n    fn fmt(&self, f: &mut\
    \ std::fmt::Formatter<'_>) -> std::fmt::Result {\n        write!(f, \"{}\", self.hash)\n\
    \    }\n}\n\nimpl<T: Into<ModInt61>> From<T> for RollingHash {\n    fn from(value:\
    \ T) -> Self {\n        Self {\n            hash: value.into() + random::<0x_ADD>(),\n\
    \            base_pow: Self::base(),\n        }\n    }\n}\n\nimpl<T> FromIterator<T>\
    \ for RollingHash\nwhere\n    T: Into<ModInt61>,\n{\n    fn from_iter<I: IntoIterator<Item\
    \ = T>>(iter: I) -> Self {\n        iter.into_iter()\n            .fold(RollingHash::default(),\
    \ |acc, x| RollingHash {\n                hash: acc.hash + (x.into() + random::<0x_ADD>())\
    \ * acc.base_pow,\n                base_pow: acc.base_pow * Self::base(),\n  \
    \          })\n    }\n}\n\nimpl<T> Extend<T> for RollingHash\nwhere\n    T: Into<ModInt61>,\n\
    {\n    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {\n        for\
    \ x in iter {\n            *self = Self {\n                hash: self.hash + (x.into()\
    \ + random::<0x_ADD>()) * self.base_pow,\n                base_pow: self.base_pow\
    \ * Self::base(),\n            };\n        }\n    }\n}\n\nthread_local! {\n  \
    \  static RNG: RefCell<Pcg64Fast> = RefCell::new(Pcg64Fast::default());\n}\n\n\
    pub(crate) fn random<const ID: u64>() -> ModInt61 {\n    thread_local! {\n   \
    \     static VALUE: OnceCell<ModInt61> = OnceCell::new();\n    }\n    VALUE.with(|v|\
    \ *v.get_or_init(|| RNG.with(|rng| ModInt61::new(rng.borrow_mut().u64()))))\n\
    }\n"
  dependsOn:
  - crates/algebra/algebraic_traits/src/lib.rs
  - crates/algebra/algebraic_traits/src/macros.rs
  - crates/algebra/algebraic_traits/src/traits.rs
  - crates/misc/as_half_open_range/src/lib.rs
  - crates/misc/random/src/lib.rs
  - crates/number_theory/modint/modint_61/src/lib.rs
  - crates/number_theory/modint/modint_61/src/numeric.rs
  - crates/number_theory/modint/modint_61/src/ops.rs
  - crates/string/rolling_hash/src/monoid.rs
  - crates/string/rolling_hash/src/sequence.rs
  isVerificationFile: false
  path: crates/string/rolling_hash/src/lib.rs
  requiredBy:
  - crates/string/rolling_hash/src/monoid.rs
  - crates/string/rolling_hash/src/sequence.rs
  timestamp: '2025-04-22 06:09:15+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/string/zalgorithm_rh/src/main.rs
documentation_of: crates/string/rolling_hash/src/lib.rs
layout: document
redirect_from:
- /library/crates/string/rolling_hash/src/lib.rs
- /library/crates/string/rolling_hash/src/lib.rs.html
title: crates/string/rolling_hash/src/lib.rs
---
