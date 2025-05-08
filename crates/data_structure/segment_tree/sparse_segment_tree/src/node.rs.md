---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/data_structure/segment_tree/sparse_segment_tree/src/lib.rs
    title: crates/data_structure/segment_tree/sparse_segment_tree/src/lib.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/data_structure/segment_tree/sparse_segment_tree/src/lib.rs
    title: crates/data_structure/segment_tree/sparse_segment_tree/src/lib.rs
  - icon: ':warning:'
    path: verify/sandbox/test/src/main.rs
    title: verify/sandbox/test/src/main.rs
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
  code: "use algebraic_traits::Monoid;\nuse simple_arena::Arena;\n\nuse std::{cell::RefCell,\
    \ ptr::NonNull};\n\nthread_local! {\n    static ARENA: RefCell<Arena> = RefCell::new(Arena::new(1024\
    \ * 1024 * 1024));\n}\n\npub(crate) struct Node<M>\nwhere\n    M: Monoid,\n{\n\
    \    pub l: Option<NonNull<Node<M>>>,\n    pub r: Option<NonNull<Node<M>>>,\n\
    \    pub x: M::Value,\n}\n\nimpl<M> Node<M>\nwhere\n    M: Monoid,\n{\n    pub\
    \ fn new(x: M::Value) -> Self {\n        Self {\n            l: None,\n      \
    \      r: None,\n            x,\n        }\n    }\n\n    pub fn new_ptr(x: M::Value)\
    \ -> NonNull<Self> {\n        ARENA.with(|arena| NonNull::new(arena.borrow_mut().alloc(Self::new(x))).unwrap())\n\
    \    }\n}\n"
  dependsOn:
  - crates/data_structure/segment_tree/sparse_segment_tree/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/segment_tree/sparse_segment_tree/src/node.rs
  requiredBy:
  - verify/sandbox/test/src/main.rs
  - crates/data_structure/segment_tree/sparse_segment_tree/src/lib.rs
  timestamp: '2025-04-30 05:56:56+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data_structure/segment_tree/sparse_segment_tree/src/node.rs
layout: document
redirect_from:
- /library/crates/data_structure/segment_tree/sparse_segment_tree/src/node.rs
- /library/crates/data_structure/segment_tree/sparse_segment_tree/src/node.rs.html
title: crates/data_structure/segment_tree/sparse_segment_tree/src/node.rs
---
