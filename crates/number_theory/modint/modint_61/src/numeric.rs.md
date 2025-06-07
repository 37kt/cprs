---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/number_theory/modint/modint_61/src/lib.rs
    title: crates/number_theory/modint/modint_61/src/lib.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/modint_61/src/ops.rs
    title: crates/number_theory/modint/modint_61/src/ops.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/number_theory/modint/modint_61/src/lib.rs
    title: crates/number_theory/modint/modint_61/src/lib.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/modint_61/src/ops.rs
    title: crates/number_theory/modint/modint_61/src/ops.rs
  - icon: ':heavy_check_mark:'
    path: crates/string/rolling_hash/src/lib.rs
    title: crates/string/rolling_hash/src/lib.rs
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
  code: "use numeric_traits::{Cast, Numeric, One, Recip, Zero};\n\nuse crate::ModInt61;\n\
    \nimpl Zero for ModInt61 {\n    fn zero() -> Self {\n        Self::from_raw(0)\n\
    \    }\n}\n\nimpl One for ModInt61 {\n    fn one() -> Self {\n        Self::from_raw(1)\n\
    \    }\n}\n\nimpl Numeric for ModInt61 {}\n\nimpl Recip for ModInt61 {\n    fn\
    \ recip(self) -> Self {\n        self.recip()\n    }\n}\n\nmacro_rules! impl_cast\
    \ {\n    ($($t:ty),*) => {\n        $(impl Cast<ModInt61> for $t {\n         \
    \   fn cast(self) -> ModInt61 {\n                ModInt61::from(self)\n      \
    \      }\n        })*\n    }\n}\n\nimpl_cast! {\n    u8, u16, u32, u64, u128,\
    \ i8, i16, i32, i64, i128, usize, isize\n}\n"
  dependsOn:
  - crates/number_theory/modint/modint_61/src/lib.rs
  - crates/number_theory/modint/modint_61/src/ops.rs
  isVerificationFile: false
  path: crates/number_theory/modint/modint_61/src/numeric.rs
  requiredBy:
  - crates/string/rolling_hash/src/lib.rs
  - crates/number_theory/modint/modint_61/src/ops.rs
  - crates/number_theory/modint/modint_61/src/lib.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/number_theory/modint/modint_61/src/numeric.rs
layout: document
redirect_from:
- /library/crates/number_theory/modint/modint_61/src/numeric.rs
- /library/crates/number_theory/modint/modint_61/src/numeric.rs.html
title: crates/number_theory/modint/modint_61/src/numeric.rs
---
