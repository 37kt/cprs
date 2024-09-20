---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/graph/max-flow/src/lib.rs
    title: crates/graph/max-flow/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://atcoder.jp/contests/abc285/tasks/abc285_g
    - https://tubo28.me/compprog/algorithm/flow_with_lu_bound/
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// reference: https://tubo28.me/compprog/algorithm/flow_with_lu_bound/\n\
    // verify: https://atcoder.jp/contests/abc285/tasks/abc285_g\n\nuse max_flow::{FlowType,\
    \ MaxFlow};\n\npub struct MaxFlowLowerBound {\n    n: usize,\n    mf: MaxFlow,\n\
    \    sum_lower: FlowType,\n}\n\nimpl MaxFlowLowerBound {\n    pub fn new(n: usize)\
    \ -> Self {\n        Self {\n            n,\n            mf: MaxFlow::new(n +\
    \ 2),\n            sum_lower: 0,\n        }\n    }\n\n    // from->to \u306B\u6D41\
    \u91CF\u5236\u7D04 [lower, upper] \u306E\u8FBA\u3092\u5F35\u308B\n    pub fn add_edge(&mut\
    \ self, from: usize, to: usize, lower: FlowType, upper: FlowType) {\n        assert!(from\
    \ != to);\n        assert!(0 <= lower && lower <= upper);\n        let ss = self.n;\n\
    \        let tt = self.n + 1;\n        self.mf.add_edge(from, to, upper - lower);\n\
    \        self.mf.add_edge(ss, to, lower);\n        self.mf.add_edge(from, tt,\
    \ lower);\n        self.sum_lower += lower;\n    }\n\n    pub fn max_flow(&mut\
    \ self, s: usize, t: usize) -> Option<FlowType> {\n        let ss = self.n;\n\
    \        let tt = self.n + 1;\n        let a = self.mf.max_flow(ss, tt);\n   \
    \     let b = self.mf.max_flow(s, tt);\n        let c = self.mf.max_flow(ss, t);\n\
    \        let d = self.mf.max_flow(s, t);\n        if a + c == self.sum_lower &&\
    \ a + b == self.sum_lower {\n            Some(b + d)\n        } else {\n     \
    \       None\n        }\n    }\n}\n"
  dependsOn:
  - crates/graph/max-flow/src/lib.rs
  isVerificationFile: false
  path: crates/graph/max-flow-lower-bound/src/lib.rs
  requiredBy: []
  timestamp: '2023-04-27 21:13:38+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/graph/max-flow-lower-bound/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/max-flow-lower-bound/src/lib.rs
- /library/crates/graph/max-flow-lower-bound/src/lib.rs.html
title: crates/graph/max-flow-lower-bound/src/lib.rs
---
