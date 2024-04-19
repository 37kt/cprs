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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.3/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.3/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::mem::swap;\n\n#[derive(Clone)]\npub struct RollbackUnionFind {\n\
    \    par: Vec<i32>,\n    his: Vec<(usize, i32)>,\n}\n\nimpl RollbackUnionFind\
    \ {\n    pub fn new(n: usize) -> Self {\n        Self {\n            par: vec![-1;\
    \ n],\n            his: vec![],\n        }\n    }\n\n    pub fn len(&self) ->\
    \ usize {\n        self.par.len()\n    }\n\n    pub fn merge(&mut self, x: usize,\
    \ y: usize) -> bool {\n        let mut x = self.leader(x);\n        let mut y\
    \ = self.leader(y);\n        self.his.push((x, self.par[x]));\n        self.his.push((y,\
    \ self.par[y]));\n        if x == y {\n            return false;\n        }\n\
    \        if -self.par[x] < -self.par[y] {\n            swap(&mut x, &mut y);\n\
    \        }\n        self.par[x] += self.par[y];\n        self.par[y] = x as i32;\n\
    \        true\n    }\n\n    pub fn leader(&self, mut x: usize) -> usize {\n  \
    \      while self.par[x] >= 0 {\n            x = self.par[x] as usize;\n     \
    \   }\n        x\n    }\n\n    pub fn same(&self, x: usize, y: usize) -> bool\
    \ {\n        self.leader(x) == self.leader(y)\n    }\n\n    pub fn size(&self,\
    \ x: usize) -> usize {\n        -self.par[self.leader(x)] as usize\n    }\n\n\
    \    pub fn undo(&mut self) {\n        for _ in 0..2 {\n            let (x, par)\
    \ = self.his.pop().unwrap();\n            self.par[x] = par;\n        }\n    }\n\
    \n    pub fn groups(&self) -> Vec<Vec<usize>> {\n        let mut res = vec![vec![];\
    \ self.len()];\n        for x in 0..self.len() {\n            res[self.leader(x)].push(x);\n\
    \        }\n        res.into_iter().filter(|g| g.len() > 0).collect()\n    }\n\
    }\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/data-structure/rollback-union-find/src/lib.rs
  requiredBy: []
  timestamp: '2023-05-21 09:38:35+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data-structure/rollback-union-find/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/rollback-union-find/src/lib.rs
- /library/crates/data-structure/rollback-union-find/src/lib.rs.html
title: crates/data-structure/rollback-union-find/src/lib.rs
---
