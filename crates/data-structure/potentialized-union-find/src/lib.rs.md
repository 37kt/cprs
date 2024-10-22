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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{\n    mem::swap,\n    ops::{Add, Neg, Sub},\n};\n\n#[derive(Clone)]\n\
    pub struct PotentializedUnionFind<T, const UNION_BY_SIZE: bool = true>\nwhere\n\
    \    T: Clone + Default + Add<Output = T> + Sub<Output = T> + Neg<Output = T>\
    \ + Eq,\n{\n    par: Vec<i32>,\n    pot: Vec<T>,\n}\n\nimpl<T, const UNION_BY_SIZE:\
    \ bool> PotentializedUnionFind<T, UNION_BY_SIZE>\nwhere\n    T: Clone + Default\
    \ + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Eq,\n{\n    pub fn new(n:\
    \ usize) -> Self {\n        Self {\n            par: vec![-1; n],\n          \
    \  pot: vec![T::default(); n],\n        }\n    }\n\n    pub fn len(&self) -> usize\
    \ {\n        self.par.len()\n    }\n\n    /// \u5165\u529B: P(x) = P(y) + w\n\
    \    /// \u51FA\u529B: \u6574\u5408\u6027\u304C\u3042\u308B\u304B\n    pub fn\
    \ merge(&mut self, x: usize, y: usize, mut w: T) -> bool {\n        w = w + self.potential(y)\
    \ - self.potential(x);\n        let mut x = self.leader(x);\n        let mut y\
    \ = self.leader(y);\n        if x == y {\n            return w == T::default();\n\
    \        }\n        if UNION_BY_SIZE && -self.par[x] > -self.par[y] {\n      \
    \      swap(&mut x, &mut y);\n            w = -w;\n        }\n        self.par[y]\
    \ += self.par[x];\n        self.par[x] = y as i32;\n        self.pot[x] = w;\n\
    \        true\n    }\n\n    pub fn leader(&mut self, x: usize) -> usize {\n  \
    \      if self.par[x] < 0 {\n            x\n        } else {\n            let\
    \ r = self.leader(self.par[x] as usize);\n            self.pot[x] = self.pot[x].clone()\
    \ + self.pot[self.par[x] as usize].clone();\n            self.par[x] = r as i32;\n\
    \            r\n        }\n    }\n\n    pub fn same(&mut self, x: usize, y: usize)\
    \ -> bool {\n        self.leader(x) == self.leader(y)\n    }\n\n    pub fn size(&mut\
    \ self, x: usize) -> usize {\n        let x = self.leader(x);\n        -self.par[x]\
    \ as usize\n    }\n\n    /// P(x) - P(leader(x))\n    pub fn potential(&mut self,\
    \ x: usize) -> T {\n        self.leader(x);\n        self.pot[x].clone()\n   \
    \ }\n\n    /// P(x) - P(y)\n    pub fn diff(&mut self, x: usize, y: usize) ->\
    \ Option<T> {\n        if self.same(x, y) {\n            Some(self.potential(x)\
    \ - self.potential(y))\n        } else {\n            None\n        }\n    }\n\
    \n    pub fn groups(&mut self) -> Vec<Vec<usize>> {\n        let mut res = vec![vec![];\
    \ self.len()];\n        for x in 0..self.len() {\n            res[self.leader(x)].push(x);\n\
    \        }\n        res.into_iter().filter(|g| g.len() > 0).collect()\n    }\n\
    }\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/data-structure/potentialized-union-find/src/lib.rs
  requiredBy: []
  timestamp: '2024-10-22 10:00:24+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data-structure/potentialized-union-find/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/potentialized-union-find/src/lib.rs
- /library/crates/data-structure/potentialized-union-find/src/lib.rs.html
title: crates/data-structure/potentialized-union-find/src/lib.rs
---
