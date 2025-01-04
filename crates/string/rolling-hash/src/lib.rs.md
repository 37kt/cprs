---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/math/modint61/src/lib.rs
    title: crates/math/modint61/src/lib.rs
  - icon: ':warning:'
    path: crates/misc/random/src/lib.rs
    title: crates/misc/random/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/suffixarray_rolling_hash/src/main.rs
    title: verify/suffixarray_rolling_hash/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/zalgorithm_rolling_hash/src/main.rs
    title: verify/zalgorithm_rolling_hash/src/main.rs
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
  code: "fn gen() -> ModInt61 {\n    const FACTORS: [usize; 12] = [2, 3, 5, 7, 11,\
    \ 13, 31, 41, 61, 151, 331, 1321];\n    let mut rng = Pcg64Fast::default();\n\
    \    loop {\n        let x = ModInt61::new(rng.u64());\n        if FACTORS\n \
    \           .iter()\n            .all(|&f| x.pow((ModInt61::modulus() as usize\
    \ - 1) / f).val() > 1)\n        {\n            return x;\n        }\n    }\n}\n\
    \nfn base() -> ModInt61 {\n    thread_local! {\n        static BASE: ModInt61\
    \ = gen();\n    }\n    BASE.with(|base| *base)\n}\n\n/// \u30ED\u30FC\u30EA\u30F3\
    \u30B0\u30CF\u30C3\u30B7\u30E5\npub struct RollingHash {\n    pow: RefCell<Vec<ModInt61>>,\n\
    }\n\nimpl RollingHash {\n    pub fn new() -> Self {\n        Self {\n        \
    \    pow: RefCell::new(vec![ModInt61::one(), base()]),\n        }\n    }\n\n \
    \   pub fn base() -> ModInt61 {\n        base()\n    }\n\n    /// \u914D\u5217\
    \ `s` \u306E\u90E8\u5206\u5217\u306E\u30CF\u30C3\u30B7\u30E5\u3092\u6C42\u3081\
    \u308B\u305F\u3081\u306E\u30C6\u30FC\u30D6\u30EB\u3092\u69CB\u7BC9\n    pub fn\
    \ build_table<'a, 'b, T>(&'a self, s: &'b [T]) -> RollingHashTable<'a, 'b, T>\n\
    \    where\n        T: Clone + Into<ModInt61>,\n    {\n        let n = s.len();\n\
    \        let mut hash = vec![ModInt61::zero(); n + 1];\n        for i in 0..n\
    \ {\n            hash[i + 1] = hash[i] * self.pow(1) + s[i].clone().into();\n\
    \        }\n        RollingHashTable { rh: self, s, hash }\n    }\n\n    /// \u914D\
    \u5217 `s` \u306E\u30CF\u30C3\u30B7\u30E5\u3092\u6C42\u3081\u308B\n    pub fn\
    \ hash<T>(s: &[T]) -> RollingHashedSequence\n    where\n        T: Clone + Into<ModInt61>,\n\
    \    {\n        RollingHashedSequence::from_slice(s)\n    }\n\n    fn expand(&self,\
    \ n: usize) {\n        let mut pow = self.pow.borrow_mut();\n        for i in\
    \ pow.len()..=n {\n            let x = pow[i - 1] * base();\n            pow.push(x);\n\
    \        }\n    }\n\n    fn pow(&self, n: usize) -> ModInt61 {\n        self.expand(n);\n\
    \        self.pow.borrow()[n]\n    }\n}\n\n#[derive(Clone)]\npub struct RollingHashTable<'a,\
    \ 'b, T> {\n    rh: &'a RollingHash,\n    s: &'b [T],\n    hash: Vec<ModInt61>,\n\
    }\n\nimpl<'a, 'b, T> RollingHashTable<'a, 'b, T> {\n    /// \u914D\u5217\u306E\
    \u9577\u3055\u3092\u8FD4\u3059\n    pub fn len(&self) -> usize {\n        self.s.len()\n\
    \    }\n\n    /// \u914D\u5217\u306E\u90E8\u5206\u5217\u306E\u30CF\u30C3\u30B7\
    \u30E5\u3092\u6C42\u3081\u308B\n    pub fn get(&self, index: impl RangeBounds<usize>)\
    \ -> RollingHashedSequence {\n        let (l, r) = range_to_pair(index, self.s.len());\n\
    \        RollingHashedSequence {\n            hash: self.hash[l] * -self.rh.pow(r\
    \ - l) + self.hash[r],\n            len: r - l,\n            pow: self.rh.pow(r\
    \ - l),\n        }\n    }\n\n    /// 2 \u3064\u306E\u914D\u5217\u306E\u6700\u9577\
    \u5171\u901A\u63A5\u982D\u8F9E\u306E\u9577\u3055\u3092\u6C42\u3081\u308B\n   \
    \ pub fn lcp(\n        &self,\n        index1: impl RangeBounds<usize>,\n    \
    \    other: &Self,\n        index2: impl RangeBounds<usize>,\n    ) -> usize {\n\
    \        let (l1, r1) = range_to_pair(index1, self.len());\n        let (l2, r2)\
    \ = range_to_pair(index2, other.len());\n        let n = (r1 - l1).min(r2 - l2);\n\
    \        let mut ok = 0;\n        let mut ng = n + 1;\n        while ok + 1 <\
    \ ng {\n            let md = (ok + ng) / 2;\n            if self.get(l1..l1 +\
    \ md) == other.get(l2..l2 + md) {\n                ok = md;\n            } else\
    \ {\n                ng = md;\n            }\n        }\n        ok\n    }\n\n\
    \    /// 2 \u3064\u306E\u914D\u5217\u306E\u8F9E\u66F8\u9806\u6BD4\u8F03\n    pub\
    \ fn compare(\n        &self,\n        index1: impl RangeBounds<usize>,\n    \
    \    other: &Self,\n        index2: impl RangeBounds<usize>,\n    ) -> Ordering\n\
    \    where\n        T: Ord,\n    {\n        let (l1, r1) = range_to_pair(index1,\
    \ self.len());\n        let (l2, r2) = range_to_pair(index2, other.len());\n \
    \       let n = self.lcp(l1..r1, other, l2..r2);\n        if l1 + n == r1 {\n\
    \            if l2 + n == r2 {\n                Ordering::Equal\n            }\
    \ else {\n                Ordering::Less\n            }\n        } else if l2\
    \ + n == r2 {\n            Ordering::Greater\n        } else {\n            self.s[l1\
    \ + n].cmp(&other.s[l2 + n])\n        }\n    }\n}\n\n/// \u30CF\u30C3\u30B7\u30E5\
    \u5316\u3055\u308C\u305F\u6570\u5217\n#[derive(Clone, Copy, PartialEq, Eq, Hash)]\n\
    pub struct RollingHashedSequence {\n    hash: ModInt61,\n    len: usize,\n   \
    \ pow: ModInt61,\n}\n\nimpl Default for RollingHashedSequence {\n    fn default()\
    \ -> Self {\n        Self {\n            hash: ModInt61::zero(),\n           \
    \ len: 0,\n            pow: ModInt61::one(),\n        }\n    }\n}\n\nimpl RollingHashedSequence\
    \ {\n    /// \u914D\u5217 `s` \u306E\u30CF\u30C3\u30B7\u30E5\u3092\u6C42\u3081\
    \u308B\n    pub fn from_slice<T>(s: &[T]) -> Self\n    where\n        T: Clone\
    \ + Into<ModInt61>,\n    {\n        let mut hash = ModInt61::zero();\n       \
    \ for c in s {\n            hash = hash * base() + c.clone().into();\n       \
    \ }\n        Self {\n            hash,\n            len: s.len(),\n          \
    \  pow: base().pow(s.len()),\n        }\n    }\n\n    /// \u30CF\u30C3\u30B7\u30E5\
    \u3092\u8FD4\u3059\n    pub fn hash(self) -> ModInt61 {\n        self.hash\n \
    \   }\n\n    /// \u914D\u5217\u306E\u9577\u3055\u3092\u8FD4\u3059\n    pub fn\
    \ len(self) -> usize {\n        self.len\n    }\n\n    /// 2 \u3064\u306E\u30CF\
    \u30C3\u30B7\u30E5\u5316\u3055\u308C\u305F\u6570\u5217\u3092\u9023\u7D50\n   \
    \ pub fn concat(self, other: Self) -> Self {\n        Self {\n            hash:\
    \ self.hash * other.pow + other.hash,\n            len: self.len.wrapping_add(other.len),\n\
    \            pow: self.pow * other.pow,\n        }\n    }\n\n    /// \u30CF\u30C3\
    \u30B7\u30E5\u5316\u3055\u308C\u305F\u6570\u5217\u3092 `n` \u56DE\u7E70\u308A\u8FD4\
    \u3059\n    pub fn repeat(self, n: usize) -> Self {\n        RollingHashMonoid::pow(&self,\
    \ n)\n    }\n}\n\n#[derive(Clone, Copy)]\npub enum RollingHashMonoid {}\n\nimpl\
    \ Algebra for RollingHashMonoid {\n    type S = RollingHashedSequence;\n}\n\n\
    impl Monoid for RollingHashMonoid {\n    fn e() -> Self::S {\n        Self::S::default()\n\
    \    }\n\n    fn op(a: &Self::S, b: &Self::S) -> Self::S {\n        a.concat(*b)\n\
    \    }\n}\n\nuse std::{\n    cell::RefCell,\n    cmp::Ordering,\n    ops::{Bound,\
    \ RangeBounds},\n};\n\nuse algebraic::{Algebra, Monoid, One, Zero};\nuse modint61::ModInt61;\n\
    use random::Pcg64Fast;\n\nfn range_to_pair(range: impl RangeBounds<usize>, n:\
    \ usize) -> (usize, usize) {\n    let l = match range.start_bound() {\n      \
    \  Bound::Included(&l) => l,\n        Bound::Excluded(&l) => l + 1,\n        Bound::Unbounded\
    \ => 0,\n    };\n    let r = match range.end_bound() {\n        Bound::Included(&r)\
    \ => r + 1,\n        Bound::Excluded(&r) => r,\n        Bound::Unbounded => n,\n\
    \    };\n    (l, r)\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/math/modint61/src/lib.rs
  - crates/misc/random/src/lib.rs
  isVerificationFile: false
  path: crates/string/rolling-hash/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-04 07:22:45+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/suffixarray_rolling_hash/src/main.rs
  - verify/zalgorithm_rolling_hash/src/main.rs
documentation_of: crates/string/rolling-hash/src/lib.rs
layout: document
redirect_from:
- /library/crates/string/rolling-hash/src/lib.rs
- /library/crates/string/rolling-hash/src/lib.rs.html
title: crates/string/rolling-hash/src/lib.rs
---
