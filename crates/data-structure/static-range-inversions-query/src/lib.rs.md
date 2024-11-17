---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/math/div/src/lib.rs
    title: crates/math/div/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/static_range_inversions_query_2/src/main.rs
    title: verify/static_range_inversions_query_2/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{\n    cmp::Ordering,\n    ops::{Bound, RangeBounds},\n};\n\nuse\
    \ div::{div_ceil, div_floor};\n\n#[derive(Clone)]\npub struct StaticRangeInversionsQuery<T>\n\
    where\n    T: Copy + PartialOrd,\n{\n    n: usize,\n    block_size: usize,\n \
    \   block_num: usize,\n    sorted: Vec<Vec<(T, usize)>>,\n    inv: Vec<Vec<usize>>,\n\
    \    inv_rev: Vec<Vec<usize>>,\n}\n\nimpl<T> StaticRangeInversionsQuery<T>\nwhere\n\
    \    T: Copy + PartialOrd + Ord + Default,\n{\n    pub fn new(a: &[T]) -> Self\
    \ {\n        let n = a.len();\n        let block_size = 1.max((n as f64).sqrt().ceil()\
    \ as usize * 2);\n        let block_num = 1.max(div_ceil(n, block_size));\n  \
    \      let mut a = a.to_vec();\n        let max = a.iter().max().copied().unwrap_or_else(T::default);\n\
    \        a.extend(std::iter::repeat(max).take(block_num * block_size - a.len()));\n\
    \n        let mut sorted = vec![vec![]; block_num];\n        for block_i in 0..block_num\
    \ {\n            let l = block_i * block_size;\n            let r = (block_i +\
    \ 1) * block_size;\n            sorted[block_i] = a[l..r]\n                .iter()\n\
    \                .copied()\n                .enumerate()\n                .map(|(i,\
    \ x)| (x, l + i))\n                .collect::<Vec<_>>();\n            sorted[block_i].sort_unstable();\n\
    \        }\n\n        let mut res = Self {\n            n,\n            block_size,\n\
    \            block_num,\n            sorted,\n            inv: vec![],\n     \
    \       inv_rev: vec![],\n        };\n\n        res.inv = res.build(&a, |x, y|\
    \ x.cmp(y));\n        a.reverse();\n        res.inv_rev = res.build(&a, |x, y|\
    \ y.cmp(x));\n        res.inv_rev.reverse();\n        res\n    }\n\n    pub fn\
    \ inversions(&self, range: impl RangeBounds<usize>) -> usize {\n        let (l,\
    \ r) = self.range_to_pair(range);\n        assert!(l <= r && r <= self.n);\n\n\
    \        let bl = div_floor(l, self.block_size);\n        let br = div_ceil(r,\
    \ self.block_size);\n        if br - bl == 1 {\n            let mut res = self.inv[bl][r\
    \ - bl * self.block_size]\n                + self.inv_rev[bl + 1][(bl + 1) * self.block_size\
    \ - l]\n                - self.inv[bl][self.block_size];\n            let mut\
    \ cnt = 0;\n            for k in (0..self.block_size).rev() {\n              \
    \  if self.sorted[bl][k].1 < l {\n                    cnt += 1;\n            \
    \    }\n                if self.sorted[bl][k].1 >= r {\n                    res\
    \ += cnt;\n                }\n            }\n            res\n        } else {\n\
    \            let ml = (bl + 1) * self.block_size;\n            let mr = (br -\
    \ 1) * self.block_size;\n            let mut res =\n                self.inv[bl\
    \ + 1][r - ml] + self.inv_rev[br - 1][mr - l] - self.inv[bl + 1][mr - ml];\n \
    \           let mut cnt = 0;\n            let mut j = self.block_size;\n     \
    \       for k in (0..self.block_size).rev() {\n                if self.sorted[br\
    \ - 1][k].1 >= r {\n                    continue;\n                }\n       \
    \         while j > 0 && self.sorted[bl][j - 1].0 > self.sorted[br - 1][k].0 {\n\
    \                    j -= 1;\n                    if self.sorted[bl][j].1 >= l\
    \ {\n                        cnt += 1;\n                    }\n              \
    \  }\n                res += cnt;\n            }\n            res\n        }\n\
    \    }\n\n    fn build(&mut self, a: &[T], cmp: impl Fn(&T, &T) -> Ordering) ->\
    \ Vec<Vec<usize>> {\n        let mut inv = vec![vec![]; self.block_num + 1];\n\
    \n        let mut suf: Vec<(T, usize)> = vec![];\n        for block_i in (0..self.block_num).rev()\
    \ {\n            let l = block_i * self.block_size;\n            let r = (block_i\
    \ + 1) * self.block_size;\n            inv[block_i] = vec![0; self.block_size\
    \ * (self.block_num - block_i) + 1];\n            let mut pre = a[l..r]\n    \
    \            .iter()\n                .copied()\n                .enumerate()\n\
    \                .map(|(i, x)| (x, l + i))\n                .collect::<Vec<_>>();\n\
    \            pre.sort_unstable_by(|x, y| cmp(&x.0, &y.0));\n            for i\
    \ in 0..self.block_size {\n                inv[block_i][i + 1] = inv[block_i][i];\n\
    \                for j in 0..i {\n                    if cmp(&a[l + i], &a[l +\
    \ j]) == Ordering::Less {\n                        inv[block_i][i + 1] += 1;\n\
    \                    }\n                }\n            }\n\n            let mut\
    \ j = self.block_size;\n            for i in (0..self.block_size * (self.block_num\
    \ - 1 - block_i)).rev() {\n                while j > 0 && cmp(&suf[i].0, &pre[j\
    \ - 1].0) == Ordering::Less {\n                    j -= 1;\n                }\n\
    \                inv[block_i][suf[i].1 - l + 1] += self.block_size - j;\n    \
    \        }\n            for i in self.block_size..self.block_size * (self.block_num\
    \ - block_i) {\n                inv[block_i][i + 1] += inv[block_i][i] + inv[block_i\
    \ + 1][i + 1 - self.block_size]\n                    - inv[block_i + 1][i - self.block_size];\n\
    \            }\n\n            suf = Self::merge(&suf, &pre, |x, y| cmp(&x.0, &y.0));\n\
    \        }\n\n        inv\n    }\n\n    fn merge<U: Copy>(a: &[U], b: &[U], cmp:\
    \ impl Fn(&U, &U) -> Ordering) -> Vec<U> {\n        let mut res = Vec::with_capacity(a.len()\
    \ + b.len());\n        let (mut i, mut j) = (0, 0);\n        while i < a.len()\
    \ && j < b.len() {\n            if cmp(&a[i], &b[j]) != Ordering::Greater {\n\
    \                res.push(a[i]);\n                i += 1;\n            } else\
    \ {\n                res.push(b[j]);\n                j += 1;\n            }\n\
    \        }\n        res.extend_from_slice(&a[i..]);\n        res.extend_from_slice(&b[j..]);\n\
    \        res\n    }\n\n    fn range_to_pair(&self, range: impl RangeBounds<usize>)\
    \ -> (usize, usize) {\n        let l = match range.start_bound() {\n         \
    \   Bound::Included(&l) => l,\n            Bound::Excluded(&l) => l + 1,\n   \
    \         Bound::Unbounded => 0,\n        };\n        let r = match range.end_bound()\
    \ {\n            Bound::Included(&r) => r + 1,\n            Bound::Excluded(&r)\
    \ => r,\n            Bound::Unbounded => self.n,\n        };\n        (l, r)\n\
    \    }\n}\n"
  dependsOn:
  - crates/math/div/src/lib.rs
  isVerificationFile: false
  path: crates/data-structure/static-range-inversions-query/src/lib.rs
  requiredBy: []
  timestamp: '2024-11-17 18:56:30+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/static_range_inversions_query_2/src/main.rs
documentation_of: crates/data-structure/static-range-inversions-query/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/static-range-inversions-query/src/lib.rs
- /library/crates/data-structure/static-range-inversions-query/src/lib.rs.html
title: crates/data-structure/static-range-inversions-query/src/lib.rs
---
