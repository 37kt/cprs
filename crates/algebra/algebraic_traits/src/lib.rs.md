---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_traits/src/macros.rs
    title: crates/algebra/algebraic_traits/src/macros.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_traits/src/traits.rs
    title: crates/algebra/algebraic_traits/src/traits.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/lib.rs
    title: crates/algebra/algebraic_structure/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_traits/src/macros.rs
    title: crates/algebra/algebraic_traits/src/macros.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_traits/src/traits.rs
    title: crates/algebra/algebraic_traits/src/traits.rs
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
    path: crates/data_structure/segment_tree/segment_tree/src/lib.rs
    title: crates/data_structure/segment_tree/segment_tree/src/lib.rs
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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: 'pub mod traits;

    pub use traits::*;


    pub mod macros;

    #[allow(unused_imports)]

    pub use macros::*;

    '
  dependsOn:
  - crates/algebra/algebraic_traits/src/macros.rs
  - crates/algebra/algebraic_traits/src/traits.rs
  isVerificationFile: false
  path: crates/algebra/algebraic_traits/src/lib.rs
  requiredBy:
  - crates/convolution/convolution/src/lib.rs
  - crates/convolution/bitwise_and_convolution/src/lib.rs
  - crates/convolution/bitwise_or_convolution/src/lib.rs
  - crates/convolution/lcm_convolution/src/lib.rs
  - crates/convolution/gcd_convolution/src/lib.rs
  - crates/convolution/bitwise_xor_convolution/src/lib.rs
  - crates/string/rolling_hash/src/lib.rs
  - crates/data_structure/union_find/union_find/src/lib.rs
  - crates/data_structure/wavelet_matrix/src/lib.rs
  - crates/data_structure/fenwick_tree/fenwick_tree/src/lib.rs
  - crates/data_structure/foldable_deque/src/lib.rs
  - crates/data_structure/disjoint_sparse_table/src/lib.rs
  - crates/data_structure/segment_tree/dual_segment_tree/src/lib.rs
  - crates/data_structure/segment_tree/segment_tree/src/lib.rs
  - crates/data_structure/segment_tree/lazy_segment_tree/src/lib.rs
  - crates/linear_algebra/matrix/src/lib.rs
  - crates/algebra/algebraic_traits/src/traits.rs
  - crates/algebra/algebraic_traits/src/macros.rs
  - crates/algebra/algebraic_structure/src/lib.rs
  timestamp: '2025-03-07 00:14:11+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/tree/vertex_set_path_composite/src/main.rs
  - verify/library_checker/data_structure/unionfind_with_potential_non_commutative_group/src/main.rs
documentation_of: crates/algebra/algebraic_traits/src/lib.rs
layout: document
redirect_from:
- /library/crates/algebra/algebraic_traits/src/lib.rs
- /library/crates/algebra/algebraic_traits/src/lib.rs.html
title: crates/algebra/algebraic_traits/src/lib.rs
---
