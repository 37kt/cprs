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
    path: crates/heuristic/simulated_annealing/src/temperature.rs
    title: crates/heuristic/simulated_annealing/src/temperature.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/heuristic/simulated_annealing/src/lib.rs
    title: crates/heuristic/simulated_annealing/src/lib.rs
  - icon: ':warning:'
    path: crates/heuristic/simulated_annealing/src/log_rand.rs
    title: crates/heuristic/simulated_annealing/src/log_rand.rs
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
  code: "use crate::{log_rand::LogRand, temperature::Temperature};\n\npub struct Scheduler<T:\
    \ Temperature> {\n    log_rand: LogRand,\n    time_limit: f64,\n    timer: std::time::Instant,\n\
    \    temperature_function: T,\n    progress: f64,\n    temperature: f64,\n}\n\n\
    impl<T: Temperature> Scheduler<T> {\n    pub fn new(time_limit: f64, temperature_function:\
    \ T) -> Self {\n        let mut scheduler = Self {\n            log_rand: LogRand::new(),\n\
    \            time_limit,\n            timer: std::time::Instant::now(),\n    \
    \        temperature_function,\n            progress: 0.0,\n            temperature:\
    \ 0.0,\n        };\n        scheduler.update();\n        scheduler\n    }\n\n\
    \    /// \u6E29\u5EA6\u3092\u66F4\u65B0\u3059\u308B\u3002\u6642\u9593\u5207\u308C\
    \u306A\u3089 false \u3092\u8FD4\u3059\u3002\n    pub fn update(&mut self) -> bool\
    \ {\n        let elapsed = self.timer.elapsed().as_secs_f64();\n        if elapsed\
    \ >= self.time_limit {\n            return false;\n        }\n        self.progress\
    \ = elapsed / self.time_limit;\n        self.temperature = self.temperature_function.temperature(self.progress);\n\
    \        true\n    }\n\n    pub fn progress(&self) -> f64 {\n        self.progress\n\
    \    }\n\n    pub fn temperature(&self) -> f64 {\n        self.temperature\n \
    \   }\n\n    /// \u5927\u304D\u3044\u307B\u3069 accept \u3055\u308C\u3084\u3059\
    \u3044\n    pub fn accept(&mut self, score_diff: f64) -> bool {\n        score_diff\
    \ > self.log_rand.next() * self.temperature\n    }\n}\n"
  dependsOn:
  - crates/heuristic/simulated_annealing/src/lib.rs
  - crates/heuristic/simulated_annealing/src/log_rand.rs
  - crates/heuristic/simulated_annealing/src/temperature.rs
  isVerificationFile: false
  path: crates/heuristic/simulated_annealing/src/scheduler.rs
  requiredBy:
  - crates/heuristic/simulated_annealing/src/lib.rs
  - crates/heuristic/simulated_annealing/src/log_rand.rs
  - crates/heuristic/simulated_annealing/src/temperature.rs
  timestamp: '2025-04-26 05:33:09+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/heuristic/simulated_annealing/src/scheduler.rs
layout: document
redirect_from:
- /library/crates/heuristic/simulated_annealing/src/scheduler.rs
- /library/crates/heuristic/simulated_annealing/src/scheduler.rs.html
title: crates/heuristic/simulated_annealing/src/scheduler.rs
---
