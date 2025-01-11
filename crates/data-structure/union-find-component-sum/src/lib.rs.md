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
  code: "use std::mem::swap;\n\nuse algebraic::Monoid;\n\n/// \u9023\u7D50\u6210\u5206\
    \u306E\u7DCF\u7A4D\u3092\u7BA1\u7406\u3059\u308B UnionFind  \n/// \u53EF\u63DB\
    \u534A\u7FA4\u3092\u6271\u3046\u304C\u3001\u5B9F\u88C5\u306E\u90FD\u5408\u4E0A\
    \ Monoid \u3068\u3057\u3066\u3044\u308B\u3002\n#[derive(Clone)]\npub struct UnionFindComponentSum<M,\
    \ const UNION_BY_SIZE: bool = true>\nwhere\n    M: Monoid,\n    M::S: Clone,\n\
    {\n    par: Vec<i32>,\n    sum: Vec<M::S>,\n    cnt: usize,\n}\n\nimpl<M, const\
    \ UNION_BY_SIZE: bool> UnionFindComponentSum<M, UNION_BY_SIZE>\nwhere\n    M:\
    \ Monoid,\n    M::S: Clone,\n{\n    /// \u9802\u70B9 i \u3092 a[i] \u3067\u521D\
    \u671F\u5316\n    pub fn new(a: &[M::S]) -> Self {\n        let n = a.len();\n\
    \        Self {\n            par: vec![-1; n],\n            sum: a.to_vec(),\n\
    \            cnt: n,\n        }\n    }\n\n    /// \u9802\u70B9\u6570\u3092\u53D6\
    \u5F97\u3059\u308B\u3002\n    pub fn len(&self) -> usize {\n        self.par.len()\n\
    \    }\n\n    /// \u9023\u7D50\u6210\u5206\u306E\u500B\u6570\u3092\u53D6\u5F97\
    \u3059\u308B\u3002\n    pub fn count(&self) -> usize {\n        self.cnt\n   \
    \ }\n\n    /// \u9802\u70B9 x \u3068 y \u3092\u7D50\u5408\u3059\u308B\u3002  \n\
    \    /// \u3059\u3067\u306B\u540C\u3058\u9023\u7D50\u6210\u5206\u306B\u5C5E\u3057\
    \u3066\u3044\u308B\u5834\u5408\u306F false \u3092\u8FD4\u3059\u3002\n    pub fn\
    \ merge(&mut self, x: usize, y: usize) -> bool {\n        let mut x = self.leader(x);\n\
    \        let mut y = self.leader(y);\n        if x == y {\n            return\
    \ false;\n        }\n        self.cnt -= 1;\n        if UNION_BY_SIZE && -self.par[x]\
    \ < -self.par[y] {\n            swap(&mut x, &mut y);\n        }\n        self.par[x]\
    \ += self.par[y];\n        self.sum[x] = M::op(&self.sum[x], &self.sum[y]);\n\
    \        self.par[y] = x as i32;\n        true\n    }\n\n    /// \u9802\u70B9\
    \ x \u304C\u5C5E\u3059\u308B\u9023\u7D50\u6210\u5206\u306E\u30EA\u30FC\u30C0\u30FC\
    \u3092\u53D6\u5F97\u3059\u308B\u3002\n    pub fn leader(&mut self, x: usize) ->\
    \ usize {\n        let mut v = x;\n        while self.par[v] >= 0 {\n        \
    \    v = self.par[v] as usize;\n        }\n        let mut u = x;\n        while\
    \ self.par[u] >= 0 {\n            let t = self.par[u] as usize;\n            self.par[u]\
    \ = v as i32;\n            u = t;\n        }\n        u\n    }\n\n    /// \u9802\
    \u70B9 x \u3068 y \u304C\u540C\u3058\u9023\u7D50\u6210\u5206\u306B\u5C5E\u3057\
    \u3066\u3044\u308B\u304B\u3092\u5224\u5B9A\u3059\u308B\u3002\n    pub fn same(&mut\
    \ self, x: usize, y: usize) -> bool {\n        self.leader(x) == self.leader(y)\n\
    \    }\n\n    /// \u9802\u70B9 x \u304C\u5C5E\u3059\u308B\u9023\u7D50\u6210\u5206\
    \u306E\u30B5\u30A4\u30BA\u3092\u53D6\u5F97\u3059\u308B\u3002\n    pub fn size(&mut\
    \ self, x: usize) -> usize {\n        let x = self.leader(x);\n        -self.par[x]\
    \ as usize\n    }\n\n    /// \u9023\u7D50\u6210\u5206\u3092\u53D6\u5F97\u3059\u308B\
    \u3002\n    pub fn groups(&mut self) -> Vec<Vec<usize>> {\n        let mut res\
    \ = vec![vec![]; self.len()];\n        for x in 0..self.len() {\n            res[self.leader(x)].push(x);\n\
    \        }\n        res.into_iter().filter(|g| g.len() > 0).collect()\n    }\n\
    \n    /// \u9802\u70B9 x \u304C\u5C5E\u3059\u308B\u9023\u7D50\u6210\u5206\u306E\
    \u7DCF\u7A4D\u3092\u53D6\u5F97\u3059\u308B\u3002\n    pub fn sum(&mut self, x:\
    \ usize) -> M::S {\n        let x = self.leader(x);\n        self.sum[x].clone()\n\
    \    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  isVerificationFile: false
  path: crates/data-structure/union-find-component-sum/src/lib.rs
  requiredBy:
  - crates/tree/zero-one-on-tree/src/lib.rs
  timestamp: '2025-01-11 09:03:35+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data-structure/union-find-component-sum/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/union-find-component-sum/src/lib.rs
- /library/crates/data-structure/union-find-component-sum/src/lib.rs.html
title: crates/data-structure/union-find-component-sum/src/lib.rs
---
