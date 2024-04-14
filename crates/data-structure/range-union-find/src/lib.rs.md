---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/segment-tree/src/lib.rs
    title: crates/data-structure/segment-tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/math/modint61/src/lib.rs
    title: crates/math/modint61/src/lib.rs
  - icon: ':warning:'
    path: crates/misc/random/src/lib.rs
    title: crates/misc/random/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{\n    mem::swap,\n    ops::{Bound, RangeBounds},\n};\n\nuse algebraic::{algebra,\
    \ monoid};\nuse modint61::ModInt61;\nuse random::Pcg64Fast;\nuse segment_tree::SegmentTree;\n\
    \nalgebra!(M, (ModInt61, ModInt61));\nmonoid!(\n    M,\n    (0.into(), 1.into()),\n\
    \    |&(a, b): &(ModInt61, ModInt61), &(c, d): &(ModInt61, ModInt61)| (a * d +\
    \ c, b * d)\n);\n\npub struct RangeUnionFind {\n    n: usize,\n    base: ModInt61,\n\
    \    seg: SegmentTree<M>,\n    data: Vec<i32>,\n    next: Vec<usize>,\n}\n\nimpl\
    \ RangeUnionFind {\n    pub fn new(n: usize) -> Self {\n        let base = base();\n\
    \        let seg = SegmentTree::from((0..n).map(|i| (i.into(), base)).collect::<Vec<_>>());\n\
    \        Self {\n            n,\n            base,\n            seg,\n       \
    \     data: vec![-1; n],\n            next: vec![!0; n],\n        }\n    }\n\n\
    \    pub fn leader(&self, x: usize) -> usize {\n        if self.data[x] < 0 {\n\
    \            x\n        } else {\n            self.data[x] as usize\n        }\n\
    \    }\n\n    pub fn size(&self, x: usize) -> usize {\n        -self.data[self.leader(x)]\
    \ as usize\n    }\n\n    pub fn connected_components(&self, x: usize) -> Vec<usize>\
    \ {\n        let mut x = self.leader(x);\n        let mut res = vec![x];\n   \
    \     while self.next[x] != !0 {\n            x = self.next[x];\n            res.push(x);\n\
    \        }\n        res\n    }\n\n    pub fn merge_range(&mut self, xs: impl RangeBounds<usize>,\
    \ ys: impl RangeBounds<usize>) {\n        let (mut a, b) = self.range_to_pair(xs);\n\
    \        let (mut c, d) = self.range_to_pair(ys);\n        assert!(b - a == d\
    \ - c);\n        loop {\n            let mut ok = 0;\n            let mut ng =\
    \ b - a + 1;\n            while ok + 1 < ng {\n                let md = (ok +\
    \ ng) / 2;\n                if self.seg.prod(a..a + md) == self.seg.prod(c..c\
    \ + md) {\n                    ok = md;\n                } else {\n          \
    \          ng = md;\n                }\n            }\n            if ok == b\
    \ - a {\n                break;\n            }\n            a += ok;\n       \
    \     c += ok;\n            let mut x = self.leader(a);\n            let mut y\
    \ = self.leader(c);\n            assert!(x != y);\n            if self.size(x)\
    \ < self.size(y) {\n                swap(&mut x, &mut y);\n            }\n   \
    \         while self.next[y] != !0 {\n                let v = self.next[y];\n\
    \                self.next[y] = self.next[v];\n                self.seg.set(v,\
    \ (x.into(), self.base));\n                self.data[v] = x as i32;\n        \
    \        self.data[x] -= 1;\n                self.next[v] = self.next[x];\n  \
    \              self.next[x] = v;\n            }\n            self.seg.set(y, (x.into(),\
    \ self.base));\n            self.data[y] = x as i32;\n            self.data[x]\
    \ -= 1;\n            self.next[y] = self.next[x];\n            self.next[x] =\
    \ y;\n        }\n    }\n\n    pub fn merge(&mut self, x: usize, y: usize) {\n\
    \        self.merge_range(x..x + 1, y..y + 1);\n    }\n\n    fn range_to_pair(&self,\
    \ range: impl RangeBounds<usize>) -> (usize, usize) {\n        let l = match range.start_bound()\
    \ {\n            Bound::Included(&l) => l,\n            Bound::Excluded(&l) =>\
    \ l + 1,\n            Bound::Unbounded => 0,\n        };\n        let r = match\
    \ range.end_bound() {\n            Bound::Included(&r) => r + 1,\n           \
    \ Bound::Excluded(&r) => r,\n            Bound::Unbounded => self.n,\n       \
    \ };\n        (l, r)\n    }\n}\n\nfn base() -> ModInt61 {\n    fn gen() -> ModInt61\
    \ {\n        const FACTORS: [usize; 12] = [2, 3, 5, 7, 11, 13, 31, 41, 61, 151,\
    \ 331, 1321];\n        let mut rng = Pcg64Fast::default();\n        loop {\n \
    \           let x = ModInt61::new(rng.u64());\n            if FACTORS\n      \
    \          .iter()\n                .all(|&f| x.pow((ModInt61::modulus() as usize\
    \ - 1) / f).val() > 1)\n            {\n                return x;\n           \
    \ }\n        }\n    }\n\n    thread_local! {\n        static BASE: ModInt61 =\
    \ gen();\n    }\n    BASE.with(|base| *base)\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/data-structure/segment-tree/src/lib.rs
  - crates/math/modint61/src/lib.rs
  - crates/misc/random/src/lib.rs
  isVerificationFile: false
  path: crates/data-structure/range-union-find/src/lib.rs
  requiredBy: []
  timestamp: '2024-04-14 10:29:36+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data-structure/range-union-find/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/range-union-find/src/lib.rs
- /library/crates/data-structure/range-union-find/src/lib.rs.html
title: crates/data-structure/range-union-find/src/lib.rs
---
