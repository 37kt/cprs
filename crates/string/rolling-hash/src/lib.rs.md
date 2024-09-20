---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{\n    cmp::Ordering,\n    marker::PhantomData,\n    ops::{Add,\
    \ Bound, Mul, Neg, RangeBounds},\n};\n\nuse modint61::ModInt61;\nuse nimber::Nimber;\n\
    use random::Pcg64Fast;\n\npub type RollingHashModInt61<'a, C> = RollingHash<'a,\
    \ C, ModInt61>;\npub type RollingHashNimber<'a, C> = RollingHash<'a, C, Nimber>;\n\
    \npub struct GenBaseImpl<T>(PhantomData<fn() -> T>);\n\npub trait GenBase {\n\
    \    type H;\n    fn base() -> Self::H;\n}\n\nimpl GenBase for GenBaseImpl<Nimber>\
    \ {\n    type H = Nimber;\n\n    fn base() -> Nimber {\n        fn gen() -> Nimber\
    \ {\n            let mut rng = Pcg64Fast::default();\n            Nimber::new(rng.u64())\n\
    \        }\n\n        thread_local! {\n            static BASE: Nimber = gen();\n\
    \        }\n        BASE.with(|base| *base)\n    }\n}\n\nimpl GenBase for GenBaseImpl<ModInt61>\
    \ {\n    type H = ModInt61;\n\n    fn base() -> ModInt61 {\n        fn gen() ->\
    \ ModInt61 {\n            const FACTORS: [usize; 12] = [2, 3, 5, 7, 11, 13, 31,\
    \ 41, 61, 151, 331, 1321];\n            let mut rng = Pcg64Fast::default();\n\
    \            loop {\n                let x = ModInt61::new(rng.u64());\n     \
    \           if FACTORS\n                    .iter()\n                    .all(|&f|\
    \ x.pow((ModInt61::modulus() as usize - 1) / f).val() > 1)\n                {\n\
    \                    return x;\n                }\n            }\n        }\n\n\
    \        thread_local! {\n            static BASE: ModInt61 = gen();\n       \
    \ }\n        BASE.with(|base| *base)\n    }\n}\n\n#[derive(Hash, PartialEq, Eq,\
    \ Clone, Copy)]\npub struct Hash<H>\nwhere\n    H: Copy + Eq + Add<Output = H>\
    \ + Mul<Output = H> + Neg<Output = H>,\n{\n    hash: H,\n    pow: H,\n}\n\nimpl<H>\
    \ Hash<H>\nwhere\n    H: Copy + Eq + Add<Output = H> + Mul<Output = H> + Neg<Output\
    \ = H>,\n{\n    pub fn val(&self) -> H {\n        self.hash\n    }\n\n    pub\
    \ fn concat(&self, other: Self) -> Self {\n        Self {\n            hash: self.hash\
    \ * other.pow + other.hash,\n            pow: self.pow * other.pow,\n        }\n\
    \    }\n}\n\n#[derive(Clone)]\npub struct RollingHash<'a, C, H>\nwhere\n    C:\
    \ Copy + Eq + Into<H>,\n    H: Copy + Eq + Add<Output = H> + Mul<Output = H> +\
    \ Neg<Output = H>,\n    GenBaseImpl<H>: GenBase<H = H>,\n{\n    s: &'a [C],\n\
    \    hs: Box<[H]>,\n    pw: Box<[H]>,\n}\n\nimpl<'a, C, H> RollingHash<'a, C,\
    \ H>\nwhere\n    C: Copy + Eq + Into<H>,\n    H: Copy + Eq + From<u64> + Add<Output\
    \ = H> + Mul<Output = H> + Neg<Output = H>,\n    GenBaseImpl<H>: GenBase<H = H>,\n\
    {\n    pub fn new(s: &'a [C]) -> Self {\n        let n = s.len();\n        let\
    \ base = GenBaseImpl::<H>::base();\n        let mut hs = vec![H::from(0); n +\
    \ 1];\n        let mut pw = vec![H::from(1); n + 1];\n        for i in 0..n {\n\
    \            hs[i + 1] = hs[i] * base + s[i].into();\n            pw[i + 1] =\
    \ pw[i] * base;\n        }\n        Self {\n            s,\n            hs: hs.into_boxed_slice(),\n\
    \            pw: pw.into_boxed_slice(),\n        }\n    }\n\n    pub fn base(&self)\
    \ -> H {\n        GenBaseImpl::<H>::base()\n    }\n\n    pub fn len(&self) ->\
    \ usize {\n        self.s.len()\n    }\n\n    pub fn get(&self, index: impl RangeBounds<usize>)\
    \ -> Hash<H> {\n        let (l, r) = range_to_pair(index, self.len());\n     \
    \   Hash {\n            hash: self.hs[l] * -self.pw[r - l] + self.hs[r],\n   \
    \         pow: self.pw[r - l],\n        }\n    }\n\n    pub fn lcp(\n        &self,\n\
    \        index1: impl RangeBounds<usize>,\n        other: &Self,\n        index2:\
    \ impl RangeBounds<usize>,\n    ) -> usize {\n        let (l1, r1) = range_to_pair(index1,\
    \ self.len());\n        let (l2, r2) = range_to_pair(index2, other.len());\n \
    \       let n = (r1 - l1).min(r2 - l2);\n        let mut ok = 0;\n        let\
    \ mut ng = n + 1;\n        while ok + 1 < ng {\n            let md = (ok + ng)\
    \ / 2;\n            if self.get(l1..l1 + md) == other.get(l2..l2 + md) {\n   \
    \             ok = md;\n            } else {\n                ng = md;\n     \
    \       }\n        }\n        ok\n    }\n}\n\nimpl<'a, C, H> RollingHash<'a, C,\
    \ H>\nwhere\n    C: Copy + Eq + Into<H> + Ord,\n    H: Copy + Eq + From<u64> +\
    \ Add<Output = H> + Mul<Output = H> + Neg<Output = H>,\n    GenBaseImpl<H>: GenBase<H\
    \ = H>,\n{\n    pub fn compare(\n        &self,\n        index1: impl RangeBounds<usize>,\n\
    \        other: &Self,\n        index2: impl RangeBounds<usize>,\n    ) -> Ordering\
    \ {\n        let (l1, r1) = range_to_pair(index1, self.len());\n        let (l2,\
    \ r2) = range_to_pair(index2, other.len());\n        let n = self.lcp(l1..r1,\
    \ other, l2..r2);\n        if l1 + n == r1 {\n            if l2 + n == r2 {\n\
    \                Ordering::Equal\n            } else {\n                Ordering::Less\n\
    \            }\n        } else if l2 + n == r2 {\n            Ordering::Greater\n\
    \        } else {\n            self.s[l1 + n].cmp(&other.s[l2 + n])\n        }\n\
    \    }\n}\n\nfn range_to_pair(range: impl RangeBounds<usize>, n: usize) -> (usize,\
    \ usize) {\n    let l = match range.start_bound() {\n        Bound::Included(&l)\
    \ => l,\n        Bound::Excluded(&l) => l + 1,\n        Bound::Unbounded => 0,\n\
    \    };\n    let r = match range.end_bound() {\n        Bound::Included(&r) =>\
    \ r + 1,\n        Bound::Excluded(&r) => r,\n        Bound::Unbounded => n,\n\
    \    };\n    (l, r)\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/string/rolling-hash/src/lib.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/string/rolling-hash/src/lib.rs
layout: document
redirect_from:
- /library/crates/string/rolling-hash/src/lib.rs
- /library/crates/string/rolling-hash/src/lib.rs.html
title: crates/string/rolling-hash/src/lib.rs
---
