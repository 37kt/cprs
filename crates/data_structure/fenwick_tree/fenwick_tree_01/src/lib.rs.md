---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/add.rs
    title: crates/algebra/algebraic_structure/src/add.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/affine.rs
    title: crates/algebra/algebraic_structure/src/affine.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/count_sum.rs
    title: crates/algebra/algebraic_structure/src/count_sum.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/countsum_affine.rs
    title: crates/algebra/algebraic_structure/src/countsum_affine.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/lib.rs
    title: crates/algebra/algebraic_structure/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/max.rs
    title: crates/algebra/algebraic_structure/src/max.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/min.rs
    title: crates/algebra/algebraic_structure/src/min.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/mul.rs
    title: crates/algebra/algebraic_structure/src/mul.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/semiring.rs
    title: crates/algebra/algebraic_structure/src/semiring.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/trivial_group.rs
    title: crates/algebra/algebraic_structure/src/trivial_group.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/xor.rs
    title: crates/algebra/algebraic_structure/src/xor.rs
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
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/fenwick_tree/fenwick_tree/src/lib.rs
    title: crates/data_structure/fenwick_tree/fenwick_tree/src/lib.rs
  - icon: ':warning:'
    path: crates/misc/as_half_open_range/src/lib.rs
    title: crates/misc/as_half_open_range/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/majority_voting/src/main.rs
    title: verify/library_checker/data_structure/majority_voting/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/point_set_range_frequency/src/main.rs
    title: verify/library_checker/data_structure/point_set_range_frequency/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// TODO: \u4E8C\u5206\u63A2\u7D22\n\nuse std::ops::RangeBounds;\n\nuse algebraic_structure::magma::AddOperator;\n\
    use as_half_open_range::AsHalfOpenRange;\nuse fenwick_tree::FenwickTree;\nuse\
    \ numeric_traits::Integer;\n\n#[derive(Clone)]\npub struct FenwickTree01 {\n \
    \   n: usize,\n    ft: FenwickTree<AddOperator<i32>>,\n    v: Vec<u64>,\n}\n\n\
    impl FromIterator<usize> for FenwickTree01 {\n    fn from_iter<I: IntoIterator<Item\
    \ = usize>>(iter: I) -> Self {\n        let a = iter.into_iter().collect::<Vec<_>>();\n\
    \        let n = a.len();\n        let m = n.ceil_div(64);\n        let mut v\
    \ = vec![0; m];\n        let mut s = vec![0; m];\n        for (i, &x) in a.iter().enumerate()\
    \ {\n            assert!(x == 0 || x == 1);\n            let y = x as i32;\n \
    \           v[i / 64] |= (y as u64) << (i % 64);\n            s[i / 64] += y;\n\
    \        }\n        Self {\n            n,\n            ft: FenwickTree::<AddOperator<i32>>::from_iter(s),\n\
    \            v,\n        }\n    }\n}\n\nimpl FenwickTree01 {\n    pub fn from_fn(n:\
    \ usize, f: impl FnMut(usize) -> usize) -> Self {\n        Self::from_iter((0..n).map(f))\n\
    \    }\n\n    pub fn new(n: usize) -> Self {\n        Self {\n            n,\n\
    \            ft: FenwickTree::<AddOperator<i32>>::new(n.ceil_div(64)),\n     \
    \       v: vec![0; n.ceil_div(64)],\n        }\n    }\n\n    pub fn len(&self)\
    \ -> usize {\n        self.n\n    }\n\n    pub fn is_empty(&self) -> bool {\n\
    \        self.n == 0\n    }\n\n    pub fn get(&self, i: usize) -> usize {\n  \
    \      assert!(i < self.n);\n        (self.v[i / 64] >> (i % 64) & 1) as usize\n\
    \    }\n\n    pub fn set(&mut self, i: usize, x: usize) {\n        assert!(i <\
    \ self.n);\n        assert!(x == 0 || x == 1);\n        if self.get(i) == x {\n\
    \            return;\n        }\n        if x == 1 {\n            self.v[i / 64]\
    \ |= 1 << (i % 64);\n            self.ft.add(i / 64, 1);\n        } else {\n \
    \           self.v[i / 64] &= !(1 << (i % 64));\n            self.ft.add(i / 64,\
    \ -1);\n        }\n    }\n\n    pub fn fold_prefix(&self, i: usize) -> usize {\n\
    \        assert!(i <= self.n);\n        let mut res = self.ft.fold_prefix(i /\
    \ 64) as usize;\n        if i % 64 > 0 {\n            res += (self.v[i / 64] &\
    \ ((1 << (i % 64)) - 1)).count_ones() as usize;\n        }\n        res\n    }\n\
    \n    pub fn fold(&self, range: impl RangeBounds<usize>) -> usize {\n        let\
    \ (l, r) = range.as_half_open_range(0, self.n);\n        self.fold_prefix(r) -\
    \ self.fold_prefix(l)\n    }\n}\n"
  dependsOn:
  - crates/algebra/algebraic_structure/src/add.rs
  - crates/algebra/algebraic_structure/src/affine.rs
  - crates/algebra/algebraic_structure/src/count_sum.rs
  - crates/algebra/algebraic_structure/src/countsum_affine.rs
  - crates/algebra/algebraic_structure/src/lib.rs
  - crates/algebra/algebraic_structure/src/max.rs
  - crates/algebra/algebraic_structure/src/min.rs
  - crates/algebra/algebraic_structure/src/mul.rs
  - crates/algebra/algebraic_structure/src/semiring.rs
  - crates/algebra/algebraic_structure/src/trivial_group.rs
  - crates/algebra/algebraic_structure/src/xor.rs
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
  - crates/data_structure/fenwick_tree/fenwick_tree/src/lib.rs
  - crates/misc/as_half_open_range/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/fenwick_tree/fenwick_tree_01/src/lib.rs
  requiredBy: []
  timestamp: '2025-05-23 03:46:52+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/point_set_range_frequency/src/main.rs
  - verify/library_checker/data_structure/majority_voting/src/main.rs
documentation_of: crates/data_structure/fenwick_tree/fenwick_tree_01/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/fenwick_tree/fenwick_tree_01/src/lib.rs
- /library/crates/data_structure/fenwick_tree/fenwick_tree_01/src/lib.rs.html
title: crates/data_structure/fenwick_tree/fenwick_tree_01/src/lib.rs
---
