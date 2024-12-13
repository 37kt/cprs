---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://atcoder.jp/contests/abc217/tasks/abc217_h
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verify: https://atcoder.jp/contests/abc217/tasks/abc217_h\n\nuse std::{cmp::Reverse,\
    \ collections::BinaryHeap, mem::swap};\n\npub struct SlopeTrick {\n    add_l:\
    \ i64,\n    add_r: i64,\n    min_y: i64,\n    lq: BinaryHeap<(i64, i64)>,\n  \
    \  rq: BinaryHeap<Reverse<(i64, i64)>>,\n}\n\nimpl SlopeTrick {\n    pub fn new()\
    \ -> Self {\n        Self {\n            add_l: 0,\n            add_r: 0,\n  \
    \          min_y: 0,\n            lq: Default::default(),\n            rq: Default::default(),\n\
    \        }\n    }\n\n    /// (min, argmin)\n    pub fn min(&self) -> (i64, i64)\
    \ {\n        (\n            self.min_y,\n            self.get_l().or(self.get_r()).map(|(x,\
    \ _)| x).unwrap_or(0),\n        )\n    }\n\n    pub fn add_a(&mut self, a: i64)\
    \ {\n        self.min_y += a;\n    }\n\n    /// f(x) <- f(x - a)\n    pub fn shift(&mut\
    \ self, a: i64) {\n        self.add_l += a;\n        self.add_r += a;\n    }\n\
    \n    /// f(x) <- min f(y) (x-b <= y <= x-a)\n    pub fn slide_min(&mut self,\
    \ a: i64, b: i64) {\n        assert!(a <= b);\n        self.add_l += a;\n    \
    \    self.add_r += b;\n    }\n\n    /// add (x-a).max(0)\n    pub fn add_x_minus_a(&mut\
    \ self, a: i64, c: i64) {\n        let mut used = 0;\n        while used < c &&\
    \ !self.lq.is_empty() {\n            let (x, d) = self.get_l().unwrap();\n   \
    \         if x <= a {\n                break;\n            }\n            self.pop_l();\n\
    \            let t = d.min(c - used);\n            self.push_r(x, t);\n      \
    \      if d != t {\n                self.push_l(x, d - t);\n            }\n  \
    \          self.min_y += (x - a) * t;\n            used += t;\n        }\n   \
    \     if used != 0 {\n            self.push_l(a, used);\n        }\n        if\
    \ c - used != 0 {\n            self.push_r(a, c - used);\n        }\n    }\n\n\
    \    /// add (a-x).max(0)\n    pub fn add_a_minus_x(&mut self, a: i64, c: i64)\
    \ {\n        let mut used = 0;\n        while used < c && !self.rq.is_empty()\
    \ {\n            let (x, d) = self.get_r().unwrap();\n            if x >= a {\n\
    \                break;\n            }\n            self.pop_r();\n          \
    \  let t = d.min(c - used);\n            self.push_l(x, t);\n            if d\
    \ != t {\n                self.push_r(x, d - t);\n            }\n            self.min_y\
    \ += (a - x) * t;\n            used += t;\n        }\n        if used != 0 {\n\
    \            self.push_r(a, used);\n        }\n        if c - used != 0 {\n  \
    \          self.push_l(a, c - used);\n        }\n    }\n\n    /// add |x-a|\n\
    \    pub fn add_abs_x_minus_a(&mut self, a: i64, c: i64) {\n        self.add_x_minus_a(a,\
    \ c);\n        self.add_a_minus_x(a, c);\n    }\n\n    /// \u7D2F\u7A4Dmin\n \
    \   pub fn chmin_right(&mut self) {\n        self.rq.clear();\n    }\n\n    ///\
    \ \u9006\u7D2F\u7A4Dmin\n    pub fn chmin_left(&mut self) {\n        self.lq.clear();\n\
    \    }\n\n    pub fn merge(&mut self, mut other: Self) {\n        if self.lq.len()\
    \ + self.rq.len() < other.lq.len() + other.rq.len() {\n            swap(self,\
    \ &mut other);\n        }\n        while let Some((x, c)) = other.lq.pop() {\n\
    \            self.add_a_minus_x(x, c);\n        }\n        while let Some(Reverse((x,\
    \ c))) = other.rq.pop() {\n            self.add_x_minus_a(x, c);\n        }\n\
    \        self.add_a(other.min_y);\n    }\n\n    fn push_l(&mut self, x: i64, c:\
    \ i64) {\n        self.lq.push((x - self.add_l, c));\n    }\n\n    fn push_r(&mut\
    \ self, x: i64, c: i64) {\n        self.rq.push(Reverse((x - self.add_r, c)));\n\
    \    }\n\n    fn get_l(&self) -> Option<(i64, i64)> {\n        self.lq.peek().map(|&(x,\
    \ c)| (x + self.add_l, c))\n    }\n\n    fn get_r(&self) -> Option<(i64, i64)>\
    \ {\n        self.rq.peek().map(|&Reverse((x, c))| (x + self.add_r, c))\n    }\n\
    \n    fn pop_l(&mut self) {\n        self.lq.pop();\n    }\n\n    fn pop_r(&mut\
    \ self) {\n        self.rq.pop();\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/data-structure/slope-trick/src/lib.rs
  requiredBy: []
  timestamp: '2024-11-25 08:41:50+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data-structure/slope-trick/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/slope-trick/src/lib.rs
- /library/crates/data-structure/slope-trick/src/lib.rs.html
title: crates/data-structure/slope-trick/src/lib.rs
---
