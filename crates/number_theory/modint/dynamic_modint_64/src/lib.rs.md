---
data:
  _extendedDependsOn:
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
  - icon: ':warning:'
    path: crates/number_theory/modint/barrett_reduction/src/barrett_reduction_32.rs
    title: crates/number_theory/modint/barrett_reduction/src/barrett_reduction_32.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/barrett_reduction/src/barrett_reduction_64.rs
    title: crates/number_theory/modint/barrett_reduction/src/barrett_reduction_64.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/barrett_reduction/src/lib.rs
    title: crates/number_theory/modint/barrett_reduction/src/lib.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/dynamic_modint_64/src/barrett_reduction.rs
    title: crates/number_theory/modint/dynamic_modint_64/src/barrett_reduction.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/dynamic_modint_64/src/numeric.rs
    title: crates/number_theory/modint/dynamic_modint_64/src/numeric.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/dynamic_modint_64/src/ops.rs
    title: crates/number_theory/modint/dynamic_modint_64/src/ops.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/modint/src/lib.rs
    title: crates/number_theory/modint/modint/src/lib.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/number_theory/modint/dynamic_modint_64/src/barrett_reduction.rs
    title: crates/number_theory/modint/dynamic_modint_64/src/barrett_reduction.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/dynamic_modint_64/src/numeric.rs
    title: crates/number_theory/modint/dynamic_modint_64/src/numeric.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/dynamic_modint_64/src/ops.rs
    title: crates/number_theory/modint/dynamic_modint_64/src/ops.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/prime_factorization/src/lib.rs
    title: crates/number_theory/prime_factorization/src/lib.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{\n    hash::{Hash, Hasher},\n    marker::PhantomData,\n};\n\nuse\
    \ modint::ModInt;\n\nmod barrett_reduction;\nmod numeric;\nmod ops;\n\npub enum\
    \ DefaultDynamicModInt64Id {}\n\n#[repr(transparent)]\npub struct DynamicModInt64<Id>(u64,\
    \ PhantomData<fn() -> Id>);\n\npub type DefaultDynamicModInt64 = DynamicModInt64<DefaultDynamicModInt64Id>;\n\
    \nimpl<Id> Clone for DynamicModInt64<Id> {\n    fn clone(&self) -> Self {\n  \
    \      Self::from_raw(self.0)\n    }\n}\n\nimpl<Id> Copy for DynamicModInt64<Id>\
    \ {}\n\nimpl<Id> Default for DynamicModInt64<Id> {\n    fn default() -> Self {\n\
    \        Self::from_raw(0)\n    }\n}\n\nimpl<Id> PartialEq for DynamicModInt64<Id>\
    \ {\n    fn eq(&self, other: &Self) -> bool {\n        self.0 == other.0\n   \
    \ }\n}\n\nimpl<Id> Eq for DynamicModInt64<Id> {}\n\nimpl<Id> Hash for DynamicModInt64<Id>\
    \ {\n    fn hash<H: Hasher>(&self, state: &mut H) {\n        self.0.hash(state);\n\
    \    }\n}\n\nimpl<Id> ModInt for DynamicModInt64<Id> {\n    type Value = u64;\n\
    \n    fn new<T: Into<Self>>(val: T) -> Self {\n        val.into()\n    }\n\n \
    \   fn modulus() -> Self::Value {\n        Self::modulus()\n    }\n\n    fn from_raw(val:\
    \ Self::Value) -> Self {\n        Self::from_raw(val)\n    }\n\n    fn val(self)\
    \ -> Self::Value {\n        self.val()\n    }\n\n    fn recip(self) -> Self {\n\
    \        self.recip()\n    }\n\n    fn pow(self, exp: usize) -> Self {\n     \
    \   self.pow(exp)\n    }\n\n    fn sqrt(self) -> Option<Self> {\n        self.sqrt()\n\
    \    }\n}\n"
  dependsOn:
  - crates/algebra/numeric_traits/src/cast.rs
  - crates/algebra/numeric_traits/src/inf.rs
  - crates/algebra/numeric_traits/src/integer.rs
  - crates/algebra/numeric_traits/src/lib.rs
  - crates/algebra/numeric_traits/src/numeric.rs
  - crates/algebra/numeric_traits/src/signed.rs
  - crates/algebra/numeric_traits/src/zero_one.rs
  - crates/number_theory/modint/barrett_reduction/src/barrett_reduction_32.rs
  - crates/number_theory/modint/barrett_reduction/src/barrett_reduction_64.rs
  - crates/number_theory/modint/barrett_reduction/src/lib.rs
  - crates/number_theory/modint/dynamic_modint_64/src/barrett_reduction.rs
  - crates/number_theory/modint/dynamic_modint_64/src/numeric.rs
  - crates/number_theory/modint/dynamic_modint_64/src/ops.rs
  - crates/number_theory/modint/modint/src/lib.rs
  isVerificationFile: false
  path: crates/number_theory/modint/dynamic_modint_64/src/lib.rs
  requiredBy:
  - crates/number_theory/prime_factorization/src/lib.rs
  - crates/number_theory/modint/dynamic_modint_64/src/barrett_reduction.rs
  - crates/number_theory/modint/dynamic_modint_64/src/numeric.rs
  - crates/number_theory/modint/dynamic_modint_64/src/ops.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/number_theory/modint/dynamic_modint_64/src/lib.rs
layout: document
redirect_from:
- /library/crates/number_theory/modint/dynamic_modint_64/src/lib.rs
- /library/crates/number_theory/modint/dynamic_modint_64/src/lib.rs.html
title: crates/number_theory/modint/dynamic_modint_64/src/lib.rs
---
