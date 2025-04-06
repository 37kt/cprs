---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/data_structure/persistent_array/src/lib.rs
    title: crates/data_structure/persistent_array/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/persistent_unionfind/src/main.rs
    title: verify/library_checker/data_structure/persistent_unionfind/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use persistent_array::PersistentArray;\n\nconst M: usize = 8;\n\n#[derive(Clone,\
    \ Copy)]\npub struct PersistentUnionFind(PersistentArray<i32, M>);\n\nimpl PersistentUnionFind\
    \ {\n    pub fn new(n: usize) -> Self {\n        Self(PersistentArray::from_fn(n,\
    \ |_| -1))\n    }\n\n    pub fn root_and_size(&self, mut x: usize) -> (usize,\
    \ usize) {\n        let mut v = self.0[x];\n        while v >= 0 {\n         \
    \   x = v as usize;\n            v = self.0[x];\n        }\n        (x, -v as\
    \ usize)\n    }\n\n    pub fn root(&self, x: usize) -> usize {\n        self.root_and_size(x).0\n\
    \    }\n\n    pub fn size(&self, x: usize) -> usize {\n        self.root_and_size(x).1\n\
    \    }\n\n    pub fn same(&self, x: usize, y: usize) -> bool {\n        self.root(x)\
    \ == self.root(y)\n    }\n\n    pub fn merge(&self, x: usize, y: usize) -> (Self,\
    \ bool) {\n        let (mut x, mut xs) = self.root_and_size(x);\n        let (mut\
    \ y, mut ys) = self.root_and_size(y);\n        if x == y {\n            return\
    \ (*self, false);\n        }\n        if xs < ys {\n            std::mem::swap(&mut\
    \ x, &mut y);\n            std::mem::swap(&mut xs, &mut ys);\n        }\n    \
    \    let new_arr = self.0.set(x, -((xs + ys) as i32)).set(y, x as i32);\n    \
    \    (PersistentUnionFind(new_arr), true)\n    }\n}\n"
  dependsOn:
  - crates/data_structure/persistent_array/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/union_find/persistent_union_find/src/lib.rs
  requiredBy: []
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/persistent_unionfind/src/main.rs
documentation_of: crates/data_structure/union_find/persistent_union_find/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/union_find/persistent_union_find/src/lib.rs
- /library/crates/data_structure/union_find/persistent_union_find/src/lib.rs.html
title: crates/data_structure/union_find/persistent_union_find/src/lib.rs
---
