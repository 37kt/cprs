---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/double_ended_priority_queue/src/main.rs
    title: verify/double_ended_priority_queue/src/main.rs
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
  code: "use std::{fmt::Debug, mem::swap};\n\n#[derive(Clone)]\npub struct IntervalHeap<T:\
    \ Clone + Ord>(Vec<T>);\n\nimpl<T: Clone + Ord + Debug> Debug for IntervalHeap<T>\
    \ {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n\
    \        write!(f, \"{:?}\", self.0)\n    }\n}\n\nimpl<T: Clone + Ord> From<Vec<T>>\
    \ for IntervalHeap<T> {\n    fn from(value: Vec<T>) -> Self {\n        let mut\
    \ a = Self(value);\n        a.make_heap();\n        a\n    }\n}\n\nimpl<T: Clone\
    \ + Ord> IntervalHeap<T> {\n    pub fn new() -> Self {\n        Self(vec![])\n\
    \    }\n\n    pub fn len(&self) -> usize {\n        self.0.len()\n    }\n\n  \
    \  pub fn is_empty(&self) -> bool {\n        self.0.is_empty()\n    }\n\n    pub\
    \ fn push(&mut self, x: T) {\n        let k = self.len();\n        self.0.push(x);\n\
    \        self.up(k, 1);\n    }\n\n    pub fn append(&mut self, other: &mut Self)\
    \ {\n        if self.len() < other.len() {\n            swap(self, other);\n \
    \       }\n        let n = self.len();\n        self.0.append(&mut other.0);\n\
    \        for i in (n..self.len()).rev() {\n            self.up(i, 1);\n      \
    \  }\n    }\n\n    pub fn clear(&mut self) {\n        self.0.clear();\n    }\n\
    \n    pub fn peek_min(&self) -> Option<&T> {\n        if self.is_empty() {\n \
    \           None\n        } else if self.len() == 1 {\n            Some(&self.0[0])\n\
    \        } else {\n            Some(&self.0[1])\n        }\n    }\n\n    pub fn\
    \ peek_max(&self) -> Option<&T> {\n        if self.is_empty() {\n            None\n\
    \        } else {\n            Some(&self.0[0])\n        }\n    }\n\n    pub fn\
    \ pop_min(&mut self) -> Option<T> {\n        if self.is_empty() {\n          \
    \  None\n        } else if self.len() < 3 {\n            self.0.pop()\n      \
    \  } else {\n            let n = self.len();\n            self.0.swap(1, n - 1);\n\
    \            let res = self.0.pop();\n            let k = self.down(1);\n    \
    \        self.up(k, 1);\n            res\n        }\n    }\n\n    pub fn pop_max(&mut\
    \ self) -> Option<T> {\n        if self.is_empty() {\n            None\n     \
    \   } else if self.len() < 2 {\n            self.0.pop()\n        } else {\n \
    \           let n = self.len();\n            self.0.swap(0, n - 1);\n        \
    \    let res = self.0.pop();\n            let k = self.down(0);\n            self.up(k,\
    \ 1);\n            res\n        }\n    }\n\n    fn make_heap(&mut self) {\n  \
    \      for i in (0..self.0.len()).rev() {\n            if i % 2 == 1 && self.0[i\
    \ - 1] < self.0[i] {\n                self.0.swap(i - 1, i);\n            }\n\
    \            let k = self.down(i);\n            self.up(k, i);\n        }\n  \
    \  }\n\n    fn parent(k: usize) -> usize {\n        (k >> 1).wrapping_sub(1) &\
    \ !1\n    }\n\n    fn down(&mut self, mut k: usize) -> usize {\n        let n\
    \ = self.0.len();\n        if k % 2 == 1 {\n            while k * 2 + 1 < n {\n\
    \                let mut c = k * 2 + 3;\n                if n <= c || self.0[c\
    \ - 2] < self.0[c] {\n                    c -= 2;\n                }\n       \
    \         if c < n && self.0[c] < self.0[k] {\n                    self.0.swap(k,\
    \ c);\n                    k = c;\n                } else {\n                \
    \    break;\n                }\n            }\n        } else {\n            while\
    \ k * 2 + 2 < n {\n                let mut c = k * 2 + 4;\n                if\
    \ n <= c || self.0[c] < self.0[c - 2] {\n                    c -= 2;\n       \
    \         }\n                if c < n && self.0[k] < self.0[c] {\n           \
    \         self.0.swap(k, c);\n                    k = c;\n                } else\
    \ {\n                    break;\n                }\n            }\n        }\n\
    \        k\n    }\n\n    fn up(&mut self, mut k: usize, root: usize) -> usize\
    \ {\n        if k | 1 < self.0.len() && self.0[k & !1] < self.0[k | 1] {\n   \
    \         self.0.swap(k & !1, k | 1);\n            k ^= 1;\n        }\n      \
    \  let mut p;\n        while root < k {\n            p = Self::parent(k);\n  \
    \          if self.0[p] >= self.0[k] {\n                break;\n            }\n\
    \            self.0.swap(p, k);\n            k = p;\n        }\n        while\
    \ root < k {\n            p = Self::parent(k) | 1;\n            if self.0[k] >=\
    \ self.0[p] {\n                break;\n            }\n            self.0.swap(p,\
    \ k);\n            k = p;\n        }\n        k\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/data-structure/interval-heap/src/lib.rs
  requiredBy: []
  timestamp: '2024-04-08 11:43:23+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/double_ended_priority_queue/src/main.rs
documentation_of: crates/data-structure/interval-heap/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/interval-heap/src/lib.rs
- /library/crates/data-structure/interval-heap/src/lib.rs.html
title: crates/data-structure/interval-heap/src/lib.rs
---
