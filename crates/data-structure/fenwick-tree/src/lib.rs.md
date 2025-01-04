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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ops::{Bound, RangeBounds};\n\nuse algebraic::{Group, Monoid};\n\n\
    /// Fenwick Tree\n///\n/// prefix sum \u3092\u7BA1\u7406\u3059\u308B\u30C7\u30FC\
    \u30BF\u69CB\u9020\u3002\n/// \u7FA4\u306B\u5BFE\u3057\u3066\u306F\u533A\u9593\
    \u548C\u30AF\u30A8\u30EA\u3082\u63D0\u4F9B\u3059\u308B\u3002\npub struct FenwickTree<M>\n\
    where\n    M: Monoid,\n    M::S: Clone,\n{\n    v: Vec<M::S>,\n}\n\nimpl<M> FenwickTree<M>\n\
    where\n    M: Monoid,\n    M::S: Clone,\n{\n    /// \u30B5\u30A4\u30BA n \u306E\
    \ Fenwick Tree \u3092\u69CB\u7BC9\u3059\u308B\u3002  \n    /// \u5358\u4F4D\u5143\
    \u3067\u521D\u671F\u5316\u3059\u308B\u3002\n    ///\n    /// # \u8A08\u7B97\u91CF\
    \n    ///\n    /// O(n)\n    pub fn new(n: usize) -> Self {\n        FenwickTree\
    \ { v: vec![M::e(); n] }\n    }\n\n    /// a[k] \u3092 op(a[k], x) \u306B\u66F4\
    \u65B0\u3059\u308B\u3002\n    ///\n    /// # \u8A08\u7B97\u91CF\n    ///\n   \
    \ /// O(log n)\n    pub fn add(&mut self, mut k: usize, x: M::S) {\n        assert!(k\
    \ <= self.v.len());\n        k += 1;\n        while k <= self.v.len() {\n    \
    \        self.v[k - 1] = M::op(&self.v[k - 1], &x);\n            k += k & k.wrapping_neg();\n\
    \        }\n    }\n\n    /// a[..k] \u306E\u7DCF\u7A4D\u3092\u53D6\u5F97\u3059\
    \u308B\u3002\n    ///\n    /// # \u8A08\u7B97\u91CF\n    ///\n    /// O(log n)\n\
    \    pub fn accum(&self, mut k: usize) -> M::S {\n        let mut res = M::e();\n\
    \        while k > 0 {\n            res = M::op(&res, &self.v[k - 1]);\n     \
    \       k &= k - 1;\n        }\n        res\n    }\n}\n\nimpl<M> FenwickTree<M>\n\
    where\n    M: Group,\n    M::S: Clone,\n{\n    /// a[range] \u306E\u7DCF\u7A4D\
    \u3092\u53D6\u5F97\u3059\u308B\u3002\n    ///\n    /// # \u8A08\u7B97\u91CF\n\
    \    ///\n    /// O(log n)\n    pub fn sum<R: RangeBounds<usize>>(&self, range:\
    \ R) -> M::S {\n        let r = match range.end_bound() {\n            Bound::Included(&r)\
    \ => r + 1,\n            Bound::Excluded(&r) => r,\n            Bound::Unbounded\
    \ => self.v.len(),\n        };\n        let l = match range.start_bound() {\n\
    \            Bound::Included(&l) => l,\n            Bound::Excluded(&l) => l +\
    \ 1,\n            Bound::Unbounded => return self.accum(r),\n        };\n    \
    \    M::op(&M::inv(&self.accum(l)), &self.accum(r))\n    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  isVerificationFile: false
  path: crates/data-structure/fenwick-tree/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-04 02:49:00+00:00'
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
