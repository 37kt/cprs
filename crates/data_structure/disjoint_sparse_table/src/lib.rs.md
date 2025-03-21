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
    path: verify/library_checker/data_structure/staticrmq_dst/src/main.rs
    title: verify/library_checker/data_structure/staticrmq_dst/src/main.rs
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
  code: "use std::ops::RangeBounds;\n\nuse algebraic_traits::Monoid;\nuse into_half_open_range::IntoHalfOpenRange;\n\
    use numeric_traits::Integer;\n\npub struct DisjointSparseTable<M>\nwhere\n   \
    \ M: Monoid,\n{\n    n: usize,\n    table: Vec<Vec<M::Value>>,\n}\n\nimpl<M> FromIterator<M::Value>\
    \ for DisjointSparseTable<M>\nwhere\n    M: Monoid,\n{\n    fn from_iter<T: IntoIterator<Item\
    \ = M::Value>>(iter: T) -> Self {\n        let a = iter.into_iter().collect::<Vec<_>>();\n\
    \        let n = a.len();\n        let h = (n + 2).ceil_log2();\n        let mut\
    \ table = (0..h)\n            .map(|_| (0..n + 2).map(|_| M::unit()).collect::<Vec<_>>())\n\
    \            .collect::<Vec<_>>();\n        for k in 1..h {\n            let w\
    \ = 1 << k;\n            for i in (w..n + 2).step_by(w * 2) {\n              \
    \  for j in (i + 1 - w..i).rev() {\n                    table[k][j - 1] = M::op(&a[j\
    \ - 1], &table[k][j]);\n                }\n                for j in i..(i + w\
    \ - 1).min(n + 1) {\n                    table[k][j + 1] = M::op(&table[k][j],\
    \ &a[j - 1]);\n                }\n            }\n        }\n        Self { n,\
    \ table }\n    }\n}\n\nimpl<M> DisjointSparseTable<M>\nwhere\n    M: Monoid,\n\
    {\n    pub fn from_fn(n: usize, f: impl FnMut(usize) -> M::Value) -> Self {\n\
    \        Self::from_iter((0..n).map(f))\n    }\n\n    pub fn len(&self) -> usize\
    \ {\n        self.n\n    }\n\n    pub fn is_empty(&self) -> bool {\n        self.n\
    \ == 0\n    }\n\n    pub fn get(&self, i: usize) -> M::Value {\n        self.fold(i..i\
    \ + 1)\n    }\n\n    pub fn fold(&self, range: impl RangeBounds<usize>) -> M::Value\
    \ {\n        let (l, r) = range.into_half_open_range(0, self.n);\n        let\
    \ r = r + 1;\n        let k = (l ^ r).floor_log2();\n        let t = &self.table[k];\n\
    \        M::op(&t[l], &t[r])\n    }\n}\n\nimpl<M> Clone for DisjointSparseTable<M>\n\
    where\n    M: Monoid,\n    M::Value: Clone,\n{\n    fn clone(&self) -> Self {\n\
    \        Self {\n            n: self.n,\n            table: self.table.clone(),\n\
    \        }\n    }\n}\n"
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
  path: crates/data_structure/disjoint_sparse_table/src/lib.rs
  requiredBy: []
  timestamp: '2025-03-20 09:27:03+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/staticrmq_dst/src/main.rs
documentation_of: crates/data_structure/disjoint_sparse_table/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/disjoint_sparse_table/src/lib.rs
- /library/crates/data_structure/disjoint_sparse_table/src/lib.rs.html
title: crates/data_structure/disjoint_sparse_table/src/lib.rs
---
