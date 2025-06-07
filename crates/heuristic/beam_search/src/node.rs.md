---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/beam_search.rs
    title: crates/heuristic/beam_search/src/beam_search.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/candidate.rs
    title: crates/heuristic/beam_search/src/candidate.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/config.rs
    title: crates/heuristic/beam_search/src/config.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/evaluation.rs
    title: crates/heuristic/beam_search/src/evaluation.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/heap.rs
    title: crates/heuristic/beam_search/src/heap.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/index.rs
    title: crates/heuristic/beam_search/src/index.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/lib.rs
    title: crates/heuristic/beam_search/src/lib.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/nop_hash.rs
    title: crates/heuristic/beam_search/src/nop_hash.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/pool.rs
    title: crates/heuristic/beam_search/src/pool.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/state.rs
    title: crates/heuristic/beam_search/src/state.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/beam_search.rs
    title: crates/heuristic/beam_search/src/beam_search.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/candidate.rs
    title: crates/heuristic/beam_search/src/candidate.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/config.rs
    title: crates/heuristic/beam_search/src/config.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/evaluation.rs
    title: crates/heuristic/beam_search/src/evaluation.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/heap.rs
    title: crates/heuristic/beam_search/src/heap.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/index.rs
    title: crates/heuristic/beam_search/src/index.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/lib.rs
    title: crates/heuristic/beam_search/src/lib.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/nop_hash.rs
    title: crates/heuristic/beam_search/src/nop_hash.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/pool.rs
    title: crates/heuristic/beam_search/src/pool.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/state.rs
    title: crates/heuristic/beam_search/src/state.rs
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
  code: "use crate::index::Index;\n\npub(crate) struct Node<Action> {\n    pub action:\
    \ Action,\n    pub parent: Index,\n    pub child: Index,     // \u9577\u7537\n\
    \    pub left: Index,      // \u5144\n    pub right: Index,     // \u5F1F\n  \
    \  pub count_cands: u32, // \u5B50\u5019\u88DC\u306E Candidate \u306E\u6570\n\
    }\n"
  dependsOn:
  - crates/heuristic/beam_search/src/beam_search.rs
  - crates/heuristic/beam_search/src/candidate.rs
  - crates/heuristic/beam_search/src/config.rs
  - crates/heuristic/beam_search/src/evaluation.rs
  - crates/heuristic/beam_search/src/heap.rs
  - crates/heuristic/beam_search/src/index.rs
  - crates/heuristic/beam_search/src/lib.rs
  - crates/heuristic/beam_search/src/nop_hash.rs
  - crates/heuristic/beam_search/src/pool.rs
  - crates/heuristic/beam_search/src/state.rs
  isVerificationFile: false
  path: crates/heuristic/beam_search/src/node.rs
  requiredBy:
  - crates/heuristic/beam_search/src/nop_hash.rs
  - crates/heuristic/beam_search/src/index.rs
  - crates/heuristic/beam_search/src/candidate.rs
  - crates/heuristic/beam_search/src/lib.rs
  - crates/heuristic/beam_search/src/config.rs
  - crates/heuristic/beam_search/src/evaluation.rs
  - crates/heuristic/beam_search/src/heap.rs
  - crates/heuristic/beam_search/src/beam_search.rs
  - crates/heuristic/beam_search/src/state.rs
  - crates/heuristic/beam_search/src/pool.rs
  timestamp: '2025-04-26 05:33:09+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/heuristic/beam_search/src/node.rs
layout: document
redirect_from:
- /library/crates/heuristic/beam_search/src/node.rs
- /library/crates/heuristic/beam_search/src/node.rs.html
title: crates/heuristic/beam_search/src/node.rs
---
