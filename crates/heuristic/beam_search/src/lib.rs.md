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
    path: crates/heuristic/beam_search/src/node.rs
    title: crates/heuristic/beam_search/src/node.rs
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
    path: crates/heuristic/beam_search/src/node.rs
    title: crates/heuristic/beam_search/src/node.rs
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
  code: 'pub mod beam_search;

    mod candidate;

    pub mod config;

    pub mod evaluation;

    mod heap;

    mod index;

    mod node;

    pub mod nop_hash;

    mod pool;

    pub mod state;


    pub use beam_search::BeamSearch;

    pub use config::{Config, HashDeduplicationScope};

    pub use evaluation::Evaluation;

    pub use state::BeamState;

    '
  dependsOn:
  - crates/heuristic/beam_search/src/beam_search.rs
  - crates/heuristic/beam_search/src/candidate.rs
  - crates/heuristic/beam_search/src/config.rs
  - crates/heuristic/beam_search/src/evaluation.rs
  - crates/heuristic/beam_search/src/heap.rs
  - crates/heuristic/beam_search/src/index.rs
  - crates/heuristic/beam_search/src/node.rs
  - crates/heuristic/beam_search/src/nop_hash.rs
  - crates/heuristic/beam_search/src/pool.rs
  - crates/heuristic/beam_search/src/state.rs
  isVerificationFile: false
  path: crates/heuristic/beam_search/src/lib.rs
  requiredBy:
  - crates/heuristic/beam_search/src/evaluation.rs
  - crates/heuristic/beam_search/src/config.rs
  - crates/heuristic/beam_search/src/node.rs
  - crates/heuristic/beam_search/src/nop_hash.rs
  - crates/heuristic/beam_search/src/beam_search.rs
  - crates/heuristic/beam_search/src/candidate.rs
  - crates/heuristic/beam_search/src/heap.rs
  - crates/heuristic/beam_search/src/state.rs
  - crates/heuristic/beam_search/src/index.rs
  - crates/heuristic/beam_search/src/pool.rs
  timestamp: '2025-04-26 05:33:09+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/heuristic/beam_search/src/lib.rs
layout: document
redirect_from:
- /library/crates/heuristic/beam_search/src/lib.rs
- /library/crates/heuristic/beam_search/src/lib.rs.html
title: crates/heuristic/beam_search/src/lib.rs
---
