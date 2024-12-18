---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/range_kth_smallest/src/main.rs
    title: verify/range_kth_smallest/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/static_range_frequency/src/main.rs
    title: verify/static_range_frequency/src/main.rs
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
  code: "use std::ops::{Bound, RangeBounds};\n\nconst W: usize = 64;\n\n#[derive(Clone)]\n\
    struct BitVector {\n    bit: Vec<usize>,\n    sum: Vec<usize>,\n}\n\npub struct\
    \ WaveletMatrix {\n    n: usize,\n    mat: Vec<BitVector>,\n    mid: Vec<usize>,\n\
    }\n\nimpl WaveletMatrix {\n    pub fn new(mut a: Vec<usize>) -> Self {\n     \
    \   let n = a.len();\n        let max = a.iter().max().max(Some(&2)).unwrap();\n\
    \        let m = 64 - max.leading_zeros() as usize;\n        let mut mat = vec![BitVector::new(n\
    \ + 1); m];\n        let mut mid = vec![0; m];\n        for d in (0..m).rev()\
    \ {\n            let mut l = vec![];\n            let mut r = vec![];\n      \
    \      for i in 0..n {\n                if a[i] >> d & 1 == 1 {\n            \
    \        mat[d].set(i);\n                    r.push(a[i]);\n                }\
    \ else {\n                    l.push(a[i]);\n                }\n            }\n\
    \            mid[d] = l.len();\n            mat[d].build();\n            a = l;\n\
    \            a.append(&mut r);\n        }\n        Self { n, mat, mid }\n    }\n\
    \n    pub fn access(&self, mut k: usize) -> usize {\n        let mut res = 0;\n\
    \        for d in (0..self.mat.len()).rev() {\n            let f = self.mat[d].access(k);\n\
    \            if f {\n                res |= 1 << d;\n                k = self.mat[d].rank1(k)\
    \ + self.mid[d];\n            } else {\n                k = self.mat[d].rank0(k);\n\
    \            }\n        }\n        res\n    }\n\n    pub fn kth_smallest<R: RangeBounds<usize>>(&self,\
    \ range: R, mut k: usize) -> usize {\n        let (mut l, mut r) = range_to_pair(range,\
    \ self.n);\n        assert!(k < r - l);\n        let mut res = 0;\n        for\
    \ d in (0..self.mat.len()).rev() {\n            let cnt = self.mat[d].rank0(r)\
    \ - self.mat[d].rank0(l);\n            if cnt <= k {\n                res |= 1\
    \ << d;\n                k -= cnt;\n                let (ll, rr) = self.succ1(l..r,\
    \ d);\n                l = ll;\n                r = rr;\n            } else {\n\
    \                let (ll, rr) = self.succ0(l..r, d);\n                l = ll;\n\
    \                r = rr;\n            }\n        }\n        res\n    }\n\n   \
    \ pub fn kth_largest<R: RangeBounds<usize>>(&self, range: R, k: usize) -> usize\
    \ {\n        let (l, r) = range_to_pair(range, self.n);\n        self.kth_smallest(l..r,\
    \ r - l - k - 1)\n    }\n\n    pub fn range_freq<IR, VR>(&self, index_range: IR,\
    \ value_range: VR) -> usize\n    where\n        IR: RangeBounds<usize>,\n    \
    \    VR: RangeBounds<usize>,\n    {\n        let (il, ir) = range_to_pair(index_range,\
    \ self.n);\n        let (vl, vr) = range_to_pair(value_range, 1 << self.mat.len());\n\
    \        self.range_freq_(il, ir, vr) - self.range_freq_(il, ir, vl)\n    }\n\n\
    \    pub fn prev_value<R: RangeBounds<usize>>(&self, range: R, upper: usize) ->\
    \ Option<usize> {\n        let (l, r) = range_to_pair(range, self.n);\n      \
    \  let cnt = self.range_freq_(l, r, upper);\n        if cnt == 0 {\n         \
    \   None\n        } else {\n            Some(self.kth_smallest(l..r, cnt - 1))\n\
    \        }\n    }\n\n    pub fn next_value<R: RangeBounds<usize>>(&self, range:\
    \ R, lower: usize) -> Option<usize> {\n        let (l, r) = range_to_pair(range,\
    \ self.n);\n        let cnt = self.range_freq_(l, r, lower);\n        if cnt ==\
    \ r - l {\n            None\n        } else {\n            Some(self.kth_smallest(l..r,\
    \ cnt))\n        }\n    }\n\n    fn succ1<R: RangeBounds<usize>>(&self, range:\
    \ R, d: usize) -> (usize, usize) {\n        let (l, r) = range_to_pair(range,\
    \ self.n);\n        (\n            self.mat[d].rank1(l) + self.mid[d],\n     \
    \       self.mat[d].rank1(r) + self.mid[d],\n        )\n    }\n\n    fn succ0<R:\
    \ RangeBounds<usize>>(&self, range: R, d: usize) -> (usize, usize) {\n       \
    \ let (l, r) = range_to_pair(range, self.n);\n        (self.mat[d].rank0(l), self.mat[d].rank0(r))\n\
    \    }\n\n    fn range_freq_(&self, mut l: usize, mut r: usize, upper: usize)\
    \ -> usize {\n        if upper >= 1 << self.mat.len() {\n            return r\
    \ - l;\n        }\n        let mut res = 0;\n        for d in (0..self.mat.len()).rev()\
    \ {\n            if upper >> d & 1 == 1 {\n                res += self.mat[d].rank0(r)\
    \ - self.mat[d].rank0(l);\n                let (ll, rr) = self.succ1(l..r, d);\n\
    \                l = ll;\n                r = rr;\n                // (l, r) =\
    \ self.succ1(l..r, d); \u306FRust 1.42.0\u3067\u306F\u3067\u304D\u306A\u3044\n\
    \            } else {\n                let (ll, rr) = self.succ0(l..r, d);\n \
    \               l = ll;\n                r = rr;\n            }\n        }\n \
    \       res\n    }\n}\n\nimpl BitVector {\n    fn new(n: usize) -> Self {\n  \
    \      Self {\n            bit: vec![0; (n + 63) / W],\n            sum: vec![0;\
    \ (n + 63) / W],\n        }\n    }\n\n    fn set(&mut self, k: usize) {\n    \
    \    self.bit[k / W] |= 1 << k % W;\n    }\n\n    fn build(&mut self) {\n    \
    \    self.sum[0] = 0;\n        for i in 1..self.sum.len() {\n            self.sum[i]\
    \ = self.sum[i - 1] + self.bit[i - 1].count_ones() as usize;\n        }\n    }\n\
    \n    fn access(&self, k: usize) -> bool {\n        self.bit[k / W] >> k % W &\
    \ 1 == 1\n    }\n\n    fn rank1(&self, k: usize) -> usize {\n        self.sum[k\
    \ / W] + (self.bit[k / W] & (1 << k % W) - 1).count_ones() as usize\n    }\n\n\
    \    fn rank0(&self, k: usize) -> usize {\n        k - self.rank1(k)\n    }\n\
    }\n\nfn range_to_pair<R: RangeBounds<usize>>(range: R, n: usize) -> (usize, usize)\
    \ {\n    let l = match range.start_bound() {\n        Bound::Unbounded => 0,\n\
    \        Bound::Excluded(&l) => (l + 1).min(n),\n        Bound::Included(&l) =>\
    \ l.min(n),\n    };\n    let r = match range.end_bound() {\n        Bound::Unbounded\
    \ => n,\n        Bound::Excluded(&r) => r.min(n),\n        Bound::Included(&r)\
    \ => (r + 1).min(n),\n    };\n    assert!(l <= r);\n    (l, r)\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/data-structure/wavelet-matrix/src/lib.rs
  requiredBy: []
  timestamp: '2023-04-22 21:59:33+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/range_kth_smallest/src/main.rs
  - verify/static_range_frequency/src/main.rs
documentation_of: crates/data-structure/wavelet-matrix/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/wavelet-matrix/src/lib.rs
- /library/crates/data-structure/wavelet-matrix/src/lib.rs.html
title: crates/data-structure/wavelet-matrix/src/lib.rs
---
