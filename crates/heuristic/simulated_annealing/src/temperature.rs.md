---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/heuristic/simulated_annealing/src/lib.rs
    title: crates/heuristic/simulated_annealing/src/lib.rs
  - icon: ':warning:'
    path: crates/heuristic/simulated_annealing/src/log_rand.rs
    title: crates/heuristic/simulated_annealing/src/log_rand.rs
  - icon: ':warning:'
    path: crates/heuristic/simulated_annealing/src/scheduler.rs
    title: crates/heuristic/simulated_annealing/src/scheduler.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/heuristic/simulated_annealing/src/lib.rs
    title: crates/heuristic/simulated_annealing/src/lib.rs
  - icon: ':warning:'
    path: crates/heuristic/simulated_annealing/src/log_rand.rs
    title: crates/heuristic/simulated_annealing/src/log_rand.rs
  - icon: ':warning:'
    path: crates/heuristic/simulated_annealing/src/scheduler.rs
    title: crates/heuristic/simulated_annealing/src/scheduler.rs
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
  code: "pub trait Temperature {\n    fn temperature(&self, progress: f64) -> f64;\n\
    }\n\npub struct Linear {\n    start: f64,\n    end_minus_start: f64,\n}\n\npub\
    \ struct Exponential {\n    start: f64,\n    end_div_start: f64,\n}\n\nimpl Linear\
    \ {\n    pub fn new(start: f64, end: f64) -> Self {\n        Self {\n        \
    \    start,\n            end_minus_start: end - start,\n        }\n    }\n}\n\n\
    impl Temperature for Linear {\n    fn temperature(&self, progress: f64) -> f64\
    \ {\n        self.start + self.end_minus_start * progress\n    }\n}\n\nimpl Exponential\
    \ {\n    pub fn new(start: f64, end: f64) -> Self {\n        Self {\n        \
    \    start,\n            end_div_start: end / start,\n        }\n    }\n}\n\n\
    impl Temperature for Exponential {\n    fn temperature(&self, progress: f64) ->\
    \ f64 {\n        self.start * self.end_div_start.powf(progress)\n    }\n}\n"
  dependsOn:
  - crates/heuristic/simulated_annealing/src/lib.rs
  - crates/heuristic/simulated_annealing/src/log_rand.rs
  - crates/heuristic/simulated_annealing/src/scheduler.rs
  isVerificationFile: false
  path: crates/heuristic/simulated_annealing/src/temperature.rs
  requiredBy:
  - crates/heuristic/simulated_annealing/src/scheduler.rs
  - crates/heuristic/simulated_annealing/src/lib.rs
  - crates/heuristic/simulated_annealing/src/log_rand.rs
  timestamp: '2025-04-26 04:59:33+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/heuristic/simulated_annealing/src/temperature.rs
layout: document
redirect_from:
- /library/crates/heuristic/simulated_annealing/src/temperature.rs
- /library/crates/heuristic/simulated_annealing/src/temperature.rs.html
title: crates/heuristic/simulated_annealing/src/temperature.rs
---
