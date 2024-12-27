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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// reference: https://tubo28.me/compprog/algorithm/flow_with_lu_bound/\n\
    // verify: https://atcoder.jp/contests/abc285/tasks/abc285_g\n\nuse max_flow::{FlowType,\
    \ MaxFlow};\n\n/// \u6D41\u91CF\u306E\u4E0B\u9650\u3082\u6307\u5B9A\u3055\u308C\
    \u3066\u3044\u308B\u5834\u5408\u306E\u6700\u5927\u6D41\u3092\u6C42\u3081\u308B\
    \npub struct MaxFlowLowerBound {\n    n: usize,\n    mf: MaxFlow,\n    sum_lower:\
    \ FlowType,\n}\n\nimpl MaxFlowLowerBound {\n    /// \u9802\u70B9\u6570 n \u3067\
    \u521D\u671F\u5316\u3059\u308B\n    pub fn new(n: usize) -> Self {\n        Self\
    \ {\n            n,\n            mf: MaxFlow::new(n + 2),\n            sum_lower:\
    \ 0,\n        }\n    }\n\n    /// from->to \u306B\u6D41\u91CF\u5236\u7D04 \\[lower,\
    \ upper\\] \u306E\u8FBA\u3092\u5F35\u308B\n    pub fn add_edge(&mut self, from:\
    \ usize, to: usize, lower: FlowType, upper: FlowType) {\n        assert!(from\
    \ != to);\n        assert!(0 <= lower && lower <= upper);\n        let ss = self.n;\n\
    \        let tt = self.n + 1;\n        self.mf.add_edge(from, to, upper - lower);\n\
    \        self.mf.add_edge(ss, to, lower);\n        self.mf.add_edge(from, tt,\
    \ lower);\n        self.sum_lower += lower;\n    }\n\n    /// s->t \u3078\u306E\
    \u6700\u5927\u6D41\u3092\u6C42\u3081\u308B  \n    /// \u6D41\u91CF\u306E\u5236\
    \u7D04\u3092\u6E80\u305F\u3059\u30D5\u30ED\u30FC\u304C\u5B58\u5728\u3057\u306A\
    \u3044\u5834\u5408\u306F None \u3092\u8FD4\u3059\n    pub fn max_flow(&mut self,\
    \ s: usize, t: usize) -> Option<FlowType> {\n        let ss = self.n;\n      \
    \  let tt = self.n + 1;\n        let a = self.mf.max_flow(ss, tt);\n        let\
    \ b = self.mf.max_flow(s, tt);\n        let c = self.mf.max_flow(ss, t);\n   \
    \     let d = self.mf.max_flow(s, t);\n        if a + c == self.sum_lower && a\
    \ + b == self.sum_lower {\n            Some(b + d)\n        } else {\n       \
    \     None\n        }\n    }\n}\n"
  dependsOn:
  - crates/graph/max-flow/src/lib.rs
  isVerificationFile: false
  path: crates/graph/max-flow-lower-bound/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-27 03:53:35+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/graph/max-flow-lower-bound/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/max-flow-lower-bound/src/lib.rs
- /library/crates/graph/max-flow-lower-bound/src/lib.rs.html
title: crates/graph/max-flow-lower-bound/src/lib.rs
---
