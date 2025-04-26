---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/number_theory/modint/barrett_reduction/src/barrett_reduction_32.rs
    title: crates/number_theory/modint/barrett_reduction/src/barrett_reduction_32.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/barrett_reduction/src/lib.rs
    title: crates/number_theory/modint/barrett_reduction/src/lib.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/number_theory/modint/barrett_reduction/src/barrett_reduction_32.rs
    title: crates/number_theory/modint/barrett_reduction/src/barrett_reduction_32.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/barrett_reduction/src/lib.rs
    title: crates/number_theory/modint/barrett_reduction/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/dynamic_modint/src/lib.rs
    title: crates/number_theory/modint/dynamic_modint/src/lib.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/dynamic_modint_64/src/lib.rs
    title: crates/number_theory/modint/dynamic_modint_64/src/lib.rs
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
  code: "// \u5FC5\u8981\u306B\u306A\u3063\u305F\u3089 quo_rem \u7B49\u3092\u5B9F\u88C5\
    \u3059\u308B\n\n#[derive(Clone, Copy)]\npub struct BarrettReduction64 {\n    m:\
    \ u64,\n    imh: u64,\n    iml: u64,\n}\n\nimpl BarrettReduction64 {\n    pub\
    \ fn new(m: u64) -> Self {\n        let mut im = !0 / m as u128;\n        if (im\
    \ * m as u128).wrapping_add(m as u128) == 0 {\n            im = im.wrapping_add(1);\n\
    \        }\n        Self {\n            m,\n            imh: (im >> 64) as u64,\n\
    \            iml: im as u64,\n        }\n    }\n\n    pub fn modulus(&self) ->\
    \ u64 {\n        self.m\n    }\n\n    pub fn mul(&self, a: u64, b: u64) -> u64\
    \ {\n        const MASK: u128 = (1 << 64) - 1;\n        let imh = self.imh as\
    \ u128;\n        let iml = self.iml as u128;\n        let m = self.m as u128;\n\
    \        let add = |x: u128, y: u128| -> u128 { x.wrapping_add(y) };\n       \
    \ let sub = |x: u128, y: u128| -> u128 { x.wrapping_sub(y) };\n        let mul\
    \ = |x: u128, y: u128| -> u128 { x.wrapping_mul(y) };\n        let mut x = a as\
    \ u128 * b as u128;\n        let mut z = mul(x & MASK, iml);\n        z = add(add(mul(x\
    \ & MASK, imh), mul(x >> 64, iml)), z >> 64);\n        z = add(mul(x >> 64, imh),\
    \ z >> 64);\n        x = sub(x, mul(z, m));\n        if m <= x {\n           \
    \ x = sub(x, m);\n        }\n        x as u64\n    }\n}\n"
  dependsOn:
  - crates/number_theory/modint/barrett_reduction/src/barrett_reduction_32.rs
  - crates/number_theory/modint/barrett_reduction/src/lib.rs
  isVerificationFile: false
  path: crates/number_theory/modint/barrett_reduction/src/barrett_reduction_64.rs
  requiredBy:
  - crates/number_theory/modint/barrett_reduction/src/lib.rs
  - crates/number_theory/modint/barrett_reduction/src/barrett_reduction_32.rs
  - crates/number_theory/modint/dynamic_modint_64/src/lib.rs
  - crates/number_theory/modint/dynamic_modint/src/lib.rs
  timestamp: '2025-03-02 08:24:34+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/number_theory/modint/barrett_reduction/src/barrett_reduction_64.rs
layout: document
redirect_from:
- /library/crates/number_theory/modint/barrett_reduction/src/barrett_reduction_64.rs
- /library/crates/number_theory/modint/barrett_reduction/src/barrett_reduction_64.rs.html
title: crates/number_theory/modint/barrett_reduction/src/barrett_reduction_64.rs
---
