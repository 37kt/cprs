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
    path: crates/heuristic/beam_search/src/node.rs
    title: crates/heuristic/beam_search/src/node.rs
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
    path: crates/heuristic/beam_search/src/node.rs
    title: crates/heuristic/beam_search/src/node.rs
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
  code: "use std::{\n    collections::{HashMap, HashSet},\n    hash::{BuildHasherDefault,\
    \ Hasher},\n};\n\npub type NopHashSet<V> = HashSet<V, BuildHasherDefault<NopHasher>>;\n\
    pub type NopHashMap<K, V> = HashMap<K, V, BuildHasherDefault<NopHasher>>;\n\n\
    #[derive(Default)]\npub struct NopHasher {\n    hash: u64,\n}\n\nimpl Hasher for\
    \ NopHasher {\n    fn write(&mut self, _: &[u8]) {\n        panic!()\n    }\n\n\
    \    #[inline]\n    fn write_u64(&mut self, n: u64) {\n        self.hash = n;\n\
    \    }\n\n    #[inline]\n    fn finish(&self) -> u64 {\n        self.hash\n  \
    \  }\n}\n"
  dependsOn:
  - crates/heuristic/beam_search/src/beam_search.rs
  - crates/heuristic/beam_search/src/candidate.rs
  - crates/heuristic/beam_search/src/config.rs
  - crates/heuristic/beam_search/src/evaluation.rs
  - crates/heuristic/beam_search/src/heap.rs
  - crates/heuristic/beam_search/src/index.rs
  - crates/heuristic/beam_search/src/lib.rs
  - crates/heuristic/beam_search/src/node.rs
  - crates/heuristic/beam_search/src/pool.rs
  - crates/heuristic/beam_search/src/state.rs
  isVerificationFile: false
  path: crates/heuristic/beam_search/src/nop_hash.rs
  requiredBy:
  - crates/heuristic/beam_search/src/lib.rs
  - crates/heuristic/beam_search/src/evaluation.rs
  - crates/heuristic/beam_search/src/pool.rs
  - crates/heuristic/beam_search/src/state.rs
  - crates/heuristic/beam_search/src/candidate.rs
  - crates/heuristic/beam_search/src/beam_search.rs
  - crates/heuristic/beam_search/src/config.rs
  - crates/heuristic/beam_search/src/index.rs
  - crates/heuristic/beam_search/src/node.rs
  - crates/heuristic/beam_search/src/heap.rs
  timestamp: '2025-04-26 04:27:33+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/heuristic/beam_search/src/nop_hash.rs
layout: document
redirect_from:
- /library/crates/heuristic/beam_search/src/nop_hash.rs
- /library/crates/heuristic/beam_search/src/nop_hash.rs.html
title: crates/heuristic/beam_search/src/nop_hash.rs
---
