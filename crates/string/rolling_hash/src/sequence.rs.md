---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/string/rolling_hash/src/lib.rs
    title: crates/string/rolling_hash/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/string/rolling_hash/src/monoid.rs
    title: crates/string/rolling_hash/src/monoid.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/string/rolling_hash/src/lib.rs
    title: crates/string/rolling_hash/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/string/rolling_hash/src/monoid.rs
    title: crates/string/rolling_hash/src/monoid.rs
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
  code: "use std::ops::RangeBounds;\n\nuse as_half_open_range::AsHalfOpenRange;\n\
    use modint_61::ModInt61;\n\nuse crate::{random, RollingHash};\n\n#[derive(Clone)]\n\
    pub struct RollingHashSequence {\n    hash: Vec<ModInt61>,\n    base_pow: Vec<ModInt61>,\n\
    }\n\nimpl<T> FromIterator<T> for RollingHashSequence\nwhere\n    T: Into<ModInt61>,\n\
    {\n    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {\n        let\
    \ mut hash: Vec<_> = iter\n            .into_iter()\n            .map(|x| x.into()\
    \ + random::<0x_ADD>())\n            .chain([0.into()])\n            .collect();\n\
    \        let n = hash.len() - 1;\n        for i in (0..n).rev() {\n          \
    \  hash[i] = hash[i] + hash[i + 1] * RollingHash::base();\n        }\n\n     \
    \   let mut base_pow = Vec::with_capacity(n + 1);\n        base_pow.push(ModInt61::from_raw(1));\n\
    \        for i in 0..n {\n            base_pow.push(base_pow[i] * RollingHash::base());\n\
    \        }\n\n        Self { hash, base_pow }\n    }\n}\n\nimpl RollingHashSequence\
    \ {\n    pub fn len(&self) -> usize {\n        self.hash.len() - 1\n    }\n\n\
    \    pub fn is_empty(&self) -> bool {\n        self.len() == 0\n    }\n\n    pub\
    \ fn base_pow(&self, exp: usize) -> ModInt61 {\n        self.base_pow[exp]\n \
    \   }\n\n    pub fn range(&self, range: impl RangeBounds<usize>) -> RollingHash\
    \ {\n        let (l, r) = range.as_half_open_range(0, self.len());\n        RollingHash\
    \ {\n            hash: self.hash[l] - self.hash[r] * self.base_pow[r - l],\n \
    \           base_pow: self.base_pow[r - l],\n        }\n    }\n\n    pub fn lcp(\n\
    \        &self,\n        range: impl RangeBounds<usize>,\n        other: &RollingHashSequence,\n\
    \        other_range: impl RangeBounds<usize>,\n    ) -> usize {\n        let\
    \ (l1, r1) = range.as_half_open_range(0, self.len());\n        let (l2, r2) =\
    \ other_range.as_half_open_range(0, other.len());\n        let max = (r1 - l1).min(r2\
    \ - l2);\n        let mut ok = 0;\n        let mut ng = max + 1;\n        while\
    \ ok + 1 < ng {\n            let md = (ok + ng) / 2;\n            if self.range(l1..l1\
    \ + md) == other.range(l2..l2 + md) {\n                ok = md;\n            }\
    \ else {\n                ng = md;\n            }\n        }\n        ok\n   \
    \ }\n}\n"
  dependsOn:
  - crates/string/rolling_hash/src/lib.rs
  - crates/string/rolling_hash/src/monoid.rs
  isVerificationFile: false
  path: crates/string/rolling_hash/src/sequence.rs
  requiredBy:
  - crates/string/rolling_hash/src/lib.rs
  - crates/string/rolling_hash/src/monoid.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/string/zalgorithm_rh/src/main.rs
documentation_of: crates/string/rolling_hash/src/sequence.rs
layout: document
redirect_from:
- /library/crates/string/rolling_hash/src/sequence.rs
- /library/crates/string/rolling_hash/src/sequence.rs.html
title: crates/string/rolling_hash/src/sequence.rs
---
