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
    path: crates/data_structure/union_find/union_find/src/potentialized_union_find.rs
    title: crates/data_structure/union_find/union_find/src/potentialized_union_find.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/union_find/src/union_find.rs
    title: crates/data_structure/union_find/union_find/src/union_find.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/union_find/src/union_find_component_sum.rs
    title: crates/data_structure/union_find/union_find/src/union_find_component_sum.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/union_find/src/union_find_impl.rs
    title: crates/data_structure/union_find/union_find/src/union_find_impl.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/union_find/src/potentialized_union_find.rs
    title: crates/data_structure/union_find/union_find/src/potentialized_union_find.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/union_find/src/union_find.rs
    title: crates/data_structure/union_find/union_find/src/union_find.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/union_find/src/union_find_component_sum.rs
    title: crates/data_structure/union_find/union_find/src/union_find_component_sum.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/union_find/src/union_find_impl.rs
    title: crates/data_structure/union_find/union_find/src/union_find_impl.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/unionfind/src/main.rs
    title: verify/library_checker/data_structure/unionfind/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/unionfind_with_potential/src/main.rs
    title: verify/library_checker/data_structure/unionfind_with_potential/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/unionfind_with_potential_non_commutative_group/src/main.rs
    title: verify/library_checker/data_structure/unionfind_with_potential_non_commutative_group/src/main.rs
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
  code: 'mod union_find_impl;


    mod union_find;

    pub use union_find::*;


    mod potentialized_union_find;

    pub use potentialized_union_find::*;


    mod union_find_component_sum;

    pub use union_find_component_sum::*;

    '
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
  - crates/data_structure/union_find/union_find/src/potentialized_union_find.rs
  - crates/data_structure/union_find/union_find/src/union_find.rs
  - crates/data_structure/union_find/union_find/src/union_find_component_sum.rs
  - crates/data_structure/union_find/union_find/src/union_find_impl.rs
  isVerificationFile: false
  path: crates/data_structure/union_find/union_find/src/lib.rs
  requiredBy:
  - crates/data_structure/union_find/union_find/src/potentialized_union_find.rs
  - crates/data_structure/union_find/union_find/src/union_find_impl.rs
  - crates/data_structure/union_find/union_find/src/union_find_component_sum.rs
  - crates/data_structure/union_find/union_find/src/union_find.rs
  timestamp: '2025-04-07 08:03:10+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/unionfind/src/main.rs
  - verify/library_checker/data_structure/unionfind_with_potential_non_commutative_group/src/main.rs
  - verify/library_checker/data_structure/unionfind_with_potential/src/main.rs
documentation_of: crates/data_structure/union_find/union_find/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/union_find/union_find/src/lib.rs
- /library/crates/data_structure/union_find/union_find/src/lib.rs.html
title: crates/data_structure/union_find/union_find/src/lib.rs
---
