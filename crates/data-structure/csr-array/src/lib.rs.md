---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/strongly-connected-components/src/lib.rs
    title: crates/graph/strongly-connected-components/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/two-edge-connected-components/src/lib.rs
    title: crates/graph/two-edge-connected-components/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy-light-decomposition/src/lib.rs
    title: crates/tree/heavy-light-decomposition/src/lib.rs
  - icon: ':warning:'
    path: crates/tree/static-top-tree/src/lib.rs
    title: crates/tree/static-top-tree/src/lib.rs
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
  code: "use std::ops::Index;\n\n/// CSR \u5F62\u5F0F\u306E\u4E8C\u6B21\u5143\u914D\
    \u5217\n#[derive(Clone)]\npub struct CSRArray<T> {\n    pos: Vec<usize>,\n   \
    \ data: Vec<T>,\n}\n\nimpl<T: Clone> CSRArray<T> {\n    /// a \u306E\u5404\u8981\
    \u7D20 (i, x) \u306B\u3064\u3044\u3066\u3001 i \u756A\u76EE\u306E\u914D\u5217\u306B\
    \ x \u304C\u683C\u7D0D\u3055\u308C\u308B\n    pub fn new(n: usize, a: &[(usize,\
    \ T)]) -> Self {\n        let mut pos = vec![0; n + 1];\n        for &(i, _) in\
    \ a {\n            pos[i] += 1;\n        }\n        for i in 0..n {\n        \
    \    pos[i + 1] += pos[i];\n        }\n        let mut ord = vec![0; a.len()];\n\
    \        for j in (0..a.len()).rev() {\n            let (i, _) = a[j];\n     \
    \       pos[i] -= 1;\n            ord[pos[i]] = j;\n        }\n        let data\
    \ = ord.into_iter().map(|i| a[i].1.clone()).collect();\n        Self { pos, data\
    \ }\n    }\n}\n\nimpl<T> CSRArray<T> {\n    pub fn len(&self) -> usize {\n   \
    \     self.pos.len() - 1\n    }\n\n    pub fn iter(&self) -> impl Iterator<Item\
    \ = &[T]> {\n        (0..self.len()).map(|i| &self[i])\n    }\n}\n\nimpl<T> Index<usize>\
    \ for CSRArray<T> {\n    type Output = [T];\n    fn index(&self, i: usize) ->\
    \ &Self::Output {\n        let start = self.pos[i];\n        let end = self.pos[i\
    \ + 1];\n        &self.data[start..end]\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/data-structure/csr-array/src/lib.rs
  requiredBy:
  - crates/graph/two-edge-connected-components/src/lib.rs
  - crates/graph/graph/src/lib.rs
  - crates/graph/strongly-connected-components/src/lib.rs
  - crates/tree/static-top-tree/src/lib.rs
  - crates/tree/heavy-light-decomposition/src/lib.rs
  timestamp: '2025-01-14 05:25:42+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data-structure/csr-array/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/csr-array/src/lib.rs
- /library/crates/data-structure/csr-array/src/lib.rs.html
title: crates/data-structure/csr-array/src/lib.rs
---
