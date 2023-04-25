---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':x:'
    path: verify/range_affine_range_sum/src/main.rs
    title: verify/range_affine_range_sum/src/main.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ops::{Bound, RangeBounds};\n\nuse algebraic::{Act, Monoid};\n\n\
    #[derive(Clone)]\npub struct LazySegmentTree<M, F>\nwhere\n    M: Monoid,\n  \
    \  M::S: Clone,\n    F: Act<X = M::S> + Monoid,\n    F::S: Clone,\n{\n    n: usize,\n\
    \    v: Vec<M::S>,\n    lz: Vec<F::S>,\n}\n\nimpl<M, F> From<Vec<M::S>> for LazySegmentTree<M,\
    \ F>\nwhere\n    M: Monoid,\n    M::S: Clone,\n    F: Act<X = M::S> + Monoid,\n\
    \    F::S: Clone,\n{\n    fn from(mut a: Vec<M::S>) -> Self {\n        let n =\
    \ a.len();\n        let mut v = vec![M::e(); n];\n        v.append(&mut a);\n\
    \        let lz = vec![F::e(); n];\n        let mut seg = LazySegmentTree { n,\
    \ v, lz };\n        for k in (1..n).rev() {\n            seg.update(k);\n    \
    \    }\n        seg\n    }\n}\n\nimpl<M, F> LazySegmentTree<M, F>\nwhere\n   \
    \ M: Monoid,\n    M::S: Clone,\n    F: Act<X = M::S> + Monoid,\n    F::S: Clone,\n\
    {\n    pub fn set(&mut self, k: usize, x: M::S) {\n        assert!(k < self.n);\n\
    \        let k = k + self.n;\n        let h = 63 - k.leading_zeros() as usize;\n\
    \        for i in (1..=h).rev() {\n            self.push(k >> i);\n        }\n\
    \        self.v[k] = x;\n        for i in 1..=h {\n            self.update(k >>\
    \ i);\n        }\n    }\n\n    pub fn get(&mut self, mut k: usize) -> M::S {\n\
    \        assert!(k < self.n);\n        k += self.n;\n        let h = 63 - k.leading_zeros()\
    \ as usize;\n        for i in (1..=h).rev() {\n            self.push(k >> i);\n\
    \        }\n        self.v[k].clone()\n    }\n\n    pub fn prod<R>(&mut self,\
    \ range: R) -> M::S\n    where\n        R: RangeBounds<usize>,\n    {\n      \
    \  let mut l = match range.start_bound() {\n            Bound::Excluded(&l) =>\
    \ l + 1,\n            Bound::Included(&l) => l,\n            Bound::Unbounded\
    \ => 0,\n        };\n        let mut r = match range.end_bound() {\n         \
    \   Bound::Excluded(&r) => r,\n            Bound::Included(&r) => r + 1,\n   \
    \         Bound::Unbounded => self.n,\n        };\n        assert!(l <= r);\n\
    \        assert!(r <= self.n);\n        if l == r {\n            return M::e();\n\
    \        }\n\n        l += self.n;\n        r += self.n;\n        let h = 63 -\
    \ self.n.leading_zeros() as usize;\n        for i in (1..=h).rev() {\n       \
    \     if (l >> i) << i != l {\n                self.push(l >> i);\n          \
    \  }\n            if (r >> i) << i != r {\n                self.push(r >> i);\n\
    \            }\n        }\n\n        let mut sl = M::e();\n        let mut sr\
    \ = M::e();\n        while l < r {\n            if l & 1 != 0 {\n            \
    \    sl = M::op(&sl, &self.v[l]);\n                l += 1;\n            }\n  \
    \          if r & 1 != 0 {\n                r -= 1;\n                sr = M::op(&self.v[r],\
    \ &sr);\n            }\n            l >>= 1;\n            r >>= 1;\n        }\n\
    \n        M::op(&sl, &sr)\n    }\n\n    pub fn apply(&mut self, k: usize, f: F::S)\
    \ {\n        assert!(k < self.n);\n        let k = k + self.n;\n        let h\
    \ = 63 - k.leading_zeros() as usize;\n        for i in (1..=h).rev() {\n     \
    \       self.push(k >> i);\n        }\n        self.v[k] = F::act(&f, &self.v[k]);\n\
    \        for i in 1..=h {\n            self.update(k >> i);\n        }\n    }\n\
    \n    pub fn apply_range<R>(&mut self, range: R, f: F::S)\n    where\n       \
    \ R: RangeBounds<usize>,\n    {\n        let mut l = match range.start_bound()\
    \ {\n            Bound::Excluded(&l) => l + 1,\n            Bound::Included(&l)\
    \ => l,\n            Bound::Unbounded => 0,\n        };\n        let mut r = match\
    \ range.end_bound() {\n            Bound::Excluded(&r) => r,\n            Bound::Included(&r)\
    \ => r + 1,\n            Bound::Unbounded => self.n,\n        };\n        assert!(l\
    \ <= r);\n        assert!(r <= self.n);\n\n        l += self.n;\n        r +=\
    \ self.n;\n        let h = 63 - self.n.leading_zeros() as usize;\n        for\
    \ i in (1..=h).rev() {\n            if (l >> i) << i != l {\n                self.push(l\
    \ >> i);\n            }\n            if (r >> i) << i != r {\n               \
    \ self.push(r - 1 >> i);\n            }\n        }\n\n        let l2 = l;\n  \
    \      let r2 = r;\n        while l < r {\n            if l & 1 != 0 {\n     \
    \           self.all_apply(l, f.clone());\n                l += 1;\n         \
    \   }\n            if r & 1 != 0 {\n                r -= 1;\n                self.all_apply(r,\
    \ f.clone());\n            }\n            l >>= 1;\n            r >>= 1;\n   \
    \     }\n        l = l2;\n        r = r2;\n\n        for i in 1..=h {\n      \
    \      if (l >> i) << i != l {\n                self.update(l >> i);\n       \
    \     }\n            if (r >> i) << i != r {\n                self.update(r -\
    \ 1 >> i);\n            }\n        }\n    }\n\n    fn update(&mut self, k: usize)\
    \ {\n        self.v[k] = M::op(&self.v[k * 2], &self.v[k * 2 + 1]);\n    }\n\n\
    \    fn all_apply(&mut self, k: usize, f: F::S) {\n        self.v[k] = F::act(&f,\
    \ &self.v[k]);\n        if k < self.n {\n            self.lz[k] = F::op(&f, &self.lz[k]);\n\
    \        }\n    }\n\n    fn push(&mut self, k: usize) {\n        self.all_apply(k\
    \ * 2, self.lz[k].clone());\n        self.all_apply(k * 2 + 1, self.lz[k].clone());\n\
    \        self.lz[k] = F::e();\n    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  isVerificationFile: false
  path: crates/data-structure/lazy-segment-tree/src/lib.rs
  requiredBy: []
  timestamp: '2023-04-25 18:17:30+09:00'
  verificationStatus: LIBRARY_ALL_WA
  verifiedWith:
  - verify/range_affine_range_sum/src/main.rs
documentation_of: crates/data-structure/lazy-segment-tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/lazy-segment-tree/src/lib.rs
- /library/crates/data-structure/lazy-segment-tree/src/lib.rs.html
title: crates/data-structure/lazy-segment-tree/src/lib.rs
---
