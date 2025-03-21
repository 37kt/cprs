---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_traits/src/lib.rs
    title: crates/algebra/algebraic_traits/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_traits/src/macros.rs
    title: crates/algebra/algebraic_traits/src/macros.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_traits/src/traits.rs
    title: crates/algebra/algebraic_traits/src/traits.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/cast.rs
    title: crates/algebra/numeric_traits/src/cast.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/inf.rs
    title: crates/algebra/numeric_traits/src/inf.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/integer.rs
    title: crates/algebra/numeric_traits/src/integer.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/lib.rs
    title: crates/algebra/numeric_traits/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/numeric.rs
    title: crates/algebra/numeric_traits/src/numeric.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/signed.rs
    title: crates/algebra/numeric_traits/src/signed.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/zero_one.rs
    title: crates/algebra/numeric_traits/src/zero_one.rs
  - icon: ':warning:'
    path: crates/misc/into_half_open_range/src/lib.rs
    title: crates/misc/into_half_open_range/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/range_affine_range_sum/src/main.rs
    title: verify/library_checker/data_structure/range_affine_range_sum/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ops::{Deref, DerefMut, RangeBounds};\n\nuse algebraic_traits::{Act,\
    \ Algebraic, Magma, Monoid, Unital};\nuse into_half_open_range::IntoHalfOpenRange;\n\
    use numeric_traits::Integer;\n\npub struct LazySegmentTree<A>\nwhere\n    A: Act,\n\
    \    A::Operand: Monoid,\n    A::Operator: Monoid,\n    <A::Operand as Algebraic>::Value:\
    \ Clone,\n    <A::Operator as Algebraic>::Value: Clone + Eq,\n{\n    n: usize,\n\
    \    sz: usize,\n    lg: usize,\n    v: Vec<<A::Operand as Algebraic>::Value>,\n\
    \    lz: Vec<<A::Operator as Algebraic>::Value>,\n}\n\nimpl<A> FromIterator<<A::Operand\
    \ as Algebraic>::Value> for LazySegmentTree<A>\nwhere\n    A: Act,\n    A::Operand:\
    \ Monoid,\n    A::Operator: Monoid,\n    <A::Operand as Algebraic>::Value: Clone,\n\
    \    <A::Operator as Algebraic>::Value: Clone + Eq,\n{\n    fn from_iter<T: IntoIterator<Item\
    \ = <A::Operand as Algebraic>::Value>>(iter: T) -> Self {\n        let a = iter.into_iter().collect::<Vec<_>>();\n\
    \        let n = a.len();\n        let lg = n.checked_ceil_log2().unwrap_or(0);\n\
    \        let sz = 1 << lg;\n        let v = (0..sz)\n            .map(|_| A::Operand::unit())\n\
    \            .chain(a)\n            .chain((0..sz - n).map(|_| A::Operand::unit()))\n\
    \            .collect();\n        let lz = vec![A::Operator::unit(); sz];\n  \
    \      let mut seg = Self { n, sz, lg, v, lz };\n        for i in (1..sz).rev()\
    \ {\n            seg.update(i);\n        }\n        seg\n    }\n}\n\nimpl<A> LazySegmentTree<A>\n\
    where\n    A: Act,\n    A::Operand: Monoid,\n    A::Operator: Monoid,\n    <A::Operand\
    \ as Algebraic>::Value: Clone,\n    <A::Operator as Algebraic>::Value: Clone +\
    \ Eq,\n{\n    pub fn new(n: usize) -> Self {\n        let lg = n.checked_ceil_log2().unwrap_or(0);\n\
    \        let sz = 1 << lg;\n        let v = vec![A::Operand::unit(); sz * 2];\n\
    \        let lz = vec![A::Operator::unit(); sz];\n        Self { n, sz, lg, v,\
    \ lz }\n    }\n\n    pub fn from_fn(n: usize, f: impl FnMut(usize) -> <A::Operand\
    \ as Algebraic>::Value) -> Self {\n        Self::from_iter((0..n).map(f))\n  \
    \  }\n\n    pub fn len(&self) -> usize {\n        self.n\n    }\n\n    pub fn\
    \ is_empty(&self) -> bool {\n        self.n == 0\n    }\n\n    pub fn set(&mut\
    \ self, mut i: usize, x: <A::Operand as Algebraic>::Value) {\n        assert!(i\
    \ < self.n);\n        i += self.sz;\n        for h in (1..=self.lg).rev() {\n\
    \            self.push(i >> h);\n        }\n        self.v[i] = x;\n        for\
    \ h in 1..=self.lg {\n            self.update(i >> h);\n        }\n    }\n\n \
    \   pub fn add(&mut self, mut i: usize, x: <A::Operand as Algebraic>::Value) {\n\
    \        assert!(i < self.n);\n        i += self.sz;\n        for h in (1..=self.lg).rev()\
    \ {\n            self.push(i >> h);\n        }\n        self.v[i] = A::Operand::op(&self.v[i],\
    \ &x);\n        for h in 1..=self.lg {\n            self.update(i >> h);\n   \
    \     }\n    }\n\n    pub fn get(&mut self, mut i: usize) -> <A::Operand as Algebraic>::Value\
    \ {\n        assert!(i < self.n);\n        i += self.sz;\n        for h in (1..=self.lg).rev()\
    \ {\n            self.push(i >> h);\n        }\n        self.v[i].clone()\n  \
    \  }\n\n    pub fn get_mut(&mut self, mut i: usize) -> GetMut<A> {\n        assert!(i\
    \ < self.n);\n        i += self.sz;\n        for h in (1..=self.lg).rev() {\n\
    \            self.push(i >> h);\n        }\n        GetMut { seg: self, i }\n\
    \    }\n\n    pub fn as_slice(&mut self) -> &[<A::Operand as Algebraic>::Value]\
    \ {\n        for i in 1..self.sz {\n            self.push(i);\n        }\n   \
    \     &self.v[self.sz..self.sz + self.n]\n    }\n\n    pub fn fold(&mut self,\
    \ range: impl RangeBounds<usize>) -> <A::Operand as Algebraic>::Value {\n    \
    \    let (mut l, mut r) = range.into_half_open_range(0, self.n);\n        if l\
    \ == r {\n            return A::Operand::unit();\n        }\n        l += self.sz;\n\
    \        r += self.sz;\n        for h in (1..=self.lg).rev() {\n            if\
    \ l >> h << h != l {\n                self.push(l >> h);\n            }\n    \
    \        if r >> h << h != r {\n                self.push(r - 1 >> h);\n     \
    \       }\n        }\n        let mut sl = A::Operand::unit();\n        let mut\
    \ sr = A::Operand::unit();\n        while l < r {\n            if l & 1 != 0 {\n\
    \                sl = A::Operand::op(&sl, &self.v[l]);\n                l += 1;\n\
    \            }\n            if r & 1 != 0 {\n                r -= 1;\n       \
    \         sr = A::Operand::op(&self.v[r], &sr);\n            }\n            l\
    \ >>= 1;\n            r >>= 1;\n        }\n        A::Operand::op(&sl, &sr)\n\
    \    }\n\n    pub fn fold_all(&self) -> <A::Operand as Algebraic>::Value {\n \
    \       self.v[1].clone()\n    }\n\n    pub fn apply_range(\n        &mut self,\n\
    \        range: impl RangeBounds<usize>,\n        f: <A::Operator as Algebraic>::Value,\n\
    \    ) {\n        let (mut l, mut r) = range.into_half_open_range(0, self.n);\n\
    \        if l == r {\n            return;\n        }\n        l += self.sz;\n\
    \        r += self.sz;\n        for h in (1..=self.lg).rev() {\n            if\
    \ l >> h << h != l {\n                self.push(l >> h);\n            }\n    \
    \        if r >> h << h != r {\n                self.push(r - 1 >> h);\n     \
    \       }\n        }\n        let l2 = l;\n        let r2 = r;\n        while\
    \ l < r {\n            if l & 1 != 0 {\n                self.apply_at(l, &f);\n\
    \                l += 1;\n            }\n            if r & 1 != 0 {\n       \
    \         r -= 1;\n                self.apply_at(r, &f);\n            }\n    \
    \        l >>= 1;\n            r >>= 1;\n        }\n        l = l2;\n        r\
    \ = r2;\n        for h in 1..=self.lg {\n            if l >> h << h != l {\n \
    \               self.update(l >> h);\n            }\n            if r >> h <<\
    \ h != r {\n                self.update(r - 1 >> h);\n            }\n        }\n\
    \    }\n\n    pub fn apply(&mut self, i: usize, f: <A::Operator as Algebraic>::Value)\
    \ {\n        self.apply_range(i..i + 1, f);\n    }\n\n    pub fn max_right(\n\
    \        &mut self,\n        l: usize,\n        mut pred: impl FnMut(&<A::Operand\
    \ as Algebraic>::Value) -> bool,\n    ) -> usize {\n        assert!(l <= self.n);\n\
    \        assert!(pred(&A::Operand::unit()));\n        if l == self.n {\n     \
    \       return self.n;\n        }\n        let mut r = l + self.sz;\n        for\
    \ h in (1..=self.lg).rev() {\n            self.push(r >> h);\n        }\n    \
    \    let mut s = A::Operand::unit();\n        loop {\n            r >>= r.lsb_index();\n\
    \            let t = A::Operand::op(&s, &self.v[r]);\n            if !pred(&t)\
    \ {\n                while r < self.sz {\n                    self.push(r);\n\
    \                    r <<= 1;\n                    let t = A::Operand::op(&s,\
    \ &self.v[r]);\n                    if pred(&t) {\n                        s =\
    \ t;\n                        r += 1;\n                    }\n               \
    \ }\n                return r - self.sz;\n            }\n            s = t;\n\
    \            r += 1;\n            if r == r.lsb() {\n                break;\n\
    \            }\n        }\n        self.n\n    }\n\n    pub fn min_left(\n   \
    \     &mut self,\n        r: usize,\n        mut pred: impl FnMut(&<A::Operand\
    \ as Algebraic>::Value) -> bool,\n    ) -> usize {\n        assert!(r <= self.n);\n\
    \        assert!(pred(&A::Operand::unit()));\n        if r == 0 {\n          \
    \  return 0;\n        }\n        let mut l = r + self.sz;\n        for h in (1..=self.lg).rev()\
    \ {\n            self.push(l - 1 >> h);\n        }\n        let mut s = A::Operand::unit();\n\
    \        loop {\n            l -= 1;\n            l >>= (!l).lsb_index();\n  \
    \          l = l.max(1);\n            let t = A::Operand::op(&self.v[l], &s);\n\
    \            if !pred(&t) {\n                while l < self.sz {\n           \
    \         self.push(l);\n                    l = l << 1 | 1;\n               \
    \     let t = A::Operand::op(&self.v[l], &s);\n                    if pred(&t)\
    \ {\n                        s = t;\n                        l -= 1;\n       \
    \             }\n                }\n                return l + 1 - self.sz;\n\
    \            }\n            s = t;\n            if l == l.lsb() {\n          \
    \      break;\n            }\n        }\n        0\n    }\n\n    fn update(&mut\
    \ self, i: usize) {\n        self.v[i] = A::Operand::op(&self.v[i << 1], &self.v[i\
    \ << 1 | 1]);\n    }\n\n    fn apply_at(&mut self, i: usize, f: &<A::Operator\
    \ as Algebraic>::Value) {\n        self.v[i] = A::act(&self.v[i], f);\n      \
    \  if i < self.sz {\n            self.lz[i] = A::Operator::op(&self.lz[i], f);\n\
    \        }\n    }\n\n    fn push(&mut self, i: usize) {\n        if self.lz[i]\
    \ == A::Operator::unit() {\n            return;\n        }\n        let f = std::mem::replace(&mut\
    \ self.lz[i], A::Operator::unit());\n        self.apply_at(i << 1, &f);\n    \
    \    self.apply_at(i << 1 | 1, &f);\n    }\n}\n\npub struct GetMut<'a, A>\nwhere\n\
    \    A: Act,\n    A::Operand: Monoid,\n    A::Operator: Monoid,\n    <A::Operand\
    \ as Algebraic>::Value: Clone,\n    <A::Operator as Algebraic>::Value: Clone +\
    \ Eq,\n{\n    seg: &'a mut LazySegmentTree<A>,\n    i: usize,\n}\n\nimpl<'a, A>\
    \ Deref for GetMut<'a, A>\nwhere\n    A: Act,\n    A::Operand: Monoid,\n    A::Operator:\
    \ Monoid,\n    <A::Operand as Algebraic>::Value: Clone,\n    <A::Operator as Algebraic>::Value:\
    \ Clone + Eq,\n{\n    type Target = <A::Operand as Algebraic>::Value;\n\n    fn\
    \ deref(&self) -> &Self::Target {\n        &self.seg.v[self.i]\n    }\n}\n\nimpl<'a,\
    \ A> DerefMut for GetMut<'a, A>\nwhere\n    A: Act,\n    A::Operand: Monoid,\n\
    \    A::Operator: Monoid,\n    <A::Operand as Algebraic>::Value: Clone,\n    <A::Operator\
    \ as Algebraic>::Value: Clone + Eq,\n{\n    fn deref_mut(&mut self) -> &mut Self::Target\
    \ {\n        &mut self.seg.v[self.i]\n    }\n}\n\nimpl<'a, A> Drop for GetMut<'a,\
    \ A>\nwhere\n    A: Act,\n    A::Operand: Monoid,\n    A::Operator: Monoid,\n\
    \    <A::Operand as Algebraic>::Value: Clone,\n    <A::Operator as Algebraic>::Value:\
    \ Clone + Eq,\n{\n    fn drop(&mut self) {\n        for h in 1..=self.seg.lg {\n\
    \            self.seg.update(self.i >> h);\n        }\n    }\n}\n\nimpl<A> Clone\
    \ for LazySegmentTree<A>\nwhere\n    A: Act,\n    A::Operand: Monoid,\n    A::Operator:\
    \ Monoid,\n    <A::Operand as Algebraic>::Value: Clone,\n    <A::Operator as Algebraic>::Value:\
    \ Clone + Eq,\n{\n    fn clone(&self) -> Self {\n        Self {\n            n:\
    \ self.n,\n            sz: self.sz,\n            lg: self.lg,\n            v:\
    \ self.v.clone(),\n            lz: self.lz.clone(),\n        }\n    }\n}\n"
  dependsOn:
  - crates/algebra/algebraic_traits/src/lib.rs
  - crates/algebra/algebraic_traits/src/macros.rs
  - crates/algebra/algebraic_traits/src/traits.rs
  - crates/algebra/numeric_traits/src/cast.rs
  - crates/algebra/numeric_traits/src/inf.rs
  - crates/algebra/numeric_traits/src/integer.rs
  - crates/algebra/numeric_traits/src/lib.rs
  - crates/algebra/numeric_traits/src/numeric.rs
  - crates/algebra/numeric_traits/src/signed.rs
  - crates/algebra/numeric_traits/src/zero_one.rs
  - crates/misc/into_half_open_range/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/segment_tree/lazy_segment_tree/src/lib.rs
  requiredBy: []
  timestamp: '2025-03-20 09:27:03+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/range_affine_range_sum/src/main.rs
documentation_of: crates/data_structure/segment_tree/lazy_segment_tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/segment_tree/lazy_segment_tree/src/lib.rs
- /library/crates/data_structure/segment_tree/lazy_segment_tree/src/lib.rs.html
title: crates/data_structure/segment_tree/lazy_segment_tree/src/lib.rs
---
