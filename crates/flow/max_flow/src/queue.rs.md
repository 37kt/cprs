---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/flow/max_flow/src/graph.rs
    title: crates/flow/max_flow/src/graph.rs
  - icon: ':heavy_check_mark:'
    path: crates/flow/max_flow/src/lib.rs
    title: crates/flow/max_flow/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/flow/binary_optimization/src/lib.rs
    title: crates/flow/binary_optimization/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/flow/max_flow/src/graph.rs
    title: crates/flow/max_flow/src/graph.rs
  - icon: ':heavy_check_mark:'
    path: crates/flow/max_flow/src/lib.rs
    title: crates/flow/max_flow/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/flow/multivalued_optimization/src/lib.rs
    title: crates/flow/multivalued_optimization/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yukicoder/flow/yuki1479_maxflow/src/main.rs
    title: verify/yukicoder/flow/yuki1479_maxflow/src/main.rs
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
  code: "#[derive(Clone)]\npub(crate) struct Queue {\n    data: Vec<usize>,\n    head:\
    \ usize,\n    tail: usize,\n}\n\nimpl Queue {\n    pub(crate) fn new() -> Self\
    \ {\n        Self {\n            data: vec![],\n            head: 0,\n       \
    \     tail: 0,\n        }\n    }\n\n    pub(crate) fn set_capacity(&mut self,\
    \ cap: usize) {\n        self.data.resize(cap, 0);\n    }\n\n    pub(crate) fn\
    \ clear(&mut self) {\n        self.head = 0;\n        self.tail = 0;\n    }\n\n\
    \    pub(crate) fn push(&mut self, v: usize) {\n        self.data[self.tail] =\
    \ v;\n        self.tail += 1;\n    }\n\n    pub(crate) fn pop(&mut self) -> Option<usize>\
    \ {\n        (self.head != self.tail).then(|| {\n            self.head += 1;\n\
    \            self.data[self.head - 1]\n        })\n    }\n}\n"
  dependsOn:
  - crates/flow/max_flow/src/graph.rs
  - crates/flow/max_flow/src/lib.rs
  isVerificationFile: false
  path: crates/flow/max_flow/src/queue.rs
  requiredBy:
  - crates/flow/multivalued_optimization/src/lib.rs
  - crates/flow/binary_optimization/src/lib.rs
  - crates/flow/max_flow/src/graph.rs
  - crates/flow/max_flow/src/lib.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yukicoder/flow/yuki1479_maxflow/src/main.rs
documentation_of: crates/flow/max_flow/src/queue.rs
layout: document
redirect_from:
- /library/crates/flow/max_flow/src/queue.rs
- /library/crates/flow/max_flow/src/queue.rs.html
title: crates/flow/max_flow/src/queue.rs
---
