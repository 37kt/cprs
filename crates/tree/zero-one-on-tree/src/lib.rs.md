---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  - icon: ':warning:'
    path: crates/data-structure/union-find-component-sum/src/lib.rs
    title: crates/data-structure/union-find-component-sum/src/lib.rs
  - icon: ':question:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  _extendedRequiredBy: []
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
  code: "use std::{\n    cmp::Ordering,\n    collections::BinaryHeap,\n    ops::{Add,\
    \ AddAssign},\n};\n\nuse algebraic::{algebra, monoid};\nuse graph::UndirectedGraph;\n\
    use union_find_component_sum::UnionFindComponentSum;\n\nalgebra!(ZeroOneMonoid,\
    \ ZeroOne);\nmonoid!(ZeroOneMonoid, ZeroOne::new(0, 0), |a, b| a + b);\n\n///\
    \ 01 on Tree\npub fn zero_one_on_tree(g: &UndirectedGraph<ZeroOne, ()>) -> usize\
    \ {\n    let n = g.len();\n    let mut par = vec![!0; n];\n    dfs(0, g, &mut\
    \ par);\n    let mut res = 0;\n    let a: Vec<_> = (0..n).map(|i| *g.vertex(i)).collect();\n\
    \    let mut uf = UnionFindComponentSum::<ZeroOneMonoid, false>::new(&a);\n  \
    \  let mut pq = BinaryHeap::new();\n    for v in 1..n {\n        pq.push((a[v],\
    \ v));\n    }\n    while let Some((x, v)) = pq.pop() {\n        if uf.sum(v) !=\
    \ x {\n            continue;\n        }\n        let p = uf.leader(par[v]);\n\
    \        res += uf.sum(p).s1 * uf.sum(v).s0;\n        uf.merge(p, v);\n      \
    \  if !uf.same(0, p) {\n            pq.push((uf.sum(p), p));\n        }\n    }\n\
    \    res\n}\n\nfn dfs(v: usize, g: &UndirectedGraph<ZeroOne, ()>, p: &mut [usize])\
    \ {\n    for &(u, _) in &g[v] {\n        if u == p[v] {\n            continue;\n\
    \        }\n        p[u] = v;\n        dfs(u, g, p);\n    }\n}\n\n#[derive(Clone,\
    \ Copy, PartialEq, Eq)]\npub struct ZeroOne {\n    s0: usize,\n    s1: usize,\n\
    }\n\nimpl ZeroOne {\n    pub fn new(s0: usize, s1: usize) -> Self {\n        Self\
    \ { s0, s1 }\n    }\n}\n\nimpl Ord for ZeroOne {\n    fn cmp(&self, other: &Self)\
    \ -> Ordering {\n        (self.s0 * other.s1).cmp(&(other.s0 * self.s1))\n   \
    \ }\n}\n\nimpl PartialOrd for ZeroOne {\n    fn partial_cmp(&self, other: &Self)\
    \ -> Option<Ordering> {\n        Some(self.cmp(other))\n    }\n}\n\nimpl Add for\
    \ ZeroOne {\n    type Output = Self;\n\n    fn add(self, other: Self) -> Self\
    \ {\n        Self {\n            s0: self.s0 + other.s0,\n            s1: self.s1\
    \ + other.s1,\n        }\n    }\n}\n\nimpl Add for &ZeroOne {\n    type Output\
    \ = ZeroOne;\n\n    fn add(self, other: Self) -> ZeroOne {\n        ZeroOne {\n\
    \            s0: self.s0 + other.s0,\n            s1: self.s1 + other.s1,\n  \
    \      }\n    }\n}\n\nimpl Add<&ZeroOne> for ZeroOne {\n    type Output = ZeroOne;\n\
    \n    fn add(self, other: &Self) -> ZeroOne {\n        ZeroOne {\n           \
    \ s0: self.s0 + other.s0,\n            s1: self.s1 + other.s1,\n        }\n  \
    \  }\n}\n\nimpl Add<ZeroOne> for &ZeroOne {\n    type Output = ZeroOne;\n\n  \
    \  fn add(self, other: ZeroOne) -> ZeroOne {\n        ZeroOne {\n            s0:\
    \ self.s0 + other.s0,\n            s1: self.s1 + other.s1,\n        }\n    }\n\
    }\n\nimpl AddAssign for ZeroOne {\n    fn add_assign(&mut self, other: Self) {\n\
    \        self.s0 += other.s0;\n        self.s1 += other.s1;\n    }\n}\n\nimpl\
    \ AddAssign<&ZeroOne> for ZeroOne {\n    fn add_assign(&mut self, other: &Self)\
    \ {\n        self.s0 += other.s0;\n        self.s1 += other.s1;\n    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/data-structure/union-find-component-sum/src/lib.rs
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/tree/zero-one-on-tree/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-14 05:25:42+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/tree/zero-one-on-tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/zero-one-on-tree/src/lib.rs
- /library/crates/tree/zero-one-on-tree/src/lib.rs.html
title: crates/tree/zero-one-on-tree/src/lib.rs
---
