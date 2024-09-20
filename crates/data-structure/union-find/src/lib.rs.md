---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/range-union-find/src/lib.rs
    title: crates/data-structure/range-union-find/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/unionfind/src/main.rs
    title: verify/unionfind/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{cell::RefCell, mem::swap};\n\n#[derive(Clone)]\npub struct UnionFind\
    \ {\n    par: RefCell<Vec<i32>>,\n    cnt: usize,\n}\n\nimpl UnionFind {\n   \
    \ pub fn new(n: usize) -> Self {\n        Self {\n            par: RefCell::new(vec![-1;\
    \ n]),\n            cnt: n,\n        }\n    }\n\n    pub fn len(&self) -> usize\
    \ {\n        self.par.borrow().len()\n    }\n\n    pub fn merge(&mut self, x:\
    \ usize, y: usize) -> bool {\n        let mut x = self.leader(x);\n        let\
    \ mut y = self.leader(y);\n        if x == y {\n            return false;\n  \
    \      }\n        self.cnt -= 1;\n        let mut par = self.par.borrow_mut();\n\
    \        if -par[x] < -par[y] {\n            swap(&mut x, &mut y);\n        }\n\
    \        par[x] += par[y];\n        par[y] = x as i32;\n        true\n    }\n\n\
    \    pub fn leader(&self, x: usize) -> usize {\n        let mut v = x;\n     \
    \   let mut par = self.par.borrow_mut();\n        while par[v] >= 0 {\n      \
    \      v = par[v] as usize;\n        }\n        let mut u = x;\n        while\
    \ par[u] >= 0 {\n            let t = par[u] as usize;\n            par[u] = v\
    \ as i32;\n            u = t;\n        }\n        u\n    }\n\n    pub fn same(&self,\
    \ x: usize, y: usize) -> bool {\n        self.leader(x) == self.leader(y)\n  \
    \  }\n\n    pub fn size(&self, x: usize) -> usize {\n        let x = self.leader(x);\n\
    \        -self.par.borrow()[x] as usize\n    }\n\n    pub fn count(&self) -> usize\
    \ {\n        self.cnt\n    }\n\n    pub fn groups(&self) -> Vec<Vec<usize>> {\n\
    \        let mut res = vec![vec![]; self.len()];\n        for x in 0..self.len()\
    \ {\n            res[self.leader(x)].push(x);\n        }\n        res.into_iter().filter(|g|\
    \ g.len() > 0).collect()\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/data-structure/union-find/src/lib.rs
  requiredBy:
  - crates/data-structure/range-union-find/src/lib.rs
  timestamp: '2024-05-09 13:55:09+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/unionfind/src/main.rs
documentation_of: crates/data-structure/union-find/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/union-find/src/lib.rs
- /library/crates/data-structure/union-find/src/lib.rs.html
title: crates/data-structure/union-find/src/lib.rs
---
