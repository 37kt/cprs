---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/union-find/src/lib.rs
    title: crates/data-structure/union-find/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/range_parallel_unionfind/src/main.rs
    title: verify/range_parallel_unionfind/src/main.rs
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
  code: "use std::ops::{Bound, RangeBounds};\n\nuse union_find::UnionFind;\n\npub\
    \ struct RangeUnionFind {\n    n: usize,\n    uf: Vec<UnionFind>,\n}\n\nimpl RangeUnionFind\
    \ {\n    pub fn new(n: usize) -> Self {\n        let mut log = 1;\n        while\
    \ 1 << log < n {\n            log += 1;\n        }\n        let uf = (0..log).map(|i|\
    \ UnionFind::new(n - (1 << i) + 1)).collect();\n        Self { n, uf }\n    }\n\
    \n    pub fn leader(&self, x: usize) -> usize {\n        self.uf[0].leader(x)\n\
    \    }\n\n    pub fn size(&self, x: usize) -> usize {\n        self.uf[0].size(x)\n\
    \    }\n\n    pub fn merge_range(\n        &mut self,\n        xs: impl RangeBounds<usize>,\n\
    \        ys: impl RangeBounds<usize>,\n    ) -> Vec<(usize, usize)> {\n      \
    \  let (a, b) = self.range_to_pair(xs);\n        let (c, d) = self.range_to_pair(ys);\n\
    \        assert!(b - a == d - c);\n        let mut res = vec![];\n        if a\
    \ == c || b - a == 0 {\n            return res;\n        }\n        let s = 63\
    \ - (b - a).leading_zeros() as usize;\n        self.merge_range_(a, c, s, &mut\
    \ res);\n        self.merge_range_(b - (1 << s), d - (1 << s), s, &mut res);\n\
    \        res\n    }\n\n    pub fn merge(&mut self, x: usize, y: usize) -> Option<(usize,\
    \ usize)> {\n        self.merge_range(x..x + 1, y..y + 1).pop()\n    }\n\n   \
    \ fn merge_range_(&mut self, l: usize, r: usize, k: usize, res: &mut Vec<(usize,\
    \ usize)>) {\n        let x = self.uf[k].leader(l);\n        let y = self.uf[k].leader(r);\n\
    \        if self.uf[k].merge(l, r) {\n            if k == 0 {\n              \
    \  let z = self.uf[k].leader(l);\n                res.push((z, x ^ y ^ z));\n\
    \            } else {\n                self.merge_range_(l, r, k - 1, res);\n\
    \                self.merge_range_(l + (1 << k - 1), r + (1 << k - 1), k - 1,\
    \ res);\n            }\n        }\n    }\n\n    fn range_to_pair(&self, range:\
    \ impl RangeBounds<usize>) -> (usize, usize) {\n        let l = match range.start_bound()\
    \ {\n            Bound::Included(&l) => l,\n            Bound::Excluded(&l) =>\
    \ l + 1,\n            Bound::Unbounded => 0,\n        };\n        let r = match\
    \ range.end_bound() {\n            Bound::Included(&r) => r + 1,\n           \
    \ Bound::Excluded(&r) => r,\n            Bound::Unbounded => self.n,\n       \
    \ };\n        (l, r)\n    }\n}\n"
  dependsOn:
  - crates/data-structure/union-find/src/lib.rs
  isVerificationFile: false
  path: crates/data-structure/range-union-find/src/lib.rs
  requiredBy: []
  timestamp: '2024-10-22 10:00:24+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/range_parallel_unionfind/src/main.rs
documentation_of: crates/data-structure/range-union-find/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/range-union-find/src/lib.rs
- /library/crates/data-structure/range-union-find/src/lib.rs.html
title: crates/data-structure/range-union-find/src/lib.rs
---
