---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/heuristic/simulated_annealing/src/log_rand.rs
    title: crates/heuristic/simulated_annealing/src/log_rand.rs
  - icon: ':warning:'
    path: crates/heuristic/simulated_annealing/src/scheduler.rs
    title: crates/heuristic/simulated_annealing/src/scheduler.rs
  - icon: ':warning:'
    path: crates/heuristic/simulated_annealing/src/temperature.rs
    title: crates/heuristic/simulated_annealing/src/temperature.rs
  - icon: ':warning:'
    path: crates/misc/random/src/lib.rs
    title: crates/misc/random/src/lib.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/heuristic/simulated_annealing/src/log_rand.rs
    title: crates/heuristic/simulated_annealing/src/log_rand.rs
  - icon: ':warning:'
    path: crates/heuristic/simulated_annealing/src/scheduler.rs
    title: crates/heuristic/simulated_annealing/src/scheduler.rs
  - icon: ':warning:'
    path: crates/heuristic/simulated_annealing/src/temperature.rs
    title: crates/heuristic/simulated_annealing/src/temperature.rs
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
  code: 'mod log_rand;

    mod scheduler;

    mod temperature;


    pub use scheduler::Scheduler;

    pub use temperature::{Exponential, Linear, Temperature};

    '
  dependsOn:
  - crates/heuristic/simulated_annealing/src/log_rand.rs
  - crates/heuristic/simulated_annealing/src/scheduler.rs
  - crates/heuristic/simulated_annealing/src/temperature.rs
  - crates/misc/random/src/lib.rs
  isVerificationFile: false
  path: crates/heuristic/simulated_annealing/src/lib.rs
  requiredBy:
  - crates/heuristic/simulated_annealing/src/log_rand.rs
  - crates/heuristic/simulated_annealing/src/scheduler.rs
  - crates/heuristic/simulated_annealing/src/temperature.rs
  timestamp: '2025-04-26 05:33:09+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/heuristic/simulated_annealing/src/lib.rs
layout: document
redirect_from:
- /library/crates/heuristic/simulated_annealing/src/lib.rs
- /library/crates/heuristic/simulated_annealing/src/lib.rs.html
title: crates/heuristic/simulated_annealing/src/lib.rs
---
