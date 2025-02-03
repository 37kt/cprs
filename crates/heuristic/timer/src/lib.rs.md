---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/heuristic/beam-search/src/lib.rs
    title: crates/heuristic/beam-search/src/lib.rs
  - icon: ':warning:'
    path: crates/heuristic/simulated-annealing-scheduler/src/lib.rs
    title: crates/heuristic/simulated-annealing-scheduler/src/lib.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "pub struct Timer {\n    start: f64,\n}\n\nimpl Timer {\n    fn time_secs()\
    \ -> f64 {\n        std::time::SystemTime::now()\n            .duration_since(std::time::UNIX_EPOCH)\n\
    \            .unwrap()\n            .as_secs_f64()\n    }\n\n    pub fn new()\
    \ -> Self {\n        Self {\n            start: Self::time_secs(),\n        }\n\
    \    }\n\n    pub fn elapsed_secs(&self) -> f64 {\n        Self::time_secs() -\
    \ self.start\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/heuristic/timer/src/lib.rs
  requiredBy:
  - crates/heuristic/simulated-annealing-scheduler/src/lib.rs
  - crates/heuristic/beam-search/src/lib.rs
  timestamp: '2025-01-25 11:36:32+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/heuristic/timer/src/lib.rs
layout: document
redirect_from:
- /library/crates/heuristic/timer/src/lib.rs
- /library/crates/heuristic/timer/src/lib.rs.html
title: crates/heuristic/timer/src/lib.rs
---
