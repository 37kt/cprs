---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/heuristic/simulated_annealing/src/lib.rs
    title: crates/heuristic/simulated_annealing/src/lib.rs
  - icon: ':warning:'
    path: crates/heuristic/simulated_annealing/src/scheduler.rs
    title: crates/heuristic/simulated_annealing/src/scheduler.rs
  - icon: ':warning:'
    path: crates/heuristic/simulated_annealing/src/temperature.rs
    title: crates/heuristic/simulated_annealing/src/temperature.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/heuristic/simulated_annealing/src/lib.rs
    title: crates/heuristic/simulated_annealing/src/lib.rs
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
  code: "const LOG_RAND_LEN: usize = 1 << 16;\n\nthread_local! {\n    static LOG_RAND:\
    \ [f64; LOG_RAND_LEN] = {\n        let mut rng = random::Pcg64Fast::default();\n\
    \        let log_u64max = 2.0f64.ln() * 64.0;\n        std::array::from_fn(|_|\
    \ (rng.u64() as f64).ln() - log_u64max)\n    };\n}\n\npub(crate) struct LogRand\
    \ {\n    index: usize,\n}\n\nimpl LogRand {\n    pub fn new() -> Self {\n    \
    \    let mut rng = random::Pcg64Fast::default();\n        Self {\n           \
    \ index: (rng.u64() as usize) % LOG_RAND_LEN,\n        }\n    }\n\n    pub fn\
    \ next(&mut self) -> f64 {\n        let res = LOG_RAND.with(|log_rand| log_rand[self.index]);\n\
    \        self.index += 1;\n        if self.index >= LOG_RAND_LEN {\n         \
    \   self.index = 0;\n        }\n        res\n    }\n}\n"
  dependsOn:
  - crates/heuristic/simulated_annealing/src/lib.rs
  - crates/heuristic/simulated_annealing/src/scheduler.rs
  - crates/heuristic/simulated_annealing/src/temperature.rs
  isVerificationFile: false
  path: crates/heuristic/simulated_annealing/src/log_rand.rs
  requiredBy:
  - crates/heuristic/simulated_annealing/src/temperature.rs
  - crates/heuristic/simulated_annealing/src/lib.rs
  - crates/heuristic/simulated_annealing/src/scheduler.rs
  timestamp: '2025-04-26 05:33:09+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/heuristic/simulated_annealing/src/log_rand.rs
layout: document
redirect_from:
- /library/crates/heuristic/simulated_annealing/src/log_rand.rs
- /library/crates/heuristic/simulated_annealing/src/log_rand.rs.html
title: crates/heuristic/simulated_annealing/src/log_rand.rs
---
