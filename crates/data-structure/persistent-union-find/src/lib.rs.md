---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/data-structure/persistent-array/src/lib.rs
    title: crates/data-structure/persistent-array/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/persistent_unionfind/src/main.rs
    title: verify/persistent_unionfind/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::mem::swap;\n\nuse persistent_array::PersistentArray;\n\n#[derive(Clone)]\n\
    pub struct PersistentUnionFind(PersistentArray<i32, 8>);\n\nimpl PersistentUnionFind\
    \ {\n    pub fn new(n: usize) -> Self {\n        Self(PersistentArray::from(vec![-1;\
    \ n]))\n    }\n\n    pub fn merge(&self, x: usize, y: usize) -> Self {\n     \
    \   let (mut x, mut xs) = self.leader_with_size(x);\n        let (mut y, mut ys)\
    \ = self.leader_with_size(y);\n        if x == y {\n            return self.clone();\n\
    \        }\n        if xs < ys {\n            swap(&mut x, &mut y);\n        \
    \    swap(&mut xs, &mut ys);\n        }\n        let t = self.0.set(x, -((xs +\
    \ ys) as i32));\n        let t = t.set(y, x as i32);\n        Self(t)\n    }\n\
    \n    pub fn leader_with_size(&self, x: usize) -> (usize, usize) {\n        let\
    \ t = *self.0.get(x).unwrap();\n        if t < 0 {\n            (x, -t as usize)\n\
    \        } else {\n            self.leader_with_size(t as usize)\n        }\n\
    \    }\n\n    pub fn leader(&self, x: usize) -> usize {\n        self.leader_with_size(x).0\n\
    \    }\n\n    pub fn size(&self, x: usize) -> usize {\n        self.leader_with_size(x).1\n\
    \    }\n\n    pub fn same(&self, x: usize, y: usize) -> bool {\n        self.leader_with_size(x)\
    \ == self.leader_with_size(y)\n    }\n}\n"
  dependsOn:
  - crates/data-structure/persistent-array/src/lib.rs
  isVerificationFile: false
  path: crates/data-structure/persistent-union-find/src/lib.rs
  requiredBy: []
  timestamp: '2024-04-08 11:43:23+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/persistent_unionfind/src/main.rs
documentation_of: crates/data-structure/persistent-union-find/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/persistent-union-find/src/lib.rs
- /library/crates/data-structure/persistent-union-find/src/lib.rs.html
title: crates/data-structure/persistent-union-find/src/lib.rs
---
