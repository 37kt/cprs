---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/static_range_inversions_query/src/main.rs
    title: verify/static_range_inversions_query/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ops::{Bound, RangeBounds};\n\nuse algebraic::{Group, Monoid};\n\n\
    pub struct FenwickTree<M>\nwhere\n    M: Monoid,\n    M::S: Clone,\n{\n    v:\
    \ Vec<M::S>,\n}\n\nimpl<M> FenwickTree<M>\nwhere\n    M: Monoid,\n    M::S: Clone,\n\
    {\n    pub fn new(n: usize) -> Self {\n        FenwickTree { v: vec![M::e(); n]\
    \ }\n    }\n\n    pub fn add(&mut self, mut k: usize, x: M::S) {\n        assert!(k\
    \ <= self.v.len());\n        k += 1;\n        while k <= self.v.len() {\n    \
    \        self.v[k - 1] = M::op(&self.v[k - 1], &x);\n            k += k & k.wrapping_neg();\n\
    \        }\n    }\n\n    pub fn accum(&self, mut k: usize) -> M::S {\n       \
    \ let mut res = M::e();\n        while k > 0 {\n            res = M::op(&res,\
    \ &self.v[k - 1]);\n            k &= k - 1;\n        }\n        res\n    }\n}\n\
    \nimpl<M> FenwickTree<M>\nwhere\n    M: Group,\n    M::S: Clone,\n{\n    pub fn\
    \ sum<R: RangeBounds<usize>>(&self, range: R) -> M::S {\n        let r = match\
    \ range.end_bound() {\n            Bound::Included(&r) => r + 1,\n           \
    \ Bound::Excluded(&r) => r,\n            Bound::Unbounded => self.v.len(),\n \
    \       };\n        let l = match range.start_bound() {\n            Bound::Included(&l)\
    \ => l,\n            Bound::Excluded(&l) => l + 1,\n            Bound::Unbounded\
    \ => return self.accum(r),\n        };\n        M::op(&M::inv(&self.accum(l)),\
    \ &self.accum(r))\n    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  isVerificationFile: false
  path: crates/data-structure/fenwick-tree/src/lib.rs
  requiredBy: []
  timestamp: '2023-04-25 15:51:20+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/static_range_inversions_query/src/main.rs
documentation_of: crates/data-structure/fenwick-tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/fenwick-tree/src/lib.rs
- /library/crates/data-structure/fenwick-tree/src/lib.rs.html
title: crates/data-structure/fenwick-tree/src/lib.rs
---
