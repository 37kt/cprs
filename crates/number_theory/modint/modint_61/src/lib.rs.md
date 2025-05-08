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
    path: crates/number_theory/modint/modint/src/lib.rs
    title: crates/number_theory/modint/modint/src/lib.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/modint_61/src/numeric.rs
    title: crates/number_theory/modint/modint_61/src/numeric.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/modint_61/src/ops.rs
    title: crates/number_theory/modint/modint_61/src/ops.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/number_theory/modint/modint_61/src/numeric.rs
    title: crates/number_theory/modint/modint_61/src/numeric.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/modint_61/src/ops.rs
    title: crates/number_theory/modint/modint_61/src/ops.rs
  - icon: ':heavy_check_mark:'
    path: crates/string/rolling_hash/src/lib.rs
    title: crates/string/rolling_hash/src/lib.rs
  - icon: ':warning:'
    path: verify/sandbox/test/src/main.rs
    title: verify/sandbox/test/src/main.rs
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
  code: "use modint::ModInt;\n\nmod ops;\n#[allow(unused_imports)]\npub use ops::*;\n\
    \nmod numeric;\n#[allow(unused_imports)]\npub use numeric::*;\n\npub(crate) const\
    \ P: u64 = (1 << 61) - 1;\n\n#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]\n\
    pub struct ModInt61(u64);\n\nimpl ModInt for ModInt61 {\n    type Value = u64;\n\
    \n    fn new<T: Into<ModInt61>>(val: T) -> Self {\n        val.into()\n    }\n\
    \n    fn modulus() -> Self::Value {\n        P\n    }\n\n    fn from_raw(val:\
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
  - crates/number_theory/modint/modint/src/lib.rs
  - crates/number_theory/modint/modint_61/src/numeric.rs
  - crates/number_theory/modint/modint_61/src/ops.rs
  isVerificationFile: false
  path: crates/number_theory/modint/modint_61/src/lib.rs
  requiredBy:
  - verify/sandbox/test/src/main.rs
  - crates/number_theory/modint/modint_61/src/numeric.rs
  - crates/number_theory/modint/modint_61/src/ops.rs
  - crates/string/rolling_hash/src/lib.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/number_theory/modint/modint_61/src/lib.rs
layout: document
redirect_from:
- /library/crates/number_theory/modint/modint_61/src/lib.rs
- /library/crates/number_theory/modint/modint_61/src/lib.rs.html
title: crates/number_theory/modint/modint_61/src/lib.rs
---
