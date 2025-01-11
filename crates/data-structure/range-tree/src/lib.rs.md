---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/segment-tree/src/lib.rs
    title: crates/data-structure/segment-tree/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/point_add_rectangle_sum/src/main.rs
    title: verify/point_add_rectangle_sum/src/main.rs
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
  code: "use std::ops::{Bound, RangeBounds};\n\nuse algebraic::Monoid;\nuse segment_tree::SegmentTree;\n\
    \n/// Range Tree\n///\n/// 2 \u6B21\u5143\u5E73\u9762\u4E0A\u306B\u914D\u7F6E\u3055\
    \u308C\u3066\u3044\u308B\u8981\u7D20\u3092\u7BA1\u7406\u3059\u308B\u30C7\u30FC\
    \u30BF\u69CB\u9020\u3002\n///\n/// # \u8A08\u7B97\u91CF\n///\n/// - \u69CB\u7BC9\
    : O(n log n)\n/// - get: O(log n)\n/// - apply: O(log n)\n/// - apply_range: O(log\
    \ n)\n///\n/// \u3053\u3053\u3067\u3001n \u306F\u7BA1\u7406\u3059\u308B\u70B9\u306E\
    \u6570\u3002\npub struct RangeTree<I, M>\nwhere\n    I: Ord + Copy,\n    M: Monoid,\n\
    \    M::S: Clone,\n{\n    n: usize,\n    seg: Vec<SegmentTree<M>>,\n    ps: Vec<(I,\
    \ I)>,\n    ys: Vec<Vec<I>>,\n}\n\nimpl<I, M> RangeTree<I, M>\nwhere\n    I: Ord\
    \ + Copy,\n    M: Monoid,\n    M::S: Clone,\n{\n    /// Range Tree \u3092\u69CB\
    \u7BC9\u3059\u308B\u3002\n    ///\n    /// # \u5F15\u6570\n    ///\n    /// *\
    \ `ps` - get \u30AF\u30A8\u30EA\u306E\u5F15\u6570\u3068\u3057\u3066\u4E0E\u3048\
    \u3089\u308C\u308B\u70B9\u306E\u96C6\u5408\n    pub fn new(mut ps: Vec<(I, I)>)\
    \ -> Self {\n        ps.sort();\n        ps.dedup();\n        let n = ps.len();\n\
    \        let mut seg: Vec<_> = (0..n * 2).map(|_| SegmentTree::new(0)).collect();\n\
    \        let mut ys = vec![vec![]; n * 2];\n        for i in 0..n {\n        \
    \    ys[n + i].push(ps[i].1);\n            seg[n + i] = SegmentTree::new(1);\n\
    \        }\n        for i in (1..n).rev() {\n            ys[i] = merge(&ys[i <<\
    \ 1], &ys[i << 1 | 1]);\n            seg[i] = SegmentTree::new(ys[i].len());\n\
    \        }\n        Self { n, seg, ps, ys }\n    }\n\n    /// \u70B9 (x, y) \u306B\
    \u5BFE\u5FDC\u3059\u308B\u8981\u7D20\u3092\u53D6\u5F97\u3059\u308B\u3002\n   \
    \ ///\n    /// # \u5F15\u6570\n    ///\n    /// * `(x, y)` - \u53D6\u5F97\u3057\
    \u305F\u3044\u8981\u7D20\u306E\u5EA7\u6A19\n    ///\n    /// # \u30D1\u30CB\u30C3\
    \u30AF\n    ///\n    /// \u6307\u5B9A\u3055\u308C\u305F\u5EA7\u6A19\u304C\u69CB\
    \u7BC9\u6642\u306B\u4E0E\u3048\u3089\u308C\u305F\u70B9\u96C6\u5408\u306B\u542B\
    \u307E\u308C\u3066\u3044\u306A\u3044\u5834\u5408\u3001\u30D1\u30CB\u30C3\u30AF\
    \u3059\u308B\u3002\n    ///\n    /// # \u8A08\u7B97\u91CF\n    ///\n    /// O(log\
    \ n)\n    pub fn get(&self, (x, y): (I, I)) -> M::S {\n        let i = self.ps.partition_point(|&p|\
    \ p < (x, y));\n        assert!(self.ps[i] == (x, y));\n        self.seg[self.n\
    \ + i].get(0)\n    }\n\n    /// \u70B9 (x, y) \u306B\u5BFE\u5FDC\u3059\u308B\u8981\
    \u7D20\u306B v \u3092\u52A0\u7B97\u3059\u308B\u3002\n    ///\n    /// # \u5F15\
    \u6570\n    ///\n    /// * `(x, y)` - \u52A0\u7B97\u3059\u308B\u8981\u7D20\u306E\
    \u5EA7\u6A19\n    /// * `v` - \u52A0\u7B97\u3059\u308B\u8981\u7D20\n    ///\n\
    \    /// # \u8A08\u7B97\u91CF\n    ///\n    /// O(log n)\n    pub fn add(&mut\
    \ self, (x, y): (I, I), v: M::S) {\n        let mut i = self.ps.partition_point(|&p|\
    \ p < (x, y));\n        assert!(self.ps[i] == (x, y));\n        i += self.n;\n\
    \        while i > 0 {\n            let j = self.ys[i].partition_point(|&t| t\
    \ < y);\n            let t = self.seg[i].get(j);\n            self.seg[i].set(j,\
    \ M::op(&t, &v));\n            i >>= 1;\n        }\n    }\n\n    /// x \u2208\
    \ range_x \u304B\u3064 y \u2208 range_y \u3092\u6E80\u305F\u3059\u5EA7\u6A19 (x,\
    \ y) \u306B\u914D\u7F6E\u3055\u308C\u3066\u3044\u308B\u8981\u7D20\u306E\u7DCF\u7A4D\
    \u3092\u53D6\u5F97\u3059\u308B\u3002\n    ///\n    /// # \u5F15\u6570\n    ///\n\
    \    /// * `range_x` - x \u5EA7\u6A19\u306E\u7BC4\u56F2\n    /// * `range_y` -\
    \ y \u5EA7\u6A19\u306E\u7BC4\u56F2\n    ///\n    /// # \u8A08\u7B97\u91CF\n  \
    \  ///\n    /// O(log^2 n)\n    pub fn prod(&self, range_x: impl RangeBounds<I>,\
    \ range_y: impl RangeBounds<I>) -> M::S {\n        let mut l = match range_x.start_bound()\
    \ {\n            Bound::Unbounded => 0,\n            Bound::Included(&l) => self.ps.partition_point(|&(x,\
    \ _)| x < l),\n            Bound::Excluded(&l) => self.ps.partition_point(|&(x,\
    \ _)| x <= l),\n        } + self.n;\n        let mut r = match range_x.end_bound()\
    \ {\n            Bound::Unbounded => self.n,\n            Bound::Excluded(&r)\
    \ => self.ps.partition_point(|&(x, _)| x < r),\n            Bound::Included(&r)\
    \ => self.ps.partition_point(|&(x, _)| x <= r),\n        } + self.n;\n       \
    \ let mut pl = M::e();\n        let mut pr = M::e();\n        while l < r {\n\
    \            if l & 1 != 0 {\n                let a = match range_y.start_bound()\
    \ {\n                    Bound::Unbounded => 0,\n                    Bound::Included(&a)\
    \ => self.ys[l].partition_point(|&y| y < a),\n                    Bound::Excluded(&a)\
    \ => self.ys[l].partition_point(|&y| y <= a),\n                };\n          \
    \      let b = match range_y.end_bound() {\n                    Bound::Unbounded\
    \ => self.ys[l].len(),\n                    Bound::Excluded(&b) => self.ys[l].partition_point(|&y|\
    \ y < b),\n                    Bound::Included(&b) => self.ys[l].partition_point(|&y|\
    \ y <= b),\n                };\n                pl = M::op(&pl, &self.seg[l].prod(a..b));\n\
    \                l += 1;\n            }\n            if r & 1 != 0 {\n       \
    \         r -= 1;\n                let a = match range_y.start_bound() {\n   \
    \                 Bound::Unbounded => 0,\n                    Bound::Included(&a)\
    \ => self.ys[r].partition_point(|&y| y < a),\n                    Bound::Excluded(&a)\
    \ => self.ys[r].partition_point(|&y| y <= a),\n                };\n          \
    \      let b = match range_y.end_bound() {\n                    Bound::Unbounded\
    \ => self.ys[r].len(),\n                    Bound::Excluded(&b) => self.ys[r].partition_point(|&y|\
    \ y < b),\n                    Bound::Included(&b) => self.ys[r].partition_point(|&y|\
    \ y <= b),\n                };\n                pr = M::op(&self.seg[r].prod(a..b),\
    \ &pr);\n            }\n            l >>= 1;\n            r >>= 1;\n        }\n\
    \        M::op(&pl, &pr)\n    }\n}\n\nfn merge<T>(a: &[T], b: &[T]) -> Vec<T>\n\
    where\n    T: Ord + Copy,\n{\n    let n = a.len();\n    let m = b.len();\n   \
    \ let mut i = 0;\n    let mut j = 0;\n    let mut c = vec![];\n    while i < n\
    \ || j < m {\n        let p = if j == m {\n            a[i]\n        } else if\
    \ i == n {\n            b[j]\n        } else {\n            a[i].min(b[j])\n \
    \       };\n        c.push(p);\n        while i < n && a[i] == p {\n         \
    \   i += 1;\n        }\n        while j < m && b[j] == p {\n            j += 1;\n\
    \        }\n    }\n    c\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/data-structure/segment-tree/src/lib.rs
  isVerificationFile: false
  path: crates/data-structure/range-tree/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-11 09:03:35+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/point_add_rectangle_sum/src/main.rs
documentation_of: crates/data-structure/range-tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/range-tree/src/lib.rs
- /library/crates/data-structure/range-tree/src/lib.rs.html
title: crates/data-structure/range-tree/src/lib.rs
---
