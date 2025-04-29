---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/data_structure/segment_tree/sparse_segment_tree/src/allocator.rs
    title: crates/data_structure/segment_tree/sparse_segment_tree/src/allocator.rs
  - icon: ':warning:'
    path: crates/data_structure/segment_tree/sparse_segment_tree/src/lib.rs
    title: crates/data_structure/segment_tree/sparse_segment_tree/src/lib.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/data_structure/segment_tree/sparse_segment_tree/src/allocator.rs
    title: crates/data_structure/segment_tree/sparse_segment_tree/src/allocator.rs
  - icon: ':warning:'
    path: crates/data_structure/segment_tree/sparse_segment_tree/src/lib.rs
    title: crates/data_structure/segment_tree/sparse_segment_tree/src/lib.rs
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
  code: "use std::ptr::NonNull;\n\nuse algebraic_traits::Monoid;\n\npub(crate) struct\
    \ Node<M>\nwhere\n    M: Monoid,\n{\n    pub l: Option<NonNull<Node<M>>>,\n  \
    \  pub r: Option<NonNull<Node<M>>>,\n    pub x: M::Value,\n}\n\nimpl<M> Node<M>\n\
    where\n    M: Monoid,\n{\n    pub fn new(x: M::Value) -> Self {\n        Self\
    \ {\n            l: None,\n            r: None,\n            x,\n        }\n \
    \   }\n}\n"
  dependsOn:
  - crates/data_structure/segment_tree/sparse_segment_tree/src/allocator.rs
  - crates/data_structure/segment_tree/sparse_segment_tree/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/segment_tree/sparse_segment_tree/src/node.rs
  requiredBy:
  - crates/data_structure/segment_tree/sparse_segment_tree/src/lib.rs
  - crates/data_structure/segment_tree/sparse_segment_tree/src/allocator.rs
  timestamp: '2025-04-29 08:06:54+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data_structure/segment_tree/sparse_segment_tree/src/node.rs
layout: document
redirect_from:
- /library/crates/data_structure/segment_tree/sparse_segment_tree/src/node.rs
- /library/crates/data_structure/segment_tree/sparse_segment_tree/src/node.rs.html
title: crates/data_structure/segment_tree/sparse_segment_tree/src/node.rs
---
