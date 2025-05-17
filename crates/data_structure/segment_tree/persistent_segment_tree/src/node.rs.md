---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segment_tree/persistent_segment_tree/src/lib.rs
    title: crates/data_structure/segment_tree/persistent_segment_tree/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segment_tree/persistent_segment_tree/src/lib.rs
    title: crates/data_structure/segment_tree/persistent_segment_tree/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/rectangle_sum_pseg/src/main.rs
    title: verify/library_checker/data_structure/rectangle_sum_pseg/src/main.rs
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
  code: "use algebraic_traits::Monoid;\nuse simple_arena::Arena;\n\nuse std::{cell::RefCell,\
    \ ptr::NonNull};\n\nthread_local! {\n    static ARENA: RefCell<Arena> = RefCell::new(Arena::new(1024\
    \ * 1024 * 1024));\n}\n\npub(crate) struct Node<M>\nwhere\n    M: Monoid,\n{\n\
    \    pub l: Option<NonNull<Node<M>>>,\n    pub r: Option<NonNull<Node<M>>>,\n\
    \    pub x: M::Value,\n}\n\nimpl<M> Clone for Node<M>\nwhere\n    M: Monoid,\n\
    {\n    fn clone(&self) -> Self {\n        Self {\n            l: self.l,\n   \
    \         r: self.r,\n            x: self.x.clone(),\n        }\n    }\n}\n\n\
    impl<M> Node<M>\nwhere\n    M: Monoid,\n{\n    pub fn new(l: Option<NonNull<Node<M>>>,\
    \ r: Option<NonNull<Node<M>>>, x: M::Value) -> Self {\n        Self { l, r, x\
    \ }\n    }\n\n    pub fn new_ptr(\n        l: Option<NonNull<Node<M>>>,\n    \
    \    r: Option<NonNull<Node<M>>>,\n        x: M::Value,\n    ) -> NonNull<Self>\
    \ {\n        ARENA.with(|arena| NonNull::new(arena.borrow_mut().alloc(Self::new(l,\
    \ r, x))).unwrap())\n    }\n\n    pub fn copy(v: Option<NonNull<Self>>) -> Option<NonNull<Self>>\
    \ {\n        v.map(|v| {\n            ARENA.with(|arena| {\n                NonNull::new(arena.borrow_mut().alloc(unsafe\
    \ { v.as_ref() }.clone())).unwrap()\n            })\n        })\n    }\n\n   \
    \ pub fn fold(v: Option<NonNull<Self>>) -> M::Value {\n        v.map_or(M::unit(),\
    \ |v| unsafe { v.as_ref() }.x.clone())\n    }\n\n    pub fn merge(l: Option<NonNull<Self>>,\
    \ r: Option<NonNull<Self>>) -> Option<NonNull<Self>> {\n        if l.is_none()\
    \ && r.is_none() {\n            None\n        } else {\n            Some(Node::new_ptr(l,\
    \ r, M::op(&Node::fold(l), &Node::fold(r))))\n        }\n    }\n\n    pub fn update(mut\
    \ v: NonNull<Self>) {\n        unsafe { v.as_mut() }.x = M::op(\n            &Node::fold(unsafe\
    \ { v.as_ref().l }),\n            &Node::fold(unsafe { v.as_ref().r }),\n    \
    \    );\n    }\n}\n"
  dependsOn:
  - crates/data_structure/segment_tree/persistent_segment_tree/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/segment_tree/persistent_segment_tree/src/node.rs
  requiredBy:
  - crates/data_structure/segment_tree/persistent_segment_tree/src/lib.rs
  timestamp: '2025-04-30 05:56:56+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/rectangle_sum_pseg/src/main.rs
documentation_of: crates/data_structure/segment_tree/persistent_segment_tree/src/node.rs
layout: document
redirect_from:
- /library/crates/data_structure/segment_tree/persistent_segment_tree/src/node.rs
- /library/crates/data_structure/segment_tree/persistent_segment_tree/src/node.rs.html
title: crates/data_structure/segment_tree/persistent_segment_tree/src/node.rs
---
