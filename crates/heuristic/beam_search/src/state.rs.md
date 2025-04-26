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
    path: crates/heuristic/beam_search/src/nop_hash.rs
    title: crates/heuristic/beam_search/src/nop_hash.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/pool.rs
    title: crates/heuristic/beam_search/src/pool.rs
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
    path: crates/heuristic/beam_search/src/nop_hash.rs
    title: crates/heuristic/beam_search/src/nop_hash.rs
  - icon: ':warning:'
    path: crates/heuristic/beam_search/src/pool.rs
    title: crates/heuristic/beam_search/src/pool.rs
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
  code: "use crate::evaluation::Evaluation;\n\npub trait BeamState {\n    type Action:\
    \ Clone;\n\n    fn enumerate_actions(&mut self) -> Vec<Self::Action>;\n    fn\
    \ apply_action(&mut self, action: &Self::Action);\n    fn revert_action(&mut self,\
    \ action: &Self::Action);\n\n    fn action_turns(&self, action: &Self::Action)\
    \ -> usize;\n\n    fn evaluate_current_state(&mut self) -> Evaluation {\n    \
    \    unimplemented!(\"evaluate_current_state \u304B evaluate_after_action \u3092\
    \u5B9F\u88C5\u3057\u3066\u304F\u3060\u3055\u3044\")\n    }\n\n    fn evaluate_after_action(&mut\
    \ self, action: &Self::Action) -> Evaluation {\n        self.apply_action(action);\n\
    \        let evaluation = self.evaluate_current_state();\n        self.revert_action(action);\n\
    \        evaluation\n    }\n}\n"
  dependsOn:
  - crates/heuristic/beam_search/src/beam_search.rs
  - crates/heuristic/beam_search/src/candidate.rs
  - crates/heuristic/beam_search/src/config.rs
  - crates/heuristic/beam_search/src/evaluation.rs
  - crates/heuristic/beam_search/src/heap.rs
  - crates/heuristic/beam_search/src/index.rs
  - crates/heuristic/beam_search/src/lib.rs
  - crates/heuristic/beam_search/src/node.rs
  - crates/heuristic/beam_search/src/nop_hash.rs
  - crates/heuristic/beam_search/src/pool.rs
  isVerificationFile: false
  path: crates/heuristic/beam_search/src/state.rs
  requiredBy:
  - crates/heuristic/beam_search/src/nop_hash.rs
  - crates/heuristic/beam_search/src/lib.rs
  - crates/heuristic/beam_search/src/evaluation.rs
  - crates/heuristic/beam_search/src/pool.rs
  - crates/heuristic/beam_search/src/candidate.rs
  - crates/heuristic/beam_search/src/beam_search.rs
  - crates/heuristic/beam_search/src/config.rs
  - crates/heuristic/beam_search/src/index.rs
  - crates/heuristic/beam_search/src/node.rs
  - crates/heuristic/beam_search/src/heap.rs
  timestamp: '2025-04-26 04:27:33+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/heuristic/beam_search/src/state.rs
layout: document
redirect_from:
- /library/crates/heuristic/beam_search/src/state.rs
- /library/crates/heuristic/beam_search/src/state.rs.html
title: crates/heuristic/beam_search/src/state.rs
---
