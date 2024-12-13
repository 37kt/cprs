---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/string/rolling-hash/src/lib.rs
    title: crates/string/rolling-hash/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/string/wildcard-pattern-matching/src/lib.rs
    title: crates/string/wildcard-pattern-matching/src/lib.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "#[derive(Clone)]\npub struct Pcg64Fast(u128);\n\nconst R: f64 = 1.0 / 0xffff_ffff_ffff_ffff_u64\
    \ as f64;\n\nimpl Default for Pcg64Fast {\n    fn default() -> Self {\n      \
    \  Self(\n            std::time::SystemTime::now()\n                .duration_since(std::time::UNIX_EPOCH)\n\
    \                .unwrap()\n                .as_nanos()\n                | 1,\n\
    \        )\n    }\n}\n\nimpl Pcg64Fast {\n    #[inline]\n    pub fn new(state:\
    \ u128) -> Self {\n        Self(state | 1)\n    }\n\n    #[inline]\n    pub fn\
    \ u32(&mut self) -> u32 {\n        self.u64() as u32\n    }\n\n    #[inline(always)]\n\
    \    pub fn u64(&mut self) -> u64 {\n        self.0 = self\n            .0\n \
    \           .wrapping_mul(0x2360_ED05_1FC6_5DA4_4385_DF64_9FCC_F645);\n      \
    \  let rot = (self.0 >> 122) as u32;\n        let xsl = (self.0 >> 64) as u64\
    \ ^ self.0 as u64;\n        xsl.rotate_right(rot)\n    }\n\n    #[inline]\n  \
    \  pub fn f64(&mut self) -> f64 {\n        self.u64() as f64 * R\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/misc/random/src/lib.rs
  requiredBy:
  - crates/string/wildcard-pattern-matching/src/lib.rs
  - crates/string/rolling-hash/src/lib.rs
  timestamp: '2023-05-20 14:50:51+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/misc/random/src/lib.rs
layout: document
redirect_from:
- /library/crates/misc/random/src/lib.rs
- /library/crates/misc/random/src/lib.rs.html
title: crates/misc/random/src/lib.rs
---
