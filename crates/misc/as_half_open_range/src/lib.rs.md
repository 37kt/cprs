---
data:
  _extendedDependsOn:
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
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/disjoint_sparse_table/src/lib.rs
    title: crates/data_structure/disjoint_sparse_table/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/fenwick_tree/fenwick_tree/src/lib.rs
    title: crates/data_structure/fenwick_tree/fenwick_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/range_minimum_query/src/lib.rs
    title: crates/data_structure/range_minimum_query/src/lib.rs
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
    path: crates/data_structure/wavelet_matrix/src/lib.rs
    title: crates/data_structure/wavelet_matrix/src/lib.rs
  - icon: ':warning:'
    path: crates/dp/aliens_dp/src/lib.rs
    title: crates/dp/aliens_dp/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/string/rolling_hash/src/lib.rs
    title: crates/string/rolling_hash/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/string/suffix_array/src/lib.rs
    title: crates/string/suffix_array/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/tree_contour_range/src/lib.rs
    title: crates/tree/tree_contour_range/src/lib.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ops::{Bound, RangeBounds};\n\nuse numeric_traits::Integer;\n\npub\
    \ trait AsHalfOpenRange<T>: RangeBounds<T>\nwhere\n    T: Integer,\n{\n    fn\
    \ as_half_open_range(&self, l: T, r: T) -> (T, T) {\n        let start = match\
    \ self.start_bound() {\n            Bound::Unbounded => l,\n            Bound::Included(&start)\
    \ => start,\n            Bound::Excluded(&start) => start + T::one(),\n      \
    \  };\n        let end = match self.end_bound() {\n            Bound::Unbounded\
    \ => r,\n            Bound::Included(&end) => end + T::one(),\n            Bound::Excluded(&end)\
    \ => end,\n        };\n        assert!(l <= start && start <= end && end <= r);\n\
    \        (start, end)\n    }\n}\n\nimpl<R, T> AsHalfOpenRange<T> for R\nwhere\n\
    \    R: RangeBounds<T>,\n    T: Integer,\n{\n}\n"
  dependsOn:
  - crates/algebra/numeric_traits/src/cast.rs
  - crates/algebra/numeric_traits/src/inf.rs
  - crates/algebra/numeric_traits/src/integer.rs
  - crates/algebra/numeric_traits/src/lib.rs
  - crates/algebra/numeric_traits/src/numeric.rs
  - crates/algebra/numeric_traits/src/signed.rs
  - crates/algebra/numeric_traits/src/zero_one.rs
  isVerificationFile: false
  path: crates/misc/as_half_open_range/src/lib.rs
  requiredBy:
  - crates/string/rolling_hash/src/lib.rs
  - crates/string/suffix_array/src/lib.rs
  - crates/dp/aliens_dp/src/lib.rs
  - crates/data_structure/disjoint_sparse_table/src/lib.rs
  - crates/data_structure/range_minimum_query/src/lib.rs
  - crates/data_structure/wavelet_matrix/src/lib.rs
  - crates/data_structure/splay_tree/src/lib.rs
  - crates/data_structure/fenwick_tree/fenwick_tree/src/lib.rs
  - crates/data_structure/segment_tree/lazy_segment_tree/src/lib.rs
  - crates/data_structure/segment_tree/dual_segment_tree/src/lib.rs
  - crates/data_structure/segment_tree/sparse_segment_tree/src/lib.rs
  - crates/data_structure/segment_tree/segment_tree/src/lib.rs
  - crates/data_structure/segment_tree/persistent_segment_tree/src/lib.rs
  - crates/tree/tree_contour_range/src/lib.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/misc/as_half_open_range/src/lib.rs
layout: document
redirect_from:
- /library/crates/misc/as_half_open_range/src/lib.rs
- /library/crates/misc/as_half_open_range/src/lib.rs.html
title: crates/misc/as_half_open_range/src/lib.rs
---
