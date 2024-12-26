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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{cell::RefCell, mem::swap};\n\n/// Union Find\n#[derive(Clone)]\n\
    pub struct UnionFind<const UNION_BY_SIZE: bool> {\n    par: RefCell<Vec<i32>>,\n\
    \    cnt: usize,\n}\n\nimpl<const UNION_BY_SIZE: bool> UnionFind<UNION_BY_SIZE>\
    \ {\n    /// \u9802\u70B9 n \u500B\u3068 0 \u672C\u306E\u8FBA\u3067\u521D\u671F\
    \u5316\u3059\u308B\u3002\n    pub fn new(n: usize) -> Self {\n        Self {\n\
    \            par: RefCell::new(vec![-1; n]),\n            cnt: n,\n        }\n\
    \    }\n\n    /// \u9802\u70B9\u6570\u3092\u53D6\u5F97\u3059\u308B\u3002\n   \
    \ pub fn len(&self) -> usize {\n        self.par.borrow().len()\n    }\n\n   \
    \ /// \u9802\u70B9 x \u3068 y \u3092\u7D50\u5408\u3059\u308B\u3002  \n    ///\
    \ \u3059\u3067\u306B\u540C\u3058\u30B0\u30EB\u30FC\u30D7\u306B\u5C5E\u3057\u3066\
    \u3044\u308B\u5834\u5408\u306F false \u3092\u8FD4\u3059\u3002\n    pub fn merge(&mut\
    \ self, x: usize, y: usize) -> bool {\n        let mut x = self.leader(x);\n \
    \       let mut y = self.leader(y);\n        if x == y {\n            return false;\n\
    \        }\n        self.cnt -= 1;\n        let mut par = self.par.borrow_mut();\n\
    \        if UNION_BY_SIZE && -par[x] < -par[y] {\n            swap(&mut x, &mut\
    \ y);\n        }\n        par[x] += par[y];\n        par[y] = x as i32;\n    \
    \    true\n    }\n\n    /// \u9802\u70B9 x \u304C\u5C5E\u3059\u308B\u9023\u7D50\
    \u6210\u5206\u306E\u30EA\u30FC\u30C0\u30FC\u3092\u53D6\u5F97\u3059\u308B\u3002\
    \n    pub fn leader(&self, x: usize) -> usize {\n        let mut v = x;\n    \
    \    let mut par = self.par.borrow_mut();\n        while par[v] >= 0 {\n     \
    \       v = par[v] as usize;\n        }\n        let mut u = x;\n        while\
    \ par[u] >= 0 {\n            let t = par[u] as usize;\n            par[u] = v\
    \ as i32;\n            u = t;\n        }\n        u\n    }\n\n    /// \u9802\u70B9\
    \ x \u3068 y \u304C\u540C\u3058\u9023\u7D50\u6210\u5206\u306B\u5C5E\u3057\u3066\
    \u3044\u308B\u304B\u3092\u5224\u5B9A\u3059\u308B\u3002\n    pub fn same(&self,\
    \ x: usize, y: usize) -> bool {\n        self.leader(x) == self.leader(y)\n  \
    \  }\n\n    /// \u9802\u70B9 x \u304C\u5C5E\u3059\u308B\u9023\u7D50\u6210\u5206\
    \u306E\u30B5\u30A4\u30BA\u3092\u53D6\u5F97\u3059\u308B\u3002\n    pub fn size(&self,\
    \ x: usize) -> usize {\n        let x = self.leader(x);\n        -self.par.borrow()[x]\
    \ as usize\n    }\n\n    /// \u9023\u7D50\u6210\u5206\u306E\u500B\u6570\u3092\u53D6\
    \u5F97\u3059\u308B\u3002\n    pub fn count(&self) -> usize {\n        self.cnt\n\
    \    }\n\n    /// \u9023\u7D50\u6210\u5206\u3092\u53D6\u5F97\u3059\u308B\u3002\
    \n    pub fn groups(&self) -> Vec<Vec<usize>> {\n        let mut res = vec![vec![];\
    \ self.len()];\n        for x in 0..self.len() {\n            res[self.leader(x)].push(x);\n\
    \        }\n        res.into_iter().filter(|g| g.len() > 0).collect()\n    }\n\
    }\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/data-structure/union-find/src/lib.rs
  requiredBy:
  - crates/data-structure/range-union-find/src/lib.rs
  timestamp: '2024-12-26 06:54:01+00:00'
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
