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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::time::Instant;\n\npub struct Timer {\n    start: Instant,\n}\n\n\
    impl Timer {\n    pub fn new() -> Self {\n        Self {\n            start: Instant::now(),\n\
    \        }\n    }\n\n    pub fn elapsed(&self) -> f64 {\n        self.start.elapsed().as_secs_f64()\n\
    \    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/heuristic/timer/src/lib.rs
  requiredBy: []
  timestamp: '2023-11-09 12:46:30+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/heuristic/timer/src/lib.rs
layout: document
redirect_from:
- /library/crates/heuristic/timer/src/lib.rs
- /library/crates/heuristic/timer/src/lib.rs.html
title: crates/heuristic/timer/src/lib.rs
---
