---
data:
  _extendedDependsOn: []
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
  code: "use libm::erf;\n\n#[derive(Debug, Clone, Copy)]\npub struct NormalDistribution\
    \ {\n    mean: f64,\n    std_dev: f64,\n}\n\nimpl NormalDistribution {\n    pub\
    \ fn new(mean: f64, std_dev: f64) -> Self {\n        Self { mean, std_dev }\n\
    \    }\n\n    pub fn mean(&self) -> f64 {\n        self.mean\n    }\n\n    pub\
    \ fn std_dev(&self) -> f64 {\n        self.std_dev\n    }\n\n    // x \u4EE5\u4E0B\
    \u306E\u78BA\u7387\n    pub fn cdf(&self, x: f64) -> f64 {\n        0.5 * (1.0\
    \ + erf((x - self.mean) / (self.std_dev * std::f64::consts::SQRT_2)))\n    }\n\
    \n    // x \u4EE5\u4E0A\u306E\u78BA\u7387\n    pub fn ccdf(&self, x: f64) -> f64\
    \ {\n        1.0 - self.cdf(x)\n    }\n\n    pub fn pdf(&self, x: f64) -> f64\
    \ {\n        let d = (x - self.mean) / self.std_dev;\n        (-0.5 * d * d).exp()\
    \ / (self.std_dev * std::f64::consts::SQRT_2 * std::f64::consts::PI)\n    }\n\
    }\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/heuristic/normal-distribution/src/lib.rs
  requiredBy: []
  timestamp: '2025-02-06 06:59:03+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/heuristic/normal-distribution/src/lib.rs
layout: document
redirect_from:
- /library/crates/heuristic/normal-distribution/src/lib.rs
- /library/crates/heuristic/normal-distribution/src/lib.rs.html
title: crates/heuristic/normal-distribution/src/lib.rs
---
