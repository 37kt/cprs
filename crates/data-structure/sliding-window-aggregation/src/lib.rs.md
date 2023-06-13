---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/deque_operate_all_composite/src/main.rs
    title: verify/deque_operate_all_composite/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/queue_operate_all_composite/src/main.rs
    title: verify/queue_operate_all_composite/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use algebraic::Monoid;\n\npub struct SlidingWindowAggregation<M>\nwhere\n\
    \    M: Monoid,\n    M::S: Clone,\n{\n    fv: Vec<M::S>,\n    bv: Vec<M::S>,\n\
    \    fs: Vec<M::S>,\n    bs: Vec<M::S>,\n}\n\nimpl<M> SlidingWindowAggregation<M>\n\
    where\n    M: Monoid,\n    M::S: Clone,\n{\n    pub fn new() -> Self {\n     \
    \   Self {\n            fv: vec![],\n            bv: vec![],\n            fs:\
    \ vec![M::e()],\n            bs: vec![M::e()],\n        }\n    }\n\n    pub fn\
    \ clear(&mut self) {\n        self.fv.clear();\n        self.bv.clear();\n   \
    \     self.fs.clear();\n        self.fs.push(M::e());\n        self.bs.clear();\n\
    \        self.bs.push(M::e());\n    }\n\n    pub fn len(&self) -> usize {\n  \
    \      self.fv.len() + self.bv.len()\n    }\n\n    pub fn is_empty(&self) -> bool\
    \ {\n        self.len() == 0\n    }\n\n    pub fn push_back(&mut self, x: M::S)\
    \ {\n        self.bs.push(M::op(self.bs.last().unwrap(), &x));\n        self.bv.push(x);\n\
    \    }\n\n    pub fn push_front(&mut self, x: M::S) {\n        self.fs.push(M::op(&x,\
    \ self.fs.last().unwrap()));\n        self.fv.push(x);\n    }\n\n    pub fn pop_back(&mut\
    \ self) -> Option<M::S> {\n        if self.len() == 0 {\n            None\n  \
    \      } else if self.len() == 1 {\n            if self.bv.len() == 1 {\n    \
    \            self.bs.pop();\n                self.bv.pop()\n            } else\
    \ {\n                self.fs.pop();\n                self.fv.pop()\n         \
    \   }\n        } else {\n            if self.bv.is_empty() {\n               \
    \ self.rebuild();\n            }\n            self.bs.pop();\n            self.bv.pop()\n\
    \        }\n    }\n\n    pub fn pop_front(&mut self) -> Option<M::S> {\n     \
    \   if self.len() == 0 {\n            None\n        } else if self.len() == 1\
    \ {\n            if self.bv.len() == 1 {\n                self.bs.pop();\n   \
    \             self.bv.pop()\n            } else {\n                self.fs.pop();\n\
    \                self.fv.pop()\n            }\n        } else {\n            if\
    \ self.fv.is_empty() {\n                self.rebuild();\n            }\n     \
    \       self.fs.pop();\n            self.fv.pop()\n        }\n    }\n\n    pub\
    \ fn prod(&self) -> M::S {\n        M::op(self.fs.last().unwrap(), self.bs.last().unwrap())\n\
    \    }\n\n    fn rebuild(&mut self) {\n        let mut v = vec![];\n        v.reserve(self.len());\n\
    \        v.append(&mut self.fv);\n        v.reverse();\n        v.append(&mut\
    \ self.bv);\n        self.clear();\n        for x in v[0..v.len() / 2].iter().rev()\
    \ {\n            self.push_front(x.clone());\n        }\n        for x in &v[v.len()\
    \ / 2..v.len()] {\n            self.push_back(x.clone());\n        }\n    }\n\
    }\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  isVerificationFile: false
  path: crates/data-structure/sliding-window-aggregation/src/lib.rs
  requiredBy: []
  timestamp: '2023-04-25 15:51:20+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/queue_operate_all_composite/src/main.rs
  - verify/deque_operate_all_composite/src/main.rs
documentation_of: crates/data-structure/sliding-window-aggregation/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/sliding-window-aggregation/src/lib.rs
- /library/crates/data-structure/sliding-window-aggregation/src/lib.rs.html
title: crates/data-structure/sliding-window-aggregation/src/lib.rs
---
