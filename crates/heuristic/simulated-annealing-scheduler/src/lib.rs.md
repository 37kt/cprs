---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/heuristic/timer/src/lib.rs
    title: crates/heuristic/timer/src/lib.rs
  - icon: ':warning:'
    path: crates/misc/random/src/lib.rs
    title: crates/misc/random/src/lib.rs
  _extendedRequiredBy: []
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
  code: "use std::cell::RefCell;\n\nuse timer::Timer;\n\nconst LOG_RAND_LEN: usize\
    \ = 1 << 16;\n\nstruct LogRand {\n    log_rand: Vec<f64>,\n    log_rand_index:\
    \ usize,\n}\n\nimpl LogRand {\n    fn new() -> Self {\n        let mut rng = random::Pcg64Fast::default();\n\
    \        let log_u64max = 2.0f64.ln() * 64.0;\n        let log_rand = (0..LOG_RAND_LEN)\n\
    \            .map(|_| (rng.u64() as f64).ln() - log_u64max)\n            .collect();\n\
    \        Self {\n            log_rand,\n            log_rand_index: 0,\n     \
    \   }\n    }\n\n    fn next(&mut self) -> f64 {\n        let res = self.log_rand[self.log_rand_index];\n\
    \        self.log_rand_index = (self.log_rand_index + 1) % LOG_RAND_LEN;\n   \
    \     res\n    }\n}\n\nthread_local! {\n    static LOG_RAND: RefCell<LogRand>\
    \ = RefCell::new(LogRand::new());\n}\n\n#[derive(Debug, Clone, Copy, PartialEq,\
    \ Eq)]\npub enum SimulatedAnnealingTemperatureFunction {\n    Linear,\n    Exponential,\n\
    }\n\npub struct SimulatedAnnealingScheduler {\n    start_temp: f64,\n    end_temp:\
    \ f64,\n    temp_function: SimulatedAnnealingTemperatureFunction,\n    time_limit:\
    \ f64,\n    timer: Timer,\n    current_temp: f64,\n    progress: f64,\n}\n\nimpl\
    \ SimulatedAnnealingScheduler {\n    pub fn new(\n        start_temp: f64,\n \
    \       end_temp: f64,\n        temp_function: SimulatedAnnealingTemperatureFunction,\n\
    \        time_limit: f64,\n    ) -> Self {\n        let timer = Timer::new();\n\
    \        let mut scheduler = Self {\n            start_temp,\n            end_temp,\n\
    \            temp_function,\n            time_limit,\n            timer,\n   \
    \         current_temp: start_temp,\n            progress: 0.0,\n        };\n\
    \        scheduler.update_temperature();\n        scheduler\n    }\n\n    pub\
    \ fn temperature(&self) -> f64 {\n        self.current_temp\n    }\n\n    pub\
    \ fn update_temperature(&mut self) -> bool {\n        let elapsed = self.timer.elapsed_secs();\n\
    \        if elapsed >= self.time_limit {\n            return false;\n        }\n\
    \n        self.progress = elapsed / self.time_limit;\n\n        match self.temp_function\
    \ {\n            SimulatedAnnealingTemperatureFunction::Linear => {\n        \
    \        self.current_temp =\n                    self.start_temp + (self.end_temp\
    \ - self.start_temp) * self.progress;\n            }\n            SimulatedAnnealingTemperatureFunction::Exponential\
    \ => {\n                self.current_temp =\n                    self.start_temp.powf(1.0\
    \ - self.progress) * self.end_temp.powf(self.progress);\n            }\n     \
    \   }\n\n        true\n    }\n\n    pub fn accept(&mut self, score_diff: f64)\
    \ -> bool {\n        let rand = LOG_RAND.with(|log_rand| log_rand.borrow_mut().next());\n\
    \        score_diff > self.current_temp * rand\n    }\n}\n"
  dependsOn:
  - crates/heuristic/timer/src/lib.rs
  - crates/misc/random/src/lib.rs
  isVerificationFile: false
  path: crates/heuristic/simulated-annealing-scheduler/src/lib.rs
  requiredBy: []
  timestamp: '2025-02-06 01:55:37+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/heuristic/simulated-annealing-scheduler/src/lib.rs
layout: document
redirect_from:
- /library/crates/heuristic/simulated-annealing-scheduler/src/lib.rs
- /library/crates/heuristic/simulated-annealing-scheduler/src/lib.rs.html
title: crates/heuristic/simulated-annealing-scheduler/src/lib.rs
---
