---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/data_structure/segment_tree/sparse_segment_tree/src/lib.rs
    title: crates/data_structure/segment_tree/sparse_segment_tree/src/lib.rs
  - icon: ':warning:'
    path: crates/data_structure/segment_tree/sparse_segment_tree/src/node.rs
    title: crates/data_structure/segment_tree/sparse_segment_tree/src/node.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/data_structure/segment_tree/sparse_segment_tree/src/lib.rs
    title: crates/data_structure/segment_tree/sparse_segment_tree/src/lib.rs
  - icon: ':warning:'
    path: crates/data_structure/segment_tree/sparse_segment_tree/src/node.rs
    title: crates/data_structure/segment_tree/sparse_segment_tree/src/node.rs
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
  code: "use std::{cell::RefCell, ptr::NonNull};\n\nuse simple_arena::Arena;\n\nthread_local!\
    \ {\n    static ARENA: RefCell<Arena> = RefCell::new(Arena::new(1024 * 1024 *\
    \ 1024));\n}\n\npub(crate) fn new_ptr<T>(val: T) -> NonNull<T> {\n    ARENA.with(|arena|\
    \ NonNull::new(arena.borrow_mut().alloc(val)).unwrap())\n}\n"
  dependsOn:
  - crates/data_structure/segment_tree/sparse_segment_tree/src/lib.rs
  - crates/data_structure/segment_tree/sparse_segment_tree/src/node.rs
  isVerificationFile: false
  path: crates/data_structure/segment_tree/sparse_segment_tree/src/allocator.rs
  requiredBy:
  - crates/data_structure/segment_tree/sparse_segment_tree/src/lib.rs
  - crates/data_structure/segment_tree/sparse_segment_tree/src/node.rs
  timestamp: '2025-04-29 08:06:54+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data_structure/segment_tree/sparse_segment_tree/src/allocator.rs
layout: document
redirect_from:
- /library/crates/data_structure/segment_tree/sparse_segment_tree/src/allocator.rs
- /library/crates/data_structure/segment_tree/sparse_segment_tree/src/allocator.rs.html
title: crates/data_structure/segment_tree/sparse_segment_tree/src/allocator.rs
---
