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
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/allocator.rs
    title: crates/data_structure/splay_tree/src/allocator.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/node.rs
    title: crates/data_structure/splay_tree/src/node.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/operator.rs
    title: crates/data_structure/splay_tree/src/operator.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/sequence.rs
    title: crates/data_structure/splay_tree/src/sequence.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/tree.rs
    title: crates/data_structure/splay_tree/src/tree.rs
  - icon: ':warning:'
    path: crates/misc/as_half_open_range/src/lib.rs
    title: crates/misc/as_half_open_range/src/lib.rs
  - icon: ':warning:'
    path: crates/misc/simple_arena/src/lib.rs
    title: crates/misc/simple_arena/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/allocator.rs
    title: crates/data_structure/splay_tree/src/allocator.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/node.rs
    title: crates/data_structure/splay_tree/src/node.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/operator.rs
    title: crates/data_structure/splay_tree/src/operator.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/sequence.rs
    title: crates/data_structure/splay_tree/src/sequence.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/tree.rs
    title: crates/data_structure/splay_tree/src/tree.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/dynamic_sequence_range_affine_range_sum/src/main.rs
    title: verify/library_checker/data_structure/dynamic_sequence_range_affine_range_sum/src/main.rs
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
  code: 'mod allocator;

    mod node;

    mod operator;

    mod sequence;

    mod tree;


    pub use operator::*;

    pub use sequence::*;

    '
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
  - crates/data_structure/splay_tree/src/allocator.rs
  - crates/data_structure/splay_tree/src/node.rs
  - crates/data_structure/splay_tree/src/operator.rs
  - crates/data_structure/splay_tree/src/sequence.rs
  - crates/data_structure/splay_tree/src/tree.rs
  - crates/misc/as_half_open_range/src/lib.rs
  - crates/misc/simple_arena/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/splay_tree/src/lib.rs
  requiredBy:
  - crates/data_structure/splay_tree/src/operator.rs
  - crates/data_structure/splay_tree/src/allocator.rs
  - crates/data_structure/splay_tree/src/tree.rs
  - crates/data_structure/splay_tree/src/sequence.rs
  - crates/data_structure/splay_tree/src/node.rs
  timestamp: '2025-04-22 06:09:15+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/dynamic_sequence_range_affine_range_sum/src/main.rs
documentation_of: crates/data_structure/splay_tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/splay_tree/src/lib.rs
- /library/crates/data_structure/splay_tree/src/lib.rs.html
title: crates/data_structure/splay_tree/src/lib.rs
---
