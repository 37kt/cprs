---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_traits/src/lib.rs
    title: crates/algebra/algebraic_traits/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_traits/src/macros.rs
    title: crates/algebra/algebraic_traits/src/macros.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/lib.rs
    title: crates/algebra/algebraic_structure/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_traits/src/lib.rs
    title: crates/algebra/algebraic_traits/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_traits/src/macros.rs
    title: crates/algebra/algebraic_traits/src/macros.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/bitwise_and_convolution/src/lib.rs
    title: crates/convolution/bitwise_and_convolution/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/bitwise_or_convolution/src/lib.rs
    title: crates/convolution/bitwise_or_convolution/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/bitwise_xor_convolution/src/lib.rs
    title: crates/convolution/bitwise_xor_convolution/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/lib.rs
    title: crates/convolution/convolution/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/gcd_convolution/src/lib.rs
    title: crates/convolution/gcd_convolution/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/lcm_convolution/src/lib.rs
    title: crates/convolution/lcm_convolution/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/disjoint_sparse_table/src/lib.rs
    title: crates/data_structure/disjoint_sparse_table/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/fenwick_tree/fenwick_tree/src/lib.rs
    title: crates/data_structure/fenwick_tree/fenwick_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/foldable_deque/src/lib.rs
    title: crates/data_structure/foldable_deque/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segment_tree/dual_segment_tree/src/lib.rs
    title: crates/data_structure/segment_tree/dual_segment_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segment_tree/lazy_segment_tree/src/lib.rs
    title: crates/data_structure/segment_tree/lazy_segment_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segment_tree/persistent_segment_tree/src/lib.rs
    title: crates/data_structure/segment_tree/persistent_segment_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segment_tree/segment_tree/src/lib.rs
    title: crates/data_structure/segment_tree/segment_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segment_tree/sparse_segment_tree/src/lib.rs
    title: crates/data_structure/segment_tree/sparse_segment_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/lib.rs
    title: crates/data_structure/splay_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/union_find/src/lib.rs
    title: crates/data_structure/union_find/union_find/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/wavelet_matrix/src/lib.rs
    title: crates/data_structure/wavelet_matrix/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/linear_algebra/matrix/src/lib.rs
    title: crates/linear_algebra/matrix/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/string/rolling_hash/src/lib.rs
    title: crates/string/rolling_hash/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/unionfind_with_potential_non_commutative_group/src/main.rs
    title: verify/library_checker/data_structure/unionfind_with_potential_non_commutative_group/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/vertex_set_path_composite/src/main.rs
    title: verify/library_checker/tree/vertex_set_path_composite/src/main.rs
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
  code: "pub trait Algebraic {\n    type Value: Clone;\n}\n\npub trait Magma: Algebraic\
    \ {\n    fn op(x: &Self::Value, y: &Self::Value) -> Self::Value;\n}\n\npub trait\
    \ Unital: Magma {\n    fn unit() -> Self::Value;\n\n    fn is_unit(x: &Self::Value)\
    \ -> bool\n    where\n        Self::Value: PartialEq,\n    {\n        &Self::unit()\
    \ == x\n    }\n}\n\npub trait Invertive: Magma {\n    fn inv(x: &Self::Value)\
    \ -> Self::Value;\n}\n\npub trait Associative: Magma {}\n\npub trait Commutative:\
    \ Magma {}\n\npub trait Idempotent: Magma {}\n\npub trait Semigroup: Associative\
    \ {}\nimpl<T: Associative> Semigroup for T {}\n\npub trait Monoid: Associative\
    \ + Unital {}\nimpl<T: Associative + Unital> Monoid for T {}\n\npub trait CommutativeMonoid:\
    \ Associative + Unital + Commutative {}\nimpl<T: Associative + Unital + Commutative>\
    \ CommutativeMonoid for T {}\n\npub trait Group: Associative + Unital + Invertive\
    \ {}\nimpl<T: Associative + Unital + Invertive> Group for T {}\n\npub trait AbelianGroup:\
    \ Associative + Unital + Commutative + Invertive {}\nimpl<T: Associative + Unital\
    \ + Commutative + Invertive> AbelianGroup for T {}\n\npub trait Band: Associative\
    \ + Idempotent {}\nimpl<T: Associative + Idempotent> Band for T {}\n\npub trait\
    \ Pow: Monoid {\n    fn pow(x: &Self::Value, mut exp: usize) -> Self::Value {\n\
    \        let mut res = Self::unit();\n        let mut x = Self::op(&res, x);\n\
    \        while exp > 0 {\n            if exp & 1 == 1 {\n                res =\
    \ Self::op(&res, &x);\n            }\n            x = Self::op(&x, &x);\n    \
    \        exp >>= 1;\n        }\n        res\n    }\n}\n\npub trait Act {\n   \
    \ type Operand: Monoid;\n    type Operator: Monoid;\n\n    fn act(\n        x:\
    \ &<Self::Operand as Algebraic>::Value,\n        f: &<Self::Operator as Algebraic>::Value,\n\
    \    ) -> <Self::Operand as Algebraic>::Value;\n}\n\npub trait Semiring: Algebraic\
    \ {\n    type Additive: CommutativeMonoid<Value = Self::Value>;\n    type Multiplicative:\
    \ Monoid<Value = Self::Value>;\n\n    fn zero() -> Self::Value {\n        Self::Additive::unit()\n\
    \    }\n\n    fn one() -> Self::Value {\n        Self::Multiplicative::unit()\n\
    \    }\n\n    fn add(x: &Self::Value, y: &Self::Value) -> Self::Value {\n    \
    \    Self::Additive::op(x, y)\n    }\n\n    fn mul(x: &Self::Value, y: &Self::Value)\
    \ -> Self::Value {\n        Self::Multiplicative::op(x, y)\n    }\n}\n\npub trait\
    \ Ring: Semiring\nwhere\n    Self::Additive: Invertive,\n{\n    fn neg(x: &Self::Value)\
    \ -> Self::Value {\n        Self::Additive::inv(x)\n    }\n\n    fn sub(x: &Self::Value,\
    \ y: &Self::Value) -> Self::Value {\n        Self::Additive::op(x, &Self::neg(y))\n\
    \    }\n}\nimpl<T> Ring for T\nwhere\n    T: Semiring,\n    T::Additive: Invertive,\n\
    {\n}\n\npub trait Field: Ring\nwhere\n    Self::Additive: Invertive,\n    Self::Multiplicative:\
    \ Invertive,\n{\n    fn recip(x: &Self::Value) -> Self::Value {\n        Self::Multiplicative::inv(x)\n\
    \    }\n\n    fn div(x: &Self::Value, y: &Self::Value) -> Self::Value {\n    \
    \    Self::Multiplicative::op(x, &Self::recip(y))\n    }\n}\nimpl<T> Field for\
    \ T\nwhere\n    T: Ring,\n    T::Additive: Invertive,\n    T::Multiplicative:\
    \ Invertive,\n{\n}\n"
  dependsOn:
  - crates/algebra/algebraic_traits/src/lib.rs
  - crates/algebra/algebraic_traits/src/macros.rs
  isVerificationFile: false
  path: crates/algebra/algebraic_traits/src/traits.rs
  requiredBy:
  - crates/string/rolling_hash/src/lib.rs
  - crates/convolution/bitwise_or_convolution/src/lib.rs
  - crates/convolution/convolution/src/lib.rs
  - crates/convolution/bitwise_and_convolution/src/lib.rs
  - crates/convolution/gcd_convolution/src/lib.rs
  - crates/convolution/lcm_convolution/src/lib.rs
  - crates/convolution/bitwise_xor_convolution/src/lib.rs
  - crates/data_structure/disjoint_sparse_table/src/lib.rs
  - crates/data_structure/wavelet_matrix/src/lib.rs
  - crates/data_structure/splay_tree/src/lib.rs
  - crates/data_structure/fenwick_tree/fenwick_tree/src/lib.rs
  - crates/data_structure/segment_tree/lazy_segment_tree/src/lib.rs
  - crates/data_structure/segment_tree/dual_segment_tree/src/lib.rs
  - crates/data_structure/segment_tree/sparse_segment_tree/src/lib.rs
  - crates/data_structure/segment_tree/segment_tree/src/lib.rs
  - crates/data_structure/segment_tree/persistent_segment_tree/src/lib.rs
  - crates/data_structure/foldable_deque/src/lib.rs
  - crates/data_structure/union_find/union_find/src/lib.rs
  - crates/linear_algebra/matrix/src/lib.rs
  - crates/algebra/algebraic_structure/src/lib.rs
  - crates/algebra/algebraic_traits/src/lib.rs
  - crates/algebra/algebraic_traits/src/macros.rs
  timestamp: '2025-04-22 06:09:15+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/unionfind_with_potential_non_commutative_group/src/main.rs
  - verify/library_checker/tree/vertex_set_path_composite/src/main.rs
documentation_of: crates/algebra/algebraic_traits/src/traits.rs
layout: document
redirect_from:
- /library/crates/algebra/algebraic_traits/src/traits.rs
- /library/crates/algebra/algebraic_traits/src/traits.rs.html
title: crates/algebra/algebraic_traits/src/traits.rs
---
