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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::mem::swap;\n\nuse persistent_array::PersistentArray;\n\n/// \u5B8C\
    \u5168\u6C38\u7D9A Union-Find\n#[derive(Clone)]\npub struct PersistentUnionFind(PersistentArray<i32,\
    \ 8>);\n\nimpl PersistentUnionFind {\n    /// \u8FBA\u306E\u306A\u3044 Union-Find\
    \ \u3092\u69CB\u7BC9\u3059\u308B\u3002\n    pub fn new(n: usize) -> Self {\n \
    \       Self(PersistentArray::from(vec![-1; n]))\n    }\n\n    /// \u9802\u70B9\
    \ x \u304C\u542B\u307E\u308C\u308B\u96C6\u5408\u3068\u9802\u70B9 y \u304C\u542B\
    \u307E\u308C\u308B\u96C6\u5408\u3092\u4F75\u5408\u3059\u308B\u3002\n    ///\n\
    \    /// # \u623B\u308A\u5024\n    ///\n    /// \u4F75\u5408\u5F8C\u306E Union-Find\n\
    \    pub fn merge(&self, x: usize, y: usize) -> Self {\n        let (mut x, mut\
    \ xs) = self.leader_with_size(x);\n        let (mut y, mut ys) = self.leader_with_size(y);\n\
    \        if x == y {\n            return self.clone();\n        }\n        if\
    \ xs < ys {\n            swap(&mut x, &mut y);\n            swap(&mut xs, &mut\
    \ ys);\n        }\n        let t = self.0.set(x, -((xs + ys) as i32));\n     \
    \   let t = t.set(y, x as i32);\n        Self(t)\n    }\n\n    /// \u9802\u70B9\
    \ x \u304C\u542B\u307E\u308C\u308B\u96C6\u5408\u306E\u30EA\u30FC\u30C0\u30FC\u3068\
    \u8981\u7D20\u6570\u3092\u53D6\u5F97\u3059\u308B\u3002\n    ///\n    /// # \u623B\
    \u308A\u5024\n    ///\n    /// - \u30EA\u30FC\u30C0\u30FC\n    /// - \u8981\u7D20\
    \u6570\n    pub fn leader_with_size(&self, x: usize) -> (usize, usize) {\n   \
    \     let t = *self.0.get(x).unwrap();\n        if t < 0 {\n            (x, -t\
    \ as usize)\n        } else {\n            self.leader_with_size(t as usize)\n\
    \        }\n    }\n\n    /// \u9802\u70B9 x \u304C\u542B\u307E\u308C\u308B\u96C6\
    \u5408\u306E\u30EA\u30FC\u30C0\u30FC\u3092\u53D6\u5F97\u3059\u308B\u3002\n   \
    \ pub fn leader(&self, x: usize) -> usize {\n        self.leader_with_size(x).0\n\
    \    }\n\n    /// \u9802\u70B9 x \u304C\u542B\u307E\u308C\u308B\u96C6\u5408\u306E\
    \u8981\u7D20\u6570\u3092\u53D6\u5F97\u3059\u308B\u3002\n    pub fn size(&self,\
    \ x: usize) -> usize {\n        self.leader_with_size(x).1\n    }\n\n    /// \u9802\
    \u70B9 x \u3068\u9802\u70B9 y \u304C\u540C\u3058\u96C6\u5408\u306B\u542B\u307E\
    \u308C\u308B\u304B\u3092\u5224\u5B9A\u3059\u308B\u3002\n    pub fn same(&self,\
    \ x: usize, y: usize) -> bool {\n        self.leader_with_size(x) == self.leader_with_size(y)\n\
    \    }\n}\n"
  dependsOn:
  - crates/data-structure/persistent-array/src/lib.rs
  isVerificationFile: false
  path: crates/data-structure/persistent-union-find/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-25 10:01:29+00:00'
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
