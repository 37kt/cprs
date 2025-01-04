---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/dynamic_graph_vertex_add_component_sum/src/main.rs
    title: verify/dynamic_graph_vertex_add_component_sum/src/main.rs
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
  code: "use std::mem::swap;\n\nuse algebraic::Monoid;\n\n/// \u30ED\u30FC\u30EB\u30D0\
    \u30C3\u30AF\u53EF\u80FD\u306A\u3001\u9023\u7D50\u6210\u5206\u306E\u7DCF\u7A4D\
    \u3092\u7BA1\u7406\u3059\u308BUnionFind  \n/// \u53EF\u63DB\u534A\u7FA4\u3092\u6271\
    \u3046\u304C\u3001\u5B9F\u88C5\u306E\u90FD\u5408\u4E0A Monoid \u3068\u3057\u3066\
    \u3044\u308B\u3002\n#[derive(Clone)]\npub struct RollbackUnionFindComponentSum<M,\
    \ const UNION_BY_SIZE: bool = true>\nwhere\n    M: Monoid,\n    M::S: Clone,\n\
    {\n    par: Vec<i32>,\n    sum: Vec<M::S>,\n    his: Vec<(usize, i32, M::S)>,\n\
    \    cnt: usize,\n}\n\nimpl<M, const UNION_BY_SIZE: bool> RollbackUnionFindComponentSum<M,\
    \ UNION_BY_SIZE>\nwhere\n    M: Monoid,\n    M::S: Clone,\n{\n    /// \u9802\u70B9\
    \ i \u3092 a[i] \u3067\u521D\u671F\u5316\n    pub fn new(a: &[M::S]) -> Self {\n\
    \        let n = a.len();\n        Self {\n            par: vec![-1; n],\n   \
    \         sum: a.to_vec(),\n            his: vec![],\n            cnt: n,\n  \
    \      }\n    }\n\n    /// \u9802\u70B9\u6570\u3092\u53D6\u5F97\n    pub fn len(&self)\
    \ -> usize {\n        self.par.len()\n    }\n\n    /// \u9023\u7D50\u6210\u5206\
    \u306E\u6570\u3092\u53D6\u5F97\n    pub fn count(&self) -> usize {\n        self.cnt\n\
    \    }\n\n    /// x \u3068 y \u3092\u30DE\u30FC\u30B8\u3059\u308B\u3002\n    ///\
    \ x \u3068 y \u304C\u540C\u3058\u9023\u7D50\u6210\u5206\u306B\u5C5E\u3057\u3066\
    \u3044\u306A\u3044\u5834\u5408 true \u3092\u8FD4\u3059\u3002\n    pub fn merge(&mut\
    \ self, x: usize, y: usize) -> bool {\n        let mut x = self.leader(x);\n \
    \       let mut y = self.leader(y);\n        self.his.push((x, self.par[x], self.sum[x].clone()));\n\
    \        self.his.push((y, self.par[y], self.sum[y].clone()));\n        if x ==\
    \ y {\n            return false;\n        }\n        self.cnt -= 1;\n        if\
    \ UNION_BY_SIZE && -self.par[x] < -self.par[y] {\n            swap(&mut x, &mut\
    \ y);\n        }\n        self.par[x] += self.par[y];\n        self.sum[x] = M::op(&self.sum[x],\
    \ &self.sum[y]);\n        self.par[y] = x as i32;\n        true\n    }\n\n   \
    \ /// x \u304C\u542B\u307E\u308C\u308B\u9023\u7D50\u6210\u5206\u306E\u30EA\u30FC\
    \u30C0\u30FC\u3092\u53D6\u5F97\n    pub fn leader(&self, mut x: usize) -> usize\
    \ {\n        while self.par[x] >= 0 {\n            x = self.par[x] as usize;\n\
    \        }\n        x\n    }\n\n    /// x \u3068 y \u304C\u540C\u3058\u9023\u7D50\
    \u6210\u5206\u306B\u5C5E\u3057\u3066\u3044\u308B\u304B\u3092\u5224\u5B9A\n   \
    \ pub fn same(&self, x: usize, y: usize) -> bool {\n        self.leader(x) ==\
    \ self.leader(y)\n    }\n\n    /// x \u304C\u542B\u307E\u308C\u308B\u9023\u7D50\
    \u6210\u5206\u306E\u9802\u70B9\u6570\u3092\u53D6\u5F97\n    pub fn size(&self,\
    \ x: usize) -> usize {\n        -self.par[self.leader(x)] as usize\n    }\n\n\
    \    /// \u76F4\u524D\u306E merge \u3092\u30ED\u30FC\u30EB\u30D0\u30C3\u30AF\u3059\
    \u308B\n    pub fn undo(&mut self) {\n        for _ in 0..2 {\n            let\
    \ (x, par, sum) = self.his.pop().unwrap();\n            if self.par[x] >= 0 &&\
    \ par < 0 {\n                self.cnt += 1;\n            }\n            self.par[x]\
    \ = par;\n            self.sum[x] = sum;\n        }\n    }\n\n    /// \u9023\u7D50\
    \u6210\u5206\u306E\u30EA\u30B9\u30C8\u3092\u53D6\u5F97\n    pub fn groups(&self)\
    \ -> Vec<Vec<usize>> {\n        let mut res = vec![vec![]; self.len()];\n    \
    \    for x in 0..self.len() {\n            res[self.leader(x)].push(x);\n    \
    \    }\n        res.into_iter().filter(|g| g.len() > 0).collect()\n    }\n\n \
    \   /// x \u304C\u542B\u307E\u308C\u308B\u9023\u7D50\u6210\u5206\u306E\u7DCF\u7A4D\
    \u3092\u53D6\u5F97\n    pub fn sum(&self, x: usize) -> M::S {\n        let x =\
    \ self.leader(x);\n        self.sum[x].clone()\n    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  isVerificationFile: false
  path: crates/data-structure/rollback-union-find-component-sum/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-04 02:49:00+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/dynamic_graph_vertex_add_component_sum/src/main.rs
documentation_of: crates/data-structure/rollback-union-find-component-sum/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/rollback-union-find-component-sum/src/lib.rs
- /library/crates/data-structure/rollback-union-find-component-sum/src/lib.rs.html
title: crates/data-structure/rollback-union-find-component-sum/src/lib.rs
---
