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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use algebraic::Monoid;\n\n/// \u30E2\u30CE\u30A4\u30C9\u7DCF\u7A4D\u3092\u6301\
    \u3064 VecDeque\npub struct SlidingWindowAggregation<M>\nwhere\n    M: Monoid,\n\
    \    M::S: Clone,\n{\n    fv: Vec<M::S>,\n    bv: Vec<M::S>,\n    fs: Vec<M::S>,\n\
    \    bs: Vec<M::S>,\n}\n\nimpl<M> SlidingWindowAggregation<M>\nwhere\n    M: Monoid,\n\
    \    M::S: Clone,\n{\n    /// \u7A7A\u306E\u5217\u3067\u521D\u671F\u5316\u3059\
    \u308B\u3002\n    pub fn new() -> Self {\n        Self {\n            fv: vec![],\n\
    \            bv: vec![],\n            fs: vec![M::e()],\n            bs: vec![M::e()],\n\
    \        }\n    }\n\n    /// \u5217\u3092\u30AF\u30EA\u30A2\u3059\u308B\u3002\n\
    \    pub fn clear(&mut self) {\n        self.fv.clear();\n        self.bv.clear();\n\
    \        self.fs.clear();\n        self.fs.push(M::e());\n        self.bs.clear();\n\
    \        self.bs.push(M::e());\n    }\n\n    /// \u5217\u306E\u9577\u3055\u3092\
    \u53D6\u5F97\u3059\u308B\u3002\n    pub fn len(&self) -> usize {\n        self.fv.len()\
    \ + self.bv.len()\n    }\n\n    /// \u5217\u304C\u7A7A\u304B\u3069\u3046\u304B\
    \u3092\u5224\u5B9A\u3059\u308B\u3002\n    pub fn is_empty(&self) -> bool {\n \
    \       self.len() == 0\n    }\n\n    /// \u5217\u306E\u672B\u5C3E\u306B\u8981\
    \u7D20\u3092\u8FFD\u52A0\u3059\u308B\u3002\n    pub fn push_back(&mut self, x:\
    \ M::S) {\n        self.bs.push(M::op(self.bs.last().unwrap(), &x));\n       \
    \ self.bv.push(x);\n    }\n\n    /// \u5217\u306E\u5148\u982D\u306B\u8981\u7D20\
    \u3092\u8FFD\u52A0\u3059\u308B\u3002\n    pub fn push_front(&mut self, x: M::S)\
    \ {\n        self.fs.push(M::op(&x, self.fs.last().unwrap()));\n        self.fv.push(x);\n\
    \    }\n\n    /// \u5217\u306E\u672B\u5C3E\u306E\u8981\u7D20\u3092\u53D6\u308A\
    \u51FA\u3059\u3002\n    pub fn pop_back(&mut self) -> Option<M::S> {\n       \
    \ if self.len() == 0 {\n            None\n        } else if self.len() == 1 {\n\
    \            if self.bv.len() == 1 {\n                self.bs.pop();\n       \
    \         self.bv.pop()\n            } else {\n                self.fs.pop();\n\
    \                self.fv.pop()\n            }\n        } else {\n            if\
    \ self.bv.is_empty() {\n                self.rebuild();\n            }\n     \
    \       self.bs.pop();\n            self.bv.pop()\n        }\n    }\n\n    ///\
    \ \u5217\u306E\u5148\u982D\u306E\u8981\u7D20\u3092\u53D6\u308A\u51FA\u3059\u3002\
    \n    pub fn pop_front(&mut self) -> Option<M::S> {\n        if self.len() ==\
    \ 0 {\n            None\n        } else if self.len() == 1 {\n            if self.bv.len()\
    \ == 1 {\n                self.bs.pop();\n                self.bv.pop()\n    \
    \        } else {\n                self.fs.pop();\n                self.fv.pop()\n\
    \            }\n        } else {\n            if self.fv.is_empty() {\n      \
    \          self.rebuild();\n            }\n            self.fs.pop();\n      \
    \      self.fv.pop()\n        }\n    }\n\n    /// \u5217\u306E\u5148\u982D\u306E\
    \u8981\u7D20\u3092\u53D6\u5F97\u3059\u308B\u3002\n    pub fn front(&self) -> Option<&M::S>\
    \ {\n        self.fv.first()\n    }\n\n    /// \u5217\u306E\u672B\u5C3E\u306E\u8981\
    \u7D20\u3092\u53D6\u5F97\u3059\u308B\u3002\n    pub fn back(&self) -> Option<&M::S>\
    \ {\n        self.bv.last()\n    }\n\n    /// \u5217\u306E\u7DCF\u7A4D\u3092\u53D6\
    \u5F97\u3059\u308B\u3002\n    pub fn prod(&self) -> M::S {\n        M::op(self.fs.last().unwrap(),\
    \ self.bs.last().unwrap())\n    }\n\n    fn rebuild(&mut self) {\n        let\
    \ mut v = vec![];\n        v.reserve(self.len());\n        v.append(&mut self.fv);\n\
    \        v.reverse();\n        v.append(&mut self.bv);\n        self.clear();\n\
    \        for x in v[0..v.len() / 2].iter().rev() {\n            self.push_front(x.clone());\n\
    \        }\n        for x in &v[v.len() / 2..v.len()] {\n            self.push_back(x.clone());\n\
    \        }\n    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  isVerificationFile: false
  path: crates/data-structure/sliding-window-aggregation/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-04 02:49:00+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/deque_operate_all_composite/src/main.rs
  - verify/queue_operate_all_composite/src/main.rs
documentation_of: crates/data-structure/sliding-window-aggregation/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/sliding-window-aggregation/src/lib.rs
- /library/crates/data-structure/sliding-window-aggregation/src/lib.rs.html
title: crates/data-structure/sliding-window-aggregation/src/lib.rs
---
