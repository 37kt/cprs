---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/beam_search.rs
    title: crates/heuristic/beam_search/src/beam_search.rs
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
  code: "use std::cmp::Ordering;\n\nuse crate::index::Index;\n\n#[derive(Clone)]\n\
    pub(crate) struct Candidate<Action> {\n    pub action: Action,\n    pub parent:\
    \ Index,\n    pub score: i64,\n    pub hash: u64,\n    pub valid: bool,\n}\n\n\
    impl<Action> PartialEq for Candidate<Action> {\n    fn eq(&self, other: &Self)\
    \ -> bool {\n        self.score == other.score\n    }\n}\n\nimpl<Action> Eq for\
    \ Candidate<Action> {}\n\nimpl<Action> PartialOrd for Candidate<Action> {\n  \
    \  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {\n        Some(self.cmp(other))\n\
    \    }\n}\n\n// BinaryHeap \u306B\u5165\u308C\u305F\u3068\u304D\u306B\u3001\u4F4E\
    \u3044(\u60AA\u3044)\u30B9\u30B3\u30A2\u304C top \u306B\u6765\u308B\u3088\u3046\
    \u306B\u3059\u308B\nimpl<Action> Ord for Candidate<Action> {\n    fn cmp(&self,\
    \ other: &Self) -> Ordering {\n        self.score.cmp(&other.score).reverse()\n\
    \    }\n}\n"
  dependsOn:
  - crates/heuristic/beam_search/src/beam_search.rs
  - crates/heuristic/beam_search/src/config.rs
  - crates/heuristic/beam_search/src/evaluation.rs
  - crates/heuristic/beam_search/src/heap.rs
  - crates/heuristic/beam_search/src/index.rs
  - crates/heuristic/beam_search/src/lib.rs
  - crates/heuristic/beam_search/src/node.rs
  - crates/heuristic/beam_search/src/nop_hash.rs
  - crates/heuristic/beam_search/src/pool.rs
  - crates/heuristic/beam_search/src/state.rs
  isVerificationFile: false
  path: crates/heuristic/beam_search/src/candidate.rs
  requiredBy:
  - crates/heuristic/beam_search/src/lib.rs
  - crates/heuristic/beam_search/src/evaluation.rs
  - crates/heuristic/beam_search/src/config.rs
  - crates/heuristic/beam_search/src/node.rs
  - crates/heuristic/beam_search/src/nop_hash.rs
  - crates/heuristic/beam_search/src/beam_search.rs
  - crates/heuristic/beam_search/src/heap.rs
  - crates/heuristic/beam_search/src/state.rs
  - crates/heuristic/beam_search/src/index.rs
  - crates/heuristic/beam_search/src/pool.rs
  timestamp: '2025-04-26 05:33:09+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/heuristic/beam_search/src/candidate.rs
layout: document
redirect_from:
- /library/crates/heuristic/beam_search/src/candidate.rs
- /library/crates/heuristic/beam_search/src/candidate.rs.html
title: crates/heuristic/beam_search/src/candidate.rs
---
