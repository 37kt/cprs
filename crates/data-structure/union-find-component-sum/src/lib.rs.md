---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/tree/zero-one-on-tree/src/lib.rs
    title: crates/tree/zero-one-on-tree/src/lib.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::mem::swap;\n\nuse algebraic::Monoid;\n\n#[derive(Clone)]\npub struct\
    \ UnionFindComponentSum<M, const UNION_BY_SIZE: bool = true>\nwhere\n    M: Monoid,\n\
    \    M::S: Clone,\n{\n    par: Vec<i32>,\n    sum: Vec<M::S>,\n    cnt: usize,\n\
    }\n\nimpl<M, const UNION_BY_SIZE: bool> UnionFindComponentSum<M, UNION_BY_SIZE>\n\
    where\n    M: Monoid,\n    M::S: Clone,\n{\n    pub fn new(a: &[M::S]) -> Self\
    \ {\n        let n = a.len();\n        Self {\n            par: vec![-1; n],\n\
    \            sum: a.to_vec(),\n            cnt: n,\n        }\n    }\n\n    pub\
    \ fn len(&self) -> usize {\n        self.par.len()\n    }\n\n    pub fn count(&self)\
    \ -> usize {\n        self.cnt\n    }\n\n    pub fn merge(&mut self, x: usize,\
    \ y: usize) -> bool {\n        let mut x = self.leader(x);\n        let mut y\
    \ = self.leader(y);\n        if x == y {\n            return false;\n        }\n\
    \        self.cnt -= 1;\n        if UNION_BY_SIZE && -self.par[x] < -self.par[y]\
    \ {\n            swap(&mut x, &mut y);\n        }\n        self.par[x] += self.par[y];\n\
    \        self.sum[x] = M::op(&self.sum[x], &self.sum[y]);\n        self.par[y]\
    \ = x as i32;\n        true\n    }\n\n    pub fn leader(&mut self, x: usize) ->\
    \ usize {\n        let mut v = x;\n        while self.par[v] >= 0 {\n        \
    \    v = self.par[v] as usize;\n        }\n        let mut u = x;\n        while\
    \ self.par[u] >= 0 {\n            let t = self.par[u] as usize;\n            self.par[u]\
    \ = v as i32;\n            u = t;\n        }\n        u\n    }\n\n    pub fn same(&mut\
    \ self, x: usize, y: usize) -> bool {\n        self.leader(x) == self.leader(y)\n\
    \    }\n\n    pub fn size(&mut self, x: usize) -> usize {\n        let x = self.leader(x);\n\
    \        -self.par[x] as usize\n    }\n\n    pub fn groups(&mut self) -> Vec<Vec<usize>>\
    \ {\n        let mut res = vec![vec![]; self.len()];\n        for x in 0..self.len()\
    \ {\n            res[self.leader(x)].push(x);\n        }\n        res.into_iter().filter(|g|\
    \ g.len() > 0).collect()\n    }\n\n    pub fn sum(&mut self, x: usize) -> M::S\
    \ {\n        let x = self.leader(x);\n        self.sum[x].clone()\n    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  isVerificationFile: false
  path: crates/data-structure/union-find-component-sum/src/lib.rs
  requiredBy:
  - crates/tree/zero-one-on-tree/src/lib.rs
  timestamp: '2024-12-25 03:34:39+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data-structure/union-find-component-sum/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/union-find-component-sum/src/lib.rs
- /library/crates/data-structure/union-find-component-sum/src/lib.rs.html
title: crates/data-structure/union-find-component-sum/src/lib.rs
---
