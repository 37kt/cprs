---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/unionfind_with_potential_non_commutative_group/src/main.rs
    title: verify/unionfind_with_potential_non_commutative_group/src/main.rs
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
  code: "use std::{\n    mem::swap,\n    ops::{Add, Neg},\n};\n\n/// \u30DD\u30C6\u30F3\
    \u30B7\u30E3\u30EB\u4ED8\u304D Union-Find\n#[derive(Clone)]\npub struct PotentializedUnionFind<T,\
    \ const UNION_BY_SIZE: bool = true>\nwhere\n    T: Clone + Default + Add<Output\
    \ = T> + Neg<Output = T> + Eq,\n{\n    par: Vec<i32>,\n    pot: Vec<T>,\n}\n\n\
    impl<T, const UNION_BY_SIZE: bool> PotentializedUnionFind<T, UNION_BY_SIZE>\n\
    where\n    T: Clone + Default + Add<Output = T> + Neg<Output = T> + Eq,\n{\n \
    \   /// \u30DD\u30C6\u30F3\u30B7\u30E3\u30EB\u306E\u5DEE\u306E\u60C5\u5831\u3092\
    \u6301\u305F\u306A\u3044 Union-Find \u3092\u69CB\u7BC9\u3059\u308B\u3002\n   \
    \ pub fn new(n: usize) -> Self {\n        Self {\n            par: vec![-1; n],\n\
    \            pot: vec![T::default(); n],\n        }\n    }\n\n    /// \u8981\u7D20\
    \u6570\u3092\u53D6\u5F97\u3059\u308B\u3002\n    pub fn len(&self) -> usize {\n\
    \        self.par.len()\n    }\n\n    /// P(x) = P(y) + w \u3067\u3042\u308B\u3068\
    \u3044\u3046\u60C5\u5831\u3092\u4E0E\u3048\u308B\u3002\n    ///\n    /// # \u623B\
    \u308A\u5024\n    ///\n    /// - \u6574\u5408\u6027\u304C\u3042\u308B\u304B\n\
    \    pub fn merge(&mut self, x: usize, y: usize, mut w: T) -> bool {\n       \
    \ w = self.potential(x) + -w + -self.potential(y);\n        let mut x = self.leader(x);\n\
    \        let mut y = self.leader(y);\n        if x == y {\n            return\
    \ w == T::default();\n        }\n        if UNION_BY_SIZE && -self.par[x] < -self.par[y]\
    \ {\n            swap(&mut x, &mut y);\n            w = -w;\n        }\n     \
    \   self.par[x] += self.par[y];\n        self.par[y] = x as i32;\n        self.pot[y]\
    \ = w;\n        true\n    }\n\n    /// \u9802\u70B9 x \u304B\u3089\u8FBF\u308C\
    \u308B\u8981\u7D20\u304B\u3089\u306A\u308B\u96C6\u5408\u306E\u30EA\u30FC\u30C0\
    \u30FC\u3092\u53D6\u5F97\u3059\u308B\u3002\n    pub fn leader(&mut self, x: usize)\
    \ -> usize {\n        if self.par[x] < 0 {\n            x\n        } else {\n\
    \            let r = self.leader(self.par[x] as usize);\n            self.pot[x]\
    \ = self.pot[self.par[x] as usize].clone() + self.pot[x].clone();\n          \
    \  self.par[x] = r as i32;\n            r\n        }\n    }\n\n    /// \u9802\u70B9\
    \ x \u304B\u3089\u9802\u70B9 y \u307E\u3067\u8FBF\u308C\u308B\u304B\u3092\u5224\
    \u5B9A\u3059\u308B\u3002\n    pub fn same(&mut self, x: usize, y: usize) -> bool\
    \ {\n        self.leader(x) == self.leader(y)\n    }\n\n    /// \u9802\u70B9 x\
    \ \u304B\u3089\u8FBF\u308C\u308B\u8981\u7D20\u304B\u3089\u306A\u308B\u96C6\u5408\
    \u306E\u8981\u7D20\u6570\u3092\u53D6\u5F97\u3059\u308B\u3002\n    pub fn size(&mut\
    \ self, x: usize) -> usize {\n        let x = self.leader(x);\n        -self.par[x]\
    \ as usize\n    }\n\n    /// \u9802\u70B9 x \u3068\u9802\u70B9 leader(x) \u306E\
    \u30DD\u30C6\u30F3\u30B7\u30E3\u30EB\u306E\u5DEE\u3092\u53D6\u5F97\u3059\u308B\
    \u3002  \n    /// -P(leader(x)) + P(x)\n    pub fn potential(&mut self, x: usize)\
    \ -> T {\n        self.leader(x);\n        self.pot[x].clone()\n    }\n\n    ///\
    \ \u9802\u70B9 x \u3068\u9802\u70B9 y \u306E\u30DD\u30C6\u30F3\u30B7\u30E3\u30EB\
    \u306E\u5DEE\u3092\u53D6\u5F97\u3059\u308B\u3002  \n    /// -P(y) + P(x)\n   \
    \ pub fn diff(&mut self, x: usize, y: usize) -> Option<T> {\n        if self.same(x,\
    \ y) {\n            Some(-self.potential(y) + self.potential(x))\n        } else\
    \ {\n            None\n        }\n    }\n\n    /// \u30B0\u30EB\u30FC\u30D7\u3092\
    \u53D6\u5F97\u3059\u308B\u3002\n    pub fn groups(&mut self) -> Vec<Vec<usize>>\
    \ {\n        let mut res = vec![vec![]; self.len()];\n        for x in 0..self.len()\
    \ {\n            res[self.leader(x)].push(x);\n        }\n        res.into_iter().filter(|g|\
    \ g.len() > 0).collect()\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/data-structure/potentialized-union-find/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-25 10:01:29+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/unionfind_with_potential_non_commutative_group/src/main.rs
documentation_of: crates/data-structure/potentialized-union-find/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/potentialized-union-find/src/lib.rs
- /library/crates/data-structure/potentialized-union-find/src/lib.rs.html
title: crates/data-structure/potentialized-union-find/src/lib.rs
---
