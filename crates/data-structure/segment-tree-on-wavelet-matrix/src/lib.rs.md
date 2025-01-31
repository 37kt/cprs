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
    path: verify/point_add_rectangle_sum_2/src/main.rs
    title: verify/point_add_rectangle_sum_2/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::mem::swap;\n\nuse algebraic::Monoid;\nuse segment_tree::SegmentTree;\n\
    \n/// \u30A6\u30A7\u30FC\u30D6\u30EC\u30C3\u30C8\u884C\u5217\u306B\u30BB\u30B0\
    \u30E1\u30F3\u30C8\u6728\u3092\u8F09\u305B\u305F\u3082\u306E  \n/// \u77E9\u5F62\
    \u7BC4\u56F2\u306E\u7DCF\u7A4D\u3092\u53D6\u5F97\u3067\u304D\u308B\u3002\npub\
    \ struct SegmentTreeOnWaveletMatrix<I, M>\nwhere\n    I: Ord + Copy,\n    M: Monoid,\n\
    \    M::S: Clone,\n{\n    n: usize,\n    lg: usize,\n    bv: Vec<BitVector>,\n\
    \    seg: Vec<SegmentTree<M>>,\n    mid: Vec<usize>,\n    ps: Vec<(I, I)>,\n \
    \   ys: Vec<I>,\n}\n\nimpl<I, M> SegmentTreeOnWaveletMatrix<I, M>\nwhere\n   \
    \ I: Ord + Copy,\n    M: Monoid,\n    M::S: Clone,\n{\n    /// 2 \u6B21\u5143\u5EA7\
    \u6A19\u4E0A\u306E\u70B9\u306B\u3042\u308B\u8981\u7D20\u3092\u5358\u4F4D\u5143\
    \u3067\u521D\u671F\u5316\u3059\u308B\u3002\n    ///\n    /// # \u5F15\u6570\n\
    \    ///\n    /// - `ps`: \u30AF\u30A8\u30EA\u306E\u5F15\u6570\u3068\u3057\u3066\
    \u4E0E\u3048\u3089\u308C\u308B\u70B9\u306E\u96C6\u5408\n    ///\n    /// # \u8A08\
    \u7B97\u91CF\n    ///\n    /// TODO\n    pub fn new(mut ps: Vec<(I, I)>) -> Self\
    \ {\n        ps.sort();\n        ps.dedup();\n        let n = ps.len();\n    \
    \    let mut ys = ps.iter().map(|&(_, y)| y).collect::<Vec<_>>();\n        ys.sort();\n\
    \        ys.dedup();\n        let mut cur = vec![0; n];\n        let mut nxt =\
    \ vec![0; n];\n        for i in 0..n {\n            cur[i] = ys.binary_search(&ps[i].1).unwrap();\n\
    \        }\n        let lg = 64 - n.max(1).leading_zeros() as usize + 1;\n   \
    \     let mut bv = vec![BitVector::new(n + 1); lg];\n        let seg = (0..lg).map(|_|\
    \ SegmentTree::<M>::new(n)).collect();\n        let mut mid = vec![0; lg];\n \
    \       for h in (0..lg).rev() {\n            for i in 0..n {\n              \
    \  if cur[i] >> h & 1 == 1 {\n                    bv[h].set(i);\n            \
    \    }\n            }\n            bv[h].build();\n            let mut it = [0,\
    \ bv[h].rank0(n)];\n            mid[h] = it[1];\n            for i in 0..n {\n\
    \                let t = bv[h].access(i) as usize;\n                nxt[it[t]]\
    \ = cur[i];\n                it[t] += 1;\n            }\n            swap(&mut\
    \ cur, &mut nxt);\n        }\n        Self {\n            n,\n            lg,\n\
    \            bv,\n            seg,\n            mid,\n            ps,\n      \
    \      ys,\n        }\n    }\n\n    /// \u70B9 (x, y) \u306B\u5BFE\u5FDC\u3059\
    \u308B\u8981\u7D20\u3092 v \u306B\u66F4\u65B0\u3059\u308B\u3002\n    ///\n   \
    \ /// # \u5F15\u6570\n    ///\n    /// - `(x, y)`: \u70B9\n    /// - `v`: \u66F4\
    \u65B0\u3059\u308B\u5024\n    ///\n    /// # \u8A08\u7B97\u91CF\n    ///\n   \
    \ /// TODO\n    pub fn set(&mut self, (x, y): (I, I), v: M::S) {\n        let\
    \ mut i = self.ps.binary_search(&(x, y)).unwrap();\n        for h in (0..self.lg).rev()\
    \ {\n            let i0 = self.bv[h].rank0(i);\n            if self.bv[h].access(i)\
    \ {\n                i += self.bv[h].rank0(self.n) - i0;\n            } else {\n\
    \                i = i0;\n            }\n            self.seg[h].set(i, v.clone());\n\
    \        }\n    }\n\n    /// \u70B9 (x, y) \u306B\u5BFE\u5FDC\u3059\u308B\u8981\
    \u7D20\u306B v \u3092\u52A0\u7B97\u3059\u308B\u3002\n    ///\n    /// # \u5F15\
    \u6570\n    ///\n    /// - `(x, y)`: \u70B9\n    /// - `v`: \u52A0\u7B97\u3059\
    \u308B\u5024\n    ///\n    /// # \u8A08\u7B97\u91CF\n    ///\n    /// TODO\n \
    \   pub fn add(&mut self, (x, y): (I, I), v: M::S) {\n        let mut i = self.ps.binary_search(&(x,\
    \ y)).unwrap();\n        for h in (0..self.lg).rev() {\n            let i0 = self.bv[h].rank0(i);\n\
    \            if self.bv[h].access(i) {\n                i += self.bv[h].rank0(self.n)\
    \ - i0;\n            } else {\n                i = i0;\n            }\n      \
    \      let t = self.seg[h].get(i);\n            self.seg[h].set(i, M::op(&t, &v));\n\
    \        }\n    }\n\n    /// \u77E9\u5F62\u7BC4\u56F2\u306E\u7DCF\u7A4D\u3092\u53D6\
    \u5F97\u3059\u308B\u3002\n    ///\n    /// # \u5F15\u6570\n    ///\n    /// -\
    \ `range_x`: x \u5EA7\u6A19\u306E\u7BC4\u56F2\n    /// - `range_y`: y \u5EA7\u6A19\
    \u306E\u7BC4\u56F2\n    ///\n    /// # \u8A08\u7B97\u91CF\n    ///\n    /// TODO\n\
    \    pub fn prod(\n        &self,\n        range_x: impl std::ops::RangeBounds<I>,\n\
    \        range_y: impl std::ops::RangeBounds<I>,\n    ) -> M::S {\n        let\
    \ l = match range_x.start_bound() {\n            std::ops::Bound::Unbounded =>\
    \ 0,\n            std::ops::Bound::Included(&l) => self.ps.partition_point(|&(x,\
    \ _)| x < l),\n            std::ops::Bound::Excluded(&l) => self.ps.partition_point(|&(x,\
    \ _)| x <= l),\n        };\n        let r = match range_x.end_bound() {\n    \
    \        std::ops::Bound::Unbounded => self.ps.len(),\n            std::ops::Bound::Included(&r)\
    \ => self.ps.partition_point(|&(x, _)| x <= r),\n            std::ops::Bound::Excluded(&r)\
    \ => self.ps.partition_point(|&(x, _)| x < r),\n        };\n        let d = match\
    \ range_y.start_bound() {\n            std::ops::Bound::Unbounded => 0,\n    \
    \        std::ops::Bound::Included(&d) => self.ys.partition_point(|&y| y < d),\n\
    \            std::ops::Bound::Excluded(&d) => self.ys.partition_point(|&y| y <=\
    \ d),\n        };\n        let u = match range_y.end_bound() {\n            std::ops::Bound::Unbounded\
    \ => self.ys.len(),\n            std::ops::Bound::Included(&u) => self.ys.partition_point(|&y|\
    \ y <= u),\n            std::ops::Bound::Excluded(&u) => self.ys.partition_point(|&y|\
    \ y < u),\n        };\n        self.prod_(l, r, d, u, self.lg)\n    }\n\n    fn\
    \ prod_(&self, l: usize, r: usize, d: usize, u: usize, h: usize) -> M::S {\n \
    \       let u = u.min(1 << h);\n        if l >= r || d >= u {\n            return\
    \ M::e();\n        }\n        if d == 0 && u == 1 << h {\n            return self.seg[h].prod(l..r);\n\
    \        }\n        let h = h - 1;\n        let l0 = self.bv[h].rank0(l);\n  \
    \      let r0 = self.bv[h].rank0(r);\n        M::op(\n            &self.prod_(l0,\
    \ r0, d, u, h),\n            &self.prod_(\n                l + self.mid[h] - l0,\n\
    \                r + self.mid[h] - r0,\n                d.saturating_sub(1 <<\
    \ h),\n                u.saturating_sub(1 << h),\n                h,\n       \
    \     ),\n        )\n    }\n}\n\nconst W: usize = 64;\n\n#[derive(Clone)]\nstruct\
    \ BitVector {\n    bit: Vec<usize>,\n    sum: Vec<usize>,\n}\n\nimpl BitVector\
    \ {\n    fn new(n: usize) -> Self {\n        Self {\n            bit: vec![0;\
    \ (n + 63) / W],\n            sum: vec![0; (n + 63) / W],\n        }\n    }\n\n\
    \    fn set(&mut self, k: usize) {\n        self.bit[k / W] |= 1 << k % W;\n \
    \   }\n\n    fn build(&mut self) {\n        self.sum[0] = 0;\n        for i in\
    \ 1..self.sum.len() {\n            self.sum[i] = self.sum[i - 1] + self.bit[i\
    \ - 1].count_ones() as usize;\n        }\n    }\n\n    fn access(&self, k: usize)\
    \ -> bool {\n        self.bit[k / W] >> k % W & 1 == 1\n    }\n\n    fn rank1(&self,\
    \ k: usize) -> usize {\n        self.sum[k / W] + (self.bit[k / W] & (1 << k %\
    \ W) - 1).count_ones() as usize\n    }\n\n    fn rank0(&self, k: usize) -> usize\
    \ {\n        k - self.rank1(k)\n    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/data-structure/segment-tree/src/lib.rs
  isVerificationFile: false
  path: crates/data-structure/segment-tree-on-wavelet-matrix/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-11 09:03:35+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/point_add_rectangle_sum_2/src/main.rs
documentation_of: crates/data-structure/segment-tree-on-wavelet-matrix/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/segment-tree-on-wavelet-matrix/src/lib.rs
- /library/crates/data-structure/segment-tree-on-wavelet-matrix/src/lib.rs.html
title: crates/data-structure/segment-tree-on-wavelet-matrix/src/lib.rs
---
